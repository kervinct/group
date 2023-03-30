use crate::prelude::*;

#[derive(Accounts)]
pub struct CloseSignature<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        constraint = !group.load()?.freeze @ GroupError::FrozenGroup,
    )]
    pub group: AccountLoader<'info, GroupAccount>,
    pub member: Account<'info, MemberAccount>,
    /// CHECK: only for check whether it is closed
    pub proposal: AccountInfo<'info>,
    #[account(mut,
        close = authority,
        constraint = signature.signer == authority.key() @ GroupError::MismatchedSigner,
        constraint = signature.proposal == proposal.key() @ GroupError::MismatchedProposal,
    )]
    pub signature: Account<'info, SignatureAccount>,
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}
