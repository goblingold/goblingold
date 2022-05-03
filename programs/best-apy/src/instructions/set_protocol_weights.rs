use crate::error::ErrorCode;
use crate::vault::{VaultAccount, WEIGHTS_SCALE};
use crate::VAULT_ACCOUNT_SEED;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

#[derive(Accounts)]
pub struct SetProtocolWeights<'info> {
    pub user_signer: Signer<'info>,
    #[account(
        mut,
        seeds = [VAULT_ACCOUNT_SEED, &[vault_account.seed_number], vault_account.input_mint_pubkey.as_ref()],
        bump = vault_account.bumps.vault
    )]
    pub vault_account: Box<Account<'info, VaultAccount>>,
}

pub fn handler(ctx: Context<SetProtocolWeights>, weights: Vec<u32>) -> Result<()> {
    require!(
        weights.len() == ctx.accounts.vault_account.protocols.len(),
        ErrorCode::InvalidWeights
    );

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

    ctx.accounts
        .vault_account
        .protocols
        .iter_mut()
        .zip(weights)
        .for_each(|(protocol, weight)| protocol.weight = weight);

    Ok(())
}
