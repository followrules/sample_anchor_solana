use crate::state::Multisig;
use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct RefundSpl<'info> {
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
    pub to: Account<'info, TokenAccount>, // ATA user/refund target

    #[account(signer)]
    pub authority: Signer<'info>, // Harus authorized, misal multisig owner atau signer threshold

    pub token_program: Program<'info, Token>,
}

pub fn refund_spl_handler(ctx: Context<RefundSpl>, amount: u64) -> Result<()> {
    let office_id_vec = ctx.accounts.multisig.office_id.as_bytes().to_vec();
    let multisig_bump = ctx.bumps.multisig;

    let signer_seeds: [&[u8]; 3] = [
        b"multisig",
        office_id_vec.as_slice(),
        &[multisig_bump],
    ];

    // INI YANG PENTING:
    let seeds = &[&signer_seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.spl_vault.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.multisig.to_account_info(),
        },
        seeds,
    );
    transfer(cpi_ctx, amount)?;
    Ok(())
}

