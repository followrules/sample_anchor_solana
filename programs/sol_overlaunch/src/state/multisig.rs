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
    pub to: Pubkey,
    pub amount: u64,
    pub did_approve: Vec<bool>,
    pub num_approved: u8,
    pub executed: bool,
    pub is_spl: bool,
    pub mint: Option<Pubkey>,  
    pub nonce: u64,
}