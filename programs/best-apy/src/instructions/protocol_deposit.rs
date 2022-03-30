use crate::vault::ProtocolData;
use anchor_lang::prelude::*;

/// Deposit into the protocol
pub trait ProtocolDeposit<'info> {
    /// Return a mutable refrence of the data
    fn protocol_data_as_mut(&mut self) -> &mut ProtocolData;

    /// Compute the amount to deposit
    fn get_amount(&self) -> Result<u64>;

    /// Deposit into the protocol
    fn cpi_deposit(&self, amount: u64) -> Result<()>;
}

/// Deposit into the protocol and update protocol data
pub fn handler<'info, T: ProtocolDeposit<'info>>(ctx: Context<T>) -> Result<()> {
    let amount = ctx.accounts.get_amount()?;
    ctx.accounts.cpi_deposit(amount)?;
    ctx.accounts
        .protocol_data_as_mut()
        .update_after_deposit(amount)?;

    Ok(())
}
