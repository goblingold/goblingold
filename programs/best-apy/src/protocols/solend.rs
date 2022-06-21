use crate::check_hash::*;
use crate::error::ErrorCode;
use crate::instructions::protocol_withdraw_max::*;
use crate::instructions::{protocol_deposit::*, protocol_rewards::*, protocol_withdraw::*};
use crate::macros::generate_seeds;
use crate::protocols::Protocols;
use crate::vault::ProtocolData;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    hash::{hashv, Hash},
    program::invoke_signed,
    program_pack::Pack,
    pubkey::Pubkey,
};
use anchor_spl::token::TokenAccount;

/// Program id
pub mod solend_program_id {
    use anchor_lang::declare_id;
    #[cfg(feature = "devnet")]
    declare_id!("ALend7Ketfx5bxh6ghsCDXAoDrhvEmsXT3cynB6aPLgx");
    #[cfg(not(feature = "devnet"))]
    declare_id!("So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo");
}

#[derive(Accounts)]
pub struct SolendDeposit<'info> {
    pub generic_accs: GenericDepositAccounts<'info>,
    #[account(constraint = solend_program_id.key == &solend_program_id::ID)]
    /// CHECK: Solend CPI
    pub solend_program_id: AccountInfo<'info>,
    #[account(
        mut,
        associated_token::mint = vault_solend_collateral_token_account.mint,
        associated_token::authority = generic_accs.vault_account,
    )]
    pub vault_solend_collateral_token_account: Box<Account<'info, TokenAccount>>,
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
}

impl<'info> CheckHash<'info> for SolendDeposit<'info> {
    fn hash(&self) -> Hash {
        hashv(&[
            self.vault_solend_collateral_token_account.key().as_ref(),
            self.solend_reserve_account.key.as_ref(),
            self.solend_reserve_liquidity_supply_spl_token_account
                .key
                .as_ref(),
            self.solend_reserve_collateral_spl_token_mint.key.as_ref(),
            self.solend_lending_market_account.key.as_ref(),
            self.solend_derived_lending_market_authority.key.as_ref(),
        ])
    }

    fn target_hash(&self, protocol: Protocols) -> [u8; CHECKHASH_BYTES] {
        let protocol_idx = self
            .generic_accs
            .vault_account
            .protocol_position(protocol)
            .unwrap();
        self.generic_accs.vault_account.protocols[protocol_idx]
            .hash_pubkey
            .hash_deposit
    }
}

impl<'info> ProtocolDeposit<'info> for SolendDeposit<'info> {
    fn protocol_position(&self, protocol: Protocols) -> Result<usize> {
        self.generic_accs.vault_account.protocol_position(protocol)
    }

    fn protocol_data_as_mut(&mut self, protocol_idx: usize) -> &mut ProtocolData {
        &mut self.generic_accs.vault_account.protocols[protocol_idx]
    }

    fn get_amount(&self, protocol_idx: usize) -> Result<u64> {
        self.generic_accs.amount_to_deposit(protocol_idx)
    }

    fn cpi_deposit(&self, amount: u64) -> Result<()> {
        let seeds = generate_seeds!(self.generic_accs.vault_account);
        let signer = &[&seeds[..]];

        let ix = solend_token_lending::instruction::deposit_reserve_liquidity(
            solend_program_id::ID,
            amount,
            self.generic_accs.vault_input_token_account.key(),
            self.vault_solend_collateral_token_account.key(),
            *self.solend_reserve_account.key,
            *self.solend_reserve_liquidity_supply_spl_token_account.key,
            *self.solend_reserve_collateral_spl_token_mint.key,
            *self.solend_lending_market_account.key,
            self.generic_accs.vault_account.key(),
        );
        let accounts = [
            self.generic_accs
                .vault_input_token_account
                .to_account_info(),
            self.vault_solend_collateral_token_account.to_account_info(),
            self.solend_reserve_account.to_account_info(),
            self.solend_reserve_liquidity_supply_spl_token_account
                .to_account_info(),
            self.solend_reserve_collateral_spl_token_mint
                .to_account_info(),
            self.solend_lending_market_account.to_account_info(),
            self.solend_derived_lending_market_authority
                .to_account_info(),
            self.generic_accs.vault_account.to_account_info(),
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
        associated_token::mint = vault_solend_collateral_token_account.mint,
        associated_token::authority = generic_accs.vault_account,
    )]
    pub vault_solend_collateral_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    /// CHECK: Solend CPI
    pub solend_reserve_account: AccountInfo<'info>,
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

impl<'info> CheckHash<'info> for SolendWithdraw<'info> {
    fn hash(&self) -> Hash {
        hashv(&[
            self.vault_solend_collateral_token_account.key().as_ref(),
            self.solend_reserve_account.key.as_ref(),
            self.solend_lending_market_account.key.as_ref(),
            self.solend_derived_lending_market_authority.key.as_ref(),
            self.solend_reserve_collateral_spl_token_mint.key.as_ref(),
            self.solend_reserve_liquidity_supply_spl_token_account
                .key
                .as_ref(),
        ])
    }

    fn target_hash(&self, protocol: Protocols) -> [u8; CHECKHASH_BYTES] {
        let protocol_idx = self
            .generic_accs
            .vault_account
            .protocol_position(protocol)
            .unwrap();
        self.generic_accs.vault_account.protocols[protocol_idx]
            .hash_pubkey
            .hash_withdraw
    }
}

impl<'info> ProtocolWithdraw<'info> for SolendWithdraw<'info> {
    fn protocol_position(&self, protocol: Protocols) -> Result<usize> {
        self.generic_accs.vault_account.protocol_position(protocol)
    }

    fn protocol_data_as_mut(&mut self, protocol_idx: usize) -> &mut ProtocolData {
        &mut self.generic_accs.vault_account.protocols[protocol_idx]
    }

    fn input_token_account_as_mut(&mut self) -> &mut Account<'info, TokenAccount> {
        &mut self.generic_accs.vault_input_token_account
    }

    fn get_amount(&self, protocol_idx: usize) -> Result<u64> {
        self.generic_accs.amount_to_withdraw(protocol_idx)
    }

    fn liquidity_to_collateral(&self, amount: u64) -> Result<u64> {
        let reserve = solend_token_lending::state::Reserve::unpack(
            &self.solend_reserve_account.data.borrow(),
        )?;
        let lp_amount = reserve
            .collateral_exchange_rate()?
            .liquidity_to_collateral(amount)?;
        Ok(lp_amount)
    }

    fn cpi_withdraw(&self, amount: u64) -> Result<()> {
        let seeds = generate_seeds!(self.generic_accs.vault_account);
        let signer = &[&seeds[..]];

        let ix = solend_token_lending::instruction::redeem_reserve_collateral(
            solend_program_id::ID,
            amount,
            self.vault_solend_collateral_token_account.key(),
            self.generic_accs.vault_input_token_account.key(),
            *self.solend_reserve_account.key,
            *self.solend_reserve_collateral_spl_token_mint.key,
            *self.solend_reserve_liquidity_supply_spl_token_account.key,
            *self.solend_lending_market_account.key,
            self.generic_accs.vault_account.key(),
        );
        let accounts = [
            self.vault_solend_collateral_token_account.to_account_info(),
            self.generic_accs
                .vault_input_token_account
                .to_account_info(),
            self.solend_reserve_account.to_account_info(),
            self.solend_reserve_collateral_spl_token_mint
                .to_account_info(),
            self.solend_reserve_liquidity_supply_spl_token_account
                .to_account_info(),
            self.solend_lending_market_account.to_account_info(),
            self.solend_derived_lending_market_authority
                .to_account_info(),
            self.generic_accs.vault_account.to_account_info(),
            self.generic_accs.clock.to_account_info(),
            self.generic_accs.token_program.to_account_info(),
        ];
        invoke_signed(&ix, &accounts, signer)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SolendWithdrawMax<'info> {
    pub user_signer: Signer<'info>,
    pub generic_accs: GenericWithdrawAccounts<'info>,
    #[account(constraint = solend_program_id.key == &solend_program_id::ID)]
    /// CHECK: Solend CPI
    pub solend_program_id: AccountInfo<'info>,
    #[account(
        mut,
        associated_token::mint = vault_solend_collateral_token_account.mint,
        associated_token::authority = generic_accs.vault_account,
    )]
    pub vault_solend_collateral_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    /// CHECK: Solend CPI
    pub solend_reserve_account: AccountInfo<'info>,
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

impl<'info> CheckHash<'info> for SolendWithdrawMax<'info> {
    fn hash(&self) -> Hash {
        hashv(&[
            self.vault_solend_collateral_token_account.key().as_ref(),
            self.solend_reserve_account.key.as_ref(),
            self.solend_lending_market_account.key.as_ref(),
            self.solend_derived_lending_market_authority.key.as_ref(),
            self.solend_reserve_collateral_spl_token_mint.key.as_ref(),
            self.solend_reserve_liquidity_supply_spl_token_account
                .key
                .as_ref(),
        ])
    }

    fn target_hash(&self, protocol: Protocols) -> [u8; CHECKHASH_BYTES] {
        let protocol_idx = self
            .generic_accs
            .vault_account
            .protocol_position(protocol)
            .unwrap();
        self.generic_accs.vault_account.protocols[protocol_idx]
            .hash_pubkey
            .hash_withdraw
    }
}

impl<'info> ProtocolWithdrawMax<'info> for SolendWithdrawMax<'info> {
    fn protocol_position(&self, protocol: Protocols) -> Result<usize> {
        self.generic_accs.vault_account.protocol_position(protocol)
    }

    fn protocol_data_as_mut(&mut self, protocol_idx: usize) -> &mut ProtocolData {
        &mut self.generic_accs.vault_account.protocols[protocol_idx]
    }

    fn input_token_account_as_mut(&mut self) -> &mut Account<'info, TokenAccount> {
        &mut self.generic_accs.vault_input_token_account
    }

    fn get_amount(&self, min_amount: u64) -> Result<u64> {
        let reserve = solend_token_lending::state::Reserve::unpack(
            &self.solend_reserve_account.data.borrow(),
        )?;

        let amount = reserve.liquidity.available_amount;
        require!(amount > min_amount, ErrorCode::InvalidProtocolWithdraw);
        Ok(amount)
    }

    fn liquidity_to_collateral(&self, amount: u64) -> Result<u64> {
        let reserve = solend_token_lending::state::Reserve::unpack(
            &self.solend_reserve_account.data.borrow(),
        )?;
        let lp_amount = reserve
            .collateral_exchange_rate()?
            .liquidity_to_collateral(amount)?;
        Ok(lp_amount)
    }

    fn cpi_withdraw(&self, amount: u64) -> Result<()> {
        let seeds = generate_seeds!(self.generic_accs.vault_account);
        let signer = &[&seeds[..]];

        let ix = solend_token_lending::instruction::redeem_reserve_collateral(
            solend_program_id::ID,
            amount,
            self.vault_solend_collateral_token_account.key(),
            self.generic_accs.vault_input_token_account.key(),
            *self.solend_reserve_account.key,
            *self.solend_reserve_collateral_spl_token_mint.key,
            *self.solend_reserve_liquidity_supply_spl_token_account.key,
            *self.solend_lending_market_account.key,
            self.generic_accs.vault_account.key(),
        );
        let accounts = [
            self.vault_solend_collateral_token_account.to_account_info(),
            self.generic_accs
                .vault_input_token_account
                .to_account_info(),
            self.solend_reserve_account.to_account_info(),
            self.solend_reserve_collateral_spl_token_mint
                .to_account_info(),
            self.solend_reserve_liquidity_supply_spl_token_account
                .to_account_info(),
            self.solend_lending_market_account.to_account_info(),
            self.solend_derived_lending_market_authority
                .to_account_info(),
            self.generic_accs.vault_account.to_account_info(),
            self.generic_accs.clock.to_account_info(),
            self.generic_accs.token_program.to_account_info(),
        ];
        invoke_signed(&ix, &accounts, signer)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SolendTVL<'info> {
    pub generic_accs: GenericTVLAccounts<'info>,
    #[account(owner = solend_program_id::ID)]
    /// CHECK: hash, owner and mint & collateral data fields are checked
    pub reserve: AccountInfo<'info>,
    #[account(
        associated_token::mint = vault_solend_collateral_token_account.mint,
        associated_token::authority = generic_accs.vault_account,
    )]
    pub vault_solend_collateral_token_account: Account<'info, TokenAccount>,
}

impl<'info> CheckHash<'info> for SolendTVL<'info> {
    fn hash(&self) -> Hash {
        hashv(&[
            self.reserve.key.as_ref(),
            self.vault_solend_collateral_token_account.key().as_ref(),
        ])
    }

    fn target_hash(&self, protocol: Protocols) -> [u8; CHECKHASH_BYTES] {
        let protocol_idx = self
            .generic_accs
            .vault_account
            .protocol_position(protocol)
            .unwrap();
        self.generic_accs.vault_account.protocols[protocol_idx]
            .hash_pubkey
            .hash_tvl
    }
}

impl<'info> ProtocolRewards<'info> for SolendTVL<'info> {
    fn protocol_position(&self, protocol: Protocols) -> Result<usize> {
        self.generic_accs.vault_account.protocol_position(protocol)
    }

    fn input_mint_pubkey(&self) -> Pubkey {
        self.generic_accs.vault_account.input_mint_pubkey
    }

    fn protocol_data_as_mut(&mut self, protocol_idx: usize) -> &mut ProtocolData {
        &mut self.generic_accs.vault_account.protocols[protocol_idx]
    }

    fn max_withdrawable(&self) -> Result<u64> {
        let reserve = solend_token_lending::state::Reserve::unpack(&self.reserve.data.borrow())?;

        require!(
            reserve.liquidity.mint_pubkey == self.generic_accs.vault_account.input_mint_pubkey,
            ErrorCode::InvalidMint
        );

        require!(
            reserve.collateral.mint_pubkey == self.vault_solend_collateral_token_account.mint,
            ErrorCode::InvalidMint
        );

        let lp_amount = self.vault_solend_collateral_token_account.amount;
        let tvl = reserve
            .collateral_exchange_rate()?
            .collateral_to_liquidity(lp_amount)?;

        Ok(tvl)
    }
}
