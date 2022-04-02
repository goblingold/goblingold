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

impl<'info> Withdraw<'info> {
    fn current_lp_price(&self) -> LpPrice {
        LpPrice {
            total_tokens: self.vault_account.current_tvl,
            minted_tokens: self.vault_lp_token_mint_pubkey.supply,
        }
    }

    fn transfer_from_vault_to_user_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.vault_input_token_account.to_account_info(),
                to: self.user_input_token_account.to_account_info(),
                authority: self.vault_account.to_account_info(),
            },
        )
    }

    fn burn_user_lps_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Burn {
                mint: self.vault_lp_token_mint_pubkey.to_account_info(),
                to: self.user_lp_token_account.to_account_info(),
                authority: self.user_signer.to_account_info(),
            },
        )
    }
}

/// Withdraw the required input tokens from the vault and send them back to the user
pub fn handler(ctx: Context<Withdraw>, lp_amount: u64) -> Result<()> {
    let current_price = ctx.accounts.current_lp_price();
    let previous_price = ctx.accounts.vault_account.previous_lp_price;

    if previous_price != LpPrice::default() {
        require!(current_price > previous_price, ErrorCode::InvalidLpPrice);
    }

    // Use previous value of LP in order to avoid depositors.
    // Also add a 1 lamport fee due precision errors when withdrawing from lending protocols
    let amount = previous_price.lp_to_token(lp_amount)?;
    let amount_conservative = amount.saturating_sub(1);

    require!(amount_conservative > 1, ErrorCode::InvalidZeroWithdraw);

    let seeds = generate_seeds!(ctx.accounts.vault_account);
    let signer = &[&seeds[..]];

    token::burn(ctx.accounts.burn_user_lps_ctx(), lp_amount)?;
    token::transfer(
        ctx.accounts
            .transfer_from_vault_to_user_ctx()
            .with_signer(signer),
        amount_conservative,
    )?;

    // Update total withdraw (assuming we have lost 1 lamport due precision errors)
    ctx.accounts.vault_account.current_tvl = ctx
        .accounts
        .vault_account
        .current_tvl
        .checked_sub(amount)
        .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

    Ok(())
}
