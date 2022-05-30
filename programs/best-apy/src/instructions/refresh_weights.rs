use crate::error::ErrorCode;
use crate::macros::generate_seeds;
use crate::vault::{LpPrice, VaultAccount};
use crate::{VAULT_ACCOUNT_SEED, VAULT_LP_TOKEN_MINT_SEED};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program_option::COption, pubkey::Pubkey};
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};
use std::convert::{TryFrom, TryInto};

/// Maximum elapsed slots for computing the protocols TVL
const MAX_ELAPSED_SLOTS_FOR_TVL: u64 = 30;

/// Protocol fee
const FEE: u128 = 100; // in per mil

#[event]
pub struct RefreshWeightsEvent {
    token: Pubkey,
    current_price: LpPrice,
}

#[derive(Accounts)]
pub struct RefreshWeights<'info> {
    #[account(
        mut,
        seeds = [VAULT_ACCOUNT_SEED, &[vault_account.seed_number][..], vault_account.input_mint_pubkey.as_ref()],
        bump = vault_account.bumps.vault
    )]
    pub vault_account: Box<Account<'info, VaultAccount>>,
    #[account(
        mut,
        constraint = vault_lp_token_mint_pubkey.mint_authority == COption::Some(vault_account.key()),
        seeds = [VAULT_LP_TOKEN_MINT_SEED, vault_account.key().as_ref()],
        bump = vault_account.bumps.lp_token_mint
    )]
    pub vault_lp_token_mint_pubkey: Account<'info, Mint>,
    #[account(mut, address = vault_account.dao_treasury_lp_token_account)]
    pub dao_treasury_lp_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

impl<'info> RefreshWeights<'info> {
    fn current_lp_price(&self) -> LpPrice {
        LpPrice {
            total_tokens: self.vault_account.current_tvl,
            minted_tokens: self.vault_lp_token_mint_pubkey.supply,
        }
    }

    fn mint_lps_to_treasury_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            MintTo {
                mint: self.vault_lp_token_mint_pubkey.to_account_info(),
                to: self.dao_treasury_lp_token_account.to_account_info(),
                authority: self.vault_account.to_account_info(),
            },
        )
    }

    /// Mint LP tokens to the treasury account in order to take the fees
    fn mint_fees_and_update_tvl(&mut self) -> Result<()> {
        let rewards = self.vault_account.rewards_sum;
        if rewards > 0 {
            let lp_fee: u64 = FEE
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
                token::mint_to(self.mint_lps_to_treasury_ctx().with_signer(signer), lp_fee)?;

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

/// Refresh the protocol weights
pub fn handler(ctx: Context<RefreshWeights>) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;

    if ctx.accounts.vault_account.refresh.min_elapsed_time != i64::default() {
        let elapsed_time = Clock::get()?
            .unix_timestamp
            .checked_sub(ctx.accounts.vault_account.last_refresh_time)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

        require!(
            elapsed_time > ctx.accounts.vault_account.refresh.min_elapsed_time,
            ErrorCode::ForbiddenRefresh
        );
    }

    if ctx.accounts.vault_account.last_refresh_time != i64::default() {
        for protocol in ctx.accounts.vault_account.protocols.iter() {
            if protocol.is_active() {
                let last_updated = protocol.rewards.last_slot;
                require!(
                    Clock::get()?
                        .slot
                        .checked_sub(last_updated)
                        .ok_or_else(|| error!(ErrorCode::MathOverflow))?
                        < MAX_ELAPSED_SLOTS_FOR_TVL,
                    ErrorCode::StaleProtocolTVL
                )
            }
        }
    }

    ctx.accounts.vault_account.last_refresh_time = current_time;

    let protocol_rewards = ctx
        .accounts
        .vault_account
        .protocols
        .iter()
        .try_fold(0_i64, |acc, protocol| {
            acc.checked_add(protocol.rewards.amount)
        })
        .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

    // Due to precision errors, we lost some lamports. We may need to reduce the TVL
    if protocol_rewards < 0 {
        // Check if we can compensate with the accumulated rewards
        let rewards_with_losses = i64::try_from(ctx.accounts.vault_account.rewards_sum)
            .unwrap()
            .checked_add(protocol_rewards)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

        if rewards_with_losses >= 0 {
            ctx.accounts.vault_account.rewards_sum = u64::try_from(rewards_with_losses).unwrap();
        } else {
            ctx.accounts.vault_account.rewards_sum = 0;
            ctx.accounts.vault_account.current_tvl = ctx
                .accounts
                .vault_account
                .current_tvl
                .checked_sub(u64::try_from(rewards_with_losses.abs()).unwrap())
                .ok_or_else(|| error!(ErrorCode::MathOverflow))?;
        }
    } else {
        ctx.accounts.vault_account.rewards_sum = ctx
            .accounts
            .vault_account
            .rewards_sum
            .checked_add(u64::try_from(protocol_rewards).unwrap())
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;
    }

    ctx.accounts.vault_account.update_protocol_weights()?;
    ctx.accounts
        .vault_account
        .protocols
        .iter_mut()
        .for_each(|protocol| {
            protocol.update_tvl().unwrap();
            protocol.rewards.reset_integral().unwrap();
        });

    ctx.accounts.vault_account.previous_lp_price = ctx.accounts.current_lp_price();

    ctx.accounts.mint_fees_and_update_tvl()?;
    ctx.accounts.vault_lp_token_mint_pubkey.reload()?;

    emit!(RefreshWeightsEvent {
        token: ctx.accounts.vault_account.input_mint_pubkey,
        current_price: ctx.accounts.current_lp_price(),
    });

    Ok(())
}
