use crate::vault::VaultAccount;
use crate::vault_old::OldVaultAccount;
use crate::VAULT_ACCOUNT_SEED;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct VaultTransfer<'info> {
    pub user_signer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub input_token_mint_address: Account<'info, Mint>,

    #[account(
        constraint = input_token_mint_address.key() == new_vault.input_mint_pubkey,
        seeds = [VAULT_ACCOUNT_SEED, &[new_vault.seed_number][..], new_vault.input_mint_pubkey.as_ref()],
        bump = new_vault.bumps.vault
    )]
    pub new_vault: Box<Account<'info, VaultAccount>>,
    #[account(
        mut,
        associated_token::mint = new_vault.input_mint_pubkey,
        associated_token::authority = new_vault,
    )]
    pub new_ata: Account<'info, TokenAccount>,

    #[account(
        constraint = input_token_mint_address.key() == new_vault.input_mint_pubkey,
        seeds = [VAULT_ACCOUNT_SEED, old_vault.input_mint_pubkey.as_ref()],
        bump = old_vault.bumps.vault
    )]
    pub old_vault: Box<Account<'info, OldVaultAccount>>,
    #[account(
        mut,
        associated_token::mint = old_vault.input_mint_pubkey,
        associated_token::authority = old_vault,
    )]
    pub old_ata: Account<'info, TokenAccount>,
}

pub fn handler(ctx: Context<VaultTransfer>) -> Result<()> {
    let amount = ctx.accounts.old_ata.amount;

    let seeds = &[
        "vault".as_ref(),
        ctx.accounts.old_vault.input_mint_pubkey.as_ref(),
        &[ctx.accounts.old_vault.bumps.vault],
    ];
    let signer = &[&seeds[..]];

    let ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.old_ata.to_account_info(),
            to: ctx.accounts.new_ata.to_account_info(),
            authority: ctx.accounts.old_vault.to_account_info(),
        },
    )
    .with_signer(signer);

    token::transfer(ctx, amount)?;

    Ok(())
}
