use crate::prelude::*;

#[derive(Accounts)]
pub struct JoinGroup<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK:
    pub user: AccountInfo<'info>,
    #[account(mut)]
    pub group: AccountLoader<'info, GroupAccount>,
    #[account(init, payer = authority, space = 8 + MemberAccount::LEN,
        seeds = [group.key().as_ref(), user.key().as_ref(), MemberAccount::SEEDS],
        bump,
        constraint = !group.load()?.freeze @ GroupError::FrozenGroup,
    )]
    pub member: Account<'info, MemberAccount>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
}
