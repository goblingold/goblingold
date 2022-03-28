// Some functions here have a RESTRICTED ACCESS.
// Besides others, to make them unrestricted it should be checked that the vault token accounts
// are deterministic (associated token accounts)

use anchor_lang::prelude::borsh::{BorshDeserialize, BorshSerialize};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program_option::COption, pubkey::Pubkey, sysvar};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use error::ErrorCode;
use protocols::{francium::*, mango::*, port::*, solend::*, tulip::*, PROTOCOLS_LEN};
use std::mem::size_of;
use std::str::FromStr;
use vault::{Bumps, InitVaultAccountParams, VaultAccount, HASH_PUBKEYS_LEN};

mod deposit;
mod error;
mod generic_accounts;
mod macros;
mod protocols;
mod refresh;
mod vault;
mod withdraw;

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
    pub fn initialize_strategy(ctx: Context<InitializeStrategy>) -> Result<()> {
        ctx.accounts
            .vault_account
            .set_inner(VaultAccount::init(InitVaultAccountParams {
                bumps: Bumps {
                    vault: *ctx.bumps.get("vault_account").unwrap(),
                    lp_token_mint: *ctx.bumps.get("vault_lp_token_mint_pubkey").unwrap(),
                },
                input_mint_pubkey: ctx.accounts.input_token_mint_address.key(),
                dao_treasury_lp_token_account: ctx.accounts.dao_treasury_lp_token_account.key(),
            }));

        Ok(())
    }

    // ACCESS RESTRICTED. ONLY ALLOWED_DEPLOYER
    pub fn set_protocol_weights(
        ctx: Context<SetProtocolWeights>,
        weights: [u16; PROTOCOLS_LEN],
    ) -> Result<()> {
        let weights_sum = weights
            .iter()
            .try_fold(0_u16, |acc, &x| acc.checked_add(x))
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

        require!(weights_sum == 1000, ErrorCode::InvalidWeights);

        ctx.accounts
            .vault_account
            .protocols
            .iter_mut()
            .zip(weights)
            .for_each(|(protocol, weight)| protocol.weight = weight);

        Ok(())
    }

    //pub fn close_account(_ctx: Context<CloseAccount>) -> Result<()> {
    //    Ok(())
    //}

    /// Deposit user input tokens into the vault account
    #[access_control(program_not_paused())]
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    /// Withdraw the required input tokens from the vault and send them back to the user
    #[access_control(program_not_paused())]
    pub fn withdraw(ctx: Context<Withdraw>, lp_amount: u64) -> Result<()> {
        ctx.accounts.withdraw(lp_amount)
    }

    /// Refresh the protocol weights
    pub fn refresh_rewards_weights(ctx: Context<RefreshRewardsWeights>) -> Result<()> {
        ctx.accounts.refresh()
    }

    // ACCESS RESTRICTED. ONLY ALLOWED_DEPLOYER
    /// Set hash of a protocol for a specific action
    pub fn set_hash(
        ctx: Context<SetHash>,
        protocol: usize,
        action: String,
        hash: [u8; HASH_PUBKEYS_LEN],
    ) -> Result<()> {
        ctx.accounts.set_hash(protocol, action, hash)
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
        ctx.accounts.create_and_initialize()
    }

    /// Francium: Deposit from the vault account
    #[access_control(ctx.accounts.check_hash())]
    pub fn francium_deposit(ctx: Context<FranciumDeposit>) -> Result<()> {
        ctx.accounts.deposit()
    }

    /// Francium: Withdraw to the vault account
    #[access_control(ctx.accounts.check_hash())]
    pub fn francium_withdraw(ctx: Context<FranciumWithdraw>) -> Result<()> {
        ctx.accounts.withdraw()
    }

    /// Francium: Compute the TVL
    #[access_control(ctx.accounts.check_hash())]
    pub fn francium_tvl(ctx: Context<FranciumTVL>) -> Result<()> {
        ctx.accounts.update_rewards()
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
pub struct InitializeStrategy<'info> {
    // Only deployer can initialize
    #[account(
        mut,
        constraint = Pubkey::from_str(ALLOWED_DEPLOYER).unwrap()== *user_signer.key
    )]
    pub user_signer: Signer<'info>,
    pub input_token_mint_address: Account<'info, Mint>,
    #[account(
        init,
        payer = user_signer,
        space = 8 + size_of::<VaultAccount>(),
        seeds = [VAULT_ACCOUNT_SEED, input_token_mint_address.key().as_ref()],
        bump,
    )]
    pub vault_account: Box<Account<'info, VaultAccount>>,
    #[account(
        init,
        payer = user_signer,
        associated_token::mint = input_token_mint_address,
        associated_token::authority = vault_account,
    )]
    pub vault_input_token_account: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = user_signer,
        mint::decimals = input_token_mint_address.decimals,
        mint::authority = vault_account.key(),
        seeds = [VAULT_LP_TOKEN_MINT_SEED, vault_account.key().as_ref()],
        bump,
    )]
    pub vault_lp_token_mint_pubkey: Account<'info, Mint>,
    #[account(
        init,
        payer = user_signer,
        associated_token::mint = vault_lp_token_mint_pubkey,
        associated_token::authority = dao_treasury_owner,
    )]
    pub dao_treasury_lp_token_account: Account<'info, TokenAccount>,
    #[account(constraint = dao_treasury_owner.key == &Pubkey::from_str(ALLOWED_DEPLOYER).unwrap())]
    /// CHECKED: address is checked
    pub dao_treasury_owner: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct SetProtocolWeights<'info> {
    // Only deployer can modify weights
    #[account(constraint = Pubkey::from_str(ALLOWED_DEPLOYER).unwrap()== *user_signer.key)]
    pub user_signer: Signer<'info>,
    #[account(
        mut,
        seeds = [VAULT_ACCOUNT_SEED, vault_account.input_mint_pubkey.as_ref()],
        bump = vault_account.bumps.vault
    )]
    pub vault_account: Box<Account<'info, VaultAccount>>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    pub user_signer: Signer<'info>,
    #[account(mut, constraint = user_input_token_account.owner == *user_signer.key)]
    pub user_input_token_account: Account<'info, TokenAccount>,
    #[account(mut, constraint = user_lp_token_account.owner == *user_signer.key)]
    pub user_lp_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [VAULT_ACCOUNT_SEED, vault_account.input_mint_pubkey.as_ref()],
        bump = vault_account.bumps.vault
    )]
    pub vault_account: Box<Account<'info, VaultAccount>>,
    #[account(
        mut,
        constraint = vault_lp_token_mint_pubkey.mint_authority == COption::Some(vault_account.key()),
        seeds = [VAULT_LP_TOKEN_MINT_SEED, vault_account.key().as_ref()],
        bump = vault_account.bumps.lp_token_mint
    )]
    pub vault_lp_token_mint_pubkey: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = vault_account.input_mint_pubkey,
        associated_token::authority = vault_account,
    )]
    pub vault_input_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub user_signer: Signer<'info>,
    #[account(mut, constraint = user_input_token_account.owner == *user_signer.key)]
    pub user_input_token_account: Account<'info, TokenAccount>,
    #[account(mut, constraint = user_lp_token_account.owner == *user_signer.key)]
    pub user_lp_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [VAULT_ACCOUNT_SEED, vault_account.input_mint_pubkey.as_ref()],
        bump = vault_account.bumps.vault
    )]
    pub vault_account: Box<Account<'info, VaultAccount>>,
    #[account(
        mut,
        constraint = vault_lp_token_mint_pubkey.mint_authority == COption::Some(vault_account.key()),
        seeds = [VAULT_LP_TOKEN_MINT_SEED, vault_account.key().as_ref()],
        bump = vault_account.bumps.lp_token_mint
    )]
    pub vault_lp_token_mint_pubkey: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = vault_account.input_mint_pubkey,
        associated_token::authority = vault_account,
    )]
    pub vault_input_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    #[account(address = sysvar::instructions::ID)]
    /// CHECK: address is checked
    pub instructions: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RefreshRewardsWeights<'info> {
    #[account(
        mut,
        seeds = [VAULT_ACCOUNT_SEED, vault_account.input_mint_pubkey.as_ref()],
        bump = vault_account.bumps.vault
    )]
    pub vault_account: Box<Account<'info, VaultAccount>>,
    #[account(
        associated_token::mint = vault_account.input_mint_pubkey,
        associated_token::authority = vault_account,
    )]
    pub vault_input_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = vault_lp_token_mint_pubkey.mint_authority == COption::Some(vault_account.key()),
        seeds = [VAULT_LP_TOKEN_MINT_SEED, vault_account.key().as_ref()],
        bump = vault_account.bumps.lp_token_mint
    )]
    pub vault_lp_token_mint_pubkey: Account<'info, Mint>,
    #[account(mut, address = vault_account.dao_treasury_lp_token_account)]
    pub dao_treasury_lp_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct SetHash<'info> {
    #[account(constraint = Pubkey::from_str(ALLOWED_DEPLOYER).unwrap()== *user_signer.key)]
    pub user_signer: Signer<'info>,
    #[account(
        mut,
        seeds = [VAULT_ACCOUNT_SEED, vault_account.input_mint_pubkey.as_ref()],
        bump = vault_account.bumps.vault
    )]
    pub vault_account: Account<'info, VaultAccount>,
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
