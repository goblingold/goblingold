use crate::macros::generate_seeds;
use crate::vault::VaultAccount;
use crate::VAULT_ACCOUNT_SEED;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct WithdrawAndClose<'info> {
    pub user_signer: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = vault_account.input_mint_pubkey,
        associated_token::authority = user_signer,
    )]
    pub admin_input_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [VAULT_ACCOUNT_SEED, &[vault_account.seed_number][..], vault_account.input_mint_pubkey.as_ref()],
        bump = vault_account.bumps.vault
    )]
    pub vault_account: Box<Account<'info, VaultAccount>>,
    #[account(
        mut,
        associated_token::mint = vault_account.input_mint_pubkey,
        associated_token::authority = vault_account,
    )]
    pub vault_input_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

impl<'info> WithdrawAndClose<'info> {
    fn transfer_from_vault_to_admin_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.vault_input_token_account.to_account_info(),
                to: self.admin_input_token_account.to_account_info(),
                authority: self.vault_account.to_account_info(),
            },
        )
    }
}

/// Transfer funds to admin to simplify user reimbursements
pub fn handler(ctx: Context<WithdrawAndClose>) -> Result<()> {
    let amount = ctx.accounts.vault_input_token_account.amount;

    let seeds = generate_seeds!(ctx.accounts.vault_account);
    let signer = &[&seeds[..]];

    token::transfer(
        ctx.accounts
            .transfer_from_vault_to_admin_ctx()
            .with_signer(signer),
        amount,
    )?;

    // Uninitialize all fields in the vault
    ctx.accounts
        .vault_account
        .set_inner(VaultAccount::default());

    Ok(())
}
