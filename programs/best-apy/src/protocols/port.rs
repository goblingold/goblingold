//#! cargo clippy
use crate::check_hash::*;
use crate::error::ErrorCode;
use crate::instructions::{protocol_deposit::*, protocol_rewards::*, protocol_withdraw::*};
use crate::macros::generate_seeds;
use crate::protocols::Protocols;
use crate::vault::ProtocolData;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    hash::{hashv, Hash},
    pubkey::Pubkey,
};
use anchor_spl::token::TokenAccount;

/// Program ids
pub mod port_lending_program_id {
    use anchor_lang::declare_id;
    #[cfg(feature = "devnet")]
    declare_id!("pdQ2rQQU5zH2rDgZ7xH2azMBJegUzUyunJ5Jd637hC4");
    #[cfg(not(feature = "devnet"))]
    declare_id!("Port7uDYB3wk6GJAw4KT1WpTeMtSu9bTcChBHkX2LfR");
}

#[derive(Accounts)]
pub struct PortDeposit<'info> {
    pub generic_accs: GenericDepositAccounts<'info>,
    #[account(constraint = port_lending_program_id.key == &port_lending_program_id::ID)]
    /// CHECK: Port CPI
    pub port_lending_program_id: AccountInfo<'info>,
    #[account(
        mut,
        associated_token::mint = vault_port_collateral_token_account.mint,
        associated_token::authority = generic_accs.vault_account,
    )]
    pub vault_port_collateral_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// CHECK: Port CPI
    pub port_reserve_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Port CPI
    pub port_reserve_liquidity_supply_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Port CPI
    pub port_reserve_collateral_mint_account: AccountInfo<'info>,
    /// CHECK: Port CPI
    pub port_lending_market_account: AccountInfo<'info>,
    /// CHECK: Port CPI
    pub port_lending_market_authority_account: AccountInfo<'info>,
}

impl<'info> CheckHash<'info> for PortDeposit<'info> {
    fn hash(&self) -> Hash {
        hashv(&[
            self.vault_port_collateral_token_account.key().as_ref(),
            self.port_reserve_account.key.as_ref(),
            self.port_reserve_liquidity_supply_account.key.as_ref(),
            self.port_reserve_collateral_mint_account.key.as_ref(),
            self.port_lending_market_account.key.as_ref(),
            self.port_lending_market_authority_account.key.as_ref(),
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

impl<'info> ProtocolDeposit<'info> for PortDeposit<'info> {
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

        let cpi_ctx = CpiContext::new_with_signer(
            self.port_lending_program_id.to_account_info(),
            port_anchor_adaptor::Deposit {
                source_liquidity: self
                    .generic_accs
                    .vault_input_token_account
                    .to_account_info(),
                reserve: self.port_reserve_account.to_account_info(),
                reserve_liquidity_supply: self
                    .port_reserve_liquidity_supply_account
                    .to_account_info(),
                reserve_collateral_mint: self
                    .port_reserve_collateral_mint_account
                    .to_account_info(),
                lending_market: self.port_lending_market_account.to_account_info(),
                lending_market_authority: self
                    .port_lending_market_authority_account
                    .to_account_info(),
                destination_collateral: self.vault_port_collateral_token_account.to_account_info(),
                transfer_authority: self.generic_accs.vault_account.to_account_info(),
                clock: self.generic_accs.clock.to_account_info(),
                token_program: self.generic_accs.token_program.to_account_info(),
            },
            signer,
        );
        port_anchor_adaptor::deposit_reserve(cpi_ctx, amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct PortWithdraw<'info> {
    pub generic_accs: GenericWithdrawAccounts<'info>,
    #[account(constraint = port_lending_program_id.key == &port_lending_program_id::ID)]
    /// CHECK: Port CPI
    pub port_lending_program_id: AccountInfo<'info>,
    #[account(
        mut,
        associated_token::mint = vault_port_collateral_token_account.mint,
        associated_token::authority = generic_accs.vault_account,
    )]
    pub vault_port_collateral_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    /// CHECK: Port CPI
    pub port_reserve_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Port CPI
    pub port_reserve_liquidity_supply_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Port CPI
    pub port_reserve_collateral_mint_account: AccountInfo<'info>,
    /// CHECK: Port CPI
    pub port_lending_market_account: AccountInfo<'info>,
    /// CHECK: Port CPI
    pub port_lending_market_authority_account: AccountInfo<'info>,
}

impl<'info> CheckHash<'info> for PortWithdraw<'info> {
    fn hash(&self) -> Hash {
        hashv(&[
            self.vault_port_collateral_token_account.key().as_ref(),
            self.port_reserve_account.key.as_ref(),
            self.port_reserve_liquidity_supply_account.key.as_ref(),
            self.port_reserve_collateral_mint_account.key.as_ref(),
            self.port_lending_market_account.key.as_ref(),
            self.port_lending_market_authority_account.key.as_ref(),
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

impl<'info> ProtocolWithdraw<'info> for PortWithdraw<'info> {
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
        let mut account_data_slice: &[u8] = &self.port_reserve_account.try_borrow_data()?;
        let reserve = port_anchor_adaptor::PortReserve::try_deserialize(&mut account_data_slice)?;
        let lp_amount = reserve
            .collateral_exchange_rate()?
            .liquidity_to_collateral(amount)?;
        Ok(lp_amount)
    }

    fn cpi_withdraw(&self, amount: u64) -> Result<()> {
        let seeds = generate_seeds!(self.generic_accs.vault_account);
        let signer = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            self.port_lending_program_id.to_account_info(),
            port_anchor_adaptor::Redeem {
                source_collateral: self.vault_port_collateral_token_account.to_account_info(),
                destination_liquidity: self
                    .generic_accs
                    .vault_input_token_account
                    .to_account_info(),
                reserve: self.port_reserve_account.to_account_info(),
                reserve_collateral_mint: self
                    .port_reserve_collateral_mint_account
                    .to_account_info(),
                reserve_liquidity_supply: self
                    .port_reserve_liquidity_supply_account
                    .to_account_info(),
                lending_market: self.port_lending_market_account.to_account_info(),
                lending_market_authority: self
                    .port_lending_market_authority_account
                    .to_account_info(),
                transfer_authority: self.generic_accs.vault_account.to_account_info(),
                clock: self.generic_accs.clock.to_account_info(),
                token_program: self.generic_accs.token_program.to_account_info(),
            },
            signer,
        );
        port_anchor_adaptor::redeem(cpi_ctx, amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct PortTVL<'info> {
    pub generic_accs: GenericTVLAccounts<'info>,
    #[account(owner = port_lending_program_id::ID)]
    /// CHECK: hash, owner and mint & collateral data fields are checked
    pub reserve: AccountInfo<'info>,
    #[account(
        associated_token::mint = vault_port_collateral_token_account.mint,
        associated_token::authority = generic_accs.vault_account,
    )]
    pub vault_port_collateral_token_account: Account<'info, TokenAccount>,
}

impl<'info> CheckHash<'info> for PortTVL<'info> {
    fn hash(&self) -> Hash {
        hashv(&[
            self.reserve.key.as_ref(),
            self.vault_port_collateral_token_account.key().as_ref(),
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

impl<'info> ProtocolRewards<'info> for PortTVL<'info> {
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
        let mut reserve_data: &[u8] = &self.reserve.try_borrow_data()?;
        let reserve = port_anchor_adaptor::PortReserve::try_deserialize(&mut reserve_data)?;

        require!(
            reserve.liquidity.mint_pubkey == self.generic_accs.vault_account.input_mint_pubkey,
            ErrorCode::InvalidMint
        );

        require!(
            reserve.collateral.mint_pubkey == self.vault_port_collateral_token_account.mint,
            ErrorCode::InvalidMint
        );

        let lp_amount = self.vault_port_collateral_token_account.amount;
        let tvl = reserve
            .collateral_exchange_rate()?
            .collateral_to_liquidity(lp_amount)?;

        Ok(tvl)
    }
}
