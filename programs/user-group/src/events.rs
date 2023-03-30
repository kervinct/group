use crate::prelude::*;

#[event]
pub struct JoinGroupEvent {
    pub user: Pubkey,
    pub group: Pubkey,
    pub member: Pubkey,
    pub funder: Pubkey,
    pub time: i64,
    #[index]
    pub label: String,
}

#[event]
pub struct ExitGroupEvent {
    pub user: Pubkey,
    pub group: Pubkey,
    pub member: Pubkey,
    pub funder: Pubkey,
    pub time: i64,
    #[index]
    pub label: String,
}

#[event]
pub struct FreezeGroupEvent {
    pub authority: Pubkey,
    pub admin: Pubkey,
    pub group: Pubkey,
    #[index]
    pub label: String,
}

#[event]
pub struct ThawGroupEvent {
    pub authority: Pubkey,
    pub admin: Pubkey,
    pub group: Pubkey,
    #[index]
    pub label: String,
}

#[event]
pub struct UpgradeGroupEvent {
    pub authority: Pubkey,
    pub admin: Pubkey,
    pub group: Pubkey,
    pub rate: ParticipateRate,
    #[index]
    pub label: String,
}

#[repr(u8)]
#[derive(Clone, Debug, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub enum ProposalEventType {
    Upgrade,
    Downgrade,
    Withdraw,
    UpdateGroup,
    ReElection,
}
impl From<ProposalType> for ProposalEventType {
    fn from(proposal_type: ProposalType) -> Self {
        match proposal_type {
            ProposalType::Upgrade => ProposalEventType::Upgrade,
            ProposalType::Downgrade => ProposalEventType::Downgrade,
            ProposalType::Withdraw { .. } => ProposalEventType::Withdraw,
            ProposalType::UpdateGroup { .. } => ProposalEventType::UpdateGroup,
            ProposalType::ReElection => ProposalEventType::ReElection,
        }
    }
}

#[event]
pub struct SubmitProposalEvent {
    pub ptype: ProposalEventType,
    pub submitter: Pubkey,
    pub submitter_member: Pubkey,
    pub beneficiary: Pubkey,
    pub bene_member: Pubkey,
    pub group: Pubkey,
    pub proposal: Pubkey,
    pub deadline: i64,
    #[index]
    pub label: String,
}

#[event]
pub struct SignProposalEvent {
    pub ptype: ProposalEventType,
    pub submitter: Pubkey,
    pub beneficiary: Pubkey,
    pub group: Pubkey,
    pub proposal: Pubkey,
    pub stype: SignType,
    pub signer: Pubkey,
    pub signature: Pubkey,
    #[index]
    pub label: String,
}

#[repr(u8)]
#[derive(Clone, Debug, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub enum ProposalResult {
    Passed,
    Rejected,
}

#[event]
pub struct ExecuteProposalEvent {
    pub ptype: ProposalEventType,
    pub submitter: Pubkey,
    pub beneficiary: Pubkey,
    pub group: Pubkey,
    pub proposal: Pubkey,
    pub result: ProposalResult,
    #[index]
    pub label: String,
}

#[event]
pub struct DepositTokenEvent {
    pub user: Pubkey,
    pub group: Pubkey,
    pub vault: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    #[index]
    pub label: String,
}

#[event]
pub struct UpdateProposalEvent {
    pub group: Pubkey,
    pub proposal: Pubkey,
    pub submitter: Pubkey,
    pub deadline: i64,
    #[index]
    pub label: String,
}