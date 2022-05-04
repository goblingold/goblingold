use crate::check_hash::*;
use crate::error::ErrorCode;
use crate::instructions::{protocol_deposit::*, protocol_rewards::*, protocol_withdraw::*};
use crate::macros::generate_seeds;
use crate::protocols::{state::francium_lending_pool, Protocols};
use crate::vault::ProtocolData;

use anchor_lang::prelude::borsh::{BorshDeserialize, BorshSerialize};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    hash::{hashv, Hash},
    instruction::Instruction,
    program::invoke_signed,
    program_pack::Pack,
    sysvar,
};
use anchor_spl::token::TokenAccount;

/// Program ids
pub mod francium_lending_program_id {
    use anchor_lang::declare_id;
    declare_id!("FC81tbGt6JWRXidaWYFXxGnTk4VgobhJHATvTRVMqgWj");
}

/// Instruction data
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct InstructionData {
    pub instruction: u8,
}

/// Instruction data with amount
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct InstructionAmountData {
    pub instruction: u8,
    pub amount: u64,
}

#[derive(Accounts)]
pub struct FranciumDeposit<'info> {
    pub generic_accs: GenericDepositAccounts<'info>,
    #[account(address = sysvar::instructions::ID)]
    /// CHECK: address is checked
    pub instructions: AccountInfo<'info>,
    #[account(constraint = francium_lending_program_id.key == &francium_lending_program_id::ID)]
    /// CHECK: Francium CPI
    pub francium_lending_program_id: AccountInfo<'info>,
    #[account(
        mut,
        associated_token::mint = vault_francium_collateral_token_account.mint,
        associated_token::authority = generic_accs.vault_account,
    )]
    pub vault_francium_collateral_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// CHECK: Francium CPI
    pub francium_lending_pool_info_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Francium CPI
    pub francium_lending_pool_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Francium CPI
    pub francium_farming_pool_stake_token_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Francium CPI
    pub francium_market_info_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Francium CPI
    pub francium_lending_market_authority: AccountInfo<'info>,
}

impl<'info> CheckHash<'info> for FranciumDeposit<'info> {
    fn hash(&self) -> Hash {
        hashv(&[
            self.vault_francium_collateral_token_account.key().as_ref(),
            self.francium_lending_pool_info_account.key.as_ref(),
            self.francium_lending_pool_token_account.key.as_ref(),
            self.francium_farming_pool_stake_token_mint.key.as_ref(),
            self.francium_market_info_account.key.as_ref(),
            self.francium_lending_market_authority.key.as_ref(),
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

impl<'info> ProtocolDeposit<'info> for FranciumDeposit<'info> {
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

        let accounts = [
            self.generic_accs
                .vault_input_token_account
                .to_account_info(),
            self.vault_francium_collateral_token_account
                .to_account_info(),
            self.francium_lending_pool_info_account.to_account_info(),
            self.francium_lending_pool_token_account.to_account_info(),
            self.francium_farming_pool_stake_token_mint
                .to_account_info(),
            self.francium_market_info_account.to_account_info(),
            self.francium_lending_market_authority.to_account_info(),
            self.generic_accs.vault_account.to_account_info(),
            self.generic_accs.clock.to_account_info(),
            self.generic_accs.token_program.to_account_info(),
        ];
        let account_metas = accounts
            .iter()
            .map(|acc| {
                if acc.key == &self.generic_accs.vault_account.key() {
                    AccountMeta::new(*acc.key, true)
                } else if acc.is_writable {
                    AccountMeta::new(*acc.key, false)
                } else {
                    AccountMeta::new_readonly(*acc.key, false)
                }
            })
            .collect::<Vec<_>>();
        let ix = Instruction::new_with_borsh(
            francium_lending_program_id::ID,
            &InstructionAmountData {
                instruction: 4,
                amount,
            },
            account_metas,
        );
        invoke_signed(&ix, &accounts, signer)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct FranciumWithdraw<'info> {
    pub generic_accs: GenericWithdrawAccounts<'info>,
    #[account(constraint = francium_lending_program_id.key == &francium_lending_program_id::ID)]
    /// CHECK: Francium CPI
    pub francium_lending_program_id: AccountInfo<'info>,
    #[account(
        mut,
        associated_token::mint = vault_francium_collateral_token_account.mint,
        associated_token::authority = generic_accs.vault_account,
    )]
    pub vault_francium_collateral_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// CHECK: Francium CPI
    pub francium_lending_pool_info_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Francium CPI
    pub francium_lending_pool_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Francium CPI
    pub francium_farming_pool_stake_token_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Francium CPI
    pub francium_market_info_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Francium CPI
    pub francium_lending_market_authority: AccountInfo<'info>,
}

impl<'info> CheckHash<'info> for FranciumWithdraw<'info> {
    fn hash(&self) -> Hash {
        hashv(&[
            self.vault_francium_collateral_token_account.key().as_ref(),
            self.francium_lending_pool_info_account.key.as_ref(),
            self.francium_lending_pool_token_account.key.as_ref(),
            self.francium_farming_pool_stake_token_mint.key.as_ref(),
            self.francium_market_info_account.key.as_ref(),
            self.francium_lending_market_authority.key.as_ref(),
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

impl<'info> ProtocolWithdraw<'info> for FranciumWithdraw<'info> {
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
        let lending_pool = francium_lending_pool::LendingPool::unpack(
            &self.francium_lending_pool_info_account.data.borrow(),
        )?;

        let lp_amount = lending_pool
            .collateral_exchange_rate()?
            .liquidity_to_collateral(amount)?;

        Ok(lp_amount)
    }

    fn cpi_withdraw(&self, amount: u64) -> Result<()> {
        let seeds = generate_seeds!(self.generic_accs.vault_account);
        let signer = &[&seeds[..]];

        let accounts = [
            self.vault_francium_collateral_token_account
                .to_account_info(),
            self.generic_accs
                .vault_input_token_account
                .to_account_info(),
            self.francium_lending_pool_info_account.to_account_info(),
            self.francium_farming_pool_stake_token_mint
                .to_account_info(),
            self.francium_lending_pool_token_account.to_account_info(),
            self.francium_market_info_account.to_account_info(),
            self.francium_lending_market_authority.to_account_info(),
            self.generic_accs.vault_account.to_account_info(),
            self.generic_accs.clock.to_account_info(),
            self.generic_accs.token_program.to_account_info(),
        ];
        let account_metas = accounts
            .iter()
            .map(|acc| {
                if acc.key == &self.generic_accs.vault_account.key() {
                    AccountMeta::new(*acc.key, true)
                } else if acc.is_writable {
                    AccountMeta::new(*acc.key, false)
                } else {
                    AccountMeta::new_readonly(*acc.key, false)
                }
            })
            .collect::<Vec<_>>();
        let ix = Instruction::new_with_borsh(
            francium_lending_program_id::ID,
            &InstructionAmountData {
                instruction: 5,
                amount,
            },
            account_metas,
        );
        invoke_signed(&ix, &accounts, signer)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct FranciumTVL<'info> {
    pub generic_accs: GenericTVLAccounts<'info>,
    #[account(owner = francium_lending_program_id::ID)]
    /// CHECK: hash, owner and mint & collateral data fields are checked
    pub lending_pool: AccountInfo<'info>,
    #[account(
        associated_token::mint = vault_francium_collateral_token_account.mint,
        associated_token::authority = generic_accs.vault_account,
    )]
    pub vault_francium_collateral_token_account: Account<'info, TokenAccount>,
}

impl<'info> CheckHash<'info> for FranciumTVL<'info> {
    fn hash(&self) -> Hash {
        hashv(&[
            self.lending_pool.key.as_ref(),
            self.vault_francium_collateral_token_account.key().as_ref(),
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

impl<'info> ProtocolRewards<'info> for FranciumTVL<'info> {
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
        let lending = francium_lending_pool::LendingPool::unpack(&self.lending_pool.data.borrow())?;

        require!(
            lending.liquidity.mint_pubkey == self.generic_accs.vault_account.input_mint_pubkey,
            ErrorCode::InvalidMint
        );

        require!(
            lending.share.mint_pubkey == self.vault_francium_collateral_token_account.mint,
            ErrorCode::InvalidMint
        );

        let lp_amount = self.vault_francium_collateral_token_account.amount;
        let tvl = lending
            .collateral_exchange_rate()?
            .collateral_to_liquidity(lp_amount)?;

        Ok(tvl)
    }
}
