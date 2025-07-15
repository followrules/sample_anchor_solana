use anchor_lang::prelude::*;

use crate::{Multisig, PayrollError, Proposal};

#[derive(Accounts)]
pub struct ApprovePayment<'info> {
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    pub signer: Signer<'info>,
}

impl<'info> ApprovePayment<'info> {
    pub fn handler(
        ctx: Context<ApprovePayment>,
        signer_index: u8,
    ) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        let multisig = &ctx.accounts.multisig;
        let signer = ctx.accounts.signer.key();

        require!(!proposal.executed, PayrollError::AlreadyExecuted);
        require!(
            (signer_index as usize) < multisig.signers.len(),
            PayrollError::InvalidSignerIndex
        );
        require!(
            !proposal.did_approve[signer_index as usize],
            PayrollError::AlreadyApproved
        );
        require!(
            signer == multisig.signers[signer_index as usize],
            PayrollError::Unauthorized
        );

        proposal.did_approve[signer_index as usize] = true;
        proposal.num_approved += 1;
        Ok(())
    }
}
