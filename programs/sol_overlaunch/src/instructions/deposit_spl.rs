use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, transfer};
use crate::state::Multisig;

#[derive(Accounts)]
pub struct DepositSpl<'info> {
    #[account(
        seeds = [b"multisig", multisig.office_id.as_bytes()],
        bump
    )]
    pub multisig: Account<'info, Multisig>,

    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"vault", multisig.key().as_ref(), mint.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = multisig,
    )]
    pub spl_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub from: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn deposit_spl_handler(ctx: Context<DepositSpl>, amount: u64) -> Result<()> {
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.spl_vault.to_account_info(),
            authority: ctx.accounts.from.to_account_info(),
        },
    );
    transfer(cpi_ctx, amount)?;
    Ok(())
}
