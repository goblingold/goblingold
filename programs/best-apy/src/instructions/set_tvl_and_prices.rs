use crate::check_hash::CHECKHASH_BYTES;
use crate::vault::{LpPrice, VaultAccount};
use crate::{VAULT_ACCOUNT_SEED, VAULT_LP_TOKEN_MINT_SEED};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program_option::COption, pubkey::Pubkey};
use anchor_spl::token::{Mint, TokenAccount};

#[derive(Accounts)]
pub struct SetTVLAndPrices<'info> {
    pub user_signer: Signer<'info>,
    #[account(
        mut,
        seeds = [VAULT_ACCOUNT_SEED, &[vault_account.seed_number][..], vault_account.input_mint_pubkey.as_ref()],
        bump = vault_account.bumps.vault
    )]
    pub vault_account: Box<Account<'info, VaultAccount>>,
    #[account(
        mut,
        associated_token::mint = vault_account.input_mint_pubkey,
        associated_token::authority = vault_account,
    )]
    pub vault_input_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = vault_lp_token_mint_pubkey.mint_authority == COption::Some(vault_account.key()),
        seeds = [VAULT_LP_TOKEN_MINT_SEED, vault_account.key().as_ref()],
        bump = vault_account.bumps.lp_token_mint
    )]
    pub vault_lp_token_mint_pubkey: Account<'info, Mint>,
}

impl<'info> SetTVLAndPrices<'info> {
    fn current_lp_price(&self) -> LpPrice {
        LpPrice {
            total_tokens: self.vault_account.current_tvl,
            minted_tokens: self.vault_lp_token_mint_pubkey.supply,
        }
    }
}

/// Set hash of a protocol for a specific action
pub fn handler(ctx: Context<SetTVLAndPrices>) -> Result<()> {
    ctx.accounts.vault_account.current_tvl = ctx.accounts.vault_input_token_account.amount;
    ctx.accounts.vault_account.previous_lp_price = ctx.accounts.current_lp_price();
    Ok(())
}
