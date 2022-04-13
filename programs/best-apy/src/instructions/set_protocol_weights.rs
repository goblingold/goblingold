use crate::error::ErrorCode;
use crate::protocols::PROTOCOLS_LEN;
use crate::vault::{ProtocolData, VaultAccount, WEIGHTS_SCALE};
use crate::VAULT_ACCOUNT_SEED;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

#[derive(Accounts)]
pub struct SetProtocolWeights<'info> {
    pub user_signer: Signer<'info>,
    #[account(
        mut,
        seeds = [VAULT_ACCOUNT_SEED, vault_account.input_mint_pubkey.as_ref()],
        bump = vault_account.bumps.vault
    )]
    pub vault_account: Box<Account<'info, VaultAccount>>,
}

pub fn handler(ctx: Context<SetProtocolWeights>, weights: [u32; PROTOCOLS_LEN]) -> Result<()> {
    let weights_sum = weights
        .iter()
        .try_fold(0_u32, |acc, &x| acc.checked_add(x))
        .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

    // For security reasons we might want to set weights_sum == 0 in order to withdraw everything
    // from every protocol
    require!(
        weights_sum == WEIGHTS_SCALE || weights_sum == 0,
        ErrorCode::InvalidWeights
    );

    if ctx.accounts.vault_account.protocols.is_empty() {
        ctx.accounts.vault_account.protocols = vec![ProtocolData::default(); PROTOCOLS_LEN];
    }
    ctx.accounts
        .vault_account
        .protocols
        .iter_mut()
        .zip(weights)
        .for_each(|(protocol, weight)| protocol.weight = weight);

    Ok(())
}
