use crate::{Multisig, PayrollError, Proposal};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ApprovePayment<'info> {
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,
    #[account(mut, has_one = multisig)]
    pub proposal: Account<'info, Proposal>,
    #[account(signer)]
    pub signer: Signer<'info>,
}

impl<'info> ApprovePayment<'info> {
    pub fn handler(ctx: Context<ApprovePayment>) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        let multisig = &ctx.accounts.multisig;
        let signer_key = ctx.accounts.signer.key();

        require!(!proposal.executed, PayrollError::AlreadyExecuted);

        // Cari index signer di multisig
        let signer_index = multisig
            .signers
            .iter()
            .position(|x| x == &signer_key)
            .ok_or(PayrollError::InvalidSigner)?;

        // Cek apakah sudah approve
        if *proposal.did_approve.get(signer_index).unwrap_or(&false) {
            return err!(PayrollError::AlreadyApproved);
        }

        // Approve
        proposal.did_approve[signer_index] = true;
        proposal.num_approved += 1;

        // Eksekusi jika threshold terpenuhi
        if proposal.num_approved >= multisig.threshold {
            proposal.executed = true;
        }

        Ok(())
    }
}
