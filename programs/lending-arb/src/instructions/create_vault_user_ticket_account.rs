use crate::vault::VaultAccount;
use crate::{VAULT_ACCOUNT_SEED, VAULT_TICKET_MINT_SEED};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program_option::COption, pubkey::Pubkey};
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct CreateVaultUserTicketAccount<'info> {
    #[account(mut)]
    pub user_signer: Signer<'info>,
    /// CHECKED: can create account to anyone
    pub user_ticket_account_owner: AccountInfo<'info>,
    #[account(
        init,
        payer = user_signer,
        token::mint = vault_ticket_mint_pubkey,
        token::authority = vault_account,
        seeds = [VAULT_TICKET_MINT_SEED, vault_ticket_mint_pubkey.key().as_ref(), user_ticket_account_owner.key().as_ref()],
        bump,
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
        constraint = vault_ticket_mint_pubkey.mint_authority == COption::Some(vault_account.key()),
        seeds = [VAULT_TICKET_MINT_SEED, vault_account.key().as_ref()],
        bump = vault_account.bumps.ticket_mint
    )]
    pub vault_ticket_mint_pubkey: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

/// Create vault_user_ticket_account
pub fn handler(_ctx: Context<CreateVaultUserTicketAccount>) -> Result<()> {
    Ok(())
}
