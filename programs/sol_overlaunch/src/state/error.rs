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
    #[msg("Invalid signer")]
    InvalidSigner,
    #[msg("Already approved")]
    AlreadyApproved,
    #[msg("Not enough approvals")]
    NotEnoughApprovals,
    #[msg("Already executed")]
    AlreadyExecuted,
    #[msg("Not an SPL proposal")]
    NotSplProposal,
    #[msg("Invalid mint")]
    InvalidMint,
    #[msg("Insufficient remaining accounts")]
    InsufficientRemainingAccounts,
    #[msg("not enough signers")]
    NotEnoughSigners,
    #[msg("unauthorized")]
    Unauthorized,
    #[msg("Proposal already executed")]
    ProposalAlreadyExecuted,
    #[msg("Not SPL payment")]
    NotSplPayment,
    #[msg("Not enough approval")]
    NotEnoughApproval,
    #[msg("Accounts are invalid")]
    InvalidAccounts,
    #[msg("Mint Pubkey is required")]
    MintRequired,
    #[msg("Invalid depositor")]
    InvalidDepositor,
    #[msg("Invalid Amount")]
    InvalidAmount,
    #[msg("Invalid InvalidProgram")]
    InvalidProgram,
    #[msg("Invalid vault")]
    InvalidVault,
    #[msg("Invalid Receiver")]
    InvalidReceiver,
}