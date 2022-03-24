use crate::error::ErrorCode;
use crate::macros::generate_seeds;
use crate::vault::LpPrice;
use crate::Withdraw;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Transfer};

impl<'info> Withdraw<'info> {
    /// Withdraw the required input tokens from the vault and send them back to the user
    pub fn withdraw(&mut self, lp_amount: u64) -> Result<()> {
        msg!("GoblinGold: Withdraw");
        // Use previous value of LP
        // In order to avoid depositors
        let amount = self
            .vault_account
            .previous_lp_price
            .lp_to_token(lp_amount)?;
        let amount_current_price = LpPrice {
            total_tokens: self.vault_account.current_tvl,
            minted_tokens: self.vault_lp_token_mint_pubkey.supply,
        }
        .lp_to_token(amount)?;
        require!(amount > 0, ErrorCode::InvalidZeroWithdraw);
        require!(amount < amount_current_price, ErrorCode::InvalidLpPrice);

        let seeds = generate_seeds!(self.vault_account);
        let signer = &[&seeds[..]];

        // Burn user vault tokens
        let cpi_accounts = Burn {
            mint: self.vault_lp_token_mint_pubkey.to_account_info(),
            to: self.user_lp_token_account.to_account_info(),
            authority: self.user_signer.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::burn(cpi_ctx, lp_amount)?;

        // Transfer tokens back to user
        let cpi_accounts = Transfer {
            from: self.vault_input_token_account.to_account_info(),
            to: self.user_input_token_account.to_account_info(),
            authority: self.vault_signer.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, amount)?;

        // Update total withdraw
        self.vault_account.current_tvl = self
            .vault_account
            .current_tvl
            .checked_sub(amount)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

        Ok(())
    }
}
