use crate::vault::{RefreshParams, VaultAccount};
use crate::VAULT_ACCOUNT_SEED;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct SetRefreshParams<'info> {
    pub user_signer: Signer<'info>,
    #[account(
        mut,
        seeds = [VAULT_ACCOUNT_SEED, vault_account.input_mint_pubkey.as_ref()],
        bump = vault_account.bumps.vault
    )]
    pub vault_account: Box<Account<'info, VaultAccount>>,
}

pub fn handler(ctx: Context<SetRefreshParams>, params: RefreshParams) -> Result<()> {
    ctx.accounts.vault_account.refresh = params;
    Ok(())
}
