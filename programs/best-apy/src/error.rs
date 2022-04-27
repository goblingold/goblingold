use anchor_lang::prelude::*;

/// Error codes
#[error_code]
pub enum ErrorCode {
    #[msg("Invalid withdraw. Amount cannot be cero")]
    InvalidZeroWithdraw,
    #[msg("Invalid lp price")]
    InvalidLpPrice,
    #[msg("Can not refresh weights yet, not enough slots since last refresh")]
    ForbiddenRefresh,
    #[msg("A protocol TVL is stale and must be refreshed")]
    StaleProtocolTVL,
    #[msg("Invalid amount to deposit")]
    InvalidProtocolDeposit,
    #[msg("Invalid amount to withdraw")]
    InvalidProtocolWithdraw,
    #[msg("Invalid deposited amount. Please deposit more than 100 lamports")]
    InvalidDepositAmount,
    #[msg("Invalid owner")]
    InvalidOwner,
    #[msg("Invalid mint")]
    InvalidMint,
    #[msg("Deposits and withdraws aren't allowed on PAUSE")]
    OnPaused,
    #[msg("The instructions provided are invalid")]
    InvalidInstructions,
    #[msg("Math operation overflow")]
    MathOverflow,
    #[msg("Non-valid weights")]
    InvalidWeights,
    #[msg("Invalid hash from provided pubkeys")]
    InvalidHash,
    #[msg("Invalid size for array")]
    InvalidArraySize,
    #[msg("Invalid obligation account owner")]
    InvalidObligationOwner,
    #[msg("Invalid obligation reserve account")]
    InvalidObligationReserve,
    #[msg("Unauthorized user")]
    UnauthorizedUser,
    #[msg("Protocol not found in vault")]
    ProtocolNotFoundInVault,
}
