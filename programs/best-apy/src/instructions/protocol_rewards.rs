use crate::error::ErrorCode;
use crate::protocols::Protocols;
use crate::vault::ProtocolData;
use crate::VaultAccount;
use crate::VAULT_ACCOUNT_SEED;
use anchor_lang::prelude::*;
use solana_maths::WAD;
use std::convert::TryFrom;
use std::convert::TryInto;

#[event]
pub struct ProtocolRewardsEvent {
    protocol_id: u8,
    token: Pubkey,
    rewards: i64,
    lamports: u64,
    initial_slot: u64,
}

/// Get the rewards produced by the protocol
pub trait ProtocolRewards<'info> {
    /// Return the protcol position in the vector
    fn protocol_position(&self, protocol: Protocols) -> Result<usize>;

    /// Get the input token mint pubkey
    fn input_mint_pubkey(&self) -> Pubkey;

    /// Return a mutable refrence of the data
    fn protocol_data_as_mut(&mut self, protocol_idx: usize) -> &mut ProtocolData;

    /// Compute the maximam withdrawable units
    fn max_withdrawable(&self) -> Result<u64>;
}

/// Update the rewards
pub fn handler<'info, T: ProtocolRewards<'info>>(
    ctx: Context<T>,
    protocol: Protocols,
) -> Result<()> {
    let protocol_idx = ctx.accounts.protocol_position(protocol)?;
    let protocol_id: u8 = (protocol as usize).try_into().unwrap();
    let token = ctx.accounts.input_mint_pubkey();

    let tvl = ctx.accounts.max_withdrawable()?;

    let protocol_data = ctx.accounts.protocol_data_as_mut(protocol_idx);
    let rewards: i64 = i64::try_from(tvl)
        .unwrap()
        .checked_sub(i64::try_from(protocol_data.amount).unwrap())
        .unwrap();

    protocol_data
        .rewards
        .update(rewards, protocol_data.amount)?;

    let deposited_lamports: u64 = protocol_data
        .rewards
        .deposited_avg_wad
        .checked_div(WAD as u128)
        .ok_or_else(|| error!(ErrorCode::MathOverflow))?
        .try_into()
        .map_err(|_| ErrorCode::MathOverflow)?;

    emit!(ProtocolRewardsEvent {
        protocol_id,
        token,
        rewards: protocol_data.rewards.amount,
        lamports: deposited_lamports,
        initial_slot: protocol_data.rewards.deposited_integral.initial_slot
    });

    Ok(())
}

#[derive(Accounts)]
pub struct GenericTVLAccounts<'info> {
    #[account(
        mut,
        seeds = [VAULT_ACCOUNT_SEED, &[vault_account.seed_number][..], vault_account.input_mint_pubkey.as_ref()],
        bump = vault_account.bumps.vault
    )]
    pub vault_account: Box<Account<'info, VaultAccount>>,
}
