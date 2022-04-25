use crate::vault::ProtocolData;

use anchor_lang::prelude::*;
use std::convert::TryInto;

#[event]
pub struct ProtocolRewardsEvent {
    protocol_id: u8,
    token: Pubkey,
    rewards: u64,
    lamports: u64,
    initial_slot: u64,
}

/// Get the rewards produced by the protocol
pub trait ProtocolRewards<'info> {
    /// Get the protocol ID
    fn protocol_id(&self) -> usize;

    /// Get the input token mint pubkey
    fn input_mint_pubkey(&self) -> Pubkey;

    /// Return a mutable refrence of the data
    fn protocol_data_as_mut(&mut self) -> &mut ProtocolData;

    /// Compute the maximam withdrawable units
    fn max_withdrawable(&self) -> Result<u64>;
}

/// Update the rewards
pub fn handler<'info, T: ProtocolRewards<'info>>(ctx: Context<T>) -> Result<()> {
    let protocol_id: u8 = ctx.accounts.protocol_id().try_into().unwrap();
    let token = ctx.accounts.input_mint_pubkey();

    let tvl = ctx.accounts.max_withdrawable()?;
    let protocol = ctx.accounts.protocol_data_as_mut();
    let rewards = tvl.saturating_sub(protocol.amount);
    protocol.rewards.update(rewards)?;

    emit!(ProtocolRewardsEvent {
        protocol_id,
        token,
        rewards: protocol.rewards.amount,
        lamports: protocol.rewards.deposited_avg,
        initial_slot: protocol.rewards.deposited_integral.initial_slot
    });

    Ok(())
}
