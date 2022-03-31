use crate::check_hash::*;
use crate::error::ErrorCode;
use crate::macros::generate_seeds;
use crate::protocols::Protocols;
use crate::vault::VaultAccount;
use crate::{
    generic_accounts_anchor_modules::*, GenericDepositAccounts, GenericTVLAccounts,
    GenericWithdrawAccounts,
};
use crate::{ALLOWED_DEPLOYER, VAULT_ACCOUNT_SEED};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    hash::{hashv, Hash},
    program::invoke_signed,
    pubkey::Pubkey,
    system_instruction,
};
use anchor_spl::token::{Token, TokenAccount};
use std::str::FromStr;

/// Program ids
pub mod port_lending_program_id {
    use anchor_lang::declare_id;
    #[cfg(feature = "devnet")]
    declare_id!("pdQ2rQQU5zH2rDgZ7xH2azMBJegUzUyunJ5Jd637hC4");
    #[cfg(not(feature = "devnet"))]
    declare_id!("Port7uDYB3wk6GJAw4KT1WpTeMtSu9bTcChBHkX2LfR");
}

pub mod port_staking_program_id {
    use anchor_lang::declare_id;
    declare_id!("stkarvwmSzv2BygN5e2LeTwimTczLWHCKPKGC2zVLiq");
}

#[derive(Accounts)]
pub struct PortInitialize<'info> {
    #[account(constraint = Pubkey::from_str(ALLOWED_DEPLOYER).unwrap()== *user_signer.key)]
    pub user_signer: Signer<'info>,
    #[account(
        seeds = [VAULT_ACCOUNT_SEED, vault_account.input_mint_pubkey.as_ref()],
        bump = vault_account.bumps.vault
    )]
    pub vault_account: Box<Account<'info, VaultAccount>>,
    #[account(mut)]
    /// CHECK: Port CPI
    pub vault_port_obligation_account: AccountInfo<'info>,
    /// CHECK: Port CPI
    pub port_lending_market_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Port CPI
    pub vault_port_staking_account: AccountInfo<'info>,
    #[account(constraint = port_lending_program_id.key == &port_lending_program_id::ID)]
    /// CHECK: Port CPI
    pub port_lending_program_id: AccountInfo<'info>,
    #[account(constraint = port_staking_program_id.key == &port_staking_program_id::ID)]
    /// CHECK: Port CPI
    pub port_staking_program_id: AccountInfo<'info>,
    /// CHECK: Port CPI
    pub port_staking_pool_account: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> PortInitialize<'info> {
    /// Create and initialize obligation account
    fn initialize_obligation(&self) -> Result<()> {
        let seeds = generate_seeds!(self.vault_account);
        let signer = &[&seeds[..]];

        {
            let account_size = port_anchor_adaptor::PortObligation::LEN;
            let ix = system_instruction::create_account_with_seed(
                self.user_signer.key,
                self.vault_port_obligation_account.key,
                &self.vault_account.key(),
                "port",
                Rent::default().minimum_balance(account_size),
                account_size as u64,
                self.port_lending_program_id.key,
            );
            invoke_signed(
                &ix,
                &[
                    self.user_signer.to_account_info(),
                    self.vault_account.to_account_info(),
                    self.vault_port_obligation_account.to_account_info(),
                    self.system_program.to_account_info(),
                ],
                signer,
            )?;
        }

        {
            let cpi_ctx = CpiContext::new_with_signer(
                self.port_lending_program_id.to_account_info(),
                port_anchor_adaptor::InitObligation {
                    obligation: self.vault_port_obligation_account.to_account_info(),
                    lending_market: self.port_lending_market_account.to_account_info(),
                    obligation_owner: self.vault_account.to_account_info(),
                    clock: self.clock.to_account_info(),
                    rent: self.rent.to_account_info(),
                    spl_token_id: self.token_program.to_account_info(),
                },
                signer,
            );
            port_anchor_adaptor::init_obligation(cpi_ctx)?;
        }

        Ok(())
    }

    /// Create and initialize stake account
    fn initialize_stake(&self) -> Result<()> {
        let seeds = generate_seeds!(self.vault_account);
        let signer = &[&seeds[..]];

        {
            let account_size = port_anchor_adaptor::PortStakeAccount::LEN;
            let ix = system_instruction::create_account_with_seed(
                self.user_signer.key,
                self.vault_port_staking_account.key,
                &self.vault_account.key(),
                "port",
                Rent::default().minimum_balance(account_size),
                account_size as u64,
                self.port_staking_program_id.key,
            );
            invoke_signed(
                &ix,
                &[
                    self.user_signer.to_account_info(),
                    self.vault_account.to_account_info(),
                    self.vault_account.to_account_info(),
                    self.vault_port_staking_account.to_account_info(),
                    self.system_program.to_account_info(),
                ],
                signer,
            )?;
        }

        {
            let cpi_ctx = CpiContext::new_with_signer(
                self.port_staking_program_id.to_account_info(),
                port_anchor_adaptor::CreateStakeAccount {
                    stake_account: self.vault_port_staking_account.to_account_info(),
                    staking_pool: self.port_staking_pool_account.to_account_info(),
                    owner: self.vault_account.to_account_info(),
                    rent: self.rent.to_account_info(),
                },
                signer,
            );
            port_anchor_adaptor::create_stake_account(cpi_ctx)?;
        }

        Ok(())
    }
}

/// Create and initialize protocol accounts
pub fn initialize(ctx: Context<PortInitialize>) -> Result<()> {
    ctx.accounts.initialize_obligation()?;
    ctx.accounts.initialize_stake()?;
    Ok(())
}

#[derive(Accounts)]
pub struct PortDeposit<'info> {
    pub generic_accs: GenericDepositAccounts<'info>,
    #[account(constraint = port_lending_program_id.key == &port_lending_program_id::ID)]
    /// CHECK: Port CPI
    pub port_lending_program_id: AccountInfo<'info>,
    #[account(constraint = port_staking_program_id.key == &port_staking_program_id::ID)]
    /// CHECK: Port CPI
    pub port_staking_program_id: AccountInfo<'info>,
    #[account(
        mut,
        associated_token::mint = vault_port_collateral_token_account.mint,
        associated_token::authority = generic_accs.vault_account,
    )]
    pub vault_port_collateral_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// CHECK: Port CPI
    pub vault_port_obligation_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Port CPI
    pub vault_port_staking_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Port CPI
    pub port_reserve_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Port CPI
    pub port_reserve_liquidity_supply_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Port CPI
    pub port_reserve_collateral_mint_account: AccountInfo<'info>,
    /// CHECK: Port CPI
    pub port_lending_market_account: AccountInfo<'info>,
    /// CHECK: Port CPI
    pub port_lending_market_authority_account: AccountInfo<'info>,
    #[account(mut)]
    pub port_destination_deposit_collateral_account: Account<'info, TokenAccount>,
    #[account(mut)]
    /// CHECK: Port CPI
    pub port_staking_pool_account: AccountInfo<'info>,
}

impl<'info> PortDeposit<'info> {
    /// CPI deposit call
    fn cpi_deposit(&self, amount: u64) -> Result<()> {
        let seeds = generate_seeds!(self.generic_accs.vault_account);
        let signer = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            self.port_lending_program_id.to_account_info(),
            port_anchor_adaptor::DepositAndCollateralize {
                source_liquidity: self
                    .generic_accs
                    .vault_input_token_account
                    .to_account_info(),
                user_collateral: self.vault_port_collateral_token_account.to_account_info(),
                reserve: self.port_reserve_account.to_account_info(),
                reserve_liquidity_supply: self
                    .port_reserve_liquidity_supply_account
                    .to_account_info(),
                reserve_collateral_mint: self
                    .port_reserve_collateral_mint_account
                    .to_account_info(),
                lending_market: self.port_lending_market_account.to_account_info(),
                lending_market_authority: self
                    .port_lending_market_authority_account
                    .to_account_info(),
                destination_collateral: self
                    .port_destination_deposit_collateral_account
                    .to_account_info(),
                obligation: self.vault_port_obligation_account.to_account_info(),
                obligation_owner: self.generic_accs.vault_account.to_account_info(),
                transfer_authority: self.generic_accs.vault_account.to_account_info(),
                clock: self.generic_accs.clock.to_account_info(),
                token_program: self.generic_accs.token_program.to_account_info(),
                stake_account: self.vault_port_staking_account.to_account_info(),
                staking_pool: self.port_staking_pool_account.to_account_info(),
                port_staking_program: self.port_staking_program_id.to_account_info(),
            },
            signer,
        );
        port_anchor_adaptor::deposit_and_collateralize(cpi_ctx, amount)?;

        Ok(())
    }
}

impl<'info> CheckHash<'info> for PortDeposit<'info> {
    fn hash(&self) -> Hash {
        hashv(&[
            self.vault_port_collateral_token_account.key().as_ref(),
            self.vault_port_obligation_account.key.as_ref(),
            self.vault_port_staking_account.key.as_ref(),
            self.port_reserve_account.key.as_ref(),
            self.port_reserve_liquidity_supply_account.key.as_ref(),
            self.port_reserve_collateral_mint_account.key.as_ref(),
            self.port_lending_market_account.key.as_ref(),
            self.port_lending_market_authority_account.key.as_ref(),
            self.port_destination_deposit_collateral_account
                .key()
                .as_ref(),
            self.port_staking_pool_account.key.as_ref(),
        ])
    }

    fn target_hash(&self) -> [u8; CHECKHASH_BYTES] {
        self.generic_accs.vault_account.protocols[Protocols::Port as usize]
            .hash_pubkey
            .hash_deposit
    }
}

/// Deposit into protocol
pub fn deposit(ctx: Context<PortDeposit>) -> Result<()> {
    let amount = ctx
        .accounts
        .generic_accs
        .amount_to_deposit(Protocols::Port)?;

    ctx.accounts.cpi_deposit(amount)?;
    ctx.accounts.generic_accs.vault_account.protocols[Protocols::Port as usize]
        .update_after_deposit(amount)?;

    Ok(())
}

#[derive(Accounts)]
pub struct PortWithdraw<'info> {
    pub generic_accs: GenericWithdrawAccounts<'info>,
    #[account(constraint = port_lending_program_id.key == &port_lending_program_id::ID)]
    /// CHECK: Port CPI
    pub port_lending_program_id: AccountInfo<'info>,
    #[account(constraint = port_staking_program_id.key == &port_staking_program_id::ID)]
    /// CHECK: Port CPI
    pub port_staking_program_id: AccountInfo<'info>,
    #[account(
        mut,
        associated_token::mint = vault_port_collateral_token_account.mint,
        associated_token::authority = generic_accs.vault_account,
    )]
    pub vault_port_collateral_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    /// CHECK: Port CPI
    pub vault_port_obligation_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Port CPI
    pub vault_port_staking_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Port CPI
    pub port_source_collateral_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Port CPI
    pub port_reserve_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Port CPI
    pub port_reserve_liquidity_supply_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Port CPI
    pub port_reserve_collateral_mint_account: AccountInfo<'info>,
    /// CHECK: Port CPI
    pub port_lending_market_account: AccountInfo<'info>,
    /// CHECK: Port CPI
    pub port_lending_market_authority_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Port CPI
    pub port_staking_pool_account: AccountInfo<'info>,
}

impl<'info> PortWithdraw<'info> {
    /// Convert reserve liquidity to collateral
    fn liquidity_to_collateral(&self, amount: u64) -> Result<u64> {
        let mut account_data_slice: &[u8] = &self.port_reserve_account.try_borrow_data()?;
        let reserve = port_anchor_adaptor::PortReserve::try_deserialize(&mut account_data_slice)?;
        let lp_amount = reserve
            .collateral_exchange_rate()?
            .liquidity_to_collateral(amount)?;
        Ok(lp_amount)
    }

    /// Withdraw from the protocol and get the true token balance
    fn withdraw_and_get_balance(&mut self, amount: u64) -> Result<u64> {
        let lp_amount = self.liquidity_to_collateral(amount)?;
        let amount_before = self.generic_accs.vault_input_token_account.amount;

        self.cpi_withdraw(lp_amount)?;
        self.generic_accs.vault_input_token_account.reload()?;

        let amount_after = self.generic_accs.vault_input_token_account.amount;
        let amount_diff = amount_after
            .checked_sub(amount_before)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

        Ok(amount_diff)
    }

    /// CPI withdraw call
    fn cpi_withdraw(&self, amount: u64) -> Result<()> {
        let seeds = generate_seeds!(self.generic_accs.vault_account);
        let signer = &[&seeds[..]];

        {
            let cpi_ctx = CpiContext::new_with_signer(
                self.port_lending_program_id.to_account_info(),
                port_anchor_adaptor::Withdraw {
                    source_collateral: self.port_source_collateral_account.to_account_info(),
                    destination_collateral: self
                        .vault_port_collateral_token_account
                        .to_account_info(),
                    reserve: self.port_reserve_account.to_account_info(),
                    obligation: self.vault_port_obligation_account.to_account_info(),
                    lending_market: self.port_lending_market_account.to_account_info(),
                    lending_market_authority: self
                        .port_lending_market_authority_account
                        .to_account_info(),
                    obligation_owner: self.generic_accs.vault_account.to_account_info(),
                    clock: self.generic_accs.clock.to_account_info(),
                    token_program: self.generic_accs.token_program.to_account_info(),
                    stake_account: self.vault_port_staking_account.to_account_info(),
                    staking_pool: self.port_staking_pool_account.to_account_info(),
                    port_staking_program: self.port_staking_program_id.to_account_info(),
                },
                signer,
            );
            port_anchor_adaptor::withdraw(cpi_ctx, amount)?;
        }

        {
            let cpi_ctx = CpiContext::new_with_signer(
                self.port_lending_program_id.to_account_info(),
                port_anchor_adaptor::Redeem {
                    source_collateral: self.vault_port_collateral_token_account.to_account_info(),
                    destination_liquidity: self
                        .generic_accs
                        .vault_input_token_account
                        .to_account_info(),
                    reserve: self.port_reserve_account.to_account_info(),
                    reserve_collateral_mint: self
                        .port_reserve_collateral_mint_account
                        .to_account_info(),
                    reserve_liquidity_supply: self
                        .port_reserve_liquidity_supply_account
                        .to_account_info(),
                    lending_market: self.port_lending_market_account.to_account_info(),
                    lending_market_authority: self
                        .port_lending_market_authority_account
                        .to_account_info(),
                    transfer_authority: self.generic_accs.vault_account.to_account_info(),
                    clock: self.generic_accs.clock.to_account_info(),
                    token_program: self.generic_accs.token_program.to_account_info(),
                },
                signer,
            );
            port_anchor_adaptor::redeem(cpi_ctx, amount)?;
        }

        Ok(())
    }
}

impl<'info> CheckHash<'info> for PortWithdraw<'info> {
    fn hash(&self) -> Hash {
        hashv(&[
            self.vault_port_collateral_token_account.key().as_ref(),
            self.vault_port_obligation_account.key.as_ref(),
            self.vault_port_staking_account.key.as_ref(),
            self.port_source_collateral_account.key.as_ref(),
            self.port_reserve_account.key.as_ref(),
            self.port_reserve_liquidity_supply_account.key.as_ref(),
            self.port_reserve_collateral_mint_account.key.as_ref(),
            self.port_lending_market_account.key.as_ref(),
            self.port_lending_market_authority_account.key.as_ref(),
            self.port_staking_pool_account.key.as_ref(),
        ])
    }

    fn target_hash(&self) -> [u8; CHECKHASH_BYTES] {
        self.generic_accs.vault_account.protocols[Protocols::Port as usize]
            .hash_pubkey
            .hash_withdraw
    }
}

/// Withdraw from the protocol
pub fn withdraw(ctx: Context<PortWithdraw>) -> Result<()> {
    let amount = ctx
        .accounts
        .generic_accs
        .amount_to_withdraw(Protocols::Port)?;
    let amount_withdrawn = ctx.accounts.withdraw_and_get_balance(amount)?;

    ctx.accounts.generic_accs.vault_account.protocols[Protocols::Port as usize]
        .update_after_withdraw(amount_withdrawn)?;

    Ok(())
}

#[derive(Accounts)]
pub struct PortTVL<'info> {
    pub generic_accs: GenericTVLAccounts<'info>,
    #[account(owner = port_lending_program_id::ID)]
    /// CHECK: hash, owner and mint data field are checked
    pub reserve: AccountInfo<'info>,
    #[account(owner = port_lending_program_id::ID)]
    /// CHECK: hash, owner and reserve & owner fields are checked
    pub obligation: AccountInfo<'info>,
}

impl<'info> PortTVL<'info> {
    /// Calculate the max native units to withdraw
    fn max_withdrawable(&self) -> Result<u64> {
        let mut reserve_data: &[u8] = &self.reserve.try_borrow_data()?;
        let mut obligation_data: &[u8] = &self.reserve.try_borrow_data()?;
        let reserve = port_anchor_adaptor::PortReserve::try_deserialize(&mut reserve_data)?;
        let obligation =
            port_anchor_adaptor::PortObligation::try_deserialize(&mut obligation_data)?;

        require!(
            reserve.liquidity.mint_pubkey == self.generic_accs.vault_account.input_mint_pubkey,
            ErrorCode::InvalidMint
        );

        require!(
            obligation.owner == self.generic_accs.vault_account.key(),
            ErrorCode::InvalidObligationOwner
        );

        require!(
            obligation.deposits[0].deposit_reserve == *self.reserve.key,
            ErrorCode::InvalidObligationReserve
        );

        let lp_amount = obligation.deposits[0].deposited_amount;

        let tvl = reserve
            .collateral_exchange_rate()?
            .collateral_to_liquidity(lp_amount)?;

        Ok(tvl)
    }
}

impl<'info> CheckHash<'info> for PortTVL<'info> {
    fn hash(&self) -> Hash {
        hashv(&[self.reserve.key.as_ref(), self.obligation.key.as_ref()])
    }

    fn target_hash(&self) -> [u8; CHECKHASH_BYTES] {
        self.generic_accs.vault_account.protocols[Protocols::Port as usize]
            .hash_pubkey
            .hash_tvl
    }
}

/// Update the protocol TVL
pub fn update_rewards(ctx: Context<PortTVL>) -> Result<()> {
    let tvl = ctx.accounts.max_withdrawable()?;

    let protocol = &mut ctx.accounts.generic_accs.vault_account.protocols[Protocols::Port as usize];
    let rewards = tvl.saturating_sub(protocol.amount);

    protocol.rewards.update(rewards)?;

    Ok(())
}

#[derive(Accounts)]
pub struct PortClaimRewards<'info> {
    #[account(
        mut,
        seeds = [VAULT_ACCOUNT_SEED, vault_account.input_mint_pubkey.as_ref()],
        bump = vault_account.bumps.vault
    )]
    pub vault_account: Box<Account<'info, VaultAccount>>,
    #[account(constraint = dao_treasury_owner.key == &Pubkey::from_str(ALLOWED_DEPLOYER).unwrap())]
    /// CHECKED: address is checked
    pub dao_treasury_owner: AccountInfo<'info>,
    #[account(constraint = port_staking_program_id.key == &port_staking_program_id::ID)]
    /// CHECK: Port CPI
    pub port_staking_program_id: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Port CPI
    pub vault_port_staking_account: AccountInfo<'info>,
    #[account(
        mut,
        associated_token::mint = vault_port_rewards_account.mint,
        associated_token::authority = dao_treasury_owner,
    )]
    pub vault_port_rewards_account: Account<'info, TokenAccount>,
    #[account(mut)]
    /// CHECK: Port CPI
    pub port_staking_pool_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Port CPI
    pub port_rewards_token_pool: AccountInfo<'info>,
    /// CHECK: Port CPI
    pub port_rewards_account_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub clock: Sysvar<'info, Clock>,
}

/// Claim protocol rewards
pub fn claim_rewards(ctx: Context<PortClaimRewards>) -> Result<()> {
    let seeds = generate_seeds!(ctx.accounts.vault_account);
    let signer = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.port_staking_program_id.to_account_info(),
        port_anchor_adaptor::ClaimReward {
            stake_account_owner: ctx.accounts.vault_account.to_account_info(),
            stake_account: ctx.accounts.vault_port_staking_account.to_account_info(),
            staking_pool: ctx.accounts.port_staking_pool_account.to_account_info(),
            reward_token_pool: ctx.accounts.port_rewards_token_pool.to_account_info(),
            reward_dest: ctx.accounts.vault_port_rewards_account.to_account_info(),
            staking_program_authority: ctx
                .accounts
                .port_rewards_account_authority
                .to_account_info(),
            clock: ctx.accounts.clock.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        },
        signer,
    );
    port_anchor_adaptor::claim_reward(cpi_ctx)?;

    Ok(())
}
