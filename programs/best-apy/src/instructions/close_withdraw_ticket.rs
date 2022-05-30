use crate::macros::generate_seeds;
use crate::vault::VaultAccount;
use crate::{
    VAULT_ACCOUNT_SEED, VAULT_LP_TOKEN_MINT_SEED, VAULT_TICKET_MINT_SEED, VAULT_TICKET_RESERVE_SEED,
};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program_option::COption, pubkey::Pubkey};
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
#[instruction(bump_user: u8, bump_ticket: u8, bump_reserve: u8)]
pub struct CloseWithdrawTicket<'info> {
    pub user_signer: Signer<'info>,
    #[account(
        mut,
        constraint = user_input_token_account.owner == *user_signer.key
    )]
    pub user_input_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = vault_user_ticket_account.owner == vault_account.key(),
        seeds = [VAULT_TICKET_MINT_SEED, vault_lp_token_mint_pubkey.key().as_ref(), user_signer.key().as_ref()],
        bump = bump_user
    )]
    pub vault_user_ticket_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = vault_ticket_reserve_account.mint == vault_account.input_mint_pubkey,
        seeds = [VAULT_TICKET_RESERVE_SEED, vault_account.input_mint_pubkey.as_ref()],
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

impl<'info> CloseWithdrawTicket<'info> {
    fn burn_ticket_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Burn {
                mint: self.vault_ticket_mint.to_account_info(),
                to: self.vault_user_ticket_account.to_account_info(),
                authority: self.vault_account.to_account_info(),
            },
        )
    }

    fn transfer_from_ticket_reserve_to_user_ctx(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.vault_ticket_reserve_account.to_account_info(),
                to: self.user_input_token_account.to_account_info(),
                authority: self.vault_account.to_account_info(),
            },
        )
    }
}

/// Close a withdrawal ticket
pub fn handler(
    ctx: Context<CloseWithdrawTicket>,
    _bump_user: u8,
    _bump_ticket: u8,
    _bump_reserve: u8,
) -> Result<()> {
    let amount = ctx.accounts.vault_user_ticket_account.amount;

    let seeds = generate_seeds!(ctx.accounts.vault_account);
    let signer = &[&seeds[..]];

    token::burn(ctx.accounts.burn_ticket_ctx().with_signer(signer), amount)?;
    token::transfer(
        ctx.accounts
            .transfer_from_ticket_reserve_to_user_ctx()
            .with_signer(signer),
        amount,
    )?;

    Ok(())
}
