use anchor_lang::prelude::*;
use crate::{state::{Multisig, Proposal}};

#[derive(Accounts)]
#[instruction(to: Pubkey, amount: u64, is_spl: bool, mint: Option<Pubkey>, nonce: u64)]
pub struct ProposePayment<'info> {
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,
    #[account(
        init,
        seeds = [
            b"proposal",
            multisig.key().as_ref(),
            to.as_ref(),
            &nonce.to_le_bytes()
        ],
        bump,
        payer = proposer,
        space = 8 + 32 + 32 + 8 + 4 + (multisig.signers.len()) + 1 + 1 + 1 + 33 + 8,
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub proposer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> ProposePayment<'info> {
    pub fn handler(
        ctx: Context<ProposePayment>,
        to: Pubkey,
        amount: u64,
        is_spl: bool,
        mint: Option<Pubkey>,
        nonce: u64,
    ) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        let multisig = &ctx.accounts.multisig;

        proposal.multisig = multisig.key();
        proposal.to = to;
        proposal.amount = amount;
        proposal.did_approve = vec![false; multisig.signers.len()];
        proposal.num_approved = 0;
        proposal.executed = false;
        proposal.is_spl = is_spl;
        proposal.mint = mint;
        proposal.nonce = nonce;
        Ok(())
    }
}
