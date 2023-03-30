use crate::prelude::*;

#[derive(Accounts)]
pub struct FreezeGroup<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        constraint = admin.initialized @ GroupError::NotInitialized,
    )]
    pub admin: Account<'info, AdminAccount>,
    #[account(mut,
        seeds = [admin.key().as_ref(), &group.load()?.index.to_le_bytes(), GroupAccount::SEEDS],
        bump = group.load()?.seed,
        constraint = !group.load()?.freeze @ GroupError::AlreadyFrozen,
    )]
    pub group: AccountLoader<'info, GroupAccount>,
}
