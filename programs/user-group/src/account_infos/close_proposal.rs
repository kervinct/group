use crate::prelude::*;

#[derive(Accounts)]
pub struct CloseProposal<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK:
    pub beneficiary: AccountInfo<'info>,
    #[account(mut,
        constraint = bene_member.group == group.key(),
        constraint = bene_member.owner == beneficiary.key() @ GroupError::OperationUnauthorized,
    )]
    pub bene_member: Account<'info, MemberAccount>,
    #[account(mut,
        close = authority,
        constraint = proposal.submitter == authority.key() @ GroupError::OperationUnauthorized,
    )]
    pub proposal: Account<'info, ProposalAccount>,
    #[account(
        constraint = !group.load()?.freeze @ GroupError::FrozenGroup
    )]
    pub group: AccountLoader<'info, GroupAccount>,
    pub clock: Sysvar<'info, Clock>,
}