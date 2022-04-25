use crate::error::ErrorCode;

use crate::vault::ProtocolData;

use anchor_lang::prelude::*;

use anchor_spl::token::TokenAccount;

/// Deposit into the protocol
pub trait ProtocolWithdraw<'info> {
    /// Return a mutable refrence of the data
    fn protocol_data_as_mut(&mut self) -> &mut ProtocolData;

    /// Return the input token account
    fn input_token_account_as_mut(&mut self) -> &mut Account<'info, TokenAccount>;

    /// Compute the amount to deposit
    fn get_amount(&self) -> Result<u64>;

    /// Convert reserve liquidity to collateral (if any)
    fn liquidity_to_collateral(&self, amount: u64) -> Result<u64> {
        Ok(amount)
    }

    /// Withdraw from the protocol
    fn cpi_withdraw(&self, amount: u64) -> Result<()>;
}

/// Deposit into the protocol and update protocol data
pub fn handler<'info, T: ProtocolWithdraw<'info>>(ctx: Context<T>) -> Result<()> {
    let amount = ctx.accounts.get_amount()?;
    let mut lp_amount = ctx.accounts.liquidity_to_collateral(amount)?;

    // Add 1 as due to rounding. Otherwise it might happens that there wasn't enough funds
    // withdrawn from the protocol
    if amount < ctx.accounts.protocol_data_as_mut().amount {
        lp_amount = lp_amount
            .checked_add(1)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;
    }

    let amount_before = {
        let input_token_account = ctx.accounts.input_token_account_as_mut();
        input_token_account.amount
    };

    ctx.accounts.cpi_withdraw(lp_amount)?;

    let amount_after = {
        let input_token_account = ctx.accounts.input_token_account_as_mut();
        input_token_account.reload()?;
        input_token_account.amount
    };

    let amount_diff = amount_after
        .checked_sub(amount_before)
        .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

    ctx.accounts
        .protocol_data_as_mut()
        .update_after_withdraw(amount_diff)?;

    Ok(())
}
