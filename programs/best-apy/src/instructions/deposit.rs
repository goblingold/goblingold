use crate::error::ErrorCode;
use crate::macros::generate_seeds;
use crate::vault::LpPrice;
use crate::vault::VaultAccount;
use crate::{VAULT_ACCOUNT_SEED, VAULT_LP_TOKEN_MINT_SEED};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program_option::COption, pubkey::Pubkey};
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct Deposit<'info> {
    pub user_signer: Signer<'info>,
    #[account(mut, constraint = user_input_token_account.owner == *user_signer.key)]
    pub user_input_token_account: Account<'info, TokenAccount>,
    #[account(mut, constraint = user_lp_token_account.owner == *user_signer.key)]
    pub user_lp_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [VAULT_ACCOUNT_SEED, vault_account.input_mint_pubkey.as_ref()],
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
    #[account(
        mut,
        associated_token::mint = vault_account.input_mint_pubkey,
        associated_token::authority = vault_account,
    )]
    pub vault_input_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Deposit<'info> {
    fn current_lp_price(&self) -> LpPrice {
        LpPrice {
            total_tokens: self.vault_account.current_tvl,
            minted_tokens: self.vault_lp_token_mint_pubkey.supply,
        }
    }

    fn transfer_from_user_to_vault_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.user_input_token_account.to_account_info(),
                to: self.vault_input_token_account.to_account_info(),
                authority: self.user_signer.to_account_info(),
            },
        )
    }

    fn mint_lp_to_user_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            MintTo {
                mint: self.vault_lp_token_mint_pubkey.to_account_info(),
                to: self.user_lp_token_account.to_account_info(),
                authority: self.vault_account.to_account_info(),
            },
        )
    }
}

/// Deposit user input tokens into the vault account
pub fn handler(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let current_price = ctx.accounts.current_lp_price();
    let previous_price = ctx.accounts.vault_account.previous_lp_price;

    if previous_price != LpPrice::default() {
        require!(current_price > previous_price, ErrorCode::InvalidLpPrice);
    }

    require!(amount >= 100, ErrorCode::InvalidDepositAmount);

    let lp_amount = current_price.token_to_lp(amount)?;

    let seeds = generate_seeds!(ctx.accounts.vault_account);
    let signer = &[&seeds[..]];

    token::transfer(ctx.accounts.transfer_from_user_to_vault_ctx(), amount)?;
    token::mint_to(
        ctx.accounts.mint_lp_to_user_ctx().with_signer(signer),
        lp_amount,
    )?;

    // Update total deposited amounts
    ctx.accounts.vault_account.current_tvl = ctx
        .accounts
        .vault_account
        .current_tvl
        .checked_add(amount)
        .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

    Ok(())
}
