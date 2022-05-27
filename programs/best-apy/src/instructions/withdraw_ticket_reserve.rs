use crate::error::ErrorCode;
use crate::macros::generate_seeds;
use crate::vault::VaultAccount;
use crate::{
    VAULT_ACCOUNT_SEED, VAULT_LP_TOKEN_MINT_SEED, VAULT_TICKET_MINT_SEED, VAULT_TICKET_RESERVE_SEED,
};
use anchor_lang::prelude::*;

use anchor_lang::solana_program::{program_option::COption, pubkey::Pubkey};
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
#[instruction(bump_ticket: u8, bump_reserve: u8)]
pub struct WithdrawTicketReserve<'info> {
    pub user_signer: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = vault_account.input_mint_pubkey,
        associated_token::authority = vault_account,
    )]
    pub vault_input_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = vault_ticket_reserve_account.owner == vault_account.key(),
        seeds = [VAULT_TICKET_RESERVE_SEED, vault_lp_token_mint_pubkey.key().as_ref(), user_signer.key().as_ref()],
        bump = bump_reserve
    )]
    pub vault_ticket_reserve_account: Account<'info, TokenAccount>,
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
        bump = bump_ticket
    )]
    pub vault_ticket_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

impl<'info> WithdrawTicketReserve<'info> {
    fn transfer_from_vault_to_ticket_reserve(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.vault_input_token_account.to_account_info(),
                to: self.vault_ticket_reserve_account.to_account_info(),
                authority: self.user_signer.to_account_info(),
            },
        )
    }
}

/// Withdraw to the ticket reserve account
pub fn handler(
    ctx: Context<WithdrawTicketReserve>,
    _bump_ticket: u8,
    _bump_reserve: u8,
) -> Result<()> {
    let request_amount = ctx.accounts.vault_ticket_mint.supply;
    let available_amount = ctx.accounts.vault_ticket_reserve_account.amount;

    require!(
        request_amount > available_amount,
        ErrorCode::InvalidTicketAmount
    );

    let amount = request_amount
        .checked_sub(available_amount)
        .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

    let seeds = generate_seeds!(ctx.accounts.vault_account);
    let signer = &[&seeds[..]];

    token::transfer(
        ctx.accounts
            .transfer_from_vault_to_ticket_reserve()
            .with_signer(signer),
        amount,
    )?;

    // Update total withdraw 
    // TODO check 1 lamport diff in withdraw.rs
    ctx.accounts.vault_account.current_tvl = ctx
        .accounts
        .vault_account
        .current_tvl
        .checked_sub(amount)
        .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

    Ok(())
}
