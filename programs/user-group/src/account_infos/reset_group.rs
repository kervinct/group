use crate::prelude::*;

#[derive(Accounts)]
pub struct ResetGroup<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub admin: Account<'info, AdminAccount>,
    #[account(mut,
        constraint = group.load()?.admin == admin.key() @ GroupError::MismatchedSigner,
    )]
    pub group: AccountLoader<'info, GroupAccount>,
}