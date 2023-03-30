use crate::prelude::*;

#[derive(Accounts)]
pub struct AddAdmin<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK:
    pub user: AccountInfo<'info>,
    #[account(mut,
        constraint = admin.initialized @ GroupError::NotInitialized,
        constraint = !admin.is_full() @ GroupError::InsufficientAdminSlot,
    )]
    pub admin: Account<'info, AdminAccount>,
}
