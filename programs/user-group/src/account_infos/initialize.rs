use crate::prelude::*;

#[derive(Accounts)]
#[instruction(seed: u8)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub mint: Account<'info, Mint>,
    #[account(init,
        seeds = [authority.key().as_ref(), AdminAccount::SEEDS],
        space = 8 + AdminAccount::LEN,
        bump,
        payer = authority,
    )]
    pub admin: Account<'info, AdminAccount>,
    pub system_program: Program<'info, System>,
}
