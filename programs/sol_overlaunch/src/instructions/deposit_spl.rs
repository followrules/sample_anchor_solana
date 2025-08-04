use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::{state::Multisig, PayrollError};

#[derive(Accounts)]
pub struct DepositSpl<'info> {
    // Ambil multisig berdasar office_id (PDA)
    #[account(
        seeds = [b"multisig", multisig.office_id.as_bytes()],
        bump
    )]
    pub multisig: Account<'info, Multisig>,

    // Mint token yang akan digunakan (misal USDC / PYUSD)
    pub mint: Account<'info, Mint>,

    // Vault token account (PDA)
    #[account(
        mut,
        seeds = [b"vault", multisig.key().as_ref(), mint.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = multisig,
    )]
    pub spl_vault: Account<'info, TokenAccount>,

    // Sumber dana dari user
    #[account(
        mut,
        constraint = from.owner == depositor.key(),
        constraint = from.mint == mint.key()
    )]
    pub from: Account<'info, TokenAccount>,

    // Siapa yang menyetor (harus signer dari client)
    pub depositor: Signer<'info>,

    // Program SPL Token
    pub token_program: Program<'info, Token>,
}

pub fn deposit_spl_handler(
    ctx: Context<DepositSpl>,
    amount: u64,
) -> Result<()> {
    require!(amount > 0, PayrollError::InvalidAmount);

    let cpi_accounts = Transfer {
        from: ctx.accounts.from.to_account_info(),
        to: ctx.accounts.spl_vault.to_account_info(),
        authority: ctx.accounts.depositor.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
    );

    token::transfer(cpi_ctx, amount)?;

    Ok(())
}
