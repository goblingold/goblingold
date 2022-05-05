use crate::check_hash::CHECKHASH_BYTES;

use anchor_lang::prelude::*;

/// Strategy vault account
#[account]
#[derive(Default)]
pub struct OldVaultAccount {
    /// Vault version
    pub version: u8,

    /// PDA bump seeds
    pub bumps: OldBumps,
    /// Strategy input token mint address
    pub input_mint_pubkey: Pubkey,
    /// Destination fee account
    pub dao_treasury_lp_token_account: Pubkey,

    /// Last refresh slot in which protocol weights were updated
    pub last_refresh_time: i64,

    /// Strategy refresh parameters
    pub refresh: OldRefreshParams,

    /// Current TVL deposited in the strategy (considering deposits/withdraws)
    pub current_tvl: u64,
    /// Accumulated rewards until fee is minted (not accounted in current_tvl)
    pub rewards_sum: u64,

    /// Price of the LP token in the previous interval
    pub previous_lp_price: OldLpPrice,

    /// Protocol data (maximum = 10)
    pub protocols: Vec<OldProtocolData>,
}

/// PDA bump seeds
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default)]
pub struct OldBumps {
    pub vault: u8,
    pub lp_token_mint: u8,
}

/// Strategy refresh parameters
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default)]
pub struct OldRefreshParams {
    /// Minimum elapsed slots for updating the protocol weights
    pub min_elapsed_time: i64,
    /// Minimum amount of lamports to deposit in each protocol
    pub min_deposit_lamports: u64,
}

/// Protocol data
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default)]
pub struct OldProtocolData {
    /// Hashes of Pubkey
    pub hash_pubkey: OldHashPubkey,

    /// Percentage of the TVL that should be deposited in the protocol
    pub weight: u32,
    /// Deposited token amount in the protocol
    pub amount: u64,
    /// Accumulated rewards
    pub rewards: OldAccumulatedRewards,

    /// Protocol ID
    pub protocol_id: u8,

    /// Padding for other future field
    pub _padding: [u8; 31],
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default)]
pub struct OldHashPubkey {
    /// Hash of important accounts for each protocol on deposit
    pub hash_deposit: [u8; CHECKHASH_BYTES],
    /// Hash of important accounts for each protocol on withdraw
    pub hash_withdraw: [u8; CHECKHASH_BYTES],
    /// Hash of important accounts for each protocol on tvl
    pub hash_tvl: [u8; CHECKHASH_BYTES],
    // TODO additional padding
}

/// Generated rewards
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default)]
pub struct OldAccumulatedRewards {
    /// Last slot the rewards were accumulated
    pub last_slot: u64,
    /// Last accumulated rewards
    pub amount: u64,
    /// Slot-average deposited amount that generates these rewards
    pub deposited_avg_wad: u128,
    /// Slot-integrated deposited amount
    pub deposited_integral: OldSlotIntegrated,
}

/// Slot-integrated quantities
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default)]
pub struct OldSlotIntegrated {
    /// Initial slot from which the integral starts
    pub initial_slot: u64,
    /// Last slot the integral was updated
    pub last_slot: u64,
    /// Summation accumulator
    pub accumulator: u128,
}

/// Strategy LP token price
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default, Debug, PartialEq)]
pub struct OldLpPrice {
    /// Total amount of tokens to be distributed
    pub total_tokens: u64,
    /// Supply of strategy LP tokens
    pub minted_tokens: u64,
}
