use crate::error::ErrorCode;
use crate::protocols::{Protocols, PROTOCOLS_LEN};
use crate::SetHash;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hashv;
use std::{cmp, convert::TryInto};

/// Strategy vault account
#[account]
#[derive(Default)]
pub struct VaultAccount {
    /// PDA bump seeds
    pub bumps: Bumps,
    /// Strategy input token mint address
    pub input_mint_pubkey: Pubkey,
    /// Destination fee account
    pub dao_treasury_lp_token_account: Pubkey,
    /// Current TVL deposited in the strategy (considering deposits/withdraws)
    pub current_tvl: u64,
    /// Last refresh slot in which protocol rewards were accrued
    pub last_refresh_slot: u64,
    /// Price of the LP token in the previous interval
    pub previous_lp_price: LpPrice,
    /// Accumulated rewards until fee is minted (not accounted in current_tvl)
    pub rewards_sum: u64,
    /// Protocol data
    pub protocols: [ProtocolData; PROTOCOLS_LEN],
}

pub fn hash_pub_keys(keys: &[&[u8]]) -> Result<[u8; 8]> {
    let hash = hashv(keys).to_bytes();
    Ok(hash[0..8].try_into().map_err(|_| ErrorCode::MathOverflow)?)
}

impl VaultAccount {
    /// Initialize a new vault
    pub fn init(params: InitVaultAccountParams) -> Self {
        Self {
            bumps: params.bumps,
            input_mint_pubkey: params.input_mint_pubkey,
            dao_treasury_lp_token_account: params.dao_treasury_lp_token_account,
            ..Self::default()
        }
    }

    /// Update protocol weights
    pub fn update_protocol_weights(&mut self) -> Result<()> {
        let mut deposit: Vec<u128> = self
            .protocols
            .iter()
            .map(|protocol| protocol.rewards.deposited_avg as u128)
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
                if self.protocols[i].is_used() {
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

            // If one of the non-zero weights is zero now, set to one so all the used protocols get
            // deposited
            #[allow(clippy::needless_range_loop)]
            for i in 0..PROTOCOLS_LEN {
                if self.protocols[i].is_used() {
                    self.protocols[i].weight = deposit[i]
                        .checked_mul(1000)
                        .ok_or_else(|| error!(ErrorCode::MathOverflow))?
                        .checked_div(total_deposit)
                        .ok_or_else(|| error!(ErrorCode::MathOverflow))?
                        as u16;

                    if self.protocols[i].weight == 0 {
                        self.protocols[i].weight = 1
                    }
                }
            }

            // Renormalize the weights
            let (max_indx, max_protocol) = self
                .protocols
                .iter()
                .enumerate()
                .max_by_key(|&(_, protocol)| protocol.weight)
                .unwrap();

            let weights_sum: u16 = self
                .protocols
                .iter()
                .try_fold(0_u16, |acc, &protocol| acc.checked_add(protocol.weight))
                .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

            self.protocols[max_indx].weight = 1000_u16
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

        let deposited_amount = protocol.tokens.base_amount;
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

        let deposited_amount = protocol.tokens.base_amount;
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

/// Protocol data
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default)]
pub struct ProtocolData {
    /// Percentage of the TVL that should be deposited in the protocol
    pub weight: u16,
    /// Token balances in the protocol
    pub tokens: TokenBalances,
    /// Accumulated rewards
    pub rewards: AccumulatedRewards,
    /// Hashes of Pubkey
    pub hash_pubkey: HashPubkey,
}

impl ProtocolData {
    /// Check the protocol is used
    pub fn is_used(&self) -> bool {
        self.weight != u16::default()
    }

    /// Amount that should be deposited according to the weight
    fn amount_should_be_deposited(&self, total_amount: u64) -> Result<u64> {
        let amount: u64 = (total_amount as u128)
            .checked_mul(self.weight as u128)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?
            .checked_div(1000_u128)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?
            .try_into()
            .map_err(|_| ErrorCode::MathOverflow)?;
        Ok(amount)
    }

    /// Update the protocol tvl with the generated rewards
    pub fn update_tvl(&mut self) -> Result<()> {
        self.tokens.base_amount = self
            .tokens
            .base_amount
            .checked_add(self.rewards.amount)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;
        self.rewards.amount = 0_u64;
        Ok(())
    }

    /// Update token amounts after depositing in the protocol
    pub fn update_after_deposit(
        &mut self,
        current_slot: u64,
        balances: TokenBalances,
    ) -> Result<()> {
        self.rewards
            .deposited_integral
            .accumulate(current_slot, self.tokens.base_amount)?;
        self.tokens = self.tokens.add(balances)?;
        Ok(())
    }

    /// Update token amounts after withdrawing from the protocol
    pub fn update_after_withdraw(
        &mut self,
        current_slot: u64,
        balances: TokenBalances,
    ) -> Result<()> {
        self.rewards
            .deposited_integral
            .accumulate(current_slot, self.tokens.base_amount)?;
        self.tokens = self.tokens.sub(balances)?;
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default)]
pub struct HashPubkey {
    /// Hash of important accounts for each protocol on deposit
    pub hash_deposit: [u8; 8],
    /// Hash of important accounts for each protocol on withdraw
    pub hash_withdraw: [u8; 8],
    /// Hash of important accounts for each protocol on tvl
    pub hash_tvl: [u8; 8],
    // TODO additional padding
}

impl<'info> SetHash<'info> {
    pub fn set_hash(&mut self, protocol: usize, action: String, hash: [u8; 8]) -> Result<()> {
        match action.as_str() {
            "Danchor" => {
                self.vault_account.protocols[protocol]
                    .hash_pubkey
                    .hash_deposit = hash
            }
            "W" => {
                self.vault_account.protocols[protocol]
                    .hash_pubkey
                    .hash_withdraw = hash
            }
            "T" => self.vault_account.protocols[protocol].hash_pubkey.hash_tvl = hash,
            _ => return Err(ErrorCode::InvalidInstructions.into())
        }
        Ok(())
    }
}

/// Token balances in the protocol
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default)]
pub struct TokenBalances {
    /// Input tokens deposited in the protocol
    pub base_amount: u64,
    /// LP tokens returned by the protocol
    pub lp_amount: u64,
}

impl TokenBalances {
    pub fn add(&self, rhs: Self) -> Result<Self> {
        let base_amount = self
            .base_amount
            .checked_add(rhs.base_amount)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

        let lp_amount = self
            .lp_amount
            .checked_add(rhs.lp_amount)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

        Ok(Self {
            base_amount,
            lp_amount,
        })
    }

    pub fn sub(&self, rhs: Self) -> Result<Self> {
        let base_amount = self
            .base_amount
            .checked_sub(rhs.base_amount)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

        let lp_amount = self
            .lp_amount
            .checked_sub(rhs.lp_amount)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

        Ok(Self {
            base_amount,
            lp_amount,
        })
    }
}

/// Generated rewards
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default)]
pub struct AccumulatedRewards {
    /// Last slot the rewards were accumulated
    pub last_slot: u64,
    /// Last accumulated rewards
    pub amount: u64,
    /// Slot-average deposited amount that generates these rewards
    pub deposited_avg: u64,
    /// Slot-integrated deposited amount
    pub deposited_integral: SlotIntegrated,
}

impl AccumulatedRewards {
    /// Update the rewards
    pub fn update(&mut self, rewards: u64) -> Result<()> {
        let current_slot = Clock::get()?.slot;
        self.last_slot = current_slot;
        self.amount = rewards;
        self.deposited_avg = self.deposited_integral.get_average(current_slot)?;
        Ok(())
    }

    /// Reset the initegral from the last rewards values
    pub fn reset_integral(&mut self) -> Result<()> {
        let elapsed_slots_while_rewards = self
            .last_slot
            .checked_sub(self.deposited_integral.initial_slot)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

        let acc_at_rewards = (self.deposited_avg as u128)
            .checked_mul(elapsed_slots_while_rewards as u128)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

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
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default)]
pub struct SlotIntegrated {
    /// Initial slot from which the integral starts
    pub initial_slot: u64,
    /// Last slot the integral was updated
    pub last_slot: u64,
    /// Summation accumulator
    pub accumulator: u128,
}

impl SlotIntegrated {
    /// Update the summation accumulator
    pub fn accumulate(&mut self, current_slot: u64, amount: u64) -> Result<()> {
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

    /// Compute the average value
    pub fn get_average(&self, current_slot: u64) -> Result<u64> {
        let elapsed_slots = current_slot
            .checked_sub(self.initial_slot)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

        let avg: u64 = self
            .accumulator
            .checked_div(elapsed_slots as u128)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?
            .try_into()
            .map_err(|_| ErrorCode::MathOverflow)?;

        Ok(avg)
    }
}

/// Strategy LP token price
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default, Debug)]
pub struct LpPrice {
    /// Total amount of tokens to be distributed
    pub total_tokens: u64,
    /// Supply of strategy LP tokens
    pub minted_tokens: u64,
}

impl LpPrice {
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

    /// Returns true if self price > previous_price
    pub fn greater_than(&self, previous_price: LpPrice) -> Result<bool> {
        let lhs = (self.total_tokens as u128)
            .checked_mul(previous_price.minted_tokens as u128)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;
        let lhr = (previous_price.total_tokens as u128)
            .checked_mul(self.minted_tokens as u128)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;
        Ok(lhs > lhr)
    }
}
