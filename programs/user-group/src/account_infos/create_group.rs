use crate::prelude::*;

#[derive(Accounts)]
pub struct CreateGroup<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK:
    pub sponsor: AccountInfo<'info>,
    pub mint: Account<'info, Mint>,
    #[account(init,
        seeds = [admin.key().as_ref(), &admin.groups.to_le_bytes(), GroupAccount::SEEDS],
        bump,
        payer = authority,
        space = 8 + GroupAccount::LEN,
    )]
    pub group: AccountLoader<'info, GroupAccount>,
    #[account(
        constraint = token.owner == group.key(),
        constraint = token.mint == mint.key(),
    )]
    pub token: Account<'info, TokenAccount>,
    #[account(mut,
        constraint = admin.initialized @ GroupError::NotInitialized,
    )]
    pub admin: Account<'info, AdminAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}
