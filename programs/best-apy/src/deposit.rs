use crate::error::ErrorCode;
use crate::macros::generate_seeds;
use crate::vault::LpPrice;
use crate::Deposit;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Transfer};

impl<'info> Deposit<'info> {
    /// Deposit user input tokens into the vault account
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        require!(amount >= 100, ErrorCode::InvalidDepositAmount);
        msg!("GoblinGold: Deposit");

        // Transfer user token to vault account
        let cpi_accounts = Transfer {
            from: self.user_input_token_account.to_account_info(),
            to: self.vault_input_token_account.to_account_info(),
            authority: self.user_signer.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        // Mint vault tokens to user vault account
        let lp_amount = LpPrice {
            total_tokens: self.vault_account.current_tvl,
            minted_tokens: self.vault_lp_token_mint_pubkey.supply,
        }
        .token_to_lp(amount)?;
        let lp_amount_previous_price = self
            .vault_account
            .previous_lp_price
            .lp_to_token(lp_amount)?;
        require!(
            lp_amount_previous_price < lp_amount,
            ErrorCode::InvalidLpPrice
        );

        // Update total deposited amounts
        self.vault_account.current_tvl = self
            .vault_account
            .current_tvl
            .checked_add(amount)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

        let seeds = generate_seeds!(self.vault_account);
        let signer = &[&seeds[..]];

        let cpi_accounts = MintTo {
            mint: self.vault_lp_token_mint_pubkey.to_account_info(),
            to: self.user_lp_token_account.to_account_info(),
            authority: self.vault_signer.clone(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::mint_to(cpi_ctx, lp_amount)?;

        Ok(())
    }
}
