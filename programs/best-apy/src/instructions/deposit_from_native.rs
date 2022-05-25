use crate::instructions::{self, Deposit};
use crate::vault::VaultAccount;
use crate::{VAULT_ACCOUNT_SEED, VAULT_LP_TOKEN_MINT_SEED};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    program::invoke_signed, program_option::COption, pubkey::Pubkey,
};
use anchor_lang::system_program::{self, Transfer};
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct DepositFromNative<'info> {
    pub user_signer: Signer<'info>,
    #[account(
        mut,
        constraint = user_wrapped_account.owner == *user_signer.key,
        constraint = user_wrapped_account.mint == spl_token::native_mint::ID
    )]
    pub user_wrapped_account: Account<'info, TokenAccount>,
    #[account(mut, constraint = user_lp_token_account.owner == *user_signer.key)]
    pub user_lp_token_account: Account<'info, TokenAccount>,
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
        associated_token::mint = vault_account.input_mint_pubkey,
        associated_token::authority = vault_account,
    )]
    pub vault_input_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> DepositFromNative<'info> {
    fn system_program_transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.system_program.to_account_info(),
            Transfer {
                from: self.user_signer.to_account_info(),
                to: self.user_wrapped_account.to_account_info(),
            },
        )
    }

    fn sync_native_ctx(&self) -> CpiContext<'_, '_, '_, 'info, SyncNative<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            SyncNative {
                account: self.user_wrapped_account.to_account_info(),
            },
        )
    }

    fn to_deposit_accounts(&self) -> Deposit<'info> {
        Deposit {
            user_signer: self.user_signer.clone(),
            user_input_token_account: self.user_wrapped_account.clone(),
            user_lp_token_account: self.user_lp_token_account.clone(),
            vault_account: self.vault_account.clone(),
            vault_lp_token_mint_pubkey: self.vault_lp_token_mint_pubkey.clone(),
            vault_input_token_account: self.vault_input_token_account.clone(),
            token_program: self.token_program.clone(),
        }
    }
}

// Not yet implemented in anchor
#[derive(Accounts)]
pub struct SyncNative<'info> {
    /// CHECK:
    account: AccountInfo<'info>,
}

pub fn sync_native<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, SyncNative<'info>>,
) -> Result<()> {
    let ix = spl_token::instruction::sync_native(&spl_token::ID, ctx.accounts.account.key)?;
    Ok(invoke_signed(
        &ix,
        &[ctx.accounts.account.clone()],
        ctx.signer_seeds,
    )?)
}

/// Deposit user input tokens into the vault account
pub fn handler<'info>(
    ctx: Context<'_, '_, '_, 'info, DepositFromNative<'info>>,
    amount: u64,
) -> Result<()> {
    system_program::transfer(ctx.accounts.system_program_transfer_ctx(), amount)?;
    sync_native(ctx.accounts.sync_native_ctx())?;

    instructions::deposit::handler(
        Context::new(
            ctx.program_id,
            &mut ctx.accounts.to_deposit_accounts(),
            ctx.remaining_accounts,
            ctx.bumps,
        ),
        amount,
    )?;

    Ok(())
}
