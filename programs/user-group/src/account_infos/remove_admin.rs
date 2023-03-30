use crate::prelude::*;

#[derive(Accounts)]
pub struct RemoveAdmin<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK:
    pub user: AccountInfo<'info>,
    #[account(mut,
        constraint = admin.initialized @ GroupError::NotInitialized
    )]
    pub admin: Account<'info, AdminAccount>,
}
