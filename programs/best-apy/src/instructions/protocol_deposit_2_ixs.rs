use crate::duplicated_ixs::is_last_of_duplicated_ixs;
use crate::vault::ProtocolData;
use anchor_lang::prelude::*;

/// Deposit into the protocol in two instructions
pub trait ProtocolDeposit2Ixs<'info> {
    /// Return a mutable refrence of the data
    fn protocol_data_as_mut(&mut self) -> &mut ProtocolData;

    /// Return the instructions acccount
    fn instructions_account(&self) -> AccountInfo<'info>;

    /// Compute the amount to deposit
    fn get_amount(&self) -> Result<u64>;

    /// Deposit into the protocol
    fn cpi_deposit(&self, amount: u64, is_last_deposit_ix: bool) -> Result<()>;
}

/// Deposit into the protocol and update protocol data in two ixs in order to overcome the ix
/// compute budget
pub fn handler<'info, T: ProtocolDeposit2Ixs<'info>>(ctx: Context<T>) -> Result<()> {
    let is_last_deposit_ix = is_last_of_duplicated_ixs(ctx.accounts.instructions_account())?;

    let amount: u64 = if is_last_deposit_ix {
        0
    } else {
        ctx.accounts.get_amount()?
    };

    ctx.accounts.cpi_deposit(amount, is_last_deposit_ix)?;
    ctx.accounts
        .protocol_data_as_mut()
        .update_after_deposit(amount)?;

    Ok(())
}
