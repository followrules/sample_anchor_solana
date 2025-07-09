use anchor_lang::prelude::*;

#[event]
pub struct OwnerChanged {
    pub old_owner: Pubkey,
    pub new_owner: Pubkey,
}