use crate::prelude::*;

#[derive(Accounts)]
pub struct UpdateProposal<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut,
        constraint = !group.load()?.freeze @ GroupError::FrozenGroup,
        constraint = proposal.submitter == authority.key() @ GroupError::OperationUnauthorized,
    )]
    pub proposal: Account<'info, ProposalAccount>,
    pub group: AccountLoader<'info, GroupAccount>,
    pub clock: Sysvar<'info, Clock>,
}
