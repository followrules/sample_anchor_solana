use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer, transfer};

use crate::{Multisig, PayrollError, Proposal};

#[derive(Accounts)]
pub struct ExecuteSplPayment<'info> {
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,

    #[account(
        mut,
        seeds = [b"vault", multisig.key().as_ref(), mint.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = multisig,
    )]
    pub spl_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub recipient_token: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

impl<'info> ExecuteSplPayment<'info> {
    pub fn handler(ctx: Context<ExecuteSplPayment>) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        let multisig = &ctx.accounts.multisig;

        require!(!proposal.executed, PayrollError::AlreadyExecuted);
        require!(
            proposal.num_approved >= multisig.threshold,
            PayrollError::NotEnoughSigners
        );
        require!(proposal.is_spl, PayrollError::Unauthorized); // Cegah misuse

        let vault = &ctx.accounts.spl_vault;
        let recipient = &ctx.accounts.recipient_token;

        let multisig_seeds: &[&[u8]] = &[
            b"multisig",
            multisig.office_id.as_bytes(),
            &[ctx.bumps.spl_vault],
        ];

        let cpi_accounts = Transfer {
            from: vault.to_account_info(),
            to: recipient.to_account_info(),
            authority: multisig.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        transfer(
            CpiContext::new_with_signer(cpi_program, cpi_accounts, &[multisig_seeds]),
            proposal.amount,
        )?;

        proposal.executed = true;
        Ok(())
    }
}