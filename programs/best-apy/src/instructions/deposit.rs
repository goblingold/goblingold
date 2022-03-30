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

/// Deposit user input tokens into the vault account
pub fn handler(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    require!(amount >= 100, ErrorCode::InvalidDepositAmount);
    msg!("GoblinGold: Deposit");

    // Transfer user token to vault account
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_input_token_account.to_account_info(),
        to: ctx.accounts.vault_input_token_account.to_account_info(),
        authority: ctx.accounts.user_signer.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    // Mint vault tokens to user vault account
    let lp_amount = LpPrice {
        total_tokens: ctx.accounts.vault_account.current_tvl,
        minted_tokens: ctx.accounts.vault_lp_token_mint_pubkey.supply,
    }
    .token_to_lp(amount)?;
    let lp_amount_previous_price = ctx
        .accounts
        .vault_account
        .previous_lp_price
        .lp_to_token(lp_amount)?;
    require!(
        lp_amount < lp_amount_previous_price,
        ErrorCode::InvalidLpPrice
    );

    // Update total deposited amounts
    ctx.accounts.vault_account.current_tvl = ctx
        .accounts
        .vault_account
        .current_tvl
        .checked_add(amount)
        .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

    let seeds = generate_seeds!(ctx.accounts.vault_account);
    let signer = &[&seeds[..]];

    let cpi_accounts = MintTo {
        mint: ctx.accounts.vault_lp_token_mint_pubkey.to_account_info(),
        to: ctx.accounts.user_lp_token_account.to_account_info(),
        authority: ctx.accounts.vault_account.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    token::mint_to(cpi_ctx, lp_amount)?;

    Ok(())
}
