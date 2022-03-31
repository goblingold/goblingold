use crate::check_hash::CHECKHASH_BYTES;
use crate::vault::VaultAccount;
use crate::ALLOWED_DEPLOYER;
use crate::VAULT_ACCOUNT_SEED;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
use std::str::FromStr;

#[derive(Accounts)]
pub struct SetHash<'info> {
    #[account(constraint = Pubkey::from_str(ALLOWED_DEPLOYER).unwrap()== *user_signer.key)]
    pub user_signer: Signer<'info>,
    #[account(
        mut,
        seeds = [VAULT_ACCOUNT_SEED, vault_account.input_mint_pubkey.as_ref()],
        bump = vault_account.bumps.vault
    )]
    pub vault_account: Account<'info, VaultAccount>,
}

/// Set hash of a protocol for a specific action
pub fn handler(
    ctx: Context<SetHash>,
    protocol: usize,
    action: String,
    hash: [u8; CHECKHASH_BYTES],
) -> Result<()> {
    ctx.accounts.set_hash(protocol, action, hash)
}
