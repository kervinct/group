use crate::prelude::*;

#[derive(Accounts)]
pub struct RevokeProposal<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK:
    pub beneficiary: UncheckedAccount<'info>,
    #[account(mut,
        constraint = bene_member.owner == beneficiary.key() @  GroupError::MismatchedBeneMember,
    )]
    pub bene_member: Account<'info, MemberAccount>,
    #[account(mut,
        close = authority,
        constraint = proposal.submitter == authority.key() @ GroupError::OperationUnauthorized,
        constraint = proposal.beneficiary == beneficiary.key() @ GroupError::MismatchedBeneMember,
        constraint = proposal.bene_member == bene_member.key() @ GroupError::MismatchedBeneMember,
    )]
    pub proposal: Account<'info, ProposalAccount>,
    #[account(mut,
        constraint = !group.load()?.freeze @ GroupError::FrozenGroup,
    )]
    pub group: AccountLoader<'info, GroupAccount>,
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}