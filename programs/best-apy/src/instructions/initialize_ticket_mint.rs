use crate::vault::VaultAccount;
use crate::{VAULT_ACCOUNT_SEED, VAULT_LP_TOKEN_MINT_SEED, VAULT_TICKET_MINT_SEED};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program_option::COption, pubkey::Pubkey};
use anchor_spl::token::{Mint, Token};

#[derive(Accounts)]
pub struct InitializeTicketMint<'info> {
    #[account(mut)]
    pub user_signer: Signer<'info>,
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
        init,
        payer = user_signer,
        mint::decimals = vault_lp_token_mint_pubkey.decimals,
        mint::authority = vault_account.key(),
        seeds = [VAULT_TICKET_MINT_SEED, vault_lp_token_mint_pubkey.key().as_ref()],
        bump,
    )]
    pub vault_ticket_mint_pubkey: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<InitializeTicketMint>) -> Result<()> {
    ctx.accounts.vault_account.bump_ticket_mint =
        *ctx.bumps.get("vault_ticket_mint_pubkey").unwrap();
    Ok(())
}
