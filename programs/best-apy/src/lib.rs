use anchor_lang::prelude::borsh::{BorshDeserialize, BorshSerialize};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
use check_hash::{CheckHash, CHECKHASH_BYTES};
use error::ErrorCode;
use instructions::*;
use protocols::{francium::*, mango::*, port::*, solend::*, tulip::*, PROTOCOLS_LEN};
use vault::{RefreshParams, VaultAccount};

mod check_hash;
mod duplicated_ixs;
mod error;
mod instructions;
mod macros;
mod protocols;
mod vault;

declare_id!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");

const PAUSED: bool = false;

const VAULT_ACCOUNT_SEED: &[u8; 5] = b"vault";
const VAULT_LP_TOKEN_MINT_SEED: &[u8; 4] = b"mint";

// DrrB1p8sxhwBZ3cXE8u5t2GxqEcTNuwAm7RcrQ8Yqjod
const ADMIN_PUBKEY: Pubkey = Pubkey::new_from_array([
    191, 17, 77, 109, 253, 243, 16, 188, 64, 67, 249, 18, 51, 62, 173, 81, 128, 208, 121, 29, 74,
    57, 94, 247, 114, 4, 114, 88, 209, 115, 147, 136,
]);

// 8XhNoDjjNoLP5Rys1pBJKGdE8acEC1HJsWGkfkMt6JP1
const TREASURY_PUBKEY: Pubkey = Pubkey::new_from_array([
    111, 222, 226, 197, 174, 64, 51, 181, 235, 205, 56, 138, 76, 105, 173, 158, 191, 43, 143, 141,
    91, 145, 78, 45, 130, 86, 102, 175, 146, 188, 82, 152,
]);

#[program]
pub mod best_apy {
    use super::*;

    /// Initialize the vault account and its fields
    #[access_control(is_admin(ctx.accounts.user_signer.key))]
    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        instructions::initialize_vault::handler(ctx)
    }

    /// Set protocol hashes
    #[access_control(is_admin(ctx.accounts.user_signer.key))]
    pub fn set_hashes(
        ctx: Context<SetHashes>,
        protocol_id: u8,
        hashes: [[u8; CHECKHASH_BYTES]; 3],
    ) -> Result<()> {
        instructions::set_hashes::handler(ctx, protocol_id, hashes)
    }

    /// Set the strategy refresh paraemeters
    #[access_control(is_admin(ctx.accounts.user_signer.key))]
    pub fn set_refresh_params(ctx: Context<SetRefreshParams>, params: RefreshParams) -> Result<()> {
        instructions::set_refresh_params::handler(ctx, params)
    }

    /// Set the protocol weights
    #[access_control(is_admin(ctx.accounts.user_signer.key))]
    pub fn set_protocol_weights(
        ctx: Context<SetProtocolWeights>,
        weights: [u32; PROTOCOLS_LEN],
    ) -> Result<()> {
        instructions::set_protocol_weights::handler(ctx, weights)
    }

    /// Deposit user input tokens into the vault account
    #[access_control(program_not_paused())]
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit::handler(ctx, amount)
    }

    /// Withdraw the required input tokens from the vault and send them back to the user
    #[access_control(program_not_paused())]
    pub fn withdraw(ctx: Context<Withdraw>, lp_amount: u64) -> Result<()> {
        instructions::withdraw::handler(ctx, lp_amount)
    }

    /// Refresh the protocol weights
    pub fn refresh_weights(ctx: Context<RefreshWeights>) -> Result<()> {
        instructions::refresh_weights::handler(ctx)
    }

    /// Mango: Initialize protocol accounts
    #[access_control(is_admin(ctx.accounts.user_signer.key))]
    pub fn mango_initialize(ctx: Context<MangoInitialize>) -> Result<()> {
        instructions::protocol_initialize::handler(ctx)
    }

    /// Mango: Deposit from the vault account
    #[access_control(ctx.accounts.check_hash())]
    pub fn mango_deposit(ctx: Context<MangoDeposit>) -> Result<()> {
        instructions::protocol_deposit::handler(ctx)
    }

    /// Mango: Withdraw to the vault account
    #[access_control(ctx.accounts.check_hash())]
    pub fn mango_withdraw(ctx: Context<MangoWithdraw>) -> Result<()> {
        instructions::protocol_withdraw::handler(ctx)
    }

    /// Mango: Compute the TVL
    #[access_control(ctx.accounts.check_hash())]
    pub fn mango_tvl(ctx: Context<MangoTVL>) -> Result<()> {
        instructions::protocol_rewards::handler(ctx)
    }

    /// Solend: Initialize protocol accounts
    #[access_control(is_admin(ctx.accounts.user_signer.key))]
    pub fn solend_initialize(ctx: Context<SolendInitialize>) -> Result<()> {
        instructions::protocol_initialize::handler(ctx)
    }

    /// Solend: Deposit from the vault account
    #[access_control(ctx.accounts.check_hash())]
    pub fn solend_deposit(ctx: Context<SolendDeposit>) -> Result<()> {
        instructions::protocol_deposit::handler(ctx)
    }

    /// Solend: Withdraw to the vault account
    #[access_control(ctx.accounts.check_hash())]
    pub fn solend_withdraw(ctx: Context<SolendWithdraw>) -> Result<()> {
        instructions::protocol_withdraw::handler(ctx)
    }

    /// Solend: Compute the TVL
    #[access_control(ctx.accounts.check_hash())]
    pub fn solend_tvl(ctx: Context<SolendTVL>) -> Result<()> {
        instructions::protocol_rewards::handler(ctx)
    }

    /// Port: Initialize protocol accounts
    #[access_control(is_admin(ctx.accounts.user_signer.key))]
    pub fn port_initialize(ctx: Context<PortInitialize>) -> Result<()> {
        instructions::protocol_initialize::handler(ctx)
    }

    /// Port: Deposit from the vault account
    #[access_control(ctx.accounts.check_hash())]
    pub fn port_deposit(ctx: Context<PortDeposit>) -> Result<()> {
        instructions::protocol_deposit::handler(ctx)
    }

    /// Port: Withdraw to the vault account
    #[access_control(ctx.accounts.check_hash())]
    pub fn port_withdraw(ctx: Context<PortWithdraw>) -> Result<()> {
        instructions::protocol_withdraw::handler(ctx)
    }

    /// Port: Compute the TVL
    #[access_control(ctx.accounts.check_hash())]
    pub fn port_tvl(ctx: Context<PortTVL>) -> Result<()> {
        instructions::protocol_rewards::handler(ctx)
    }

    /// Port: Claim rewards
    pub fn port_claim_rewards(ctx: Context<PortClaimRewards>) -> Result<()> {
        protocols::port::claim_rewards(ctx)
    }

    /// Tulip: Deposit from the vault account
    #[access_control(ctx.accounts.check_hash())]
    pub fn tulip_deposit(ctx: Context<TulipDeposit>) -> Result<()> {
        instructions::protocol_deposit::handler(ctx)
    }

    /// Tulip: Withdraw to the vault account
    #[access_control(ctx.accounts.check_hash())]
    pub fn tulip_withdraw(ctx: Context<TulipWithdraw>) -> Result<()> {
        instructions::protocol_withdraw::handler(ctx)
    }

    /// Tulip: Compute the TVL
    #[access_control(ctx.accounts.check_hash())]
    pub fn tulip_tvl(ctx: Context<TulipTVL>) -> Result<()> {
        instructions::protocol_rewards::handler(ctx)
    }

    /// Francium: Initialize protocol accounts
    #[access_control(is_admin(ctx.accounts.user_signer.key))]
    pub fn francium_initialize(ctx: Context<FranciumInitialize>) -> Result<()> {
        instructions::protocol_initialize::handler(ctx)
    }

    /// Francium: Deposit from the vault account
    #[access_control(ctx.accounts.check_hash())]
    pub fn francium_deposit(ctx: Context<FranciumDeposit>) -> Result<()> {
        instructions::protocol_deposit_2_ixs::handler(ctx)
    }

    /// Francium: Withdraw to the vault account
    #[access_control(ctx.accounts.check_hash())]
    pub fn francium_withdraw(ctx: Context<FranciumWithdraw>) -> Result<()> {
        instructions::protocol_withdraw_2_ixs::handler(ctx)
    }

    /// Francium: Compute the TVL
    #[access_control(ctx.accounts.check_hash())]
    pub fn francium_tvl(ctx: Context<FranciumTVL>) -> Result<()> {
        instructions::protocol_rewards::handler(ctx)
    }
}

/// Check if the program is paused
fn program_not_paused() -> Result<()> {
    require!(!PAUSED, ErrorCode::OnPaused);
    Ok(())
}

/// Check if target key is authorized
fn is_admin(key: &Pubkey) -> Result<()> {
    #[cfg(not(feature = "test"))]
    require!(key == &ADMIN_PUBKEY, UnauthorizedUser);
    Ok(())
}
