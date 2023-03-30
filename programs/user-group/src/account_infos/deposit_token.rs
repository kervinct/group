use crate::prelude::*;

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct DepositToken<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub mint: Account<'info, Mint>,
    #[account(
        constraint = member.owner == authority.key() @ GroupError::OperationUnauthorized,
        constraint = member.group == group.key() @ GroupError::MismatchedGroup,
    )]
    pub member: Account<'info, MemberAccount>,
    #[account(mut,
        constraint = token.amount >= amount @ GroupError::InsufficientTokenBalance,
    )]
    pub token: Account<'info, TokenAccount>,
    #[account(
        constraint = !group.load()?.freeze @ GroupError::FrozenGroup,
    )]
    pub group: AccountLoader<'info, GroupAccount>,
    #[account(mut,
        constraint = vault.mint == mint.key(),
        constraint = vault.owner == group.key(),
    )]
    pub vault: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}
impl<'info> DepositToken<'info> {
    pub fn as_token_transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.token.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.authority.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
