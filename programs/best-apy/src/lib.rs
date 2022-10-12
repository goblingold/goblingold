use anchor_lang::prelude::borsh::{BorshDeserialize, BorshSerialize};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
use check_hash::{CheckHash, CHECKHASH_BYTES};
use error::ErrorCode;
use instructions::*;
use protocols::{francium::*, mango::*, port::*, solend::*, tulip::*, Protocols};
use vault::{RefreshParams, VaultAccount};

mod check_hash;
mod error;
mod instructions;
mod macros;
mod protocols;
mod vault;

declare_id!("GGo1dnYpjKfe9omzUaFtaCyizvwpAMf3NhxSCMD61F3A");

const PAUSED_DEPOSIT: bool = true;
const PAUSED_WITHDRAW: bool = true;

const VAULT_ACCOUNT_SEED: &[u8; 5] = b"vault";
const VAULT_LP_TOKEN_MINT_SEED: &[u8; 4] = b"mint";

const VAULT_TICKET_MINT_SEED: &[u8; 11] = b"ticket_mint";

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
    pub fn initialize_vault(ctx: Context<InitializeVault>, account_number: u8) -> Result<()> {
        instructions::initialize_vault::handler(ctx, account_number)
    }

    /// Initialize the ticket mint
    #[access_control(is_admin(ctx.accounts.user_signer.key))]
    pub fn initialize_ticket_mint(ctx: Context<InitializeTicketMint>) -> Result<()> {
        instructions::initialize_ticket_mint::handler(ctx)
    }

    /// Add a new protocol to the vault_account
    #[access_control(is_admin(ctx.accounts.user_signer.key))]
    pub fn add_protocol(ctx: Context<AddProtocol>, protocol_id: u8) -> Result<()> {
        instructions::add_protocol::handler(ctx, protocol_id)
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
    pub fn set_protocol_weights(ctx: Context<SetProtocolWeights>, weights: Vec<u32>) -> Result<()> {
        instructions::set_protocol_weights::handler(ctx, weights)
    }

    /// Deposit user input tokens into the vault account
    #[access_control(deposit_not_paused())]
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit::handler(ctx, amount)
    }

    /// Deposit user input native SOL into the vault account
    #[access_control(deposit_not_paused())]
    pub fn deposit_from_native<'info>(
        ctx: Context<'_, '_, '_, 'info, DepositFromNative<'info>>,
        amount: u64,
    ) -> Result<()> {
        instructions::deposit_from_native::handler(ctx, amount)
    }

    /// Withdraw the required input tokens from the vault and send them back to the user
    #[access_control(withdraw_not_paused())]
    pub fn withdraw(ctx: Context<Withdraw>, lp_amount: u64) -> Result<()> {
        instructions::withdraw::handler(ctx, lp_amount)
    }

    /// Creates a vault_user_ticket_account
    pub fn create_vault_user_ticket_account(
        ctx: Context<CreateVaultUserTicketAccount>,
    ) -> Result<()> {
        instructions::create_vault_user_ticket_account::handler(ctx)
    }

    /// Open a withdrawal ticket (for delayed withdrawals)
    #[access_control(withdraw_not_paused())]
    pub fn open_withdraw_ticket(
        ctx: Context<OpenWithdrawTicket>,
        bump_user: u8,
        lp_amount: u64,
    ) -> Result<()> {
        instructions::open_withdraw_ticket::handler(ctx, bump_user, lp_amount)
    }

    /// Close a withdrawal ticket
    #[access_control(withdraw_not_paused())]
    pub fn close_withdraw_ticket(
        ctx: Context<CloseWithdrawTicket>,
        bump_user: u8,
        lp_amount: u64,
    ) -> Result<()> {
        instructions::close_withdraw_ticket::handler(ctx, bump_user, lp_amount)
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
    #[access_control(ctx.accounts.check_hash(Protocols::Mango))]
    pub fn mango_deposit(ctx: Context<MangoDeposit>) -> Result<()> {
        instructions::protocol_deposit::handler(ctx, Protocols::Mango)
    }

    /// Mango: Withdraw to the vault account
    #[access_control(ctx.accounts.check_hash(Protocols::Mango))]
    pub fn mango_withdraw(ctx: Context<MangoWithdraw>) -> Result<()> {
        instructions::protocol_withdraw::handler(ctx, Protocols::Mango)
    }

    /// Mango: Compute the TVL
    #[access_control(ctx.accounts.check_hash(Protocols::Mango))]
    pub fn mango_tvl(ctx: Context<MangoTVL>) -> Result<()> {
        instructions::protocol_rewards::handler(ctx, Protocols::Mango)
    }

    /// Solend: Deposit from the vault account
    #[access_control(ctx.accounts.check_hash(Protocols::Solend))]
    pub fn solend_deposit(ctx: Context<SolendDeposit>) -> Result<()> {
        instructions::protocol_deposit::handler(ctx, Protocols::Solend)
    }

    /// Solend: Withdraw to the vault account
    #[access_control(ctx.accounts.check_hash(Protocols::Solend))]
    pub fn solend_withdraw(ctx: Context<SolendWithdraw>) -> Result<()> {
        instructions::protocol_withdraw::handler(ctx, Protocols::Solend)
    }

    /// Solend: Compute the TVL
    #[access_control(ctx.accounts.check_hash(Protocols::Solend))]
    pub fn solend_tvl(ctx: Context<SolendTVL>) -> Result<()> {
        instructions::protocol_rewards::handler(ctx, Protocols::Solend)
    }

    /// SolendIsolatedPool: Deposit from the vault account
    #[access_control(ctx.accounts.check_hash(Protocols::SolendStablePool))]
    pub fn solend_isolated_pool_deposit(ctx: Context<SolendDeposit>) -> Result<()> {
        instructions::protocol_deposit::handler(ctx, Protocols::SolendStablePool)
    }

    /// SolendIsolatedPool: Withdraw to the vault account
    #[access_control(ctx.accounts.check_hash(Protocols::SolendStablePool))]
    pub fn solend_isolated_pool_withdraw(ctx: Context<SolendWithdraw>) -> Result<()> {
        instructions::protocol_withdraw::handler(ctx, Protocols::SolendStablePool)
    }

    /// SolendIsolatedPool: Compute the TVL
    #[access_control(ctx.accounts.check_hash(Protocols::SolendStablePool))]
    pub fn solend_isolated_pool_tvl(ctx: Context<SolendTVL>) -> Result<()> {
        instructions::protocol_rewards::handler(ctx, Protocols::SolendStablePool)
    }

    /// Port: Deposit from the vault account
    #[access_control(ctx.accounts.check_hash(Protocols::Port))]
    pub fn port_deposit(ctx: Context<PortDeposit>) -> Result<()> {
        instructions::protocol_deposit::handler(ctx, Protocols::Port)
    }

    /// Port: Withdraw to the vault account
    #[access_control(ctx.accounts.check_hash(Protocols::Port))]
    pub fn port_withdraw(ctx: Context<PortWithdraw>) -> Result<()> {
        instructions::protocol_withdraw::handler(ctx, Protocols::Port)
    }

    /// Port: Compute the TVL
    #[access_control(ctx.accounts.check_hash(Protocols::Port))]
    pub fn port_tvl(ctx: Context<PortTVL>) -> Result<()> {
        instructions::protocol_rewards::handler(ctx, Protocols::Port)
    }

    /// Tulip: Deposit from the vault account
    #[access_control(ctx.accounts.check_hash(Protocols::Tulip))]
    pub fn tulip_deposit(ctx: Context<TulipDeposit>) -> Result<()> {
        instructions::protocol_deposit::handler(ctx, Protocols::Tulip)
    }

    /// Tulip: Withdraw to the vault account
    #[access_control(ctx.accounts.check_hash(Protocols::Tulip))]
    pub fn tulip_withdraw(ctx: Context<TulipWithdraw>) -> Result<()> {
        instructions::protocol_withdraw::handler(ctx, Protocols::Tulip)
    }

    /// Tulip: Compute the TVL
    #[access_control(ctx.accounts.check_hash(Protocols::Tulip))]
    pub fn tulip_tvl(ctx: Context<TulipTVL>) -> Result<()> {
        instructions::protocol_rewards::handler(ctx, Protocols::Tulip)
    }

    /// Francium: Deposit from the vault account
    #[access_control(ctx.accounts.check_hash(Protocols::Francium))]
    pub fn francium_deposit(ctx: Context<FranciumDeposit>) -> Result<()> {
        instructions::protocol_deposit::handler(ctx, Protocols::Francium)
    }

    /// Francium: Withdraw to the vault account
    #[access_control(ctx.accounts.check_hash(Protocols::Francium))]
    pub fn francium_withdraw(ctx: Context<FranciumWithdraw>) -> Result<()> {
        instructions::protocol_withdraw::handler(ctx, Protocols::Francium)
    }

    /// Francium: Compute the TVL
    #[access_control(ctx.accounts.check_hash(Protocols::Francium))]
    pub fn francium_tvl(ctx: Context<FranciumTVL>) -> Result<()> {
        instructions::protocol_rewards::handler(ctx, Protocols::Francium)
    }
}

/// Check if the deposit is paused
fn deposit_not_paused() -> Result<()> {
    require!(!PAUSED_DEPOSIT, ErrorCode::OnPaused);
    Ok(())
}

/// Check if the withdraw is paused
fn withdraw_not_paused() -> Result<()> {
    require!(!PAUSED_WITHDRAW, ErrorCode::OnPaused);
    Ok(())
}

/// Check if target key is authorized
fn is_admin(key: &Pubkey) -> Result<()> {
    #[cfg(not(feature = "test"))]
    require!(key == &ADMIN_PUBKEY, UnauthorizedUser);
    Ok(())
}
