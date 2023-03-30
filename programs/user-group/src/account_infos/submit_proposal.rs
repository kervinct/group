use crate::prelude::*;

#[derive(Accounts)]
pub struct SubmitProposal<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK: always wallet
    pub beneficiary: AccountInfo<'info>,
    #[account(init,
        seeds = [group.key().as_ref(), &group.load()?.proposals.to_le_bytes(), ProposalAccount::SEEDS],
        bump,
        payer = authority,
        space = 8 + ProposalAccount::LEN,
    )]
    pub proposal: Account<'info, ProposalAccount>,
    #[account(mut,
        constraint = !group.load()?.freeze @ GroupError::FrozenGroup,
    )]
    pub group: AccountLoader<'info, GroupAccount>,
    #[account(mut,
        constraint = member.group == group.key(),
        constraint = member.owner == authority.key() @ GroupError::OperationUnauthorized,
    )]
    pub member: Account<'info, MemberAccount>,
    #[account(mut,
        constraint = bene_member.group == group.key(),
        constraint = bene_member.owner == beneficiary.key() @ GroupError::OperationUnauthorized,
    )]
    pub bene_member: Account<'info, MemberAccount>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
    // Withdraw
    // group_vault_token_account_info
}
