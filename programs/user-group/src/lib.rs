mod account_infos;
mod errors;
mod events;
mod states;
mod utils;

mod prelude {
    pub use anchor_lang::prelude::*;
    pub use anchor_spl::{
        associated_token::{self, AssociatedToken},
        token::{self, Burn, Mint, Token, TokenAccount, Transfer},
    };

    pub use borsh::{BorshDeserialize, BorshSerialize};

    pub use crate::account_infos::*;
    pub use crate::errors::*;
    pub use crate::events::*;
    pub use crate::states::*;
}
use prelude::*;
use solana_security_txt::security_txt;
use utils::*;

security_txt! {
    // required fields
    name: "User Group",
    project_url: "",
    contacts: "",
    policy: ""
}

declare_id!("9fAvfoKEWUUSRmDvfeVoZ1HXKGDnkUB8rguYUMeUcs6W");

#[program]
pub mod user_group {
    use anchor_lang::solana_program::program::invoke_signed_unchecked;

    use super::*;

    // admin
    pub fn initialize(ctx: Context<Initialize>, seed: u8) -> Result<()> {
        let admin = &mut ctx.accounts.admin;
        admin.seed = seed;
        admin.current = 0;
        admin.initialized = true;
        admin.groups = 0;
        admin.token_mint = ctx.accounts.mint.key();
        admin.administrators = [Pubkey::default(); 10];
        admin.add_administrator(ctx.accounts.authority.key);

        msg!("Initialized Administrator Account");
        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.admin, ctx.accounts.authority.key))]
    #[access_control(is_not_admin(&ctx.accounts.admin, ctx.accounts.user.key))]
    pub fn add_admin(ctx: Context<AddAdmin>) -> Result<()> {
        let admin = &mut ctx.accounts.admin;
        admin.add_administrator(ctx.accounts.user.key);

        msg!("Succeeded add admin: {}", ctx.accounts.user.key.to_string());
        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.admin, ctx.accounts.authority.key))]
    #[access_control(is_admin(&ctx.accounts.admin, ctx.accounts.user.key))]
    pub fn remove_admin(ctx: Context<RemoveAdmin>) -> Result<()> {
        let admin = &mut ctx.accounts.admin;
        admin.remove_administrator(ctx.accounts.user.key);

        msg!(
            "Succeeded remove admin: {}",
            ctx.accounts.user.key.to_string()
        );
        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.admin, ctx.accounts.authority.key))]
    pub fn reset_group(ctx: Context<ResetGroup>) -> Result<()> {
        let group = &mut ctx.accounts.group.load_mut()?;

        group.electing = false;
        group.update = false;

        msg!("Succeeded reset group status");
        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.admin, ctx.accounts.authority.key))]
    pub fn create_group(ctx: Context<CreateGroup>, group_seed: u8, max_manager: u32) -> Result<()> {
        let group = &mut ctx.accounts.group.load_init()?;
        group.seed = group_seed;
        group.max_manager = max_manager;
        group.index = ctx.accounts.admin.groups;
        group.sponsor = ctx.accounts.sponsor.key().clone();
        group.admin = ctx.accounts.admin.key().clone();
        group.rate = ParticipateRate::new(100, 100);

        let admin = &mut ctx.accounts.admin;
        admin.groups += 1;

        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.admin, ctx.accounts.authority.key))]
    pub fn freeze_group(ctx: Context<FreezeGroup>) -> Result<()> {
        let group = &mut ctx.accounts.group.load_mut()?;
        group.freeze = true;

        emit!(FreezeGroupEvent {
            authority: ctx.accounts.authority.key().clone(),
            admin: ctx.accounts.admin.key().clone(),
            group: ctx.accounts.group.key().clone(),
            label: "Frozen".to_string(),
        });
        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.admin, ctx.accounts.authority.key))]
    pub fn thaw_group(ctx: Context<ThawGroup>) -> Result<()> {
        let group = &mut ctx.accounts.group.load_mut()?;
        group.freeze = false;

        emit!(ThawGroupEvent {
            authority: ctx.accounts.authority.key().clone(),
            admin: ctx.accounts.admin.key().clone(),
            group: ctx.accounts.group.key().clone(),
            label: "Thawed".to_string(),
        });
        Ok(())
    }

    #[access_control(is_admin(&ctx.accounts.admin, ctx.accounts.authority.key))]
    pub fn upgrade_group(ctx: Context<UpgradeGroup>, rate: ParticipateRate) -> Result<()> {
        let group = &mut ctx.accounts.group.load_mut()?;
        group.rate = rate;

        emit!(UpgradeGroupEvent {
            authority: ctx.accounts.authority.key().clone(),
            admin: ctx.accounts.admin.key().clone(),
            group: ctx.accounts.group.key().clone(),
            rate,
            label: "Upgraded".to_string(),
        });
        Ok(())
    }

    // user
    pub fn join_group(ctx: Context<JoinGroup>) -> Result<()> {
        let group = &mut ctx.accounts.group.load_mut()?;
        let member = &mut ctx.accounts.member;
        member.group = ctx.accounts.group.key().clone();
        member.in_promotion = false;
        member.in_withdraw = false;
        member.owner = ctx.accounts.user.key().clone();
        member.funder = ctx.accounts.authority.key().clone();

        if group.current_member == 0 && group.current_manager == 0 {
            member.position = Position::Manager;
            group.current_manager += 1;
        } else {
            member.position = Position::Member;
            group.current_member += 1;
        }

        emit!(JoinGroupEvent {
            user: ctx.accounts.user.key().clone(),
            group: member.group.clone(),
            member: member.key().clone(),
            funder: ctx.accounts.authority.key().clone(),
            time: ctx.accounts.clock.unix_timestamp,
            label: "JoinGroup".to_string(),
        });

        Ok(())
    }

    pub fn exit_group(ctx: Context<ExitGroup>) -> Result<()> {
        let group = &mut ctx.accounts.group.load_mut()?;
        if ctx.accounts.member.group != ctx.accounts.group.key() {
            return if cfg!(feature = "dev") {
                err!(GroupError::MismatchedGroup)
            } else {
                Err(GroupError::MismatchedGroup.into())
            };
        }
        match ctx.accounts.member.position {
            Position::Member => group.current_member -= 1,
            Position::Manager => group.current_manager -= 1,
        }

        emit!(ExitGroupEvent {
            user: ctx.accounts.authority.key().clone(),
            group: ctx.accounts.group.key().clone(),
            member: ctx.accounts.member.key().clone(),
            funder: ctx.accounts.member.funder.clone(),
            time: ctx.accounts.clock.unix_timestamp,
            label: "ExitGroup".to_string(),
        });
        Ok(())
    }

    #[access_control(is_valid_deadline(&ctx, deadline))]
    #[access_control(is_valid_limit(limit))]
    pub fn submit_proposal<'info>(
        ctx: Context<'_, '_, '_, 'info, SubmitProposal<'info>>,
        prop_type: ProposalType,
        limit: u64,
        deadline: i64,
    ) -> Result<()> {
        let group = &mut ctx.accounts.group.load_mut()?;
        group.proposals += 1;
        let member = &mut ctx.accounts.member;

        let proposal = &mut ctx.accounts.proposal;
        proposal.submitter = ctx.accounts.authority.key().clone();
        proposal.beneficiary = ctx.accounts.beneficiary.key().clone();
        proposal.bene_member = ctx.accounts.bene_member.key().clone();
        proposal.group = ctx.accounts.group.key().clone();
        proposal.positive = 0;
        proposal.negative = 0;
        proposal.limit = limit;
        proposal.deadline = deadline;  // 12 hours later than now
        proposal.revoke_timeout = ctx.accounts.clock.unix_timestamp + 7200;  // 2 hours
        proposal.close_timeout = deadline + 259200;  // unused field, retain for future use
        proposal.proposal_type = prop_type.clone();
        proposal.status = ProposalStatus::Progressing;

        emit!(SubmitProposalEvent {
            ptype: prop_type.into(),
            submitter: proposal.submitter.clone(),
            submitter_member: member.key().clone(),
            beneficiary: proposal.beneficiary.clone(),
            bene_member: proposal.bene_member.clone(),
            group: proposal.group.clone(),
            proposal: proposal.key().clone(),
            deadline: proposal.deadline,
            label: "SubmitProposal".to_string(),
        });

        let bene_member = &mut ctx.accounts.bene_member;
        // authority check
        match proposal.proposal_type {
            ProposalType::Upgrade => {
                if group.current_manager == group.max_manager {
                    return if cfg!(feature = "dev") {
                        err!(GroupError::UpdateFirst)
                    } else {
                        Err(GroupError::UpdateFirst.into())
                    };
                }

                if member.position.is_member() {
                    return if cfg!(feature = "dev") {
                        err!(GroupError::NotPermitted)
                    } else {
                        Err(GroupError::NotPermitted.into())
                    };
                }

                if bene_member.position.is_manager() {
                    return if cfg!(feature = "dev") {
                        err!(GroupError::AlreadyManager)
                    } else {
                        Err(GroupError::AlreadyManager.into())
                    };
                }

                if bene_member.in_promotion {
                    return if cfg!(feature = "dev") {
                        err!(GroupError::AlreadyInProposal)
                    } else {
                        Err(GroupError::AlreadyInProposal.into())
                    };
                }

                bene_member.in_promotion = true;
            }
            ProposalType::Downgrade => {
                if member.position.is_member() {
                    return if cfg!(feature = "dev") {
                        err!(GroupError::NotPermitted)
                    } else {
                        Err(GroupError::NotPermitted.into())
                    };
                }
                if bene_member.position.is_member() {
                    return if cfg!(feature = "dev") {
                        err!(GroupError::AlreadyMember)
                    } else {
                        Err(GroupError::AlreadyMember.into())
                    };
                }

                if bene_member.in_promotion {
                    return if cfg!(feature = "dev") {
                        err!(GroupError::AlreadyInProposal)
                    } else {
                        Err(GroupError::AlreadyInProposal.into())
                    };
                }

                bene_member.in_promotion = true;
            }
            ProposalType::UpdateGroup { max_manager } => {
                if member.position.is_member() {
                    return if cfg!(feature = "dev") {
                        err!(GroupError::NotPermitted)
                    } else {
                        Err(GroupError::NotPermitted.into())
                    };
                }

                if group.current_manager != group.max_manager
                    || max_manager <= group.current_manager
                    || group.update
                {
                    return if cfg!(feature = "dev") {
                        err!(GroupError::CouldNotUpdateGroup)
                    } else {
                        Err(GroupError::CouldNotUpdateGroup.into())
                    };
                }

                group.update = true;
            }
            ProposalType::ReElection => {
                if group.current_manager != 0 {
                    return if cfg!(feature = "dev") {
                        err!(GroupError::ManagerNotZero)
                    } else {
                        Err(GroupError::ManagerNotZero.into())
                    };
                }
                if group.electing {
                    return if cfg!(feature = "dev") {
                        err!(GroupError::LastElectionNotFinished)
                    } else {
                        Err(GroupError::LastElectionNotFinished.into())
                    };
                }
                proposal.revoke_timeout = deadline;
                group.electing = true;
                bene_member.in_promotion = true;
            }
            ProposalType::Withdraw {
                mint,
                receiver: _,
                amount,
            } => {
                if bene_member.in_withdraw {
                    return if cfg!(feature = "dev") {
                        err!(GroupError::LastWithdrawNotFinished)
                    } else {
                        Err(GroupError::LastWithdrawNotFinished.into())
                    };
                }
                let remaining_accounts = ctx.remaining_accounts;
                let account_iter = &mut remaining_accounts.iter();
                let group_vault_token_account_info = next_account_info(account_iter)?;
                if group_vault_token_account_info.lamports() == 0 {
                    return if cfg!(feature = "dev") {
                        err!(GroupError::AccountDoesNotExist)
                    } else {
                        Err(GroupError::AccountDoesNotExist.into())
                    };
                }
                let group_vault_token_account = TokenAccount::try_deserialize(
                    &mut &group_vault_token_account_info.data.borrow()[..],
                )?;
                if group_vault_token_account.mint != mint {
                    return if cfg!(feature = "dev") {
                        err!(GroupError::WrongVaultTokenAccount)
                    } else {
                        Err(GroupError::WrongVaultTokenAccount.into())
                    };
                }
                if group_vault_token_account.amount < amount {
                    return if cfg!(feature = "dev") {
                        err!(GroupError::InsufficientTokenBalance)
                    } else {
                        Err(GroupError::InsufficientTokenBalance.into())
                    };
                }

                bene_member.in_withdraw = true;
            }
        }
        Ok(())
    }

    pub fn update_proposal(ctx: Context<UpdateProposal>, deadline: i64) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        if proposal.submitter != ctx.accounts.authority.key() {
            return if cfg!(feature = "dev") {
                err!(GroupError::OperationUnauthorized)
            } else {
                Err(GroupError::OperationUnauthorized.into())
            };
        }
        match proposal.status {
            ProposalStatus::Progressing => {
                if deadline <= proposal.deadline {
                    return if cfg!(feature = "dev") {
                        err!(GroupError::OnlyAllowedExtend)
                    } else {
                        Err(GroupError::OnlyAllowedExtend.into())
                    };
                }

                proposal.deadline = deadline;
                proposal.status = ProposalStatus::Updated {
                    time: ctx.accounts.clock.unix_timestamp,
                };

                emit!(UpdateProposalEvent {
                    group: ctx.accounts.group.key().clone(),
                    proposal: proposal.key().clone(),
                    submitter: proposal.submitter.clone(),
                    deadline: proposal.deadline,
                    label: "UpdateProposal".to_string(),
                });

                Ok(())
            }
            ProposalStatus::Updated { .. } => {
                if cfg!(feature = "dev") {
                    err!(GroupError::AlreadyUpdated)
                } else {
                    Err(GroupError::AlreadyUpdated.into())
                }
            },
            _ => {
                if cfg!(feature = "dev") {
                    err!(GroupError::AlreadyOutdated)
                } else {
                    Err(GroupError::AlreadyOutdated.into())
                }
            },
        }
    }

    #[access_control(is_not_outdated(&ctx.accounts.proposal, ctx.accounts.clock.unix_timestamp))]
    pub fn revoke_proposal(ctx: Context<RevokeProposal>) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        match proposal.proposal_type {
            ProposalType::Upgrade | ProposalType::Downgrade => {
                let bene_member = &mut ctx.accounts.bene_member;
                bene_member.in_promotion = false;
            }
            ProposalType::Withdraw { mint: _, receiver: _, amount: _ } => {
                let bene_member = &mut ctx.accounts.bene_member;
                bene_member.in_withdraw = false;
            }
            ProposalType::ReElection => {
                let group = &mut ctx.accounts.group.load_mut()?;
                group.electing = false;
            }
            ProposalType::UpdateGroup { max_manager: _ } => {
                let group = &mut ctx.accounts.group.load_mut()?;
                group.update = false;
            }
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
        Ok(())
    }

    #[access_control(is_valid_elapse(&ctx.accounts.proposal, ctx.accounts.clock.unix_timestamp))]
    pub fn sign_proposal<'info>(
        ctx: Context<'_, '_, '_, 'info, SignProposal<'info>>,
        sign: SignType,
    ) -> Result<()> {
        token::burn(
            ctx.accounts.as_token_burn_ctx(),
            ctx.accounts.proposal.limit,
        )?;
        let proposal = &mut ctx.accounts.proposal;
        let signature = &mut ctx.accounts.signature;
        signature.signer = ctx.accounts.authority.key();
        signature.created_at = ctx.accounts.clock.unix_timestamp;
        signature.amount = proposal.limit;
        signature.proposal = proposal.key().clone();
        signature.sign_type = sign.clone();

        match signature.sign_type {
            SignType::Agreed => proposal.positive += 1,
            SignType::Denied => proposal.negative += 1,
        }

        emit!(SignProposalEvent {
            ptype: proposal.proposal_type.clone().into(),
            submitter: proposal.submitter.clone(),
            beneficiary: proposal.beneficiary.clone(),
            group: proposal.group.clone(),
            proposal: proposal.key().clone(),
            stype: sign,
            signer: signature.signer.clone(),
            signature: signature.key().clone(),
            label: "SignProposal".to_string(),
        });

        let hence = ctx.accounts.group.load()?.hence();

        if proposal.participated() >= hence {
            if proposal.positive > proposal.negative {
                proposal.status = ProposalStatus::Passed {
                    time: ctx.accounts.clock.unix_timestamp,
                };
                let group = &mut ctx.accounts.group.load_mut()?;
                let remaining_accounts = ctx.remaining_accounts;

                emit!(ExecuteProposalEvent {
                    ptype: proposal.proposal_type.clone().into(),
                    submitter: proposal.submitter.clone(),
                    beneficiary: proposal.beneficiary.clone(),
                    group: proposal.group.clone(),
                    proposal: proposal.key().clone(),
                    result: ProposalResult::Passed,
                    label: "ExecuteProposal".to_string(),
                });

                match proposal.proposal_type {
                    ProposalType::Upgrade => {
                        {
                            let account_iter = &mut remaining_accounts.iter();
                            let bene_member_info = next_account_info(account_iter)?;
                            let mut bene_member: MemberAccount =
                                AccountDeserialize::try_deserialize(
                                    &mut &bene_member_info.data.borrow()[..],
                                )?;
                            if bene_member.owner != proposal.beneficiary {
                                return if cfg!(feature = "dev") {
                                    err!(GroupError::MismatchedBeneMember)
                                } else {
                                    Err(GroupError::MismatchedBeneMember.into())
                                };
                            }
                            bene_member.position = Position::Manager;
                            bene_member.in_promotion = false;
                            bene_member
                                .try_serialize(&mut &mut bene_member_info.data.borrow_mut()[..])?;
                        }

                        group.current_member -= 1;
                        group.current_manager += 1;
                    }
                    ProposalType::Downgrade => {
                        {
                            let account_iter = &mut remaining_accounts.iter();
                            let bene_member_info = next_account_info(account_iter)?;
                            let mut bene_member: MemberAccount =
                                AccountDeserialize::try_deserialize(
                                    &mut &bene_member_info.data.borrow()[..],
                                )?;
                            if bene_member.owner != proposal.beneficiary {
                                return if cfg!(feature = "dev") {
                                    err!(GroupError::MismatchedBeneMember)
                                } else {
                                    Err(GroupError::MismatchedBeneMember.into())
                                };
                            }
                            bene_member.position = Position::Member;
                            bene_member.in_promotion = false;
                            bene_member
                                .try_serialize(&mut &mut bene_member_info.data.borrow_mut()[..])?;
                        }

                        group.current_manager -= 1;
                        group.current_member += 1;
                    }
                    ProposalType::UpdateGroup { max_manager } => {
                        group.max_manager = max_manager;
                        group.update = false;
                    }
                    ProposalType::ReElection => {
                        {
                            let account_iter = &mut remaining_accounts.iter();
                            let bene_member_info = next_account_info(account_iter)?;
                            let mut bene_member: MemberAccount =
                                AccountDeserialize::try_deserialize(
                                    &mut &bene_member_info.data.borrow()[..],
                                )?;
                            if bene_member.owner != proposal.beneficiary {
                                return if cfg!(feature = "dev") {
                                    err!(GroupError::MismatchedBeneMember)
                                } else {
                                    Err(GroupError::MismatchedBeneMember.into())
                                };
                            }
                            bene_member.position = Position::Manager;
                            bene_member.in_promotion = false;
                            bene_member
                                .try_serialize(&mut &mut bene_member_info.data.borrow_mut()[..])?;
                        }

                        group.current_member -= 1;
                        group.current_manager += 1;
                        group.electing = false;
                    }
                    ProposalType::Withdraw {
                        mint,
                        receiver,
                        amount,
                    } => {
                        let account_iter = &mut remaining_accounts.iter();
                        let bene_member_info = next_account_info(account_iter)?;
                        let group_token_account_info = next_account_info(account_iter)?;
                        let user_token_account_info = next_account_info(account_iter)?;
                        let mut bene_member: MemberAccount = AccountDeserialize::try_deserialize(
                            &mut &bene_member_info.data.borrow()[..],
                        )?;
                        if bene_member.owner != proposal.beneficiary {
                            return if cfg!(feature = "dev") {
                                err!(GroupError::MismatchedBeneMember)
                            } else {
                                Err(GroupError::MismatchedBeneMember.into())
                            };
                        }
                        let group_token = TokenAccount::try_deserialize(
                            &mut &group_token_account_info.data.borrow()[..],
                        )?;
                        if group_token.mint != mint {
                            return if cfg!(feature = "dev") {
                                err!(GroupError::MismatchedToken)
                            } else {
                                Err(GroupError::MismatchedToken.into())
                            };
                        }
                        if group_token.owner != ctx.accounts.group.key() {
                            return if cfg!(feature = "dev") {
                                err!(GroupError::WrongVaultTokenAccount)
                            } else {
                                Err(GroupError::WrongVaultTokenAccount.into())
                            };
                        }
                        if group_token.amount < amount {
                            return if cfg!(feature = "dev") {
                                err!(GroupError::InsufficientTokenBalance)
                            } else {
                                Err(GroupError::InsufficientTokenBalance.into())
                            };
                        }
                        if user_token_account_info.key() != receiver {
                            return if cfg!(feature = "dev") {
                                err!(GroupError::WrongReceiverTokenAccount)
                            } else {
                                Err(GroupError::WrongReceiverTokenAccount.into())
                            };
                        }
                        bene_member.in_withdraw = false;
                        bene_member
                            .try_serialize(&mut &mut bene_member_info.data.borrow_mut()[..])?;

                        let seeds = &[
                            group.admin.as_ref(),
                            &group.index.to_le_bytes(),
                            GroupAccount::SEEDS,
                            &[group.seed],
                        ];
                        let signer = &[&seeds[..]];
                        let ix = spl_token::instruction::transfer(
                            &spl_token::ID,
                            group_token_account_info.key,
                            user_token_account_info.key,
                            &ctx.accounts.group.key(),
                            &[],
                            amount,
                        )?;
                        invoke_signed_unchecked(
                            &ix,
                            &[
                                group_token_account_info.clone(),
                                user_token_account_info.clone(),
                                ctx.accounts.group.to_account_info().clone(),
                            ],
                            signer,
                        )?;
                    }
                }
            } else {
                emit!(ExecuteProposalEvent {
                    ptype: proposal.proposal_type.clone().into(),
                    submitter: proposal.submitter.clone(),
                    beneficiary: proposal.beneficiary.clone(),
                    group: proposal.group.clone(),
                    proposal: proposal.key().clone(),
                    result: ProposalResult::Rejected,
                    label: "ExecuteProposal".to_string(),
                });

                proposal.status = ProposalStatus::Rejected {
                    time: ctx.accounts.clock.unix_timestamp,
                };
                if let ProposalType::ReElection = proposal.proposal_type {
                    let group = &mut ctx.accounts.group.load_mut()?;
                    group.electing = false;
                }
                let remaining_accounts = ctx.remaining_accounts;
                match proposal.proposal_type {
                    ProposalType::Upgrade | ProposalType::Downgrade | ProposalType::ReElection => {
                        let account_iter = &mut remaining_accounts.iter();
                        let bene_member_info = next_account_info(account_iter)?;
                        let mut bene_member: MemberAccount = AccountDeserialize::try_deserialize(
                            &mut &bene_member_info.data.borrow()[..],
                        )?;
                        if bene_member.owner != proposal.beneficiary {
                            return if cfg!(feature = "dev") {
                                err!(GroupError::MismatchedBeneMember)
                            } else {
                                Err(GroupError::MismatchedBeneMember.into())
                            };
                        }
                        bene_member.in_promotion = false;
                        bene_member
                            .try_serialize(&mut &mut bene_member_info.data.borrow_mut()[..])?;
                    }
                    ProposalType::Withdraw { .. } => {
                        let account_iter = &mut remaining_accounts.iter();
                        let bene_member_info = next_account_info(account_iter)?;
                        let mut bene_member: MemberAccount = AccountDeserialize::try_deserialize(
                            &mut &bene_member_info.data.borrow()[..],
                        )?;
                        if bene_member.owner != proposal.beneficiary {
                            return if cfg!(feature = "dev") {
                                err!(GroupError::MismatchedBeneMember)
                            } else {
                                Err(GroupError::MismatchedBeneMember.into())
                            };
                        }
                        bene_member.in_withdraw = false;
                        bene_member
                            .try_serialize(&mut &mut bene_member_info.data.borrow_mut()[..])?;
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub fn close_signature(ctx: Context<CloseSignature>) -> Result<()> {
        if let Ok(proposal) = <ProposalAccount as AnchorDeserialize>::deserialize(
            &mut &ctx.accounts.proposal.try_borrow_data()?[..],
        ) {
            if let ProposalStatus::Progressing | ProposalStatus::Updated { .. } = proposal.status {
                if proposal.deadline > ctx.accounts.clock.unix_timestamp {
                    return if cfg!(feature = "dev") {
                        err!(GroupError::NotAllowedToClose)
                    } else {
                        Err(GroupError::NotAllowedToClose.into())
                    };
                }
            }
        }

        Ok(())
    }

    #[access_control(is_proposal_allow_close(&ctx.accounts.proposal, ctx.accounts.clock.unix_timestamp))]
    pub fn close_proposal(ctx: Context<CloseProposal>) -> Result<()> {
        Ok(())
    }

    pub fn deposit_token(ctx: Context<DepositToken>, amount: u64) -> Result<()> {
        token::transfer(ctx.accounts.as_token_transfer_ctx(), amount)?;

        emit!(DepositTokenEvent {
            user: ctx.accounts.authority.key().clone(),
            group: ctx.accounts.group.key().clone(),
            vault: ctx.accounts.vault.key().clone(),
            mint: ctx.accounts.mint.key().clone(),
            amount,
            label: "DepositToken".to_string(),
        });
        Ok(())
    }

    pub fn reset_member(ctx: Context<ResetMember>) -> Result<()> {
        let member = &mut ctx.accounts.bene_member;
        let proposal = &ctx.accounts.proposal;
        use ProposalType::*;
        match proposal.proposal_type {
            Upgrade | Downgrade | ReElection => {
                member.in_promotion = false;
            }
            Withdraw { .. } => {
                member.in_withdraw = false;
            }
            _ => {}
        }

        msg!("Succeeded reset member status by proposal");
        Ok(())
    }
}
