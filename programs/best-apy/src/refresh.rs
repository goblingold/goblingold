use crate::error::ErrorCode;
use crate::macros::generate_seeds;
use crate::vault::LpPrice;
use crate::RefreshRewardsWeights;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo};
use std::convert::TryInto;

// TODO check these limits
/// Minimum elapsed slots for refresh the protocol weights
const MIN_ELAPSED_SLOTS_FOR_REFRESH: u64 = 1500; // ~ 0.6 ms/block * 1500 block = 15 min
/// Maximum elapsed slots for computing the protocols TVL
const MAX_ELAPSED_SLOTS_FOR_TVL: u64 = 30;

/// Protocol fee
const FEE: u128 = 100; // in per mil

impl<'info> RefreshRewardsWeights<'info> {
    /// Refresh the protocol weights
    pub fn refresh(&mut self) -> Result<()> {
        msg!("GoblinGold: Refresh weights");

        let current_slot = Clock::get()?.slot;

        let elapsed_slots = current_slot
            .checked_sub(self.vault_account.last_refresh_slot)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

        require!(
            elapsed_slots > MIN_ELAPSED_SLOTS_FOR_REFRESH,
            ErrorCode::ForbiddenRefresh
        );

        if self.vault_account.last_refresh_slot != u64::default() {
            for protocol in self.vault_account.protocols.iter() {
                if protocol.is_used() {
                    let last_updated = protocol.rewards.last_slot;
                    require!(
                        current_slot
                            .checked_sub(last_updated)
                            .ok_or_else(|| error!(ErrorCode::MathOverflow))?
                            < MAX_ELAPSED_SLOTS_FOR_TVL,
                        ErrorCode::StaleProtocolTVL
                    )
                }
            }
        }

        self.vault_account.last_refresh_slot = current_slot;
        self.vault_account.rewards_sum = self
            .vault_account
            .protocols
            .iter()
            .try_fold(self.vault_account.rewards_sum, |acc, protocol| {
                acc.checked_add(protocol.rewards.amount)
            })
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

        self.vault_account.update_protocol_weights()?;
        self.vault_account
            .protocols
            .iter_mut()
            .for_each(|protocol| {
                protocol.update_tvl().unwrap();
                protocol.rewards.reset_integral().unwrap();
            });

        self.vault_account.previous_lp_price = LpPrice {
            total_tokens: self.vault_account.current_tvl,
            minted_tokens: self.vault_lp_token_mint_pubkey.supply,
        };

        self.mint_fees_and_update_tvl()?;

        Ok(())
    }

    /// Mint LP tokens to the treasury account in order to take the fees
    pub fn mint_fees_and_update_tvl(&mut self) -> Result<()> {
        let rewards = self.vault_account.rewards_sum;
        if rewards > 0 {
            let lp_fee = FEE
                .checked_mul(rewards as u128)
                .ok_or_else(|| error!(ErrorCode::MathOverflow))?
                .checked_mul(self.vault_lp_token_mint_pubkey.supply as u128)
                .ok_or_else(|| error!(ErrorCode::MathOverflow))?
                .checked_div(
                    (self.vault_account.current_tvl as u128)
                        .checked_add(
                            (1000 - FEE)
                                .checked_mul(rewards as u128)
                                .ok_or_else(|| error!(ErrorCode::MathOverflow))?
                                .checked_div(1000)
                                .ok_or_else(|| error!(ErrorCode::MathOverflow))?,
                        )
                        .ok_or_else(|| error!(ErrorCode::MathOverflow))?,
                )
                .ok_or_else(|| error!(ErrorCode::MathOverflow))?
                .checked_div(1000)
                .ok_or_else(|| error!(ErrorCode::MathOverflow))?
                .try_into()
                .map_err(|_| ErrorCode::MathOverflow)?;

            if lp_fee > 0 {
                let seeds = generate_seeds!(self.vault_account);
                let signer = &[&seeds[..]];

                let cpi_accounts = MintTo {
                    mint: self.vault_lp_token_mint_pubkey.to_account_info(),
                    to: self.dao_treasury_lp_token_account.to_account_info(),
                    authority: self.vault_signer.clone(),
                };
                let cpi_program = self.token_program.to_account_info();
                let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
                token::mint_to(cpi_ctx, lp_fee)?;

                self.vault_account.current_tvl = self
                    .vault_account
                    .current_tvl
                    .checked_add(rewards)
                    .ok_or_else(|| error!(ErrorCode::MathOverflow))?;
                self.vault_account.rewards_sum = 0_u64;
            }
        }

        Ok(())
    }
}
