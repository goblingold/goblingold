use crate::duplicated_ixs::is_last_of_duplicated_ixs;
use crate::error::ErrorCode;
use crate::macros::generate_seeds;
use crate::protocols::francium_lending_pool;
use crate::protocols::Protocols;
use crate::vault::{hash_pub_keys, TokenBalances, VaultAccount};
use crate::{
    generic_accounts_anchor_modules::*, GenericDepositAccounts, GenericTVLAccounts,
    GenericWithdrawAccounts,
};
use crate::{ALLOWED_DEPLOYER, VAULT_ACCOUNT_SEED};
use anchor_lang::prelude::borsh::{BorshDeserialize, BorshSerialize};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::Instruction, program::invoke_signed, program_pack::Pack, sysvar,
};
use anchor_spl::token::TokenAccount;
use std::str::FromStr;

/// Program ids
pub mod francium_lending_program_id {
    use anchor_lang::declare_id;
    declare_id!("FC81tbGt6JWRXidaWYFXxGnTk4VgobhJHATvTRVMqgWj");
}

pub mod francium_lending_reward_program_id {
    use anchor_lang::declare_id;
    declare_id!("3Katmm9dhvLQijAvomteYMo6rfVbY5NaCRNq9ZBqBgr6");
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
pub struct FranciumInitialize<'info> {
    #[account(constraint = Pubkey::from_str(ALLOWED_DEPLOYER).unwrap()== *user_signer.key)]
    pub user_signer: Signer<'info>,
    #[account(
        mut,
        seeds = [VAULT_ACCOUNT_SEED, vault_account.input_mint_pubkey.as_ref()],
        bump = vault_account.bumps.vault
    )]
    pub vault_account: Box<Account<'info, VaultAccount>>,
    #[account(
        mut,
        associated_token::mint = vault_francium_collateral_token_account.mint,
        associated_token::authority = vault_account,
    )]
    pub vault_francium_collateral_token_account: Account<'info, TokenAccount>,
    #[account(constraint = francium_lending_reward_program_id.key == &francium_lending_reward_program_id::ID)]
    /// CHECK: Francium CPI
    pub francium_lending_reward_program_id: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Francium CPI
    pub francium_farming_pool_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Francium CPI
    pub vault_francium_farming_account: AccountInfo<'info>,
    #[account(
        mut,
        associated_token::mint = vault_francium_account_mint_rewards.mint,
        associated_token::authority = vault_account,
    )]
    pub vault_francium_account_mint_rewards: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = vault_francium_account_mint_b_rewards.mint,
        associated_token::authority = vault_account,
    )]
    pub vault_francium_account_mint_b_rewards: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> FranciumInitialize<'info> {
    /// Create and initialize protocol account
    pub fn create_and_initialize(&self) -> Result<()> {
        let seeds = generate_seeds!(self.vault_account);
        let signer = &[&seeds[..]];

        let accounts = [
            self.vault_account.to_account_info(),
            self.vault_francium_farming_account.to_account_info(),
            self.francium_farming_pool_account.to_account_info(),
            self.vault_francium_collateral_token_account
                .to_account_info(),
            self.vault_francium_account_mint_rewards.to_account_info(),
            self.vault_francium_account_mint_b_rewards.to_account_info(),
            self.system_program.to_account_info(),
            self.rent.to_account_info(),
        ];
        let accounts_metas = accounts
            .iter()
            .map(|acc| {
                if acc.key == &self.vault_account.key() {
                    AccountMeta::new(*acc.key, true)
                } else if acc.is_writable {
                    AccountMeta::new(*acc.key, false)
                } else {
                    AccountMeta::new_readonly(*acc.key, false)
                }
            })
            .collect::<Vec<_>>();
        let ix_init = Instruction::new_with_borsh(
            francium_lending_reward_program_id::ID,
            &InstructionData { instruction: 1 },
            accounts_metas,
        );
        invoke_signed(&ix_init, &accounts, signer)?;

        Ok(())
    }
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
    #[account(constraint = francium_lending_reward_program_id.key == &francium_lending_reward_program_id::ID)]
    /// CHECK: Francium CPI
    pub francium_lending_reward_program_id: AccountInfo<'info>,
    #[account(
        mut,
        associated_token::mint = vault_francium_collateral_token_account.mint,
        associated_token::authority = generic_accs.vault_account,
    )]
    pub vault_francium_collateral_token_account: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = vault_francium_account_mint_rewards.mint,
        associated_token::authority = generic_accs.vault_account,
    )]
    pub vault_francium_account_mint_rewards: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = vault_francium_account_mint_b_rewards.mint,
        associated_token::authority = generic_accs.vault_account,
    )]
    pub vault_francium_account_mint_b_rewards: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// CHECK: Francium CPI
    pub vault_francium_farming_account: AccountInfo<'info>,
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
    #[account(mut)]
    /// CHECK: Francium CPI
    pub francium_farming_pool_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Francium CPI
    pub francium_farming_pool_authority: AccountInfo<'info>,
    #[account(mut)]
    pub francium_farming_pool_stake_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// CHECK: Francium CPI
    pub francium_farming_pool_rewards_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Francium CPI
    pub francium_farming_pool_rewards_b_token_account: AccountInfo<'info>,
}

impl<'info> FranciumDeposit<'info> {
    /// Deposit into protocol. It should be called in two different instructions in the same tx.
    /// Otherwise it would exceed max compute budget for one instruction.
    pub fn deposit(&mut self) -> Result<()> {
        let is_last_deposit = is_last_of_duplicated_ixs(self.instructions.to_account_info())?;

        let amount: u64 = if is_last_deposit {
            0
        } else {
            self.generic_accs.amount_to_deposit(Protocols::Francium)?
        };
        let balances = self.deposit_and_get_balances(amount, !is_last_deposit)?;

        self.generic_accs.vault_account.protocols[Protocols::Francium as usize]
            .update_after_deposit(self.generic_accs.clock.slot, balances)?;

        Ok(())
    }

    /// Deposit into the protocol and get the true token balances in two ixs
    fn deposit_and_get_balances(
        &mut self,
        amount: u64,
        is_first_ix: bool,
    ) -> Result<TokenBalances> {
        if is_first_ix {
            self.cpi_deposit(amount)?;
            Ok(TokenBalances {
                base_amount: amount,
                lp_amount: 0,
            })
        } else {
            let lp_before = self.francium_farming_pool_stake_token_account.amount;

            self.cpi_deposit_stake()?;
            self.francium_farming_pool_stake_token_account.reload()?;

            let lp_after = self.francium_farming_pool_stake_token_account.amount;
            let lp_amount = lp_after
                .checked_sub(lp_before)
                .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

            require!(
                self.vault_francium_collateral_token_account.amount == lp_amount,
                ErrorCode::InvalidDepositAmount
            );

            Ok(TokenBalances {
                base_amount: 0,
                lp_amount,
            })
        }
    }

    /// CPI deposit call into lending (first ixs)
    fn cpi_deposit(&self, amount: u64) -> Result<()> {
        let seeds = generate_seeds!(self.generic_accs.vault_account);
        let signer = &[&seeds[..]];

        // Deposit Lending
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

    /// CPI deposit call into stake (second ixs)
    fn cpi_deposit_stake(&self) -> Result<()> {
        let seeds = generate_seeds!(self.generic_accs.vault_account);
        let signer = &[&seeds[..]];

        let accounts = [
            self.generic_accs.vault_account.to_account_info(),
            self.vault_francium_farming_account.to_account_info(),
            self.vault_francium_collateral_token_account
                .to_account_info(),
            self.vault_francium_account_mint_rewards.to_account_info(),
            self.vault_francium_account_mint_b_rewards.to_account_info(),
            self.francium_farming_pool_account.to_account_info(),
            self.francium_farming_pool_authority.to_account_info(),
            self.francium_farming_pool_stake_token_account
                .to_account_info(),
            self.francium_farming_pool_rewards_token_account
                .to_account_info(),
            self.francium_farming_pool_rewards_b_token_account
                .to_account_info(),
            self.generic_accs.token_program.to_account_info(),
            self.generic_accs.clock.to_account_info(),
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
            francium_lending_reward_program_id::ID,
            &InstructionAmountData {
                instruction: 3,
                amount: 0,
            },
            account_metas,
        );
        invoke_signed(&ix, &accounts, signer)?;
        Ok(())
    }

    pub fn check_hash(&self) -> Result<()> {
        let has_keys = hash_pub_keys(&[
            &self.vault_francium_collateral_token_account.key().as_ref(),
            &self.vault_francium_account_mint_rewards.key().as_ref(),
            &self.vault_francium_account_mint_b_rewards.key().as_ref(),
            self.vault_francium_farming_account.key.as_ref(),
            self.francium_lending_pool_info_account.key.as_ref(),
            self.francium_lending_pool_token_account.key.as_ref(),
            self.francium_farming_pool_stake_token_mint.key.as_ref(),
            self.francium_market_info_account.key.as_ref(),
            self.francium_lending_market_authority.key.as_ref(),
            self.francium_farming_pool_account.key.as_ref(),
            self.francium_farming_pool_authority.key.as_ref(),
            &self
                .francium_farming_pool_stake_token_account
                .key()
                .as_ref(),
            self.francium_farming_pool_rewards_token_account
                .key
                .as_ref(),
            self.francium_farming_pool_rewards_b_token_account
                .key
                .as_ref(),
        ])?;
        require!(
            has_keys
                == self.generic_accs.vault_account.protocols[Protocols::Francium as usize]
                    .hash_pubkey
                    .hash_deposit,
            ErrorCode::InvalidHash
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct FranciumWithdraw<'info> {
    pub generic_accs: GenericWithdrawAccounts<'info>,
    #[account(constraint = francium_lending_program_id.key == &francium_lending_program_id::ID)]
    /// CHECK: Francium CPI
    pub francium_lending_program_id: AccountInfo<'info>,
    #[account(constraint = francium_lending_reward_program_id.key == &francium_lending_reward_program_id::ID)]
    /// CHECK: Francium CPI
    pub francium_lending_reward_program_id: AccountInfo<'info>,
    #[account(
        mut,
        associated_token::mint = vault_francium_collateral_token_account.mint,
        associated_token::authority = generic_accs.vault_account,
    )]
    pub vault_francium_collateral_token_account: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = vault_francium_account_mint_rewards.mint,
        associated_token::authority = generic_accs.vault_account,
    )]
    pub vault_francium_account_mint_rewards: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// CHECK: Francium CPI
    pub vault_francium_farming_account: AccountInfo<'info>,
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
    #[account(mut)]
    /// CHECK: Francium CPI
    pub francium_farming_pool_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Francium CPI
    pub francium_farming_pool_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Francium CPI
    pub francium_farming_pool_stake_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Francium CPI
    pub francium_farming_pool_rewards_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Francium CPI
    pub francium_farming_pool_rewards_b_token_account: AccountInfo<'info>,
}

impl<'info> FranciumWithdraw<'info> {
    /// Withdraw from the protocol in two instructions so the computer budget is not exceeded
    pub fn withdraw(&mut self) -> Result<()> {
        // Francium has 2 instructions for withdraw
        let is_last_withdraw =
            is_last_of_duplicated_ixs(self.generic_accs.instructions.to_account_info())?;
        let target_ix: usize = if is_last_withdraw { 1 } else { 2 };

        let amount = self
            .generic_accs
            .amount_to_withdraw_in_n_txs(Protocols::Francium, target_ix)?;
        let balances = self.withdraw_and_get_balances(amount, !is_last_withdraw)?;

        if is_last_withdraw {
            self.generic_accs.vault_account.protocols[Protocols::Francium as usize]
                .update_after_withdraw(self.generic_accs.clock.slot, balances)?;
        }
        Ok(())
    }

    /// Convert reserve liquidity to collateral
    fn liquidity_to_collateral(&self, amount: u64) -> Result<u64> {
        let lending_pool = francium_lending_pool::LendingPool::unpack(
            &self.francium_lending_pool_info_account.data.borrow(),
        )?;

        let lp_amount = lending_pool
            .collateral_exchange_rate()?
            .liquidity_to_collateral(amount)?;

        Ok(lp_amount)
    }

    /// Withdraw from the protocol and get the true token balances in two ixs
    fn withdraw_and_get_balances(
        &mut self,
        amount: u64,
        is_first_ix: bool,
    ) -> Result<TokenBalances> {
        let lp_amount = self.liquidity_to_collateral(amount)?;

        if is_first_ix {
            self.cpi_withdraw_stake(lp_amount)?;
            Ok(TokenBalances {
                base_amount: 0,
                lp_amount: 0,
            })
        } else {
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
    }

    /// CPI withdraw call from stake (first ixs)
    fn cpi_withdraw_stake(&self, amount: u64) -> Result<()> {
        let seeds = generate_seeds!(self.generic_accs.vault_account);
        let signer = &[&seeds[..]];

        let accounts = [
            self.generic_accs.vault_account.to_account_info(),
            self.vault_francium_farming_account.to_account_info(),
            self.vault_francium_collateral_token_account
                .to_account_info(),
            self.vault_francium_account_mint_rewards.to_account_info(),
            self.vault_francium_account_mint_rewards.to_account_info(),
            self.francium_farming_pool_account.to_account_info(),
            self.francium_farming_pool_authority.to_account_info(),
            self.francium_farming_pool_stake_token_account
                .to_account_info(),
            self.francium_farming_pool_rewards_token_account
                .to_account_info(),
            self.francium_farming_pool_rewards_b_token_account
                .to_account_info(),
            self.generic_accs.token_program.to_account_info(),
            self.generic_accs.clock.to_account_info(),
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
            francium_lending_reward_program_id::ID,
            &InstructionAmountData {
                instruction: 4,
                amount,
            },
            account_metas,
        );
        invoke_signed(&ix, &accounts, signer)?;

        Ok(())
    }

    /// CPI withdraw call from lending (second ixs)
    fn cpi_withdraw(&self, amount: u64) -> Result<()> {
        let seeds = generate_seeds!(self.generic_accs.vault_account);
        let signer = &[&seeds[..]];

        // Withdraw Lending
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

    pub fn check_hash(&self) -> Result<()> {
        let has_keys = hash_pub_keys(&[
            &self.vault_francium_collateral_token_account.key().as_ref(),
            &self.vault_francium_account_mint_rewards.key().as_ref(),
            self.vault_francium_farming_account.key.as_ref(),
            self.francium_lending_pool_info_account.key.as_ref(),
            self.francium_lending_pool_token_account.key.as_ref(),
            self.francium_farming_pool_stake_token_mint.key.as_ref(),
            self.francium_market_info_account.key.as_ref(),
            self.francium_lending_market_authority.key.as_ref(),
            self.francium_farming_pool_account.key.as_ref(),
            self.francium_farming_pool_authority.key.as_ref(),
            self.francium_farming_pool_stake_token_account.key.as_ref(),
            self.francium_farming_pool_rewards_token_account
                .key
                .as_ref(),
            self.francium_farming_pool_rewards_b_token_account
                .key
                .as_ref(),
        ])?;

        require!(
            has_keys
                == self.generic_accs.vault_account.protocols[Protocols::Francium as usize]
                    .hash_pubkey
                    .hash_withdraw,
            ErrorCode::InvalidHash
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct FranciumTVL<'info> {
    pub generic_accs: GenericTVLAccounts<'info>,
    #[account(owner = francium_lending_program_id::ID)]
    /// CHECK: owner and mint data field are checked
    pub lending_pool: AccountInfo<'info>,
}

impl<'info> FranciumTVL<'info> {
    /// Update the protocol TVL
    pub fn update_rewards(&mut self) -> Result<()> {
        let tvl = self.max_withdrawable()?;

        let protocol = &mut self.generic_accs.vault_account.protocols[Protocols::Francium as usize];
        let rewards = tvl
            .checked_sub(protocol.tokens.base_amount)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

        protocol.rewards.update(rewards)?;

        Ok(())
    }

    /// Calculate the max native units to withdraw
    fn max_withdrawable(&self) -> Result<u64> {
        let protocol = self.generic_accs.vault_account.protocols[Protocols::Francium as usize];
        self.lp_to_liquidity(protocol.tokens.lp_amount)
    }

    /// Convert reserve collateral to liquidity
    fn lp_to_liquidity(&self, lp_amount: u64) -> Result<u64> {
        let lending_pool =
            francium_lending_pool::LendingPool::unpack(&self.lending_pool.data.borrow())?;

        require!(
            lending_pool.liquidity.mint_pubkey == self.generic_accs.vault_account.input_mint_pubkey,
            ErrorCode::InvalidMint
        );

        let tvl = lending_pool
            .collateral_exchange_rate()?
            .collateral_to_liquidity(lp_amount)?;

        Ok(tvl)
    }
    pub fn check_hash(&self) -> Result<()> {
        let has_keys = hash_pub_keys(&[self.lending_pool.key.as_ref()])?;
        require!(
            has_keys
                == self.generic_accs.vault_account.protocols[Protocols::Francium as usize]
                    .hash_pubkey
                    .hash_tvl,
            ErrorCode::InvalidHash
        );
        Ok(())
    }
}
