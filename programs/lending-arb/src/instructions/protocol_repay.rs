use crate::protocols::Protocols;
use crate::vault::{ProtocolData, VaultAccount};
use crate::VAULT_ACCOUNT_SEED;
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use crate::health::{Health};

/// Repay back to the protocol
pub trait ProtocolRepay<'info> {
    /// Return the protcol position in the vector
    fn protocol_position(&self, protocol: Protocols) -> Result<usize>;

    /// Return a mutable refrence of the data
    fn protocol_data_as_mut(&mut self, protocol_idx: usize) -> &mut ProtocolData;

    /// Compute the amount to repay
    fn amount_to_repay(&self) -> Result<u64>;

    /// Borrow from the protocol
    fn cpi_repay(&self, amount: u64) -> Result<()>;
}

/// Borrow from protocol and update protocol data
pub fn handler<'info, T: ProtocolRepay<'info>>(
    ctx: Context<T>,
    protocol: Protocols,
) -> Result<()> {
    let protocol_idx = ctx.accounts.protocol_position(protocol)?;
    let amount = ctx.accounts.amount_to_repay()?;
    ctx.accounts.cpi_repay(amount)?;
    ctx.accounts
        .protocol_data_as_mut(protocol_idx)
        .update_after_repay(amount)?;

    Ok(())
}

#[derive(Accounts)]
pub struct GenericRepayAccounts<'info> {
    #[account(
        mut,
        seeds = [VAULT_ACCOUNT_SEED, &[vault_account.seed_number][..], vault_account.borrow_mint_pubkey.as_ref()],
        bump = vault_account.bumps.vault
    )]
    pub vault_account: Box<Account<'info, VaultAccount>>,
    #[account(
        mut,
        associated_token::mint = vault_account.borrow_mint_pubkey,
        associated_token::authority = vault_account,
    )]
    pub vault_borrow_token_account: Account<'info, TokenAccount>,
}
