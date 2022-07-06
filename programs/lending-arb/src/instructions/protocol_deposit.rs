use crate::protocols::Protocols;
use crate::vault::{ProtocolData, VaultAccount};
use crate::VAULT_ACCOUNT_SEED;
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

/// Deposit into the protocol
pub trait ProtocolDeposit<'info> {
    /// Return the protcol position in the vector
    fn protocol_position(&self, protocol: Protocols) -> Result<usize>;

    /// Return a mutable refrence of the data
    fn protocol_data_as_mut(&mut self, protocol_idx: usize) -> &mut ProtocolData;

    /// Compute the amount to deposit
    fn get_amount(&self, protocol_idx: usize) -> Result<u64>;

    /// Deposit into the protocol
    fn cpi_deposit(&self, amount: u64) -> Result<()>;
}

/// Deposit into the protocol and update protocol data
pub fn handler<'info, T: ProtocolDeposit<'info>>(
    ctx: Context<T>,
    protocol: Protocols,
) -> Result<()> {
    let protocol_idx = ctx.accounts.protocol_position(protocol)?;
    let amount = ctx.accounts.get_amount(protocol_idx)?;
    ctx.accounts.cpi_deposit(amount)?;
    ctx.accounts
        .protocol_data_as_mut(protocol_idx)
        .update_after_deposit(amount)?;

    Ok(())
}

#[derive(Accounts)]
pub struct GenericDepositAccounts<'info> {
    #[account(
        mut,
        seeds = [VAULT_ACCOUNT_SEED, &[vault_account.seed_number][..], vault_account.input_mint_pubkey.as_ref()],
        bump = vault_account.bumps.vault
    )]
    pub vault_account: Box<Account<'info, VaultAccount>>,
    //TODO use also borrow_mint_pubkey
    #[account(
        mut,
        // associated_token::mint = vault_account.input_mint_pubkey,
        // associated_token::authority = vault_account,
    )]
    pub vault_input_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub clock: Sysvar<'info, Clock>,
}

impl<'info> GenericDepositAccounts<'info> {
    /// Compute the amount to deposit into the protocol
    pub fn amount_to_deposit(&self, protocol_idx: usize) -> Result<u64> {
        self.vault_account
            .calculate_deposit(protocol_idx, self.vault_input_token_account.amount)
    }
}
