use crate::vault::ProtocolData;
use crate::VaultAccount;
use crate::VAULT_ACCOUNT_SEED;
use anchor_lang::prelude::*;

#[event]
pub struct ProtocolRewardsEvent {
    token: Pubkey,
    rewards: u64,
    lamports: u64,
    initial_slot: u64,
}

/// Get the rewards produced by the protocol
pub trait ProtocolRewards<'info> {
    /// Compute the maximam withdrawable units
    fn max_withdrawable(&self) -> Result<u64>;

    /// Return a mutable refrence of the data
    fn protocol_data_as_mut(&mut self) -> &mut ProtocolData;

    /// Get the input token mint pubkey
    fn input_mint_pubkey(&self) -> Pubkey;
}

/// Update the rewards
pub fn handler<'info, T: ProtocolRewards<'info>>(ctx: Context<T>) -> Result<()> {
    let input_token = ctx.accounts.input_mint_pubkey();

    let tvl = ctx.accounts.max_withdrawable()?;
    let protocol = ctx.accounts.protocol_data_as_mut();
    let rewards = tvl.saturating_sub(protocol.amount);
    protocol.rewards.update(rewards)?;

    emit!(ProtocolRewardsEvent {
        token: input_token,
        rewards: protocol.rewards.amount,
        lamports: protocol.rewards.deposited_avg,
        initial_slot: protocol.rewards.deposited_integral.initial_slot
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
