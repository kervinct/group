use crate::prelude::*;

#[derive(Accounts)]
pub struct ExitGroup<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub funder: AccountInfo<'info>,
    #[account(mut,
        close = funder,
        constraint = member.funder == funder.key() @ GroupError::MismatchedFunder,
        constraint = member.owner == authority.key() @ GroupError::OperationUnauthorized,
    )]
    pub member: Account<'info, MemberAccount>,
    #[account(mut)]
    pub group: AccountLoader<'info, GroupAccount>,
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}
