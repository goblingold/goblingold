use crate::check_hash::*;
use crate::error::ErrorCode;
use crate::instructions::{
    protocol_deposit_2_ixs::ProtocolDeposit2Ixs, protocol_deposit_isolated_pool::*,
    protocol_initialize::*, protocol_rewards::*, protocol_rewards_isolated_pool::*,
    protocol_withdraw_2_ixs::ProtocolWithdraw2Ixs, protocol_withdraw_isolated_pool::*,
};
use crate::macros::generate_seeds;
use crate::protocols::{
    state::{francium_farming_user, francium_lending_pool},
    Protocols,
};
use crate::vault::{ProtocolData, VaultAccount};
use crate::VAULT_ACCOUNT_SEED;
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

impl<'info> ProtocolInitialize<'info> for FranciumInitialize<'info> {
    fn cpi_initialize(&self) -> Result<()> {
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

impl<'info> CheckHash<'info> for FranciumDeposit<'info> {
    fn hash(&self) -> Hash {
        hashv(&[
            self.vault_francium_collateral_token_account.key().as_ref(),
            self.vault_francium_account_mint_rewards.key().as_ref(),
            self.vault_francium_account_mint_b_rewards.key().as_ref(),
            self.vault_francium_farming_account.key.as_ref(),
            self.francium_lending_pool_info_account.key.as_ref(),
            self.francium_lending_pool_token_account.key.as_ref(),
            self.francium_farming_pool_stake_token_mint.key.as_ref(),
            self.francium_market_info_account.key.as_ref(),
            self.francium_lending_market_authority.key.as_ref(),
            self.francium_farming_pool_account.key.as_ref(),
            self.francium_farming_pool_authority.key.as_ref(),
            self.francium_farming_pool_stake_token_account
                .key()
                .as_ref(),
            self.francium_farming_pool_rewards_token_account
                .key
                .as_ref(),
            self.francium_farming_pool_rewards_b_token_account
                .key
                .as_ref(),
        ])
    }

    fn target_hash(&self) -> [u8; CHECKHASH_BYTES] {
        self.generic_accs.vault_account.protocols[Protocols::Francium as usize]
            .hash_pubkey
            .hash_deposit
    }
}

impl<'info> ProtocolDeposit2Ixs<'info> for FranciumDeposit<'info> {
    fn protocol_data_as_mut(&mut self) -> &mut ProtocolData {
        &mut self.generic_accs.vault_account.protocols[Protocols::Francium as usize]
    }

    fn instructions_account(&self) -> AccountInfo<'info> {
        self.instructions.to_account_info()
    }

    fn get_amount(&self) -> Result<u64> {
        self.generic_accs.amount_to_deposit(Protocols::Francium)
    }

    fn cpi_deposit(&self, amount: u64, is_last_deposit_ix: bool) -> Result<()> {
        let seeds = generate_seeds!(self.generic_accs.vault_account);
        let signer = &[&seeds[..]];

        if !is_last_deposit_ix {
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
        } else {
            // Deposit stake
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
        }
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

impl<'info> CheckHash<'info> for FranciumWithdraw<'info> {
    fn hash(&self) -> Hash {
        hashv(&[
            self.vault_francium_collateral_token_account.key().as_ref(),
            self.vault_francium_account_mint_rewards.key().as_ref(),
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
        ])
    }

    fn target_hash(&self) -> [u8; CHECKHASH_BYTES] {
        self.generic_accs.vault_account.protocols[Protocols::Francium as usize]
            .hash_pubkey
            .hash_withdraw
    }
}

impl<'info> ProtocolWithdraw2Ixs<'info> for FranciumWithdraw<'info> {
    fn protocol_data_as_mut(&mut self) -> &mut ProtocolData {
        &mut self.generic_accs.vault_account.protocols[Protocols::Francium as usize]
    }

    fn instructions_account(&self) -> AccountInfo<'info> {
        self.generic_accs.instructions.to_account_info()
    }

    fn input_token_account_as_mut(&mut self) -> &mut Account<'info, TokenAccount> {
        &mut self.generic_accs.vault_input_token_account
    }

    fn get_amount(&self, target_withdraw_ix: usize) -> Result<u64> {
        self.generic_accs
            .amount_to_withdraw_in_n_txs(Protocols::Francium, target_withdraw_ix)
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

    fn cpi_withdraw(&self, amount: u64, is_last_withdraw_ix: bool) -> Result<()> {
        let seeds = generate_seeds!(self.generic_accs.vault_account);
        let signer = &[&seeds[..]];

        if !is_last_withdraw_ix {
            // Unstake
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
        } else {
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
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct FranciumTVL<'info> {
    pub generic_accs: GenericTVLAccounts<'info>,
    #[account(owner = francium_lending_program_id::ID)]
    /// CHECK: owner and mint data field are checked
    pub lending_pool: AccountInfo<'info>,
    #[account(owner = francium_lending_reward_program_id::ID)]
    /// CHECK: owner and owner data field are checked
    pub farming_user: AccountInfo<'info>,
}

impl<'info> CheckHash<'info> for FranciumTVL<'info> {
    fn hash(&self) -> Hash {
        hashv(&[
            self.lending_pool.key.as_ref(),
            self.farming_user.key.as_ref(),
        ])
    }

    fn target_hash(&self) -> [u8; CHECKHASH_BYTES] {
        self.generic_accs.vault_account.protocols[Protocols::Francium as usize]
            .hash_pubkey
            .hash_tvl
    }
}

impl<'info> ProtocolRewards<'info> for FranciumTVL<'info> {
    fn protocol_id(&self) -> usize {
        Protocols::Francium as usize
    }

    fn input_mint_pubkey(&self) -> Pubkey {
        self.generic_accs.vault_account.input_mint_pubkey
    }

    fn protocol_data_as_mut(&mut self) -> &mut ProtocolData {
        &mut self.generic_accs.vault_account.protocols[Protocols::Francium as usize]
    }

    fn max_withdrawable(&self) -> Result<u64> {
        let lending = francium_lending_pool::LendingPool::unpack(&self.lending_pool.data.borrow())?;
        let farming = francium_farming_user::FarmingUser::unpack(&self.farming_user.data.borrow())?;

        require!(
            lending.liquidity.mint_pubkey == self.generic_accs.vault_account.input_mint_pubkey,
            ErrorCode::InvalidMint
        );

        require!(
            farming.user_main == self.generic_accs.vault_account.key(),
            ErrorCode::InvalidObligationOwner
        );

        let lp_amount = farming.staked_amount;
        let tvl = lending
            .collateral_exchange_rate()?
            .collateral_to_liquidity(lp_amount)?;

        Ok(tvl)
    }
}
