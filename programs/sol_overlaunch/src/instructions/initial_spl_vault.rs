use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::state::Multisig;

#[derive(Accounts)]
pub struct InitSplVault<'info> {
    #[account(
        seeds = [b"multisig", multisig.office_id.as_bytes()],
        bump
    )]
    pub multisig: Account<'info, Multisig>,

    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = payer,
        seeds = [b"vault", multisig.key().as_ref(), mint.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = multisig,
    )]
    pub spl_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
    /// CHECK: ini diverifikasi via `token::` constraint di atas
    pub token_program: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn init_spl_vault_handler(
    ctx: Context<InitSplVault>,
    _vault_bump: u8,
) -> Result<()> {
    // Logging untuk debug seed dan bump
    msg!("Vault address: {:?}", ctx.accounts.spl_vault.key());
    msg!("Mint address: {:?}", ctx.accounts.mint.key());
    msg!("Multisig address: {:?}", ctx.accounts.multisig.key());

    // Tidak perlu logic tambahan — Anchor akan otomatis create TokenAccount jika belum ada
    // Kalau sudah ada, akan error "already in use" secara otomatis.
    Ok(())
}
