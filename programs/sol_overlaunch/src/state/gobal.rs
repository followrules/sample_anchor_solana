use anchor_lang::prelude::*;

#[account]
pub struct GlobalState {
    pub owner: Pubkey,
}

