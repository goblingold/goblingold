use crate::error::ErrorCode;
use crate::protocols::Protocols;
use crate::vault::{ProtocolData, VaultAccount};
use crate::VAULT_ACCOUNT_SEED;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
use std::convert::TryInto;

#[derive(Accounts)]
pub struct AddProtocol<'info> {
    pub user_signer: Signer<'info>,
    #[account(
        mut,
        seeds = [VAULT_ACCOUNT_SEED, vault_account.input_mint_pubkey.as_ref()],
        bump = vault_account.bumps.vault
    )]
    pub vault_account: Box<Account<'info, VaultAccount>>,
}

/// Add a new protocol to the vault account
pub fn handler(ctx: Context<AddProtocol>, protocol_id: u8) -> Result<()> {
    let protocol: Protocols = usize::from(protocol_id)
        .try_into()
        .map_err(|_| error!(ErrorCode::InvalidProtocolId))?;

    let vault = &mut ctx.accounts.vault_account;

    if vault.protocol_position(protocol).is_ok() {
        return Err(error!(ErrorCode::ProtocolAlreadyExists));
    } else {
        vault.protocols.push(ProtocolData {
            protocol_id,
            ..ProtocolData::default()
        });
    }

    Ok(())
}
