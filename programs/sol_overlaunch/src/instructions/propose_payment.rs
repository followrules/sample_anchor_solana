use crate::{
    state::{Multisig, Proposal, Recipient},
    PayrollError,
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(recipients: Vec<Recipient>, mint: Pubkey, nonce: u64)]
pub struct ProposePayment<'info> {
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,

    #[account(
        init,
        payer = proposer,
        seeds = [b"proposal", multisig.key().as_ref(), &nonce.to_le_bytes()],
        bump,
        space = 8 + Proposal::space(&recipients, multisig.signers.len()),
    )]
    pub proposal: Account<'info, Proposal>,

    #[account(mut)]
    pub proposer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> ProposePayment<'info> {
    pub fn handler(
        ctx: Context<ProposePayment>,
        recipients: Vec<Recipient>,
        mint: Pubkey,
        nonce: u64,
    ) -> Result<()> {
        require!(recipients.len() > 0, PayrollError::NotEnoughSigners);

        let proposal = &mut ctx.accounts.proposal;
        let multisig = &ctx.accounts.multisig;

        proposal.multisig = multisig.key();
        proposal.recipients = recipients;
        proposal.did_approve = vec![false; multisig.signers.len()];
        proposal.num_approved = 0;
        proposal.executed = false;
        proposal.mint = Some(mint);
        proposal.nonce = nonce;

        Ok(())
    }
}
