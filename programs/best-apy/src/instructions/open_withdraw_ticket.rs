use crate::error::ErrorCode;
use crate::macros::generate_seeds;
use crate::vault::{LpPrice, VaultAccount};
use crate::{VAULT_ACCOUNT_SEED, VAULT_LP_TOKEN_MINT_SEED, VAULT_TICKET_MINT_SEED};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program_option::COption, pubkey::Pubkey};
use anchor_spl::token::{self, Burn, Mint, MintTo, Token, TokenAccount};

// TODO collision for vault_lp_token_mint_pubkey & vault_ticken_mint for using the same seed?
#[derive(Accounts)]
#[instruction(bump_user: u8)]
pub struct OpenWithdrawTicket<'info> {
    pub user_signer: Signer<'info>,
    #[account(
        mut,
        constraint = user_lp_token_account.owner == *user_signer.key
    )]
    pub user_lp_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = vault_user_ticket_account.owner == vault_account.key(),
        seeds = [VAULT_TICKET_MINT_SEED, vault_lp_token_mint_pubkey.key().as_ref(), user_signer.key().as_ref()],
        bump = bump_user
    )]
    pub vault_user_ticket_account: Account<'info, TokenAccount>,
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
    #[account(
        mut,
        constraint = vault_lp_token_mint_pubkey.mint_authority == COption::Some(vault_account.key()),
        seeds = [VAULT_TICKET_MINT_SEED, vault_lp_token_mint_pubkey.key().as_ref()],
        bump = vault_account.bump_ticket_mint
    )]
    pub vault_ticket_mint_pubkey: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

impl<'info> OpenWithdrawTicket<'info> {
    fn current_lp_price(&self) -> LpPrice {
        LpPrice {
            total_tokens: self.vault_account.current_tvl,
            minted_tokens: self.vault_lp_token_mint_pubkey.supply,
        }
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

    fn mint_ticket_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            MintTo {
                mint: self.vault_ticket_mint_pubkey.to_account_info(),
                to: self.vault_user_ticket_account.to_account_info(),
                authority: self.vault_account.to_account_info(),
            },
        )
    }
}

/// Open a withdrawal ticket (for delayed withdrawals)
pub fn handler(ctx: Context<OpenWithdrawTicket>, lp_amount: u64, _bump_user: u8) -> Result<()> {
    let current_price = ctx.accounts.current_lp_price();
    let previous_price = ctx.accounts.vault_account.previous_lp_price;

    if previous_price != LpPrice::default() {
        require!(current_price >= previous_price, ErrorCode::InvalidLpPrice);
    }

    let seeds = generate_seeds!(ctx.accounts.vault_account);
    let signer = &[&seeds[..]];

    token::burn(ctx.accounts.burn_user_lps_ctx(), lp_amount)?;
    token::mint_to(
        ctx.accounts.mint_ticket_ctx().with_signer(signer),
        lp_amount,
    )?;

    Ok(())
}
