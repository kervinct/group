use crate::prelude::*;

#[derive(Accounts)]
pub struct SignProposal<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub mint: Account<'info, Mint>, // burning token
    #[account(mut)]
    pub group: AccountLoader<'info, GroupAccount>,
    #[account(mut,
        constraint = token.mint == mint.key(),
        constraint = token.owner == authority.key(),
        constraint = token.amount >= proposal.limit @ GroupError::InsufficientTokenBalance,
    )]
    pub token: Account<'info, TokenAccount>, // authority's burning token account
    #[account(
        constraint = member.owner == authority.key() @ GroupError::OperationUnauthorized,
    )]
    pub member: Box<Account<'info, MemberAccount>>,
    #[account(init,
        seeds = [group.key().as_ref(), member.key().as_ref(), proposal.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + SignatureAccount::LEN,
    )]
    pub signature: Box<Account<'info, SignatureAccount>>,
    #[account(mut,
        constraint = !group.load()?.freeze @ GroupError::FrozenGroup,
    )]
    pub proposal: Box<Account<'info, ProposalAccount>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
    // Upgrade | Downgrade | ReElection
    // bene_member_info  isWritable: true

    // Withdraw
    // bene_member_info  isWritable: true
    // group_vault_token_info isWritable: true
    // user_token_account_info isWritable: true
}

impl<'info> SignProposal<'info> {
    pub fn as_token_burn_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let cpi_accounts = Burn {
            mint: self.mint.to_account_info(),
            from: self.token.to_account_info(),
            authority: self.authority.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
