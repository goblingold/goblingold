use crate::error::ErrorCode;
use crate::macros::generate_seeds;
use crate::vault::{LpPrice, VaultAccount};
use crate::{VAULT_ACCOUNT_SEED, VAULT_LP_TOKEN_MINT_SEED};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program_option::COption, pubkey::Pubkey, sysvar};
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct Withdraw<'info> {
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
    #[account(address = sysvar::instructions::ID)]
    /// CHECK: address is checked
    pub instructions: AccountInfo<'info>,
}

/// Withdraw the required input tokens from the vault and send them back to the user
pub fn handler(ctx: Context<Withdraw>, lp_amount: u64) -> Result<()> {
    msg!("GoblinGold: Withdraw");

    let current_price = LpPrice {
        total_tokens: ctx.accounts.vault_account.current_tvl,
        minted_tokens: ctx.accounts.vault_lp_token_mint_pubkey.supply,
    };

    if ctx.accounts.vault_account.previous_lp_price != LpPrice::default() {
        require!(
            current_price > ctx.accounts.vault_account.previous_lp_price,
            ErrorCode::InvalidLpPrice
        );
    }

    // Use previous value of LP in order to avoid depositors
    let amount = ctx
        .accounts
        .vault_account
        .previous_lp_price
        .lp_to_token(lp_amount)?;

    require!(amount > 0, ErrorCode::InvalidZeroWithdraw);

    let seeds = generate_seeds!(ctx.accounts.vault_account);
    let signer = &[&seeds[..]];

    // Burn user vault tokens
    let cpi_accounts = Burn {
        mint: ctx.accounts.vault_lp_token_mint_pubkey.to_account_info(),
        to: ctx.accounts.user_lp_token_account.to_account_info(),
        authority: ctx.accounts.user_signer.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::burn(cpi_ctx, lp_amount)?;

    // Transfer tokens back to user
    // Fee = 1 lamport due precision errors when withdrawing from lending protocols
    let amount_conservative = amount.saturating_sub(1);
    let cpi_accounts = Transfer {
        from: ctx.accounts.vault_input_token_account.to_account_info(),
        to: ctx.accounts.user_input_token_account.to_account_info(),
        authority: ctx.accounts.vault_account.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    token::transfer(cpi_ctx, amount_conservative)?;

    // Update total withdraw
    ctx.accounts.vault_account.current_tvl = ctx
        .accounts
        .vault_account
        .current_tvl
        .checked_sub(amount)
        .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

    Ok(())
}
