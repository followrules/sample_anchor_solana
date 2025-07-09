use anchor_lang::prelude::*;

#[account]
pub struct GlobalState {
    pub owner: Pubkey,
}

#[error_code]
pub enum CustomError {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
    #[msg("Pyth price feed not found or invalid")]
    PriceFeedInvalid,
}