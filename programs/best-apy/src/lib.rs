// Some functions here have a RESTRICTED ACCESS.
// Besides others, to make them unrestricted it should be checked that the vault token accounts
// are deterministic (associated token accounts)

use anchor_lang::prelude::borsh::{BorshDeserialize, BorshSerialize};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{pubkey::Pubkey, sysvar};
use anchor_spl::token::{Token, TokenAccount};
use error::ErrorCode;
use instructions::*;
use protocols::{francium::*, mango::*, port::*, solend::*, tulip::*, PROTOCOLS_LEN};
use vault::{VaultAccount, HASH_PUBKEYS_LEN};

mod error;
mod generic_accounts;
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
        protocol: usize,
        action: String,
        hash: [u8; HASH_PUBKEYS_LEN],
    ) -> Result<()> {
        instructions::set_hash::handler(ctx, protocol, action, hash)
    }

    // ACCESS RESTRICTED. ONLY ALLOWED_DEPLOYER
    pub fn set_protocol_weights(
        ctx: Context<SetProtocolWeights>,
        weights: [u32; PROTOCOLS_LEN],
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
        ctx.accounts.create_and_initialize()
    }

    /// Mango: Deposit from the vault account
    #[access_control(ctx.accounts.check_hash())]
    pub fn mango_deposit(ctx: Context<MangoDeposit>) -> Result<()> {
        ctx.accounts.deposit()
    }

    /// Mango: Withdraw to the vault account
    #[access_control(ctx.accounts.check_hash())]
    pub fn mango_withdraw(ctx: Context<MangoWithdraw>) -> Result<()> {
        ctx.accounts.withdraw()
    }

    /// Mango: Compute the TVL
    #[access_control(ctx.accounts.check_hash())]
    pub fn mango_tvl(ctx: Context<MangoTVL>) -> Result<()> {
        ctx.accounts.update_rewards()
    }

    /// Solend: Initialize protocol accounts
    // ACCESS RESTRICTED. ONLY ALLOWED_DEPLOYER
    pub fn solend_initialize(ctx: Context<SolendInitialize>) -> Result<()> {
        ctx.accounts.create_and_initialize()
    }

    /// Solend: Deposit from the vault account
    #[access_control(ctx.accounts.check_hash())]
    pub fn solend_deposit(ctx: Context<SolendDeposit>) -> Result<()> {
        ctx.accounts.deposit()
    }

    /// Solend: Withdraw to the vault account
    #[access_control(ctx.accounts.check_hash())]
    pub fn solend_withdraw(ctx: Context<SolendWithdraw>) -> Result<()> {
        ctx.accounts.withdraw()
    }

    /// Solend: Compute the TVL
    #[access_control(ctx.accounts.check_hash())]
    pub fn solend_tvl(ctx: Context<SolendTVL>) -> Result<()> {
        ctx.accounts.update_rewards()
    }

    /// Port: Initialize protocol accounts
    // ACCESS RESTRICTED. ONLY ALLOWED_DEPLOYER
    pub fn port_initialize(ctx: Context<PortInitialize>) -> Result<()> {
        ctx.accounts.create_and_initialize()
    }

    /// Port: Deposit from the vault account
    #[access_control(ctx.accounts.check_hash())]
    pub fn port_deposit(ctx: Context<PortDeposit>) -> Result<()> {
        ctx.accounts.deposit()
    }

    /// Port: Withdraw to the vault account
    #[access_control(ctx.accounts.check_hash())]
    pub fn port_withdraw(ctx: Context<PortWithdraw>) -> Result<()> {
        ctx.accounts.withdraw()
    }

    /// Port: Compute the TVL
    #[access_control(ctx.accounts.check_hash())]
    pub fn port_tvl(ctx: Context<PortTVL>) -> Result<()> {
        ctx.accounts.update_rewards()
    }

    /// Port: Claim rewards
    pub fn port_claim_rewards(ctx: Context<PortClaimRewards>) -> Result<()> {
        ctx.accounts.claim_rewards()
    }

    /// Tulip: Deposit from the vault account
    #[access_control(ctx.accounts.check_hash())]
    pub fn tulip_deposit(ctx: Context<TulipDeposit>) -> Result<()> {
        ctx.accounts.deposit()
    }

    /// Tulip: Withdraw to the vault account
    #[access_control(ctx.accounts.check_hash())]
    pub fn tulip_withdraw(ctx: Context<TulipWithdraw>) -> Result<()> {
        ctx.accounts.withdraw()
    }

    /// Tulip: Compute the TVL
    #[access_control(ctx.accounts.check_hash())]
    pub fn tulip_tvl(ctx: Context<TulipTVL>) -> Result<()> {
        ctx.accounts.update_rewards()
    }

    /// Francium: Initialize protocol accounts
    // ACCESS RESTRICTED. ONLY ALLOWED_DEPLOYER
    pub fn francium_initialize(ctx: Context<FranciumInitialize>) -> Result<()> {
        protocols::francium::initialize(ctx)
    }

    /// Francium: Deposit from the vault account
    #[access_control(ctx.accounts.check_hash())]
    pub fn francium_deposit(ctx: Context<FranciumDeposit>) -> Result<()> {
        protocols::francium::deposit(ctx)
    }

    /// Francium: Withdraw to the vault account
    #[access_control(ctx.accounts.check_hash())]
    pub fn francium_withdraw(ctx: Context<FranciumWithdraw>) -> Result<()> {
        protocols::francium::withdraw(ctx)
    }

    /// Francium: Compute the TVL
    #[access_control(ctx.accounts.check_hash())]
    pub fn francium_tvl(ctx: Context<FranciumTVL>) -> Result<()> {
        protocols::francium::update_rewards(ctx)
    }
}

/// Trait to check the validity of a hash of the accounts passed
pub trait CheckHash<'info> {
    fn check_hash(&self) -> Result<()>;
}

/// Check if the program is paused
fn program_not_paused() -> Result<()> {
    require!(!PAUSED, ErrorCode::OnPaused);
    Ok(())
}

#[derive(Accounts)]
pub struct GenericDepositAccounts<'info> {
    #[account(
        mut,
        seeds = [VAULT_ACCOUNT_SEED, vault_account.input_mint_pubkey.as_ref()],
        bump = vault_account.bumps.vault
    )]
    pub vault_account: Box<Account<'info, VaultAccount>>,
    #[account(
        mut,
        associated_token::mint = vault_account.input_mint_pubkey,
        associated_token::authority = vault_account,
    )]
    pub vault_input_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct GenericWithdrawAccounts<'info> {
    #[account(
        mut,
        seeds = [VAULT_ACCOUNT_SEED, vault_account.input_mint_pubkey.as_ref()],
        bump = vault_account.bumps.vault
    )]
    pub vault_account: Box<Account<'info, VaultAccount>>,
    #[account(
        mut,
        associated_token::mint = vault_account.input_mint_pubkey,
        associated_token::authority = vault_account,
    )]
    pub vault_input_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub clock: Sysvar<'info, Clock>,
    #[account(address = sysvar::instructions::ID)]
    /// CHECK: address is checked
    pub instructions: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct GenericTVLAccounts<'info> {
    #[account(
        mut,
        seeds = [VAULT_ACCOUNT_SEED, vault_account.input_mint_pubkey.as_ref()],
        bump = vault_account.bumps.vault
    )]
    pub vault_account: Box<Account<'info, VaultAccount>>,
}

/// Anchor generated modules required for using the GenericAccounts structs as fields of
/// #[derive(Acounts)] structs in other source files
pub mod generic_accounts_anchor_modules {
    pub(crate) use super::__client_accounts_generic_deposit_accounts;
    pub(crate) use super::__client_accounts_generic_tvl_accounts;
    pub(crate) use super::__client_accounts_generic_withdraw_accounts;
    pub(crate) use super::__cpi_client_accounts_generic_deposit_accounts;
    pub(crate) use super::__cpi_client_accounts_generic_tvl_accounts;
    pub(crate) use super::__cpi_client_accounts_generic_withdraw_accounts;
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
