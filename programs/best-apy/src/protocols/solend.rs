use crate::error::ErrorCode;
use crate::macros::generate_seeds;
use crate::protocols::Protocols;
use crate::vault::{TokenBalances, VaultAccount};
use crate::PubkeyWrapper;
use crate::ALLOWED_DEPLOYER;
use crate::{
    generic_accounts_anchor_modules::*, GenericDepositAccounts, GenericTVLAccounts,
    GenericWithdrawAccounts,
};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::Instruction, program::invoke_signed, program_pack::Pack, pubkey::Pubkey,
    system_instruction,
};
use anchor_spl::token::{Token, TokenAccount};
use std::str::FromStr;

/// Program id
pub mod solend_program_id {
    use anchor_lang::declare_id;
    #[cfg(feature = "devnet")]
    declare_id!("ALend7Ketfx5bxh6ghsCDXAoDrhvEmsXT3cynB6aPLgx");
    #[cfg(not(feature = "devnet"))]
    declare_id!("So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo");
}

#[derive(Accounts)]
pub struct SolendInitialize<'info> {
    #[account(constraint = Pubkey::from_str(ALLOWED_DEPLOYER).unwrap()== *user_signer.key)]
    pub user_signer: Signer<'info>,
    #[account(mut)]
    /// CHECK: Solend CPI
    pub vault_solend_obligation_account: AccountInfo<'info>,
    /// CHECK: Solend CPI
    pub solend_lending_market_account: AccountInfo<'info>,
    pub vault_account: Box<Account<'info, VaultAccount>>,
    #[account(seeds = [vault_account.to_account_info().key.as_ref()], bump = vault_account.bump)]
    /// CHECK: only used as signing PDA
    pub vault_signer: AccountInfo<'info>,
    #[account(constraint = solend_program_id.key == &solend_program_id::ID)]
    /// CHECK: Solend CPI
    pub solend_program_id: AccountInfo<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> SolendInitialize<'info> {
    /// Create and initialize protocol accounts
    pub fn create_and_initialize(&self) -> Result<()> {
        let seeds = generate_seeds!(self.vault_account);
        let signer = &[&seeds[..]];

        {
            let account_size = solend_token_lending::state::Obligation::LEN;
            let ix = system_instruction::create_account_with_seed(
                self.user_signer.key,
                self.vault_solend_obligation_account.key,
                self.vault_signer.key,
                &self.solend_lending_market_account.key.to_string()[..32],
                Rent::default().minimum_balance(account_size),
                account_size as u64,
                self.solend_program_id.key,
            );
            invoke_signed(
                &ix,
                &[
                    self.user_signer.to_account_info(),
                    self.vault_signer.to_account_info(),
                    self.vault_solend_obligation_account.to_account_info(),
                ],
                signer,
            )?;
        }

        {
            let ix = solend_token_lending::instruction::init_obligation(
                solend_program_id::ID,
                *self.vault_solend_obligation_account.key,
                *self.solend_lending_market_account.key,
                *self.vault_signer.key,
            );
            let accounts = [
                self.vault_solend_obligation_account.to_account_info(),
                self.solend_lending_market_account.to_account_info(),
                self.vault_signer.to_account_info(),
                self.clock.to_account_info(),
                self.rent.to_account_info(),
                self.token_program.to_account_info(),
            ];
            invoke_signed(&ix, &accounts, signer)?;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SolendDeposit<'info> {
    pub generic_accs: GenericDepositAccounts<'info>,
    #[account(constraint = solend_program_id.key == &solend_program_id::ID)]
    /// CHECK: Solend CPI
    pub solend_program_id: AccountInfo<'info>,
    #[account(
        mut,
        associated_token::mint = PubkeyWrapper(vault_solend_destination_collateral_token_account.mint),
        associated_token::authority = generic_accs.vault_signer,
    )]
    pub vault_solend_destination_collateral_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// CHECK: Solend CPI
    pub vault_solend_obligation_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Solend CPI
    pub solend_reserve_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Solend CPI
    pub solend_reserve_liquidity_supply_spl_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Solend CPI
    pub solend_reserve_collateral_spl_token_mint: AccountInfo<'info>,
    /// CHECK: Solend CPI
    pub solend_lending_market_account: AccountInfo<'info>,
    /// CHECK: Solend CPI
    pub solend_derived_lending_market_authority: AccountInfo<'info>,
    #[account(mut)]
    pub solend_destination_deposit_reserve_collateral_supply_spl_token_account:
        Account<'info, TokenAccount>,
    /// CHECK: Solend CPI
    pub solend_pyth_price_oracle_account: AccountInfo<'info>,
    /// CHECK: Solend CPI
    pub solend_switchboard_price_feed_oracle_account: AccountInfo<'info>,
}

impl<'info> SolendDeposit<'info> {
    /// Deposit into protocol
    pub fn deposit(&mut self) -> Result<()> {
        let amount = self.generic_accs.amount_to_deposit(Protocols::Solend)?;
        let balances = self.deposit_and_get_balances(amount)?;

        self.generic_accs.vault_account.protocols[Protocols::Solend as usize]
            .update_after_deposit(self.generic_accs.clock.slot, balances)?;

        Ok(())
    }

    /// Deposit into the protocol and get the true token balances
    fn deposit_and_get_balances(&mut self, amount: u64) -> Result<TokenBalances> {
        let lp_before = self
            .solend_destination_deposit_reserve_collateral_supply_spl_token_account
            .amount;

        self.cpi_deposit(amount)?;

        self.solend_destination_deposit_reserve_collateral_supply_spl_token_account
            .reload()?;

        let lp_after = self
            .solend_destination_deposit_reserve_collateral_supply_spl_token_account
            .amount;
        let lp_amount = lp_after
            .checked_sub(lp_before)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

        Ok(TokenBalances {
            base_amount: amount,
            lp_amount,
        })
    }

    /// CPI deposit call
    fn cpi_deposit(&self, amount: u64) -> Result<()> {
        let seeds = generate_seeds!(self.generic_accs.vault_account);
        let signer = &[&seeds[..]];

        let ix =
            solend_token_lending::instruction::deposit_reserve_liquidity_and_obligation_collateral(
                solend_program_id::ID,
                amount,
                self.generic_accs.vault_input_token_account.key(),
                self.vault_solend_destination_collateral_token_account.key(),
                *self.solend_reserve_account.key,
                *self.solend_reserve_liquidity_supply_spl_token_account.key,
                *self.solend_reserve_collateral_spl_token_mint.key,
                *self.solend_lending_market_account.key,
                self.solend_destination_deposit_reserve_collateral_supply_spl_token_account
                    .key(),
                *self.vault_solend_obligation_account.key,
                *self.generic_accs.vault_signer.key,
                *self.solend_pyth_price_oracle_account.key,
                *self.solend_switchboard_price_feed_oracle_account.key,
                *self.generic_accs.vault_signer.key,
            );
        let accounts = [
            self.generic_accs
                .vault_input_token_account
                .to_account_info(),
            self.vault_solend_destination_collateral_token_account
                .to_account_info(),
            self.solend_reserve_account.to_account_info(),
            self.solend_reserve_liquidity_supply_spl_token_account
                .to_account_info(),
            self.solend_reserve_collateral_spl_token_mint
                .to_account_info(),
            self.solend_lending_market_account.to_account_info(),
            self.solend_derived_lending_market_authority
                .to_account_info(),
            self.solend_destination_deposit_reserve_collateral_supply_spl_token_account
                .to_account_info(),
            self.vault_solend_obligation_account.to_account_info(),
            self.generic_accs.vault_signer.to_account_info(),
            self.solend_pyth_price_oracle_account.to_account_info(),
            self.solend_switchboard_price_feed_oracle_account
                .to_account_info(),
            self.generic_accs.vault_signer.to_account_info(),
            self.generic_accs.clock.to_account_info(),
            self.generic_accs.token_program.to_account_info(),
        ];
        invoke_signed(&ix, &accounts, signer)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SolendWithdraw<'info> {
    pub generic_accs: GenericWithdrawAccounts<'info>,
    #[account(constraint = solend_program_id.key == &solend_program_id::ID)]
    /// CHECK: Solend CPI
    pub solend_program_id: AccountInfo<'info>,
    #[account(
        mut,
        associated_token::mint = PubkeyWrapper(vault_solend_destination_collateral_token_account.mint),
        associated_token::authority = generic_accs.vault_signer,
    )]
    pub vault_solend_destination_collateral_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    /// CHECK: Solend CPI
    pub vault_solend_obligation_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Solend CPI
    pub solend_source_withdraw_reserve_collateral_supply_spl_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Solend CPI
    pub solend_withdraw_reserve_account: AccountInfo<'info>,
    /// CHECK: Solend CPI
    pub solend_lending_market_account: AccountInfo<'info>,
    /// CHECK: Solend CPI
    pub solend_derived_lending_market_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Solend CPI
    pub solend_reserve_collateral_spl_token_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Solend CPI
    pub solend_reserve_liquidity_supply_spl_token_account: AccountInfo<'info>,
}

impl<'info> SolendWithdraw<'info> {
    /// Withdraw from the protocol
    pub fn withdraw(&mut self) -> Result<()> {
        let amount = self.generic_accs.amount_to_withdraw(Protocols::Solend)?;
        let balances = self.withdraw_and_get_balances(amount)?;

        self.generic_accs.vault_account.protocols[Protocols::Solend as usize]
            .update_after_withdraw(self.generic_accs.clock.slot, balances)?;

        Ok(())
    }

    /// Convert reserve liquidity to collateral
    fn liquidity_to_collateral(&self, amount: u64) -> Result<u64> {
        let reserve = solend_token_lending::state::Reserve::unpack(
            &self.solend_withdraw_reserve_account.data.borrow(),
        )?;
        let lp_amount = reserve
            .collateral_exchange_rate()?
            .liquidity_to_collateral(amount)?;
        Ok(lp_amount)
    }

    /// Withdraw from the protocol and get the true token balances
    fn withdraw_and_get_balances(&mut self, amount: u64) -> Result<TokenBalances> {
        let lp_amount = self.liquidity_to_collateral(amount)?;
        let amount_before = self.generic_accs.vault_input_token_account.amount;

        self.cpi_withdraw(lp_amount)?;

        self.generic_accs.vault_input_token_account.reload()?;

        let amount_after = self.generic_accs.vault_input_token_account.amount;
        let amount_diff = amount_after
            .checked_sub(amount_before)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

        Ok(TokenBalances {
            base_amount: amount_diff,
            lp_amount,
        })
    }

    /// CPI withdraw call
    fn cpi_withdraw(&self, amount: u64) -> Result<()> {
        let seeds = generate_seeds!(self.generic_accs.vault_account);
        let signer = &[&seeds[..]];

        // Solend does not provide a function wrapper for
        // WithdrawObligationCollateralAndRedeemReserveCollateral ix
        let accounts = [
            self.solend_source_withdraw_reserve_collateral_supply_spl_token_account
                .to_account_info(),
            self.vault_solend_destination_collateral_token_account
                .to_account_info(),
            self.solend_withdraw_reserve_account.to_account_info(),
            self.vault_solend_obligation_account.to_account_info(),
            self.solend_lending_market_account.to_account_info(),
            self.solend_derived_lending_market_authority
                .to_account_info(),
            self.generic_accs
                .vault_input_token_account
                .to_account_info(),
            self.solend_reserve_collateral_spl_token_mint
                .to_account_info(),
            self.solend_reserve_liquidity_supply_spl_token_account
                .to_account_info(),
            self.generic_accs.vault_signer.to_account_info(),
            self.generic_accs.vault_signer.to_account_info(),
            self.generic_accs.clock.to_account_info(),
            self.generic_accs.token_program.to_account_info(),
        ];
        let account_metas = accounts
            .iter()
            .map(|acc| {
                if acc.key == self.generic_accs.vault_signer.key {
                    AccountMeta::new_readonly(*acc.key, true)
                } else if acc.is_writable {
                    AccountMeta::new(*acc.key, false)
                } else {
                    AccountMeta::new_readonly(*acc.key, false)
                }
            })
            .collect::<Vec<_>>();

        use solend_token_lending::instruction::LendingInstruction;
        let ix = Instruction {
            program_id: solend_program_id::ID,
            accounts: account_metas,
            data: LendingInstruction::WithdrawObligationCollateralAndRedeemReserveCollateral {
                collateral_amount: amount,
            }
            .pack(),
        };
        invoke_signed(&ix, &accounts, signer)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SolendTVL<'info> {
    pub generic_accs: GenericTVLAccounts<'info>,
    /// CHECK: Solend CPI
    pub reserve: AccountInfo<'info>,
}

impl<'info> SolendTVL<'info> {
    /// Update the protocol TVL
    pub fn update_rewards(&mut self) -> Result<()> {
        let slot = self.generic_accs.clock.slot;
        let tvl = self.max_withdrawable()?;

        let protocol = &mut self.generic_accs.vault_account.protocols[Protocols::Solend as usize];
        msg!("Solend TVL {} and base_amount {}", tvl, protocol.tokens.base_amount);
        let rewards = tvl
        .saturating_sub(protocol.tokens.base_amount);

        protocol.rewards.update(slot, rewards)?;

        Ok(())
    }

    /// Calculate the max native units to withdraw
    fn max_withdrawable(&self) -> Result<u64> {
        let protocol = self.generic_accs.vault_account.protocols[Protocols::Solend as usize];
        self.lp_to_liquidity(protocol.tokens.lp_amount)
    }

    /// Convert reserve collateral to liquidity
    fn lp_to_liquidity(&self, lp_amount: u64) -> Result<u64> {
        let reserve = solend_token_lending::state::Reserve::unpack(&self.reserve.data.borrow())?;

        require!(
            reserve.liquidity.mint_pubkey == self.generic_accs.vault_account.input_mint_pubkey,
            ErrorCode::InvalidMint
        );

        let tvl = reserve
            .collateral_exchange_rate()?
            .collateral_to_liquidity(lp_amount)?;

        Ok(tvl)
    }
}
