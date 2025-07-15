use anchor_lang::prelude::*;

use crate::Multisig;


#[derive(Accounts)]
pub struct DepositSol<'info> {
    /// CHECK: This is a PDA system account for SOL vault. Only holds SOL and is controlled by program logic.
    #[account(
        mut,
        seeds = [b"vault", multisig.key().as_ref()],
        bump,
    )]
    pub sol_vault: AccountInfo<'info>,

    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(seeds = [b"multisig", multisig.office_id.as_bytes()], bump)]
    pub multisig: Account<'info, Multisig>,

    pub system_program: Program<'info, System>,
}

impl<'info> DepositSol<'info> {
    pub fn handler(ctx: Context<DepositSol>, amount: u64) -> Result<()> {
        let depositor = &ctx.accounts.depositor;
        let sol_vault = &ctx.accounts.sol_vault;
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &depositor.key(),
            &sol_vault.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                depositor.to_account_info(),
                sol_vault.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
        Ok(())
    }
}
