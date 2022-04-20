use crate::check_hash::CHECKHASH_BYTES;
use crate::error::ErrorCode;
use crate::protocols::{Protocols, PROTOCOLS_LEN};
use anchor_lang::prelude::*;
use solana_maths::{U192, WAD};
use std::{
    cmp::{self, Ordering},
    convert::TryInto,
};

// not used yet, only one version available
pub const _VAULT_VERSION: u8 = 1;

#[constant]
pub const WEIGHTS_SCALE: u32 = 10_000;

/// Strategy vault account
#[account]
#[derive(Default)]
pub struct VaultAccount {
    /// Vault version
    pub version: u8,

    /// PDA bump seeds
    pub bumps: Bumps,
    /// Strategy input token mint address
    pub input_mint_pubkey: Pubkey,
    /// Destination fee account
    pub dao_treasury_lp_token_account: Pubkey,

    /// Last refresh slot in which protocol weights were updated
    pub last_refresh_time: i64,

    /// Strategy refresh parameters
    pub refresh: RefreshParams,

    /// Current TVL deposited in the strategy (considering deposits/withdraws)
    pub current_tvl: u64,
    /// Accumulated rewards until fee is minted (not accounted in current_tvl)
    pub rewards_sum: u64,

    /// Price of the LP token in the previous interval
    pub previous_lp_price: LpPrice,

    /// Protocol data (maximum = 10)
    pub protocols: Vec<ProtocolData>,
}

impl VaultAccount {
    pub const SIZE: usize = 1
        + Bumps::SIZE
        + 32
        + 32
        + 8
        + RefreshParams::SIZE
        + 8
        + 8
        + LpPrice::SIZE
        + 4
        + ProtocolData::SIZE * 10;

    /// Initialize a new vault
    pub fn init(params: InitVaultAccountParams) -> Self {
        Self {
            bumps: params.bumps,
            input_mint_pubkey: params.input_mint_pubkey,
            dao_treasury_lp_token_account: params.dao_treasury_lp_token_account,
            refresh: RefreshParams {
                min_elapsed_time: 3000,
                min_deposit_lamports: 0,
            },
            ..Self::default()
        }
    }

    /// Compute the minimum weight
    fn minimum_weight(&self, total_deposit: u128) -> Result<u32> {
        let min_weight = (self.refresh.min_deposit_lamports as u128)
            .checked_mul(WEIGHTS_SCALE.into())
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?
            .checked_div(total_deposit)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?
            .try_into()
            .map_err(|_| ErrorCode::MathOverflow)?;

        Ok(std::cmp::max(1, min_weight))
    }

    /// Update protocol weights
    pub fn update_protocol_weights(&mut self) -> Result<()> {
        let mut deposit: Vec<u128> = self
            .protocols
            .iter()
            .map(|protocol| {
                protocol
                    .rewards
                    .deposited_avg_wad
                    .checked_div(WAD as u128)
                    .unwrap()
            })
            .collect();

        let rewards: Vec<u128> = self
            .protocols
            .iter()
            .map(|protocol| protocol.rewards.amount as u128)
            .collect();

        let total_deposit: u128 = deposit
            .iter()
            .try_fold(0u128, |acc, &x| acc.checked_add(x))
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

        let total_rewards: u128 = rewards
            .iter()
            .try_fold(0u128, |acc, &x| acc.checked_add(x))
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

        if total_deposit != 0 && total_rewards != 0 {
            for i in 0..PROTOCOLS_LEN {
                if self.protocols[i].is_active() {
                    let rewards_wo_i: u128 = total_rewards
                        .checked_sub(rewards[i])
                        .ok_or_else(|| error!(ErrorCode::MathOverflow))?;
                    let deposit_wo_i: u128 = total_deposit
                        .checked_sub(deposit[i])
                        .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

                    let num1: i128 = rewards[i]
                        .checked_mul(deposit_wo_i)
                        .ok_or_else(|| error!(ErrorCode::MathOverflow))?
                        as i128;
                    let num2: i128 = deposit[i]
                        .checked_mul(rewards_wo_i)
                        .ok_or_else(|| error!(ErrorCode::MathOverflow))?
                        as i128;

                    let delta: i128 = (num1
                        .checked_sub(num2)
                        .ok_or_else(|| error!(ErrorCode::MathOverflow))?)
                    .checked_div(total_rewards as i128)
                    .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

                    deposit[i] = (deposit[i] as i128)
                        .checked_add(delta)
                        .ok_or_else(|| error!(ErrorCode::MathOverflow))?
                        as u128;
                }
            }

            // Set at least the minimum weight for the active protocols
            let min_weight = self.minimum_weight(total_deposit)?;

            #[allow(clippy::needless_range_loop)]
            for i in 0..PROTOCOLS_LEN {
                if self.protocols[i].is_active() {
                    self.protocols[i].weight = deposit[i]
                        .checked_mul(WEIGHTS_SCALE.into())
                        .ok_or_else(|| error!(ErrorCode::MathOverflow))?
                        .checked_div(total_deposit)
                        .ok_or_else(|| error!(ErrorCode::MathOverflow))?
                        as u32;

                    self.protocols[i].weight = std::cmp::max(min_weight, self.protocols[i].weight);
                }
            }

            // Renormalize the weights
            let (max_indx, max_protocol) = self
                .protocols
                .iter()
                .enumerate()
                .max_by_key(|&(_, protocol)| protocol.weight)
                .unwrap();

            let weights_sum: u32 = self
                .protocols
                .iter()
                .try_fold(0_u32, |acc, &protocol| acc.checked_add(protocol.weight))
                .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

            self.protocols[max_indx].weight = WEIGHTS_SCALE
                .checked_sub(
                    weights_sum
                        .checked_sub(max_protocol.weight)
                        .ok_or_else(|| error!(ErrorCode::MathOverflow))?,
                )
                .ok_or_else(|| error!(ErrorCode::MathOverflow))?;
        }

        Ok(())
    }

    /// Calculate amount to deposit in the given protocol
    pub fn calculate_deposit(&self, protocol: Protocols, available_amount: u64) -> Result<u64> {
        let protocol = &self.protocols[protocol as usize];

        let deposited_amount = protocol.amount;
        let target_amount = protocol.amount_should_be_deposited(self.current_tvl)?;

        if target_amount > deposited_amount {
            let amount = target_amount
                .checked_sub(deposited_amount)
                .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

            Ok(cmp::min(amount, available_amount))
        } else {
            err!(ErrorCode::InvalidProtocolDeposit)
        }
    }

    /// Calculate amount to withdraw from the given protocol
    pub fn calculate_withdraw(&self, protocol: Protocols) -> Result<u64> {
        let protocol = &self.protocols[protocol as usize];

        let deposited_amount = protocol.amount;
        let target_amount = protocol.amount_should_be_deposited(self.current_tvl)?;

        if target_amount < deposited_amount {
            let amount = deposited_amount
                .checked_sub(target_amount)
                .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

            Ok(amount)
        } else {
            err!(ErrorCode::InvalidProtocolWithdraw)
        }
    }
}

/// Initialize a new vault
pub struct InitVaultAccountParams {
    /// PDA bump seeds
    pub bumps: Bumps,
    /// Strategy input token mint address
    pub input_mint_pubkey: Pubkey,
    /// Destination fee account
    pub dao_treasury_lp_token_account: Pubkey,
}

/// PDA bump seeds
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default)]
pub struct Bumps {
    pub vault: u8,
    pub lp_token_mint: u8,
}

impl Bumps {
    pub const SIZE: usize = 1 + 1;
}

/// Strategy refresh parameters
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default)]
pub struct RefreshParams {
    /// Minimum elapsed slots for updating the protocol weights
    pub min_elapsed_time: i64,
    /// Minimum amount of lamports to deposit in each protocol
    pub min_deposit_lamports: u64,
}

impl RefreshParams {
    pub const SIZE: usize = 8 + 8;
}

/// Protocol data
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default)]
pub struct ProtocolData {
    /// Hashes of Pubkey
    pub hash_pubkey: HashPubkey,

    /// Percentage of the TVL that should be deposited in the protocol
    pub weight: u32,
    /// Deposited token amount in the protocol
    pub amount: u64,
    /// Accumulated rewards
    pub rewards: AccumulatedRewards,

    /// Padding for other future field
    pub _padding: [u64; 4],
}

impl ProtocolData {
    pub const SIZE: usize = HashPubkey::SIZE + 4 + 8 + AccumulatedRewards::SIZE + 8 * 4;

    /// Check the protocol is active
    pub fn is_active(&self) -> bool {
        self.weight != u32::default()
    }

    /// Set the protocol pubkey hashes
    pub fn set_hashes(&mut self, hashes: [[u8; CHECKHASH_BYTES]; 3]) {
        self.hash_pubkey.hash_deposit = hashes[0];
        self.hash_pubkey.hash_withdraw = hashes[1];
        self.hash_pubkey.hash_tvl = hashes[2];
    }

    /// Amount that should be deposited according to the weight
    fn amount_should_be_deposited(&self, total_amount: u64) -> Result<u64> {
        let amount: u64 = (total_amount as u128)
            .checked_mul(self.weight as u128)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?
            .checked_div(WEIGHTS_SCALE.into())
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?
            .try_into()
            .map_err(|_| ErrorCode::MathOverflow)?;
        Ok(amount)
    }

    /// Update the protocol tvl with the generated rewards
    pub fn update_tvl(&mut self) -> Result<()> {
        self.amount = self
            .amount
            .checked_add(self.rewards.amount)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;
        self.rewards.amount = 0_u64;
        Ok(())
    }

    /// Update token amount after depositing in the protocol
    pub fn update_after_deposit(&mut self, amount: u64) -> Result<()> {
        self.rewards.deposited_integral.accumulate(self.amount)?;
        self.amount = self
            .amount
            .checked_add(amount)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

        Ok(())
    }

    /// Update token amount after withdrawing from the protocol
    pub fn update_after_withdraw(&mut self, amount: u64) -> Result<()> {
        self.rewards.deposited_integral.accumulate(self.amount)?;
        self.amount = self
            .amount
            .checked_sub(amount)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default)]
pub struct HashPubkey {
    /// Hash of important accounts for each protocol on deposit
    pub hash_deposit: [u8; CHECKHASH_BYTES],
    /// Hash of important accounts for each protocol on withdraw
    pub hash_withdraw: [u8; CHECKHASH_BYTES],
    /// Hash of important accounts for each protocol on tvl
    pub hash_tvl: [u8; CHECKHASH_BYTES],
    // TODO additional padding
}

impl HashPubkey {
    pub const SIZE: usize = CHECKHASH_BYTES * 3;
}

/// Generated rewards
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default)]
pub struct AccumulatedRewards {
    /// Last slot the rewards were accumulated
    pub last_slot: u64,
    /// Last accumulated rewards
    pub amount: u64,
    /// Slot-average deposited amount that generates these rewards
    pub deposited_avg_wad: u128,
    /// Slot-integrated deposited amount
    pub deposited_integral: SlotIntegrated,
}

impl AccumulatedRewards {
    pub const SIZE: usize = 8 + 8 + 16 + SlotIntegrated::SIZE;

    /// Update the rewards
    pub fn update(&mut self, rewards: u64, deposited_amount: u64) -> Result<()> {
        let current_slot = Clock::get()?.slot;
        self.last_slot = current_slot;
        self.amount = rewards;
        self.deposited_avg_wad = self
            .deposited_integral
            .get_average_wad(current_slot, deposited_amount)?;
        Ok(())
    }

    /// Reset the initegral from the last rewards values
    pub fn reset_integral(&mut self) -> Result<()> {
        let elapsed_slots_while_rewards = self
            .last_slot
            .checked_sub(self.deposited_integral.initial_slot)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

        let acc_at_rewards: u128 = (U192::from(self.deposited_avg_wad))
            .checked_mul(U192::from(elapsed_slots_while_rewards))
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?
            .checked_div(U192::from(WAD))
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?
            .as_u128();

        let acc_since_last_rewards = self
            .deposited_integral
            .accumulator
            .checked_sub(acc_at_rewards)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

        self.deposited_integral.accumulator = acc_since_last_rewards;
        self.deposited_integral.initial_slot = self.last_slot;

        Ok(())
    }
}

/// Slot-integrated quantities
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default, Debug, PartialEq)]
pub struct SlotIntegrated {
    /// Initial slot from which the integral starts
    pub initial_slot: u64,
    /// Last slot the integral was updated
    pub last_slot: u64,
    /// Summation accumulator
    pub accumulator: u128,
}

impl SlotIntegrated {
    pub const SIZE: usize = 8 + 8 + 16;

    /// Update the summation accumulator
    pub fn accumulate(&mut self, amount: u64) -> Result<()> {
        let current_slot = Clock::get()?.slot;
        let elapsed_slots = current_slot
            .checked_sub(self.last_slot)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

        let interval_avg: u128 = (elapsed_slots as u128)
            .checked_mul(amount as u128)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

        self.accumulator = self
            .accumulator
            .checked_add(interval_avg)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;
        self.last_slot = current_slot;

        Ok(())
    }

    /// Compute the average value scaled by WAD
    pub fn get_average_wad(&mut self, current_slot: u64, deposited_amount: u64) -> Result<u128> {
        self.accumulate(deposited_amount)?;

        let elapsed_slots = current_slot
            .checked_sub(self.initial_slot)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

        let avg: u128 = (U192::from(self.accumulator))
            .checked_mul(U192::from(WAD))
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?
            .checked_div(U192::from(elapsed_slots))
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?
            .as_u128();

        Ok(avg)
    }
}

/// Strategy LP token price
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default, Debug, PartialEq)]
pub struct LpPrice {
    /// Total amount of tokens to be distributed
    pub total_tokens: u64,
    /// Supply of strategy LP tokens
    pub minted_tokens: u64,
}

impl LpPrice {
    pub const SIZE: usize = 8 + 8;

    /// Transform input token amount to LP amount
    pub fn token_to_lp(&self, amount: u64) -> Result<u64> {
        if self.minted_tokens == 0 {
            Ok(amount)
        } else {
            Ok((amount as u128)
                .checked_mul(self.minted_tokens as u128)
                .ok_or_else(|| error!(ErrorCode::MathOverflow))?
                .checked_div(self.total_tokens as u128)
                .ok_or_else(|| error!(ErrorCode::MathOverflow))?
                .try_into()
                .map_err(|_| ErrorCode::MathOverflow)?)
        }
    }

    /// Transform LP amount to input token amount
    pub fn lp_to_token(&self, lp_amount: u64) -> Result<u64> {
        if self.minted_tokens == 0 {
            Ok(lp_amount)
        } else {
            Ok((lp_amount as u128)
                .checked_mul(self.total_tokens as u128)
                .ok_or_else(|| error!(ErrorCode::MathOverflow))?
                .checked_div(self.minted_tokens as u128)
                .ok_or_else(|| error!(ErrorCode::MathOverflow))?
                .try_into()
                .map_err(|_| ErrorCode::MathOverflow)?)
        }
    }
}

impl PartialOrd for LpPrice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let lhs = (self.total_tokens as u128)
            .checked_mul(other.minted_tokens as u128)
            .unwrap();

        let rhs = (other.total_tokens as u128)
            .checked_mul(self.minted_tokens as u128)
            .unwrap();

        lhs.partial_cmp(&rhs)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_update_protocol_weights() {
        let mut vault = VaultAccount::default();
        vault.protocols = vec![ProtocolData::default(); PROTOCOLS_LEN];

        let protocols = &mut vault.protocols;
        protocols[Protocols::Mango as usize] = ProtocolData {
            weight: 1,
            rewards: AccumulatedRewards {
                amount: 21,
                deposited_avg_wad: 66709779 * (WAD as u128),
                ..AccumulatedRewards::default()
            },
            ..ProtocolData::default()
        };

        protocols[Protocols::Solend as usize] = ProtocolData {
            weight: 1,
            rewards: AccumulatedRewards {
                amount: 3521693,
                deposited_avg_wad: 666831006405 * (WAD as u128),
                ..AccumulatedRewards::default()
            },
            ..ProtocolData::default()
        };

        protocols[Protocols::Port as usize] = ProtocolData {
            weight: 1,
            rewards: AccumulatedRewards {
                amount: 139,
                deposited_avg_wad: 66709780 * (WAD as u128),
                ..AccumulatedRewards::default()
            },
            ..ProtocolData::default()
        };

        protocols[Protocols::Francium as usize] = ProtocolData {
            weight: 1,
            rewards: AccumulatedRewards {
                amount: 532,
                deposited_avg_wad: 66709785 * (WAD as u128),
                ..AccumulatedRewards::default()
            },
            ..ProtocolData::default()
        };

        protocols[Protocols::Tulip as usize] = ProtocolData {
            weight: 1,
            rewards: AccumulatedRewards {
                amount: 318,
                deposited_avg_wad: 66709783 * (WAD as u128),
                ..AccumulatedRewards::default()
            },
            ..ProtocolData::default()
        };

        vault.update_protocol_weights().unwrap();
        vault
            .protocols
            .iter()
            .for_each(|protocol| println!("weigth {}", protocol.weight));
    }
}
