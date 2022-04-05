use crate::check_hash::CHECKHASH_BYTES;

use crate::vault::VaultAccount;

use crate::VAULT_ACCOUNT_SEED;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

#[derive(Accounts)]
pub struct SetHashes<'info> {
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
    ctx: Context<SetHashes>,
    protocol_id: u8,
    hashes: [[u8; CHECKHASH_BYTES]; 3],
) -> Result<()> {
    ctx.accounts.vault_account.protocols[protocol_id as usize].set_hashes(hashes);
    Ok(())
}
