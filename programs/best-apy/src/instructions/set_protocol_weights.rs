use crate::error::ErrorCode;
use crate::protocols::PROTOCOLS_LEN;
use crate::vault::{VaultAccount, WEIGHTS_SCALE};
use crate::ALLOWED_DEPLOYER;
use crate::VAULT_ACCOUNT_SEED;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
use std::str::FromStr;

#[derive(Accounts)]
pub struct SetProtocolWeights<'info> {
    // Only deployer can modify weights
    #[account(constraint = Pubkey::from_str(ALLOWED_DEPLOYER).unwrap()== *user_signer.key)]
    pub user_signer: Signer<'info>,
    #[account(
        mut,
        seeds = [VAULT_ACCOUNT_SEED, vault_account.input_mint_pubkey.as_ref()],
        bump = vault_account.bumps.vault
    )]
    pub vault_account: Box<Account<'info, VaultAccount>>,
}

pub fn handler(ctx: Context<SetProtocolWeights>, weights: [u16; PROTOCOLS_LEN]) -> Result<()> {
    let weights_sum = weights
        .iter()
        .try_fold(0_u16, |acc, &x| acc.checked_add(x))
        .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

    require!(weights_sum == WEIGHTS_SCALE, ErrorCode::InvalidWeights);

    ctx.accounts
        .vault_account
        .protocols
        .iter_mut()
        .zip(weights)
        .for_each(|(protocol, weight)| protocol.weight = weight);

    Ok(())
}
