use crate::prelude::*;

#[derive(Accounts)]
pub struct ResetMember<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK: 
    pub beneficiary: UncheckedAccount<'info>,
    #[account(mut,
        constraint = bene_member.owner == beneficiary.key() @ GroupError::MismatchedBeneMember,
        constraint = bene_member.in_promotion || bene_member.in_withdraw @ GroupError::MemberNotInProposal,
    )]
    pub bene_member: Account<'info, MemberAccount>,
    #[account(
        constraint = proposal.beneficiary == beneficiary.key() @ GroupError::MismatchedBeneMember,
        constraint = proposal.bene_member == bene_member.key() @ GroupError::MismatchedBeneMember,
        constraint = proposal.deadline <= clock.unix_timestamp @ GroupError::ProposalNotOutdated,
    )]
    pub proposal: Box<Account<'info, ProposalAccount>>,
    pub clock: Sysvar<'info, Clock>,
}