use crate::vault::ProtocolData;
use crate::VaultAccount;
use crate::VAULT_ACCOUNT_SEED;
use anchor_lang::prelude::*;

/// Get the rewards produced by the protocol
pub trait ProtocolRewards<'info> {
    /// Compute the maximam withdrawable units
    fn max_withdrawable(&self) -> Result<u64>;

    /// Return a mutable refrence of the data
    fn protocol_data_as_mut(&mut self) -> &mut ProtocolData;
}

/// Update the rewards
pub fn handler<'info, T: ProtocolRewards<'info>>(ctx: Context<T>) -> Result<()> {
    let tvl = ctx.accounts.max_withdrawable()?;
    let protocol = ctx.accounts.protocol_data_as_mut();
    let rewards = tvl.saturating_sub(protocol.amount);
    protocol.rewards.update(rewards)?;
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
