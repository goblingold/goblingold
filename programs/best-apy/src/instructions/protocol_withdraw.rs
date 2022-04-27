use crate::error::ErrorCode;
use crate::protocols::Protocols;
use crate::vault::{ProtocolData, VaultAccount};
use crate::VAULT_ACCOUNT_SEED;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;
use anchor_spl::token::{Token, TokenAccount};

/// Withdraw from the protocol and update protocol data
pub trait ProtocolWithdraw<'info> {
    /// Return a mutable refrence of the data
    fn protocol_data_as_mut(&mut self, protocol_pos: usize) -> &mut ProtocolData;

    /// Return the input token account
    fn input_token_account_as_mut(&mut self) -> &mut Account<'info, TokenAccount>;

    /// Compute the amount to deposit
    fn get_amount(&self, protocol_pos: usize) -> Result<u64>;

    /// Convert reserve liquidity to collateral (if any)
    fn liquidity_to_collateral(&self, amount: u64) -> Result<u64> {
        Ok(amount)
    }

    /// Withdraw from the protocol
    fn cpi_withdraw(&self, amount: u64) -> Result<()>;
}

/// Withdraw from the protocol and update protocol data
pub fn handler<'info, T: ProtocolWithdraw<'info>>(
    ctx: Context<T>,
    protocol: Protocols,
) -> Result<()> {
    let amount = ctx.accounts.get_amount(protocol as usize)?;
    let mut lp_amount = ctx.accounts.liquidity_to_collateral(amount)?;

    // Add 1 as due to rounding. Otherwise it might happens that there wasn't enough funds
    // withdrawn from the protocol
    if amount < ctx.accounts.protocol_data_as_mut(protocol as usize).amount {
        lp_amount = lp_amount
            .checked_add(1)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;
    }

    let amount_before = {
        let input_token_account = ctx.accounts.input_token_account_as_mut();
        input_token_account.amount
    };

    ctx.accounts.cpi_withdraw(lp_amount)?;

    let amount_after = {
        let input_token_account = ctx.accounts.input_token_account_as_mut();
        input_token_account.reload()?;
        input_token_account.amount
    };

    let amount_diff = amount_after
        .checked_sub(amount_before)
        .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

    ctx.accounts
        .protocol_data_as_mut(protocol as usize)
        .update_after_withdraw(amount_diff)?;

    Ok(())
}

#[derive(Accounts)]
pub struct GenericWithdrawAccounts<'info> {
    #[account(
        mut,
        seeds = [VAULT_ACCOUNT_SEED, vault_account.input_mint_pubkey.as_ref()],
        bump = vault_account.bumps.vault
    )]
    pub vault_account: Box<Account<'info, VaultAccount>>,
    #[account(
        mut,
        associated_token::mint = vault_account.input_mint_pubkey,
        associated_token::authority = vault_account,
    )]
    pub vault_input_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub clock: Sysvar<'info, Clock>,
    #[account(address = sysvar::instructions::ID)]
    /// CHECK: address is checked
    pub instructions: AccountInfo<'info>,
}

/// Anchor generated sighash
const IX_WITHDRAW_SIGHASH: [u8; 8] = [183, 18, 70, 156, 148, 109, 161, 34];
/// Instruction data length (sighash + u64)
const IX_WITHDRAW_DATA_LEN: usize = 16;

impl<'info> GenericWithdrawAccounts<'info> {
    /// Compute the amount to withdraw from the protocol depending on whether the instruction comes
    /// from the bot or from a user, assuming for the latter that the following ix corresponds to
    /// the `withdraw` one
    pub fn amount_to_withdraw(&self, protocol_pos: usize) -> Result<u64> {
        self.amount_to_withdraw_in_n_txs(protocol_pos, 1)
    }

    pub fn amount_to_withdraw_in_n_txs(
        &self,
        protocol_pos: usize,
        ix_offset: usize,
    ) -> Result<u64> {
        if let Some(amount) = self.read_amount_from_withdraw_ix(ix_offset)? {
            Ok(amount)
        } else {
            Ok(self.vault_account.calculate_withdraw(protocol_pos)?)
        }
    }

    /// Read the amount to withdraw from the target `withdraw` instruction
    fn read_amount_from_withdraw_ix(&self, target_ix: usize) -> Result<Option<u64>> {
        let current_index =
            sysvar::instructions::load_current_index_checked(&self.instructions)? as usize;

        if let Ok(next_ix) = sysvar::instructions::load_instruction_at_checked(
            current_index.checked_add(target_ix).unwrap(),
            &self.instructions,
        ) {
            let ix_data: &[u8] = &next_ix.data;
            require!(
                next_ix.data.len() == IX_WITHDRAW_DATA_LEN && ix_data[..8] == IX_WITHDRAW_SIGHASH,
                ErrorCode::InvalidInstructions
            );

            // Anchor generated module
            use crate::instruction;
            let ix = instruction::Withdraw::deserialize(&mut &ix_data[8..])
                .map_err(|_| ErrorCode::InvalidInstructions)?;
            let instruction::Withdraw { lp_amount } = ix;

            let amount = self
                .vault_account
                .previous_lp_price
                .lp_to_token(lp_amount)?;
            let vault_token_amount = self.vault_input_token_account.amount;
            require!(amount > vault_token_amount, ErrorCode::InvalidInstructions);

            Ok(Some(
                amount
                    .checked_sub(vault_token_amount)
                    .ok_or_else(|| error!(ErrorCode::MathOverflow))?,
            ))
        } else {
            Ok(None)
        }
    }
}
