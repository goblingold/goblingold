use anchor_lang::prelude::*;

/// Error codes
#[error_code]
pub enum ErrorCode {
    #[msg("Invalid withdraw. Amount cannot be cero")]
    InvalidZeroWithdraw,
    #[msg("Can not refresh weights yet, not enough slots since last refresh")]
    ForbiddenRefresh,
    #[msg("Invalid amount to deposit.")]
    InvalidProtocolDeposit,
    #[msg("Invalid amount to withdraw.")]
    InvalidProtocolWithdraw,
    #[msg("Invalid deposited amount. Please deposit more than 100 lamports.")]
    InvalidDepositAmount,
    #[msg("Invalid owner.")]
    InvalidOwner,
    #[msg("Invalid mint.")]
    InvalidMint,
    #[msg("Deposits and withdraws aren't allowed on PAUSE.")]
    OnPaused,
    #[msg("The instructions provided are invalid")]
    InvalidInstructions,
    #[msg("Math operation overflow")]
    MathOverflow,
}