use crate::protocols::Protocols;
use crate::vault::{ProtocolData, VaultAccount};
use crate::VAULT_ACCOUNT_SEED;
use crate::health::{Health};
use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount};
use pyth_sdk_solana::{load_price_feed_from_account_info, PriceFeed, Price};

/// Borrow from the protocol
pub trait ProtocolBorrow<'info> {
    /// Return the protcol position in the vector
    fn protocol_position(&self, protocol: Protocols) -> Result<usize>;

    /// Return a mutable refrence of the data
    fn protocol_data_as_mut(&mut self, protocol_idx: usize) -> &mut ProtocolData;

    /// Compute the amount to borrow
    fn amount_to_borrow(&self) -> Result<u64>;

    /// Borrow from the protocol
    fn cpi_borrow(&self, amount: u64) -> Result<()>;
}

/// Borrow from protocol and update protocol data
pub fn handler<'info, T: ProtocolBorrow<'info>>(
    ctx: Context<T>,
    protocol: Protocols,
) -> Result<()> {
    let protocol_idx = ctx.accounts.protocol_position(protocol)?;
    let amount = ctx.accounts.amount_to_borrow()?;
    ctx.accounts.cpi_borrow(amount)?;
    ctx.accounts
        .protocol_data_as_mut(protocol_idx)
        .update_after_borrow(amount)?;

    Ok(())
}

#[derive(Accounts)]
pub struct GenericBorrowAccounts<'info> {
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
    #[account()]
    /// CHECK inside
    pub price_account_info: AccountInfo<'info>
}

impl<'info> GenericBorrowAccounts<'info> {
    fn price_feed(&self) -> Result<Price> {
        let price_feed: PriceFeed = load_price_feed_from_account_info(&self.price_account_info.to_account_info()).unwrap();
        let current_price: Price = price_feed.get_current_price().unwrap();
        Ok(current_price)
    }
}