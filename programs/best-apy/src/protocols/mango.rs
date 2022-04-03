use crate::check_hash::*;
use crate::error::ErrorCode;
use crate::instructions::{
    protocol_deposit::*, protocol_initialize::*, protocol_rewards::*, protocol_withdraw::*,
};
use crate::macros::generate_seeds;
use crate::protocols::Protocols;
use crate::vault::{ProtocolData, VaultAccount};
use crate::VAULT_ACCOUNT_SEED;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    hash::{hashv, Hash},
    program::invoke_signed,
    pubkey::Pubkey,
};
use anchor_spl::token::TokenAccount;

/// Program ids
pub mod mango_program_id {
    use anchor_lang::declare_id;
    #[cfg(feature = "devnet")]
    declare_id!("4skJ85cdxQAFVKbcGgfun8iZPL7BadVYXG3kGEGkufqA");
    #[cfg(not(feature = "devnet"))]
    declare_id!("mv3ekLzLbnVPNxjSKvqBpU3ZeZXPQdEC3bp5MDEBG68");
}

#[derive(Accounts)]
pub struct MangoInitialize<'info> {
    pub user_signer: Signer<'info>,
    #[account(
        seeds = [VAULT_ACCOUNT_SEED, vault_account.input_mint_pubkey.as_ref()],
        bump = vault_account.bumps.vault
    )]
    pub vault_account: Box<Account<'info, VaultAccount>>,
    #[account(mut)]
    /// CHECK: Mango CPI
    pub vault_mango_account: AccountInfo<'info>,
    #[account(constraint = mango_program_id.key == &mango_program_id::ID)]
    /// CHECK: Mango CPI
    pub mango_program_id: AccountInfo<'info>,
    /// CHECK: Mango CPI
    #[account(mut)]
    pub mango_group_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> ProtocolInitialize<'info> for MangoInitialize<'info> {
    fn cpi_initialize(&self) -> Result<()> {
        let seeds = generate_seeds!(self.vault_account);
        let signer = &[&seeds[..]];

        let account_num = 1;
        let ix = mango::instruction::create_mango_account(
            &mango_program_id::ID,
            self.mango_group_account.key,
            self.vault_mango_account.key,
            &self.vault_account.key(),
            self.system_program.key,
            self.user_signer.key,
            account_num,
        )?;
        let accounts = [
            self.mango_group_account.to_account_info(),
            self.vault_mango_account.to_account_info(),
            self.vault_account.to_account_info(),
            self.system_program.to_account_info(),
            self.user_signer.to_account_info(),
        ];
        invoke_signed(&ix, &accounts, signer)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MangoDeposit<'info> {
    pub generic_accs: GenericDepositAccounts<'info>,
    #[account(constraint = mango_program_id.key == &mango_program_id::ID)]
    /// CHECK: Mango CPI
    pub mango_program_id: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Mango CPI
    pub vault_mango_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Mango CPI
    pub mango_group_account: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub mango_cache_account: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub mango_root_bank_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Mango CPI
    pub mango_node_bank_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Mango CPI
    pub mango_vault_account: AccountInfo<'info>,
}

impl<'info> CheckHash<'info> for MangoDeposit<'info> {
    fn hash(&self) -> Hash {
        hashv(&[
            self.vault_mango_account.key.as_ref(),
            self.mango_group_account.key.as_ref(),
            self.mango_cache_account.key.as_ref(),
            self.mango_root_bank_account.key.as_ref(),
            self.mango_node_bank_account.key.as_ref(),
            self.mango_vault_account.key.as_ref(),
        ])
    }

    fn target_hash(&self) -> [u8; CHECKHASH_BYTES] {
        self.generic_accs.vault_account.protocols[Protocols::Mango as usize]
            .hash_pubkey
            .hash_deposit
    }
}

impl<'info> ProtocolDeposit<'info> for MangoDeposit<'info> {
    fn protocol_data_as_mut(&mut self) -> &mut ProtocolData {
        &mut self.generic_accs.vault_account.protocols[Protocols::Mango as usize]
    }

    fn get_amount(&self) -> Result<u64> {
        self.generic_accs.amount_to_deposit(Protocols::Mango)
    }

    fn cpi_deposit(&self, amount: u64) -> Result<()> {
        let seeds = generate_seeds!(self.generic_accs.vault_account);
        let signer = &[&seeds[..]];

        let ix = mango::instruction::deposit(
            &mango_program_id::ID,
            self.mango_group_account.key,
            self.vault_mango_account.key,
            &self.generic_accs.vault_account.key(),
            self.mango_cache_account.key,
            self.mango_root_bank_account.key,
            self.mango_node_bank_account.key,
            self.mango_vault_account.key,
            &self.generic_accs.vault_input_token_account.key(),
            amount,
        )?;
        let accounts = [
            self.mango_group_account.to_account_info(),
            self.vault_mango_account.to_account_info(),
            self.generic_accs.vault_account.to_account_info(),
            self.mango_cache_account.to_account_info(),
            self.mango_root_bank_account.to_account_info(),
            self.mango_node_bank_account.to_account_info(),
            self.mango_vault_account.to_account_info(),
            self.generic_accs.token_program.to_account_info(),
            self.generic_accs
                .vault_input_token_account
                .to_account_info(),
        ];
        invoke_signed(&ix, &accounts, signer)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MangoWithdraw<'info> {
    pub generic_accs: GenericWithdrawAccounts<'info>,
    #[account(constraint = mango_program_id.key == &mango_program_id::ID)]
    /// CHECK: Mango CPI
    pub mango_program_id: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Mango CPI
    pub vault_mango_account: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub mango_cache_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Mango CPI
    pub mango_group_account: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub mango_group_signer_account: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub mango_root_bank_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Mango CPI
    pub mango_node_bank_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Mango CPI
    pub mango_vault_account: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub system_program: AccountInfo<'info>,
}

impl<'info> CheckHash<'info> for MangoWithdraw<'info> {
    fn hash(&self) -> Hash {
        hashv(&[
            self.vault_mango_account.key.as_ref(),
            self.mango_cache_account.key.as_ref(),
            self.mango_group_account.key.as_ref(),
            self.mango_group_signer_account.key.as_ref(),
            self.mango_root_bank_account.key.as_ref(),
            self.mango_node_bank_account.key.as_ref(),
            self.mango_vault_account.key.as_ref(),
        ])
    }

    fn target_hash(&self) -> [u8; CHECKHASH_BYTES] {
        self.generic_accs.vault_account.protocols[Protocols::Mango as usize]
            .hash_pubkey
            .hash_withdraw
    }
}

impl<'info> ProtocolWithdraw<'info> for MangoWithdraw<'info> {
    fn protocol_data_as_mut(&mut self) -> &mut ProtocolData {
        &mut self.generic_accs.vault_account.protocols[Protocols::Mango as usize]
    }

    fn input_token_account_as_mut(&mut self) -> &mut Account<'info, TokenAccount> {
        &mut self.generic_accs.vault_input_token_account
    }

    fn get_amount(&self) -> Result<u64> {
        self.generic_accs.amount_to_withdraw(Protocols::Mango)
    }

    fn cpi_withdraw(&self, amount: u64) -> Result<()> {
        let seeds = generate_seeds!(self.generic_accs.vault_account);
        let signer = &[&seeds[..]];

        let allow_borrow = false;
        let open_orders = vec![Pubkey::default(); mango::state::MAX_PAIRS];
        let ix = mango::instruction::withdraw(
            &mango_program_id::ID,
            self.mango_group_account.key,
            self.vault_mango_account.key,
            &self.generic_accs.vault_account.key(),
            self.mango_cache_account.key,
            self.mango_root_bank_account.key,
            self.mango_node_bank_account.key,
            self.mango_vault_account.key,
            &self.generic_accs.vault_input_token_account.key(),
            self.mango_group_signer_account.key,
            open_orders.as_slice(),
            amount,
            allow_borrow,
        )?;
        let accounts = vec![
            self.mango_group_account.to_account_info(),
            self.vault_mango_account.to_account_info(),
            self.generic_accs.vault_account.to_account_info(),
            self.mango_cache_account.to_account_info(),
            self.mango_root_bank_account.to_account_info(),
            self.mango_node_bank_account.to_account_info(),
            self.mango_vault_account.to_account_info(),
            self.generic_accs
                .vault_input_token_account
                .to_account_info(),
            self.mango_group_signer_account.to_account_info(),
            self.generic_accs.token_program.to_account_info(),
        ];
        invoke_signed(&ix, &accounts, signer)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MangoTVL<'info> {
    pub generic_accs: GenericTVLAccounts<'info>,
    /// CHECK: Mango CPI
    pub vault_mango_account: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub mango_group_account: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub mango_cache_account: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub mango_root_bank_account: AccountInfo<'info>,
    #[account(constraint = default_pubkey.key == &Pubkey::default())]
    /// CHECK: address is checked
    pub default_pubkey: AccountInfo<'info>,
}

impl<'info> CheckHash<'info> for MangoTVL<'info> {
    fn hash(&self) -> Hash {
        hashv(&[
            self.vault_mango_account.key.as_ref(),
            self.mango_group_account.key.as_ref(),
            self.mango_cache_account.key.as_ref(),
            self.mango_root_bank_account.key.as_ref(),
        ])
    }

    fn target_hash(&self) -> [u8; CHECKHASH_BYTES] {
        self.generic_accs.vault_account.protocols[Protocols::Mango as usize]
            .hash_pubkey
            .hash_tvl
    }
}

impl<'info> ProtocolRewards<'info> for MangoTVL<'info> {
    fn protocol_data_as_mut(&mut self) -> &mut ProtocolData {
        &mut self.generic_accs.vault_account.protocols[Protocols::Mango as usize]
    }

    fn max_withdrawable(&self) -> Result<u64> {
        let mango_account = mango::state::MangoAccount::load_checked(
            &self.vault_mango_account,
            &mango_program_id::ID,
            self.mango_group_account.key,
        )
        .unwrap();

        require!(
            mango_account.owner == self.generic_accs.vault_account.key(),
            ErrorCode::InvalidOwner,
        );

        let mango_group = mango::state::MangoGroup::load_checked(
            &self.mango_group_account,
            &mango_program_id::ID,
        )
        .unwrap();
        let mango_cache = mango::state::MangoCache::load_checked(
            &self.mango_cache_account,
            &mango_program_id::ID,
            &mango_group,
        )
        .unwrap();

        let open_orders_ais_vec =
            vec![self.default_pubkey.to_account_info(); mango::state::MAX_PAIRS];
        let open_orders_ais = arrayref::array_ref![open_orders_ais_vec, 0, mango::state::MAX_PAIRS];
        let active_assets =
            mango::state::UserActiveAssets::new(&mango_group, &mango_account, vec![]);
        let mut health_cache = mango::state::HealthCache::new(active_assets);
        health_cache
            .init_vals(&mango_group, &mango_cache, &mango_account, open_orders_ais)
            .unwrap();
        let health = health_cache.get_health(&mango_group, mango::state::HealthType::Init);
        let token_index = mango_group
            .find_root_bank_index(self.mango_root_bank_account.key)
            .unwrap();

        let tvl = mango_account
            .max_withdrawable(&mango_group, &mango_cache, token_index, health)
            .unwrap();

        Ok(tvl)
    }
}
