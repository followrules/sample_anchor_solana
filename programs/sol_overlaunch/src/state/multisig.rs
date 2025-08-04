use anchor_lang::prelude::*;


#[account]
pub struct Multisig {
    pub signers: Vec<Pubkey>,
    pub threshold: u8,
    pub office_id: String,
}

#[account]
pub struct Proposal {
    pub multisig: Pubkey,
    pub recipients: Vec<Recipient>,
    pub did_approve: Vec<bool>,
    pub num_approved: u8,
    pub executed: bool,
    pub mint: Option<Pubkey>,
    pub nonce: u64,
}
impl Proposal {
    pub fn space(recipients: &Vec<Recipient>, num_signers: usize) -> usize {
        8 + // discriminator
        32 + // multisig pubkey
        4 + recipients.len() * (32 + 8) + // Vec<Recipient> (Pubkey + u64)
        4 + num_signers + // Vec<bool> did_approve
        1 + // num_approved
        1 + // executed (bool)
        1 + 32 + // Option<Pubkey>
        8 // nonce (u64)
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Recipient {
    pub address: Pubkey,
    pub amount: u64,
}