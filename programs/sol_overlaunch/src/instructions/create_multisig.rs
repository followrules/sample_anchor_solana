use anchor_lang::prelude::*;
use crate::{state::PayrollError, state::Multisig};

#[derive(Accounts)]
#[instruction(signers: Vec<Pubkey>, threshold: u8, office_id: String)]
pub struct CreateMultisig<'info> {
    #[account(
        init,
        seeds = [b"multisig", office_id.as_bytes()],
        bump,
        payer = payer,
        space = 8 + 4 + (signers.len() * 32) + 1 + 4 + office_id.len()
    )]
    pub multisig: Account<'info, Multisig>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateMultisig<'info> {
    pub fn handler(
        ctx: Context<CreateMultisig>,
        signers: Vec<Pubkey>,
        threshold: u8,
        office_id: String,
    ) -> Result<()> {
        require!(signers.len() > 0, PayrollError::NoSigner);
        require!(threshold > 0 && threshold <= signers.len() as u8, PayrollError::InvalidThreshold);

        let multisig = &mut ctx.accounts.multisig;
        multisig.signers = signers;
        multisig.threshold = threshold;
        multisig.office_id = office_id;
        Ok(())
    }
}
