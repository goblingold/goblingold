use crate::check_hash::CHECKHASH_BYTES;
use crate::error::ErrorCode;
use crate::protocols::Protocols;
use crate::vault::VaultAccount;
use crate::VAULT_ACCOUNT_SEED;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
use std::convert::TryInto;

#[derive(Accounts)]
pub struct SetHashes<'info> {
    pub user_signer: Signer<'info>,
    #[account(
        mut,
        seeds = [VAULT_ACCOUNT_SEED, &[vault_account.seed_number][..], vault_account.input_mint_pubkey.as_ref()],
        bump = vault_account.bumps.vault
    )]
    pub vault_account: Box<Account<'info, VaultAccount>>,
}

/// Set hash of a protocol for a specific action
pub fn handler(
    ctx: Context<SetHashes>,
    protocol_id: u8,
    hashes: [[u8; CHECKHASH_BYTES]; 5],
) -> Result<()> {
    let protocol: Protocols = usize::from(protocol_id)
        .try_into()
        .map_err(|_| error!(ErrorCode::InvalidProtocolId))?;

    let protocol_idx = ctx.accounts.vault_account.protocol_position(protocol)?;
    ctx.accounts.vault_account.protocols[protocol_idx].set_hashes(hashes);

    Ok(())
}
