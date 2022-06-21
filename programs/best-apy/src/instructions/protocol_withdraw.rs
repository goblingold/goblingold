use crate::error::ErrorCode;
use crate::protocols::Protocols;
use crate::vault::{ProtocolData, VaultAccount};
use crate::VAULT_ACCOUNT_SEED;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;
use anchor_spl::token::{Token, TokenAccount};

/// Withdraw from the protocol
pub trait ProtocolWithdraw<'info> {
    /// Return the protcol position in the vector
    fn protocol_position(&self, protocol: Protocols) -> Result<usize>;

    /// Return a mutable refrence of the data
    fn protocol_data_as_mut(&mut self, protocol_idx: usize) -> &mut ProtocolData;

    /// Return the input token account
    fn input_token_account_as_mut(&mut self) -> &mut Account<'info, TokenAccount>;

    /// Compute the amount to withdraw
    fn get_amount(&self, protocol_idx: usize) -> Result<AmountWithCaller>;

    /// Return maximum liquidity available for withdrawal from the protocol
    fn max_liquidity(&self) -> Result<u64> {
        Ok(u64::MAX)
    }

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
    let protocol_idx = ctx.accounts.protocol_position(protocol)?;

    let AmountWithCaller { mut amount, caller } = ctx.accounts.get_amount(protocol_idx)?;
    if !ctx.accounts.protocol_data_as_mut(protocol_idx).is_active() && caller == Caller::Bot {
        amount = std::cmp::min(amount, ctx.accounts.max_liquidity()?);
    }

    let mut lp_amount = ctx.accounts.liquidity_to_collateral(amount)?;

    // Add 1 as due to rounding. Otherwise it might happens that there wasn't enough funds
    // withdrawn from the protocol
    if amount < ctx.accounts.protocol_data_as_mut(protocol_idx).amount {
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
        .protocol_data_as_mut(protocol_idx)
        .update_after_withdraw(amount_diff)?;

    Ok(())
}

/// Amount to withdraw and who call it
pub struct AmountWithCaller {
    pub amount: u64,
    pub caller: Caller,
}

/// Who can call the withdraw ix
#[derive(PartialEq, Eq)]
pub enum Caller {
    User,
    Bot,
}

#[derive(Accounts)]
pub struct GenericWithdrawAccounts<'info> {
    #[account(
        mut,
        seeds = [VAULT_ACCOUNT_SEED, &[vault_account.seed_number][..], vault_account.input_mint_pubkey.as_ref()],
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

impl<'info> GenericWithdrawAccounts<'info> {
    /// Compute the amount to withdraw from the protocol depending on whether the instruction comes
    /// from the bot or from a user, assuming for the latter that the following ix corresponds
    /// either to the `withdraw` or the `close_withdraw_ticket` one
    pub fn amount_to_withdraw(&self, protocol_idx: usize) -> Result<AmountWithCaller> {
        if let Some(amount) = self.read_amount_from_next_ix()? {
            Ok(AmountWithCaller {
                amount,
                caller: Caller::User,
            })
        } else {
            Ok(AmountWithCaller {
                amount: self.vault_account.calculate_withdraw(protocol_idx)?,
                caller: Caller::Bot,
            })
        }
    }

    /// Read the amount to withdraw from the next instruction
    fn read_amount_from_next_ix(&self) -> Result<Option<u64>> {
        if let Ok(next_ix) = sysvar::instructions::get_instruction_relative(1, &self.instructions) {
            let ix_data: &[u8] = &next_ix.data;

            let lp_amount = read_amount_from_deserialized_ix(ix_data)?;

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

/// Anchor generated sighash
const IX_WITHDRAW_SIGHASH: [u8; 8] = [183, 18, 70, 156, 148, 109, 161, 34];
const IX_CLOSE_WITHDRAW_TICKET_SIGHASH: [u8; 8] = [59, 115, 209, 162, 26, 58, 153, 83];

/// Instruction data length (sighash + args)
const IX_WITHDRAW_DATA_LEN: usize = 8 + 8;
const IX_CLOSE_WITHDRAW_TICKET_DATA_LEN: usize = 8 + 1 + 8;

fn read_amount_from_deserialized_ix(ix_data: &[u8]) -> Result<u64> {
    require!(ix_data.len() > 8, ErrorCode::InvalidInstructions);

    let sighash: [u8; 8] = {
        let mut sighash: [u8; 8] = [0; 8];
        sighash.copy_from_slice(&ix_data[..8]);
        sighash
    };

    // Anchor generated module
    use crate::instruction;
    match sighash {
        IX_WITHDRAW_SIGHASH => {
            require!(
                ix_data.len() == IX_WITHDRAW_DATA_LEN,
                ErrorCode::InvalidInstructions
            );

            let ix = instruction::Withdraw::deserialize(&mut &ix_data[8..])
                .map_err(|_| ErrorCode::InvalidInstructions)?;
            let instruction::Withdraw { lp_amount } = ix;
            Ok(lp_amount)
        }
        IX_CLOSE_WITHDRAW_TICKET_SIGHASH => {
            require!(
                ix_data.len() == IX_CLOSE_WITHDRAW_TICKET_DATA_LEN,
                ErrorCode::InvalidInstructions
            );

            let ix = instruction::CloseWithdrawTicket::deserialize(&mut &ix_data[8..])
                .map_err(|_| ErrorCode::InvalidInstructions)?;
            let instruction::CloseWithdrawTicket { lp_amount, .. } = ix;
            Ok(lp_amount)
        }
        _ => err!(ErrorCode::InvalidInstructions),
    }
}
