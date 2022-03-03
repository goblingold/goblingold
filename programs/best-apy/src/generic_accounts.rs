use crate::error::ErrorCode;
use crate::protocols::Protocols;
use crate::{GenericDepositAccounts, GenericWithdrawAccounts};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;

impl<'info> GenericDepositAccounts<'info> {
    /// Compute the amount to deposit into the protocol
    pub fn amount_to_deposit(&self, protocol: Protocols) -> Result<u64> {
        self.vault_account
            .calculate_deposit(protocol, self.vault_input_token_account.amount)
    }
}

impl<'info> GenericWithdrawAccounts<'info> {
    /// Compute the amount to withdraw from the protocol depending on whether the instruction comes
    /// from the bot or from a user, assuming for the latter that the following ix corresponds to
    /// the `withdraw` one
    pub fn amount_to_withdraw(&self, protocol: Protocols) -> Result<u64> {
        self.amount_to_withdraw_in_n_txs(protocol, 1)
    }

    /// Compute the amount to withdraw from the protocol depending on whether the instruction comes
    /// from the bot or from a user, assuming for the latter that the n-following ix corresponds to
    /// the `withdraw` one
    pub fn amount_to_withdraw_in_n_txs(
        &self,
        protocol: Protocols,
        target_ix: usize,
    ) -> Result<u64> {
        if let Some(amount_target_ix) = self.read_amount_from_withdraw_ix(target_ix)? {
            Ok(amount_target_ix)
        } else {
            Ok(self.vault_account.calculate_withdraw(protocol)?)
        }
    }

    /// Read the amount to withdraw from the target `withdraw` instruction
    fn read_amount_from_withdraw_ix(&self, target_ix_index: usize) -> Result<Option<u64>> {
        let current_index =
            sysvar::instructions::load_current_index_checked(&self.instructions)? as usize;

        if let Ok(next_ix) = sysvar::instructions::load_instruction_at_checked(
            current_index + target_ix_index,
            &self.instructions,
        ) {
            let ix_data: &[u8] = &next_ix.data;
            require!(
                // Anchor generated sighash
                ix_data[..8] == [183, 18, 70, 156, 148, 109, 161, 34],
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
                    .ok_or(ErrorCode::MathOverflow)?,
            ))
        } else {
            Ok(None)
        }
    }
}
