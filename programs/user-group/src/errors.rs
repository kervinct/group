use crate::prelude::*;

#[error_code]
pub enum GroupError {
    #[msg("Token mismatched")]
    MismatchedToken,
    #[msg("Admin account is already initialized")]
    AlreadyInitialized,
    #[msg("Admin account is not initialized")]
    NotInitialized,
    #[msg("User is already a administrator")]
    AlreadyAdmin,
    #[msg("Insufficient administrator slot")]
    InsufficientAdminSlot,
    #[msg("Operation unauthorized")]
    OperationUnauthorized,
    #[msg("No permission to submit such proposal")]
    NotPermitted,
    #[msg("Deadline must be greater than 12 hours")]
    NotEnoughTimeElapse,
    #[msg("Deadline can only be extended")]
    OnlyAllowedExtend,
    #[msg("Invalid limit")]
    InvalidLimit,
    #[msg("Already is manager")]
    AlreadyManager,
    #[msg("Already is member")]
    AlreadyMember,
    #[msg("Manager is not zero")]
    ManagerNotZero,
    #[msg("Could not update the group")]
    CouldNotUpdateGroup,
    #[msg("Manager count is full, update group first")]
    UpdateFirst,
    #[msg("Already updated proposal")]
    AlreadyUpdatedProposal,
    #[msg("Group is mismatched")]
    MismatchedGroup,
    #[msg("Funder is mismatched")]
    MismatchedFunder,
    #[msg("Beneficiary is mismatched")]
    MismatchedBeneMember,
    #[msg("Member is already in a proposal progressing")]
    AlreadyInProposal,
    #[msg("Proposal already outdated")]
    AlreadyOutdated,
    #[msg("Proposal already updated")]
    AlreadyUpdated,
    #[msg("Proposal revoke outdated, only allow revoke within 2 hours after submit")]
    RevokeOutdated,
    #[msg("Group is frozen, not allowed this operation")]
    FrozenGroup,
    #[msg("Group is already frozen")]
    AlreadyFrozen,
    #[msg("Group is not frozen")]
    GroupIsNotFrozen,
    #[msg("Group is in election")]
    LastElectionNotFinished,
    #[msg("Wrong vault token account")]
    WrongVaultTokenAccount,
    #[msg("Wrong receiver token account")]
    WrongReceiverTokenAccount,
    #[msg("Token balance is insufficient")]
    InsufficientTokenBalance,
    #[msg("Proposal can only be closed after passed or rejected or outdated")]
    NotAllowedToCloseProgressingProposal,
    #[msg("Member is in withdraw")]
    LastWithdrawNotFinished,
    #[msg("Signer is mismatched")]
    MismatchedSigner,
    #[msg("Proposal is in progressing, not allowed to close")]
    NotAllowedToClose,
    #[msg("Proposal is mismatched")]
    MismatchedProposal,
    #[msg("Account does not exist")]
    AccountDoesNotExist,
    #[msg("Proposal is outdated")]
    ProposalOutdated,
    #[msg("Proposal is not outedated")]
    ProposalNotOutdated,
    #[msg("Member is not in any proposal")]
    MemberNotInProposal,
}