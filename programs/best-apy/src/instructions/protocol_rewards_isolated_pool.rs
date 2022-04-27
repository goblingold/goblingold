use crate::protocols::Protocols;
use crate::vault::ProtocolData;
use crate::VaultAccount;
use crate::VAULT_ACCOUNT_SEED;
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
pub trait ProtocolRewardsIsolatedPool<'info> {
    /// Get the input token mint pubkey
    fn input_mint_pubkey(&self) -> Pubkey;

    /// Return a mutable refrence of the data
    fn protocol_data_as_mut(&mut self, protocol: Protocols) -> &mut ProtocolData;

    /// Compute the maximam withdrawable units
    fn max_withdrawable(&self) -> Result<u64>;
}

/// Update the rewards
pub fn handler<'info, T: ProtocolRewardsIsolatedPool<'info>>(
    ctx: Context<T>,
    protocol: Protocols,
) -> Result<()> {
    let protocol_id: u8 = (protocol as usize).try_into().unwrap();
    let token = ctx.accounts.input_mint_pubkey();

    let tvl = ctx.accounts.max_withdrawable()?;
    let protocol_data = ctx.accounts.protocol_data_as_mut(protocol);
    let rewards = tvl.saturating_sub(protocol_data.amount);
    protocol_data.rewards.update(rewards)?;

    emit!(ProtocolRewardsEvent {
        protocol_id,
        token,
        rewards: protocol_data.rewards.amount,
        lamports: protocol_data.rewards.deposited_avg,
        initial_slot: protocol_data.rewards.deposited_integral.initial_slot
    });

    Ok(())
}

#[derive(Accounts)]
pub struct GenericTVLAccounts<'info> {
    #[account(
        mut,
        seeds = [VAULT_ACCOUNT_SEED, vault_account.input_mint_pubkey.as_ref()],
        bump = vault_account.bumps.vault
    )]
    pub vault_account: Box<Account<'info, VaultAccount>>,
}
