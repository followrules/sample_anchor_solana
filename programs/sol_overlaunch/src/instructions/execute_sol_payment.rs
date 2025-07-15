use anchor_lang::prelude::*;

use crate::{Multisig, PayrollError, Proposal};

#[derive(Accounts)]
pub struct ExecuteSolPayment<'info> {
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,

    /// CHECK: This is a PDA system account for SOL vault.
    #[account(
        mut,
        seeds = [b"vault", multisig.key().as_ref()],
        bump,
    )]
    pub sol_vault: AccountInfo<'info>,

    /// CHECK: This is the recipient's wallet address for receiving SOL. Must be validated by business logic.
    #[account(mut)]
    pub recipient_sol: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> ExecuteSolPayment<'info> {
    pub fn handler(ctx: Context<ExecuteSolPayment>) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        let multisig = &ctx.accounts.multisig;

        require!(!proposal.executed, PayrollError::AlreadyExecuted);
        require!(
            proposal.num_approved >= multisig.threshold,
            PayrollError::NotEnoughSigners
        );
        require!(!proposal.is_spl, PayrollError::Unauthorized); // Cegah misuse

        let sol_vault = &ctx.accounts.sol_vault;
        let to = &ctx.accounts.recipient_sol;

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &sol_vault.key(),
            &to.key(),
            proposal.amount,
        );
        anchor_lang::solana_program::program::invoke_signed(
            &ix,
            &[
                sol_vault.to_account_info(),
                to.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[&[b"vault", multisig.key().as_ref(), &[ctx.bumps.sol_vault]]],
        )?;

        proposal.executed = true;
        Ok(())
    }
}
