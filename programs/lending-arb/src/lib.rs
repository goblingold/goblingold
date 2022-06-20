use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
use error::ErrorCode;
use check_hash::{CheckHash, CHECKHASH_BYTES};
use instructions::*;
use protocols::{francium::*, solend::*, Protocols};
use vault::{RefreshParams, VaultAccount};

mod check_hash;
mod error;
mod instructions;
mod macros;
mod protocols;
mod vault;
mod health;

declare_id!("GGo34nYpjKfe9omzUaFtaCyizvwpAMf3NhxSCMD61F3A");


const PAUSED_DEPOSIT: bool = false;
const PAUSED_WITHDRAW: bool = false;

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
pub mod lending_arb {
    use super::*;

    /// Initialize the vault account and its fields
    #[access_control(is_admin(ctx.accounts.user_signer.key))]
    pub fn initialize_vault(ctx: Context<InitializeVault>, account_number: u8) -> Result<()> {
        instructions::initialize_vault::handler(ctx, account_number)
    }


    /// Add a new protocol to the vault_account
    #[access_control(is_admin(ctx.accounts.user_signer.key))]
    pub fn add_protocol(ctx: Context<AddProtocol>, protocol_id: u8) -> Result<()> {
        instructions::add_protocol::handler(ctx, protocol_id)
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

      /// Solend: Withdraw to the vault account
      #[access_control(ctx.accounts.check_hash(Protocols::Solend))]
      pub fn solend_borrow(ctx: Context<SolendBorrow>) -> Result<()> {
          instructions::protocol_borrow::handler(ctx, Protocols::Solend)
      }

    /// Solend: Withdraw to the vault account
      #[access_control(ctx.accounts.check_hash(Protocols::Solend))]
      pub fn solend_repay(ctx: Context<SolendRepay>) -> Result<()> {
          instructions::protocol_repay::handler(ctx, Protocols::Solend)
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

    /// Creates a vault_user_ticket_account
    pub fn create_vault_user_ticket_account(
        ctx: Context<CreateVaultUserTicketAccount>,
    ) -> Result<()> {
        instructions::create_vault_user_ticket_account::handler(ctx)
    }

    /// Deposit user input tokens into the vault account
    #[access_control(deposit_not_paused())]
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit::handler(ctx, amount)
    }

     /// Withdraw the required input tokens from the vault and send them back to the user
     #[access_control(withdraw_not_paused())]
     pub fn withdraw(ctx: Context<Withdraw>, lp_amount: u64) -> Result<()> {
         instructions::withdraw::handler(ctx, lp_amount)
     }
    
    /// Open a withdrawal ticket (for delayed withdrawals)
    #[access_control(withdraw_not_paused())]
    pub fn open_withdraw_ticket(
        ctx: Context<OpenWithdrawTicket>,
        lp_amount: u64,
        bump_user: u8,
    ) -> Result<()> {
        instructions::open_withdraw_ticket::handler(ctx, lp_amount, bump_user)
    }

    /// Close a withdrawal ticket
    #[access_control(withdraw_not_paused())]
    pub fn close_withdraw_ticket(
        ctx: Context<CloseWithdrawTicket>,
        lp_amount: u64,
        bump_user: u8,
    ) -> Result<()> {
        instructions::close_withdraw_ticket::handler(ctx, lp_amount, bump_user)
    }
}

/// Check if target key is authorized
fn is_admin(key: &Pubkey) -> Result<()> {
    #[cfg(not(feature = "test"))]
    require!(key == &ADMIN_PUBKEY, UnauthorizedUser);
    Ok(())
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
