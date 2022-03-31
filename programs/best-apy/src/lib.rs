// Some functions here have a RESTRICTED ACCESS.
// Besides others, to make them unrestricted it should be checked that the vault token accounts
// are deterministic (associated token accounts)

use anchor_lang::prelude::borsh::{BorshDeserialize, BorshSerialize};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

use check_hash::{CheckHash, CHECKHASH_BYTES};
use error::ErrorCode;
use instructions::*;
use protocols::{francium::*, mango::*, port::*, solend::*, tulip::*, PROTOCOLS_LEN};
use vault::VaultAccount;

mod check_hash;
mod error;
mod instructions;
mod macros;
mod protocols;
mod vault;

declare_id!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");

pub const VAULT_ACCOUNT_SEED: &[u8; 5] = b"vault";
pub const VAULT_LP_TOKEN_MINT_SEED: &[u8; 4] = b"mint";

pub const ALLOWED_DEPLOYER: &str = "8XhNoDjjNoLP5Rys1pBJKGdE8acEC1HJsWGkfkMt6JP1";
pub const ALLOWED_RUNNER: &str = "DrrB1p8sxhwBZ3cXE8u5t2GxqEcTNuwAm7RcrQ8Yqjod";
const PAUSED: bool = false;

#[program]
pub mod best_apy {
    use super::*;

    /// Initialize the vault account and its fields
    // ACCESS RESTRICTED. ONLY ALLOWED_DEPLOYER
    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        instructions::initialize_vault::handler(ctx)
    }

    // ACCESS RESTRICTED. ONLY ALLOWED_DEPLOYER
    /// Set hash of a protocol for a specific action
    pub fn set_hash(
        ctx: Context<SetHash>,
        protocol_id: u8,
        action: String,
        hash: [u8; CHECKHASH_BYTES],
    ) -> Result<()> {
        instructions::set_hash::handler(ctx, protocol_id, action, hash)
    }

    // ACCESS RESTRICTED. ONLY ALLOWED_DEPLOYER
    pub fn set_protocol_weights(
        ctx: Context<SetProtocolWeights>,
        weights: [u16; PROTOCOLS_LEN],
    ) -> Result<()> {
        instructions::set_protocol_weights::handler(ctx, weights)
    }

    //pub fn close_account(_ctx: Context<CloseAccount>) -> Result<()> {
    //    Ok(())
    //}

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
    // ACCESS RESTRICTED. ONLY ALLOWED_DEPLOYER
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
    // ACCESS RESTRICTED. ONLY ALLOWED_DEPLOYER
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
    // ACCESS RESTRICTED. ONLY ALLOWED_DEPLOYER
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
    // ACCESS RESTRICTED. ONLY ALLOWED_DEPLOYER
    pub fn francium_initialize(ctx: Context<FranciumInitialize>) -> Result<()> {
        instructions::protocol_initialize::handler(ctx)
    }

    /// Francium: Deposit from the vault account
    #[access_control(ctx.accounts.check_hash())]
    pub fn francium_deposit(ctx: Context<FranciumDeposit>) -> Result<()> {
        instructions::protocol_deposit::handler(ctx)
    }

    /// Francium: Withdraw to the vault account
    #[access_control(ctx.accounts.check_hash())]
    pub fn francium_withdraw(ctx: Context<FranciumWithdraw>) -> Result<()> {
        instructions::protocol_withdraw::handler(ctx)
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

//#[derive(Accounts)]
//pub struct CloseAccount<'info> {
//    #[account(constraint = Pubkey::from_str(ALLOWED_DEPLOYER).unwrap()== *user_signer.key)]
//    pub user_signer: Signer<'info>,
//    #[account(mut, close = vault_account)]
//    pub vault_account: Account<'info, VaultAccount>,
//    #[account(mut)]
//    pub vault_account: AccountInfo<'info>,
//}
