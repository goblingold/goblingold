use crate::error::ErrorCode;
use crate::protocols::{Protocols, PROTOCOLS_LEN};
use anchor_lang::prelude::*;
use std::{cmp, convert::TryInto};

/// Strategy vault account
#[account]
#[derive(Default)]
pub struct VaultAccount {
    /// PDA bump seed
    pub bump: u8,
    /// Strategy input token mint address
    pub input_mint_pubkey: Pubkey,
    /// Strategy LP token mint address
    pub vault_lp_token_mint_pubkey: Pubkey,
    /// Destination fee account
    pub dao_treasury_lp_token_account: Pubkey,
    /// Current TVL deposited in the strategy (considering deposits/withdraws)
    pub current_tvl: u64,
    /// Last refresh slot in which protocol rewards were accrued
    pub last_refresh_slot: u64,
    /// Price of the LP token in the previous interval
    pub previous_lp_price: LpPrice,
    /// Protocol data
    pub protocols: [ProtocolData; PROTOCOLS_LEN],
    // TODO additional padding
}

impl VaultAccount {
    /// Initialize a new vault
    pub fn init(params: InitVaultAccountParams) -> Self {
        Self {
            bump: params.bump,
            input_mint_pubkey: params.input_mint_pubkey,
            vault_lp_token_mint_pubkey: params.vault_lp_token_mint_pubkey,
            dao_treasury_lp_token_account: params.dao_treasury_lp_token_account,
            ..Self::default()
        }
    }

    /// Update protocol weights
    pub fn update_protocol_weights(&mut self, elapsed_slots: u64) -> Result<()> {
        let mut deposit: Vec<u128> = self
            .protocols
            .iter()
            .map(|protocol| protocol.deposited.get_average(elapsed_slots).unwrap())
            .collect();

        let rewards: Vec<u128> = self
            .protocols
            .iter()
            .zip(&deposit)
            .map(|(protocol, &dep)| {
                (protocol.tvl.amount as u128)
                    .checked_sub(dep)
                    .ok_or(ErrorCode::MathOverflow)
                    .unwrap()
            })
            .collect();

        let total_deposit: u128 = deposit
            .iter()
            .try_fold(0u128, |acc, &x| acc.checked_add(x))
            .ok_or(ErrorCode::MathOverflow)?;

        let total_rewards: u128 = rewards
            .iter()
            .try_fold(0u128, |acc, &x| acc.checked_add(x))
            .ok_or(ErrorCode::MathOverflow)?;

        if total_deposit == 0 || total_rewards == 0 {
            self.protocols
                .iter_mut()
                .for_each(|protocol| protocol.weight = 1000 / (PROTOCOLS_LEN as u16));
        } else {
            for i in 0..PROTOCOLS_LEN {
                let rewards_wo_i: u128 = total_rewards
                    .checked_sub(rewards[i])
                    .ok_or(ErrorCode::MathOverflow)?;
                let deposit_wo_i: u128 = total_deposit
                    .checked_sub(deposit[i])
                    .ok_or(ErrorCode::MathOverflow)?;

                let num1: i128 = rewards[i]
                    .checked_mul(deposit_wo_i)
                    .ok_or(ErrorCode::MathOverflow)? as i128;
                let num2: i128 = deposit[i]
                    .checked_mul(rewards_wo_i)
                    .ok_or(ErrorCode::MathOverflow)? as i128;

                let delta: i128 = (num1.checked_sub(num2).ok_or(ErrorCode::MathOverflow)?)
                    .checked_div(total_rewards as i128)
                    .ok_or(ErrorCode::MathOverflow)?;

                deposit[i] = (deposit[i] as i128)
                    .checked_add(delta)
                    .ok_or(ErrorCode::MathOverflow)? as u128;
            }

            // If one weight is zero, set to one so all protocols get deposited
            #[allow(clippy::needless_range_loop)]
            for i in 0..PROTOCOLS_LEN {
                self.protocols[i].weight = deposit[i]
                    .checked_mul(1000)
                    .ok_or(ErrorCode::MathOverflow)?
                    .checked_div(total_deposit)
                    .ok_or(ErrorCode::MathOverflow)?
                    as u16;
                if self.protocols[i].weight == 0 {
                    self.protocols[i].weight = 1
                }
            }

            let (max_indx, max_protocol) = self
                .protocols
                .iter()
                .enumerate()
                .max_by_key(|&(_, protocol)| protocol.weight)
                .unwrap();

            let total_weights: u16 = self
                .protocols
                .iter()
                .try_fold(0_u16, |acc, &protocol| acc.checked_add(protocol.weight))
                .ok_or(ErrorCode::MathOverflow)?;

            self.protocols[max_indx].weight = 1000_u16
                .checked_sub(
                    total_weights
                        .checked_sub(max_protocol.weight)
                        .ok_or(ErrorCode::MathOverflow)?,
                )
                .ok_or(ErrorCode::MathOverflow)?;
        }

        Ok(())
    }

    /// Calculate amount to deposit in the given protocol
    pub fn calculate_deposit(&self, protocol: Protocols, vault_amount: u64) -> Result<u64> {
        let indx: usize = protocol as usize;

        let amount: i128 = (self.current_tvl as i128)
            .checked_mul(self.protocols[indx].weight as i128)
            .ok_or(ErrorCode::MathOverflow)?
            .checked_div(1000)
            .ok_or(ErrorCode::MathOverflow)?
            .checked_sub(self.protocols[indx].deposited.amount as i128)
            .ok_or(ErrorCode::MathOverflow)?;

        if amount <= 0 {
            err!(ErrorCode::InvalidProtocolDeposit)
        } else {
            Ok(cmp::min(vault_amount, amount as u64))
        }
    }

    /// Calculate amount to withdraw from the given protocol
    pub fn calculate_withdraw(&self, protocol: Protocols) -> Result<u64> {
        let indx: usize = protocol as usize;

        let amount: i128 = (self.current_tvl as i128)
            .checked_mul(self.protocols[indx].weight as i128)
            .ok_or(ErrorCode::MathOverflow)?
            .checked_div(1000)
            .ok_or(ErrorCode::MathOverflow)?
            .checked_sub(self.protocols[indx].deposited.amount as i128)
            .ok_or(ErrorCode::MathOverflow)?;

        if amount <= 0 {
            Ok(amount.abs() as u64)
        } else {
            err!(ErrorCode::InvalidProtocolWithdraw)
        }
    }
}

/// Initialize a new vault
pub struct InitVaultAccountParams {
    /// PDA bump seed
    pub bump: u8,
    /// Strategy input token mint address
    pub input_mint_pubkey: Pubkey,
    /// Strategy LP token mint address
    pub vault_lp_token_mint_pubkey: Pubkey,
    /// Destination fee account
    pub dao_treasury_lp_token_account: Pubkey,
}

/// Protocol data
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default)]
pub struct ProtocolData {
    /// Percentage of the TVL that should be deposited in the protocol
    pub weight: u16,
    /// Returned LP tokens
    pub lp_amount: u64,
    /// Time-average deposited amount
    pub deposited: SlotAverage,
    /// Slot-updated TVL
    pub tvl: UpdatedAmount,
}

impl ProtocolData {
    /// Initialize the average deposited amount
    pub fn initialize_average(&mut self) {
        self.deposited.slot = self.tvl.slot;
        self.deposited.amount = self.tvl.amount;
        self.deposited.avg_sum = 0_u128;
    }

    /// Update token amounts after depositing in the protocol
    pub fn update_after_deposit(
        &mut self,
        current_slot: u64,
        balances: TokenBalances,
    ) -> Result<()> {
        self.deposited.update_average(current_slot)?;
        self.deposited.amount = self
            .deposited
            .amount
            .checked_add(balances.amount)
            .ok_or(ErrorCode::MathOverflow)?;
        self.lp_amount = self
            .lp_amount
            .checked_add(balances.lp_amount)
            .ok_or(ErrorCode::MathOverflow)?;

        Ok(())
    }

    /// Update token amounts after withdrawing from the protocol
    pub fn update_after_withdraw(
        &mut self,
        current_slot: u64,
        balances: TokenBalances,
    ) -> Result<()> {
        self.deposited.update_average(current_slot)?;
        self.deposited.amount = self
            .deposited
            .amount
            .checked_sub(balances.amount)
            .ok_or(ErrorCode::MathOverflow)?;
        self.lp_amount = self
            .lp_amount
            .checked_sub(balances.lp_amount)
            .ok_or(ErrorCode::MathOverflow)?;

        Ok(())
    }
}

/// Time-average quantities
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default)]
pub struct SlotAverage {
    /// Last slot the average was updated
    pub slot: u64,
    /// Current amount
    pub amount: u64,
    /// Average summation
    pub avg_sum: u128,
}

impl SlotAverage {
    /// Update the average summation
    pub fn update_average(&mut self, current_slot: u64) -> Result<()> {
        let elapsed_slots = current_slot
            .checked_sub(self.slot)
            .ok_or(ErrorCode::MathOverflow)?;
        let interval_avg: u128 = (elapsed_slots as u128)
            .checked_mul(self.amount as u128)
            .ok_or(ErrorCode::MathOverflow)?;

        self.avg_sum = self
            .avg_sum
            .checked_add(interval_avg)
            .ok_or(ErrorCode::MathOverflow)?;
        self.slot = current_slot;
        Ok(())
    }

    /// Compute the average vaule
    pub fn get_average(&self, elapsed_slots: u64) -> Result<u128> {
        Ok(self
            .avg_sum
            .checked_div(elapsed_slots as u128)
            .ok_or(ErrorCode::MathOverflow)?)
    }
}

/// Slot-updated amounts
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default)]
pub struct UpdatedAmount {
    /// Last slot the amount was updated
    pub slot: u64,
    /// Amount value
    pub amount: u64,
}

/// Strategy LP token price
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default)]
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
                .ok_or(ErrorCode::MathOverflow)?
                .checked_div(self.total_tokens as u128)
                .ok_or(ErrorCode::MathOverflow)?
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
                .ok_or(ErrorCode::MathOverflow)?
                .checked_div(self.minted_tokens as u128)
                .ok_or(ErrorCode::MathOverflow)?
                .try_into()
                .map_err(|_| ErrorCode::MathOverflow)?)
        }
    }
}

/// Token balances in the protocols
pub struct TokenBalances {
    /// Input tokens deposited in the protocol
    pub amount: u64,
    /// LP tokens returned by the protocol
    pub lp_amount: u64,
}
