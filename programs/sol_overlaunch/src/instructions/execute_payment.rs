use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer as SplTransfer, transfer as spl_transfer};
use crate::{Multisig, PayrollError, Proposal};

#[derive(Accounts)]
pub struct ExecutePayment<'info> {
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,

    // SOL vault (boleh kosong saat SPL payment, tetap pass PDA)
    #[account(
        mut,
        seeds = [b"vault", multisig.key().as_ref()],
        bump,
    )]
    pub sol_vault: AccountInfo<'info>,

    // -- SPL payment
    /// SPL mint account (wajib saat SPL payment)
    pub mint: Option<Account<'info, Mint>>,

    // SPL vault (TokenAccount) (wajib saat SPL payment)
    #[account(
        mut,
        seeds = [b"vault", multisig.key().as_ref(), mint.as_ref().map(|m| m.key()).unwrap_or_default().as_ref()],
        bump,
        token::mint = mint,
        token::authority = multisig,
    )]
    pub spl_vault: Option<Account<'info, TokenAccount>>,

    pub token_program: Option<Program<'info, Token>>,
    pub system_program: Program<'info, System>,

    // Pass semua recipient ATA saat SPL payment (pakai remaining_accounts)
}
pub fn handler(ctx: Context<ExecutePayment>) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let multisig = &ctx.accounts.multisig;

    require!(!proposal.executed, PayrollError::AlreadyExecuted);
    require!(
        proposal.num_approved >= multisig.threshold,
        PayrollError::NotEnoughSigners
    );

    let remaining_accounts = &ctx.remaining_accounts;

    for (i, recipient) in proposal.recipients.iter().enumerate() {
        let dest_ata: &AccountInfo<'_> = remaining_accounts.get(i).ok_or(PayrollError::InvalidReceiver)?;
        let cpi_accounts = SplTransfer {
            from: ctx.accounts.spl_vault.to_account_infos(),
            to: dest_ata.clone(),
            authority: multisig.to_account_info(),
        };
        let seeds = &[b"multisig", multisig.office_id.as_bytes()];
        let multisig_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            multisig_seeds,
        );
        spl_transfer(cpi_ctx, recipient.amount)?;
    }

    proposal.executed = true;
    Ok(())
}
