use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
    #[msg("Pyth price feed not found or invalid")]
    PriceFeedInvalid,
    #[msg("Pyth price feed too old")]
    PriceFeedTooOld,
}

#[error_code]
pub enum PayrollError {
    #[msg("Already executed!")]
    AlreadyExecuted,
    #[msg("Not enough approval!")]
    NotEnoughSigners,
    #[msg("Already approved!")]
    AlreadyApproved,
    #[msg("Unauthorized signer!")]
    Unauthorized,
    #[msg("No signer provided!")]
    NoSigner,
    #[msg("Invalid threshold!")]
    InvalidThreshold,
    #[msg("Invalid signer index!")]
    InvalidSignerIndex,
}