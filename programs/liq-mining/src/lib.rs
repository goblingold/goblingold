use crate::error::ErrorCode;
use anchor_lang::prelude::borsh::{BorshDeserialize, BorshSerialize};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program_option::COption, pubkey::Pubkey};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Burn, Mint, MintTo, Token, TokenAccount, Transfer};
use quarry_mine::{program::QuarryMine, Miner};
use quarry_mint_wrapper::program::QuarryMintWrapper;
use stable_swap_anchor::StableSwap;
use std::str::FromStr;
use std::{convert::TryInto, mem::size_of};

mod error;
mod sunny;
mod swap;

declare_id!("4S9nmN6jtD6BCKAEkZsr3HaumVjWC6iSsYmMzNSxCM1j");

pub const ALLOWED_DEPLOYER: &str = "2fmQLSF1xR5FK3Yc5VhGvnrx7mjXbNSJN3d3WySYnzr6";
pub const ALLOWED_RUNNER: &str = "2fmQLSF1xR5FK3Yc5VhGvnrx7mjXbNSJN3d3WySYnzr6";
const FEE: u64 = 10; // in per cent

#[program]
pub mod liq_mining {
    use super::*;

    pub fn initialize_strategy(ctx: Context<InitializeStrategy>, bump: u8) -> Result<()> {
        let vault_account = &mut ctx.accounts.vault_account;
        vault_account.bump = bump;
        vault_account.dao_treasury_saber_token_account = *ctx
            .accounts
            .dao_treasury_saber_token_account
            .to_account_info()
            .key;
            vault_account.dao_treasury_sunny_token_account = *ctx
            .accounts
            .dao_treasury_sunny_token_account
            .to_account_info()
            .key;
        vault_account.input_mint_pubkey = *ctx
            .accounts
            .vault_lp_token_mint_pubkey
            .to_account_info()
            .key;
        vault_account.input_mint_pubkey = *ctx
            .accounts
            .vault_lp_token_mint_pubkey
            .to_account_info()
            .key;

        vault_account.stable_swap_pool_id = *ctx.accounts.stable_swap_pool_id.to_account_info().key;

        vault_account.previous_lp_price = LpPrice {
            total_tokens: 1,
            minted_tokens: 1,
        };

        Ok(())
    }

    pub fn initialize_associated_token_account(_ctx: Context<InitializeATA>) -> Result<()> {
        Ok(())
    }

    pub fn initialize_sunny(ctx: Context<InitializeSunny>, bump: u8) -> Result<()> {
        ctx.accounts.initialize_vault(bump)
    }

    pub fn initialize_sunny_miner(ctx: Context<InitializeSunnyMiner>, bump: u8) -> Result<()> {
        ctx.accounts.initialize_miner(bump)
    }

    pub fn create_quarry_miner(ctx: Context<CreateQuarryMiner>, bump: u8) -> Result<()> {
        let seeds = &[
            ctx.accounts.vault_account.to_account_info().key.as_ref(),
            &[ctx.accounts.vault_account.bump],
        ];
        let signer = &[&seeds[..]];

        let cpi_ctx: CpiContext<quarry_mine::cpi::accounts::CreateMiner> =
            CpiContext::new_with_signer(
                ctx.accounts.quarry_mine_program_id.to_account_info(),
                ctx.accounts.quarry_miner.to_cpi_accounts(),
                signer,
            );
        quarry_mine::cpi::create_miner(cpi_ctx, bump)?;

        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        // Transfer user token to vault account
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_input_token_account.to_account_info(),
            to: ctx.accounts.vault_input_token_account.to_account_info(),
            authority: ctx.accounts.user_signer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        // Mint vault tokens to user vault account
        let lp_amount = LpPrice {
            total_tokens: ctx.accounts.vault_account.current_tvl,
            minted_tokens: ctx.accounts.vault_lp_token_mint_address.supply,
        }
        .token_to_lp(amount)?;

        let seeds = &[
            ctx.accounts.vault_account.to_account_info().key.as_ref(),
            &[ctx.accounts.vault_account.bump],
        ];
        let signer = &[&seeds[..]];
        let cpi_accounts = MintTo {
            mint: ctx.accounts.vault_lp_token_mint_address.to_account_info(),
            to: ctx.accounts.user_lp_token_account.to_account_info(),
            authority: ctx.accounts.vault_signer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::mint_to(cpi_ctx, lp_amount)?;

        // Update total deposited amount
        ctx.accounts.vault_account.current_tvl = ctx
            .accounts
            .vault_account
            .current_tvl
            .checked_add(amount)
            .ok_or(ErrorCode::MathOverflow)?;

        Ok(())
    }

    // Before calling withdraw, call unstake_sunny
    pub fn withdraw(ctx: Context<Withdraw>, lp_amount: u64) -> Result<()> {
        let seeds = &[
            ctx.accounts.vault_account.to_account_info().key.as_ref(),
            &[ctx.accounts.vault_account.bump],
        ];
        let signer = &[&seeds[..]];

        let amount = ctx
            .accounts
            .vault_account
            .previous_lp_price
            .lp_to_token(lp_amount)?;

        let cpi_accounts = Transfer {
            from: ctx.accounts.vault_input_token_account.to_account_info(),
            to: ctx.accounts.user_input_token_account.to_account_info(),
            authority: ctx.accounts.vault_signer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, amount)?;

        let cpi_accounts = Burn {
            mint: ctx.accounts.vault_lp_token_mint_address.to_account_info(),
            to: ctx.accounts.user_lp_token_account.to_account_info(),
            authority: ctx.accounts.user_signer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::burn(cpi_ctx, lp_amount)?;

        ctx.accounts.vault_account.current_tvl = ctx
            .accounts
            .vault_account
            .current_tvl
            .checked_sub(amount)
            .ok_or(ErrorCode::MathOverflow)?;

        Ok(())
    }

    pub fn stake_sunny(ctx: Context<SunnyStake>) -> Result<()> {
        ctx.accounts.stake()
    }

    pub fn unstake_sunny(ctx: Context<SunnyUnstake>, amount: u64) -> Result<()> {
        ctx.accounts.unstake(amount)
    }

    pub fn claim_rewards_sunny(ctx: Context<SunnyClaimRewards>) -> Result<()> {
        ctx.accounts.claim_rewards()
    }

    pub fn claim_rewards_saber(ctx: Context<ClaimRewardsSaber>) -> Result<()> {
        let seeds = &[
            ctx.accounts.vault_account.to_account_info().key.as_ref(),
            &[ctx.accounts.vault_account.bump],
        ];
        let signer = &[&seeds[..]];

        let cpi_ctx: CpiContext<quarry_mine::cpi::accounts::ClaimRewards> =
            CpiContext::new_with_signer(
                ctx.accounts.quarry_mine_program_id.to_account_info(),
                ctx.accounts.claim_rewards.to_cpi_accounts(),
                signer,
            );
        quarry_mine::cpi::claim_rewards(cpi_ctx)?;

        Ok(())
    }

    pub fn redeem_sunny(ctx: Context<SunnyRedeem>) -> Result<()> {
         // Fee
        let saber_before = ctx.accounts.redeem_saber.redeem_ctx.redemption_destination.amount;
        let sunny_before = ctx.accounts.redeem_sunny.redeem_ctx.redemption_destination.amount;

        ctx.accounts.redeem()?;

        ctx.accounts.redeem_saber.redeem_ctx.redemption_destination.reload()?;
        ctx.accounts.redeem_sunny.redeem_ctx.redemption_destination.reload()?;

        let saber_fee = calculate_fee(ctx.accounts.redeem_saber.redeem_ctx.redemption_destination.amount.checked_sub(saber_before).ok_or(ErrorCode::MathOverflow)?)?;
        let sunny_fee = calculate_fee(ctx.accounts.redeem_sunny.redeem_ctx.redemption_destination.amount.checked_sub(sunny_before).ok_or(ErrorCode::MathOverflow)?)?;

        let cpi_accounts = Transfer {
            from: ctx.accounts.redeem_saber.redeem_ctx.redemption_destination.to_account_info(),
            to: ctx.accounts.dao_treasury_saber_token_account.to_account_info(),
            authority: ctx.accounts.vault_signer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, saber_fee)?;

        let cpi_accounts = Transfer {
            from: ctx.accounts.redeem_sunny.redeem_ctx.redemption_destination.to_account_info(),
            to: ctx.accounts.dao_treasury_sunny_token_account.to_account_info(),
            authority: ctx.accounts.vault_signer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, sunny_fee)?;

        Ok(())
    }

    pub fn swap(ctx: Context<RaydiumSwap>) -> Result<()> {
        ctx.accounts.swap_rewards()
    }

    // --------- SABER FUNCTIONS  ------------

    pub fn deposit_saber(ctx: Context<SaberDeposit>) -> Result<()> {
        let seeds = &[
            ctx.accounts.vault_account.to_account_info().key.as_ref(),
            &[ctx.accounts.vault_account.bump],
        ];
        let signer = &[&seeds[..]];

        let cpi_ctx: CpiContext<stable_swap_anchor::Deposit> = CpiContext::new_with_signer(
            ctx.accounts.stable_swap_program_id.to_account_info(),
            ctx.accounts.deposit.to_cpi_accounts(),
            signer,
        );

        // TODO set slippage
        let amount_a = ctx.accounts.deposit.input_a.user.amount;
        let amount_b = 0;
        let min_amount_mint = 0;

        let amount_before = ctx.accounts.deposit.vault_input_token_account.amount;
        ctx.accounts.vault_account.previous_lp_price = LpPrice {
            total_tokens: ctx.accounts.vault_account.current_tvl,
            minted_tokens: ctx.accounts.vault_lp_token_mint_pubkey.supply,
        };

        stable_swap_anchor::deposit(cpi_ctx, amount_a, amount_b, min_amount_mint)?;

        ctx.accounts.deposit.vault_input_token_account.reload()?;
        let amount_after = ctx.accounts.deposit.vault_input_token_account.amount;
        let amount_diff = amount_after
            .checked_sub(amount_before)
            .ok_or(ErrorCode::MathOverflow)?;

        ctx.accounts.vault_account.current_tvl = ctx
            .accounts
            .vault_account
            .current_tvl
            .checked_add(amount_diff)
            .ok_or(ErrorCode::MathOverflow)?;

        Ok(())
    }

    pub fn withdraw_saber(ctx: Context<SaberWithdraw>) -> Result<()> {
        let seeds = &[
            ctx.accounts.vault_account.to_account_info().key.as_ref(),
            &[ctx.accounts.vault_account.bump],
        ];
        let signer = &[&seeds[..]];

        let cpi_ctx: CpiContext<stable_swap_anchor::WithdrawOne> = CpiContext::new_with_signer(
            ctx.accounts.stable_swap_program_id.to_account_info(),
            ctx.accounts.withdraw.to_cpi_accounts(),
            signer,
        );

        // TODO set slippage get amount from withdraw
        let amount = ctx.accounts.withdraw.input_lp.amount;
        let min_amount = 0;
        stable_swap_anchor::withdraw_one(cpi_ctx, amount, min_amount)?;

        Ok(())
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        let seeds = &[
            ctx.accounts.vault_account.to_account_info().key.as_ref(),
            &[ctx.accounts.vault_account.bump],
        ];
        let signer = &[&seeds[..]];

        let cpi_ctx: CpiContext<quarry_mine::cpi::accounts::UserStake> =
            CpiContext::new_with_signer(
                ctx.accounts.quarry_mine_program_id.to_account_info(),
                ctx.accounts.user_stake.to_user_stake_cpi_accounts(),
                signer,
            );

        let amount = ctx.accounts.user_stake.token_account.amount;
        quarry_mine::cpi::stake_tokens(cpi_ctx, amount)?;

        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        let seeds = &[
            ctx.accounts.vault_account.to_account_info().key.as_ref(),
            &[ctx.accounts.vault_account.bump],
        ];
        let signer = &[&seeds[..]];

        let cpi_ctx: CpiContext<quarry_mine::cpi::accounts::UserStake> =
            CpiContext::new_with_signer(
                ctx.accounts.quarry_mine_program_id.to_account_info(),
                ctx.accounts.user_stake.to_user_stake_cpi_accounts(),
                signer,
            );

        // TODO get amount from withdraw
        let amount = ctx.accounts.user_stake.miner_vault.amount;
        quarry_mine::cpi::withdraw_tokens(cpi_ctx, amount)?;

        Ok(())
    }

    pub fn redeem_saber(ctx: Context<SaberRedeem>) -> Result<()> {
        let seeds = &[
            ctx.accounts.vault_account.to_account_info().key.as_ref(),
            &[ctx.accounts.vault_account.bump],
        ];
        let signer = &[&seeds[..]];

        let cpi_ctx: CpiContext<redeemer::cpi::accounts::RedeemTokensFromMintProxy> =
            CpiContext::new_with_signer(
                ctx.accounts.redeemer_program_id.to_account_info(),
                ctx.accounts.redeem.to_cpi_accounts(),
                signer,
            );

        let amount = ctx.accounts.redeem.redeem_ctx.iou_source.amount;
        redeemer::cpi::redeem_tokens_from_mint_proxy(cpi_ctx, amount)?;

        Ok(())
    }
}

pub fn calculate_fee(amount: u64) -> Result<u64>{
    Ok(amount
        .checked_mul(FEE)
        .ok_or(ErrorCode::MathOverflow)?
        .checked_div(100)
        .ok_or(ErrorCode::MathOverflow)?)
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default)]
pub struct LpPrice {
    pub total_tokens: u64,
    pub minted_tokens: u64,
}

impl LpPrice {
    /// Transform input token amount to LP amount
    pub fn token_to_lp(&self, amount: u64) -> Result<u64> {
        if self.minted_tokens == 0 {
            Ok(amount)
        } else {
            Ok((amount as u128)
                .checked_mul(self.minted_tokens as u128)
                .ok_or(ErrorCode::MathOverflow)?
                .checked_div(self.total_tokens as u128)
                .ok_or(ErrorCode::MathOverflow)?
                .try_into()
                .map_err(|_| ErrorCode::MathOverflow)?)
        }
    }

    /// Transform LP amount to input token amount
    pub fn lp_to_token(&self, lp_amount: u64) -> Result<u64> {
        Ok((lp_amount as u128)
            .checked_mul(self.total_tokens as u128)
            .ok_or(ErrorCode::MathOverflow)?
            .checked_div(self.minted_tokens as u128)
            .ok_or(ErrorCode::MathOverflow)?
            .try_into()
            .map_err(|_| ErrorCode::MathOverflow)?)
    }
}

// TODO check constraints: token_accounts mints, ...
/// Accounts for the entrypoint. For those used in cpi-calls, only the accounts related with the
/// vault are checked. The rest of them are delegated to the underlying cpi program.

#[derive(Accounts)]
pub struct InitializeStrategy<'info> {
    #[account(
        mut,
        constraint = Pubkey::from_str(ALLOWED_DEPLOYER).unwrap()== *user_signer.key
    )]
    pub user_signer: Signer<'info>,
    #[account(
        init,
        payer = user_signer,
        space = 8 + size_of::<VaultAccount>()
    )]
    pub vault_account: Box<Account<'info, VaultAccount>>,
    #[account(
        seeds = [vault_account.to_account_info().key.as_ref()],
        bump = vault_account.bump,
    )]
    /// CHECK: TODO 
    pub vault_signer: AccountInfo<'info>,
    #[account(
        init,
        payer = user_signer,
        mint::decimals = input_token_mint_address.decimals,
        mint::authority = vault_signer,
    )]
    pub vault_lp_token_mint_pubkey: Account<'info, Mint>,
    pub input_token_mint_address: Account<'info, Mint>,
    /// CHECK: TODO
    pub stable_swap_pool_id: AccountInfo<'info>,
    #[account(
        init,
        payer = user_signer,
        associated_token::mint = input_token_mint_address,
        associated_token::authority = vault_signer,
   )]
    pub vault_input_token_account: Account<'info, TokenAccount>,
    #[account(
        constraint = dao_treasury_saber_token_account.owner == Pubkey::from_str(ALLOWED_DEPLOYER).unwrap(),
        constraint = dao_treasury_saber_token_account.mint == vault_lp_token_mint_pubkey.key()
    )]
    pub dao_treasury_saber_token_account: Box<Account<'info, TokenAccount>>,
    #[account(
        constraint = dao_treasury_sunny_token_account.owner == Pubkey::from_str(ALLOWED_DEPLOYER).unwrap(),
        constraint = dao_treasury_sunny_token_account.mint == vault_lp_token_mint_pubkey.key()
    )]
    pub dao_treasury_sunny_token_account: Box<Account<'info, TokenAccount>>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct InitializeATA<'info> {
    #[account(mut)]
    pub user_signer: Signer<'info>,
    #[account(
        seeds = [vault_account.to_account_info().key.as_ref()],
        bump = vault_account.bump,
    )]
    /// CHECK: TODO 
    pub vault_signer: AccountInfo<'info>,
    pub vault_account: Account<'info, VaultAccount>,
    #[account(
         init,
         payer = user_signer,
         associated_token::mint = mint_account,
         associated_token::authority = vault_signer,
    )]
    pub vault_signer_ata: Account<'info, TokenAccount>,
    pub mint_account: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct InitializeSunny<'info> {
    pub user_signer: Signer<'info>,
    #[account(
        seeds = [vault_account.to_account_info().key.as_ref()],
        bump = vault_account.bump,
    )]
    /// CHECK: TODO 
    pub vault_signer: AccountInfo<'info>,
    pub vault_account: Account<'info, VaultAccount>,
    #[account(mut)]
    /// CHECK: TODO check seeds with anchor .21
    pub vault_sunny: AccountInfo<'info>,
    /// CHECK: TODO 
    pub sunny_pool: AccountInfo<'info>,
    #[account(constraint = sunny_program.key == &sunny::program::ID)]
    /// CHECK: TODO 
    pub sunny_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeSunnyMiner<'info> {
    #[account(mut)]
    pub user_signer: Signer<'info>,
    #[account(
        seeds = [vault_account.to_account_info().key.as_ref()],
        bump = vault_account.bump
    )]
    /// CHECK: TODO 
    pub vault_signer: AccountInfo<'info>,
    pub vault_account: Box<Account<'info, VaultAccount>>,
    /// CHECK: TODO 
    pub vault_sunny: AccountInfo<'info>,
    #[account(
        init,
        payer = user_signer,
        associated_token::mint = quarry_miner.token_mint,
        associated_token::authority = quarry_miner.miner,
    )]
    pub vault_miner_lp: Box<Account<'info, TokenAccount>>,
    /// CHECK: TODO 
    pub sunny_pool: AccountInfo<'info>,
    #[account(constraint = sunny_program.key == &sunny::program::ID)]
    /// CHECK: TODO 
    pub sunny_program: AccountInfo<'info>,
    pub quarry_mine_program_id: Program<'info, QuarryMine>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    #[account(
        constraint = quarry_miner.payer.key == user_signer.key,
        constraint = quarry_miner.authority.key == vault_sunny.key,
        constraint = quarry_miner.miner_vault.key == &vault_miner_lp.to_account_info().key(),
    )]
    pub quarry_miner: QuarryMineCreateMiner<'info>,
}

#[derive(Accounts)]
pub struct CreateQuarryMiner<'info> {
    #[account(mut)]
    pub user_signer: Signer<'info>,
    #[account(
        seeds = [vault_account.to_account_info().key.as_ref()],
        bump = vault_account.bump
    )]
    /// CHECK: TODO 
    pub vault_signer: AccountInfo<'info>,
    pub vault_account: Account<'info, VaultAccount>,
    pub quarry_mine_program_id: Program<'info, QuarryMine>,
    #[account(
        init,
        payer = user_signer,
        associated_token::mint = quarry_miner.token_mint,
        associated_token::authority = quarry_miner.miner,
    )]
    pub vault_miner_stable_swap: Box<Account<'info, TokenAccount>>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    #[account(
        constraint = quarry_miner.payer.key == user_signer.key,
        constraint = quarry_miner.authority.key == vault_signer.key,
        constraint = quarry_miner.miner_vault.key == &vault_miner_stable_swap.to_account_info().key(),
        constraint = quarry_miner.token_program.key == token_program.key,
        constraint = quarry_miner.system_program.key == system_program.key,
    )]
    pub quarry_miner: QuarryMineCreateMiner<'info>,
}

#[derive(Accounts)]
pub struct QuarryMineCreateMiner<'info> {
    /// CHECK: TODO 
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub miner: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub quarry: AccountInfo<'info>,
    /// CHECK: TODO 
    pub rewarder: AccountInfo<'info>,
    /// CHECK: TODO 
    pub system_program: AccountInfo<'info>,
    /// CHECK: TODO 
    pub payer: AccountInfo<'info>,
    /// CHECK: TODO 
    pub token_mint: AccountInfo<'info>,
    /// CHECK: TODO 
    pub miner_vault: AccountInfo<'info>,
    /// CHECK: TODO 
    pub token_program: AccountInfo<'info>,
}

impl<'info> QuarryMineCreateMiner<'info> {
    pub fn to_cpi_accounts(&self) -> quarry_mine::cpi::accounts::CreateMiner<'info> {
        quarry_mine::cpi::accounts::CreateMiner {
            authority: self.authority.to_account_info(),
            miner: self.miner.to_account_info(),
            quarry: self.quarry.to_account_info(),
            rewarder: self.rewarder.to_account_info(),
            system_program: self.system_program.to_account_info(),
            payer: self.payer.to_account_info(),
            token_mint: self.token_mint.to_account_info(),
            miner_vault: self.miner_vault.to_account_info(),
            token_program: self.token_program.to_account_info(),
        }
    }
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    pub user_signer: Signer<'info>,
    #[account(
        mut,
        constraint = user_input_token_account.mint == vault_account.input_mint_pubkey,
        constraint = user_input_token_account.owner == *user_signer.key
    )]
    pub user_input_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = user_lp_token_account.mint == vault_account.input_mint_pubkey,
        constraint = user_lp_token_account.owner == *user_signer.key,
    )]
    pub user_lp_token_account: Account<'info, TokenAccount>,
    #[account(
        seeds = [vault_account.to_account_info().key.as_ref()],
        bump = vault_account.bump
    )]
    /// CHECK: TODO 
    pub vault_signer: AccountInfo<'info>,
    pub vault_account: Box<Account<'info, VaultAccount>>,
    #[account(
        mut,
        constraint = vault_lp_token_mint_address.key() == vault_account.input_mint_pubkey,
        constraint = vault_lp_token_mint_address.mint_authority == COption::Some(*vault_signer.key),
    )]
    pub vault_lp_token_mint_address: Account<'info, Mint>,
    #[account(
        mut,
        constraint = vault_lp_token_mint_pubkey.mint_authority == COption::Some(*vault_signer.key),
        constraint = vault_account.vault_lp_token_mint_pubkey == *vault_lp_token_mint_pubkey.to_account_info().key
    )]
    pub vault_lp_token_mint_pubkey: Account<'info, Mint>,
    // #[account(
    //     mut,
    //     associated_token::mint = vault_account.input_mint_pubkey,
    //     associated_token::authority = vault_signer,
    // )]
    pub vault_input_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub user_signer: Signer<'info>,
    #[account(
        mut,
        constraint = user_input_token_account.mint == vault_account.input_mint_pubkey,
        constraint = user_input_token_account.owner == *user_signer.key
    )]
    pub user_input_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = user_lp_token_account.mint == vault_account.input_mint_pubkey,
        constraint = user_lp_token_account.owner == *user_signer.key,
    )]
    pub user_lp_token_account: Account<'info, TokenAccount>,
    #[account(
        seeds = [vault_account.to_account_info().key.as_ref()],
        bump = vault_account.bump
    )]
    /// CHECK: TODO 
    pub vault_signer: AccountInfo<'info>,
    pub vault_account: Box<Account<'info, VaultAccount>>,
    #[account(
        mut,
        constraint = vault_lp_token_mint_pubkey.mint_authority == COption::Some(*vault_signer.key),
        constraint = vault_account.vault_lp_token_mint_pubkey == *vault_lp_token_mint_pubkey.to_account_info().key
    )]
    pub vault_lp_token_mint_pubkey: Account<'info, Mint>,
    #[account(
        mut,
        constraint = vault_lp_token_mint_address.key() == vault_account.input_mint_pubkey,
        constraint = vault_lp_token_mint_address.mint_authority == COption::Some(*vault_signer.key),
    )]
    pub vault_lp_token_mint_address: Account<'info, Mint>,
    // #[account(
    //     mut,
    //     associated_token::mint = vault_account.input_mint_pubkey,
    //     associated_token::authority = vault_signer,
    // )]
    pub vault_input_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct SaberDeposit<'info> {
    #[account(
        seeds = [vault_account.to_account_info().key.as_ref()],
        bump = vault_account.bump
    )]
    /// CHECK: TODO 
    pub vault_signer: AccountInfo<'info>,
    pub vault_account: Account<'info, VaultAccount>,
    pub stable_swap_program_id: Program<'info, StableSwap>,
    #[account(
        mut,
        constraint = vault_lp_token_mint_pubkey.mint_authority == COption::Some(*vault_signer.key),
        constraint = vault_account.vault_lp_token_mint_pubkey == *vault_lp_token_mint_pubkey.to_account_info().key
    )]
    pub vault_lp_token_mint_pubkey: Account<'info, Mint>,
    #[account(
        constraint = deposit.user.swap.key == &vault_account.stable_swap_pool_id,
        constraint = deposit.user.user_authority.key == vault_signer.key,
        constraint = deposit.input_a.user.mint == vault_account.input_mint_pubkey,
        constraint = deposit.input_a.user.owner == *vault_signer.key,
        constraint = deposit.input_b.user.owner == *vault_signer.key,
    )]
    pub deposit: StableSwapAnchorDeposit<'info>,
}

#[derive(Accounts)]
pub struct SaberWithdraw<'info> {
    #[account(
        seeds = [vault_account.to_account_info().key.as_ref()],
        bump = vault_account.bump
    )]
    /// CHECK: TODO 
    pub vault_signer: AccountInfo<'info>,
    pub vault_account: Account<'info, VaultAccount>,
    pub stable_swap_program_id: Program<'info, StableSwap>,
    #[account(
        constraint = withdraw.input_lp.owner == *vault_signer.key,
        constraint = withdraw.user.swap.key == &vault_account.stable_swap_pool_id,
        constraint = withdraw.user.user_authority.key == vault_signer.key,
        constraint = withdraw.output.user_token.user.mint == vault_account.input_mint_pubkey,
        constraint = withdraw.output.user_token.user.owner == *vault_signer.key,
    )]
    pub withdraw: StableSwapWithdrawOne<'info>,
}

#[derive(Accounts)]
pub struct StableSwapAnchorDeposit<'info> {
    pub user: StableSwapAnchorSwapUserContext<'info>,
    pub input_a: StableSwapAnchorSwapToken<'info>,
    pub input_b: StableSwapAnchorSwapToken<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub pool_mint: AccountInfo<'info>,
    // #[account(
    //     mut,
    //     associated_token::mint = vault_input_token_account.mint,
    //     associated_token::authority = vault_input_token_account.owner,
    // )]
    pub vault_input_token_account: Box<Account<'info, TokenAccount>>,
}

impl<'info> StableSwapAnchorDeposit<'info> {
    pub fn to_cpi_accounts(&self) -> stable_swap_anchor::Deposit<'info> {
        stable_swap_anchor::Deposit {
            user: self.user.to_cpi_accounts(),
            input_a: self.input_a.to_cpi_accounts(),
            input_b: self.input_b.to_cpi_accounts(),
            pool_mint: self.pool_mint.to_account_info(),
            output_lp: self.vault_input_token_account.to_account_info(),
        }
    }
}

#[derive(Accounts)]
pub struct StableSwapWithdrawOne<'info> {
    pub user: StableSwapAnchorSwapUserContext<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub pool_mint: AccountInfo<'info>,
    #[account(mut)]
    pub input_lp: Account<'info, TokenAccount>,
    #[account(mut)]
    /// CHECK: TODO 
    pub quote_reserves: AccountInfo<'info>,
    pub output: StableSwapAnchorSwapOutput<'info>,
}

impl<'info> StableSwapWithdrawOne<'info> {
    pub fn to_cpi_accounts(&self) -> stable_swap_anchor::WithdrawOne<'info> {
        stable_swap_anchor::WithdrawOne {
            user: self.user.to_cpi_accounts(),
            pool_mint: self.pool_mint.to_account_info(),
            input_lp: self.input_lp.to_account_info(),
            quote_reserves: self.quote_reserves.to_account_info(),
            output: self.output.to_cpi_accounts(),
        }
    }
}

#[derive(Accounts)]
pub struct StableSwapAnchorSwapUserContext<'info> {
    /// CHECK: TODO 
    pub token_program: AccountInfo<'info>,
    /// CHECK: TODO 
    pub swap_authority: AccountInfo<'info>,
    /// CHECK: TODO 
    pub user_authority: AccountInfo<'info>,
    /// CHECK: TODO 
    pub swap: AccountInfo<'info>,
}

impl<'info> StableSwapAnchorSwapUserContext<'info> {
    pub fn to_cpi_accounts(&self) -> stable_swap_anchor::SwapUserContext<'info> {
        stable_swap_anchor::SwapUserContext {
            token_program: self.token_program.to_account_info(),
            swap_authority: self.swap_authority.to_account_info(),
            user_authority: self.user_authority.to_account_info(),
            swap: self.swap.to_account_info(),
        }
    }
}

#[derive(Accounts)]
pub struct StableSwapAnchorSwapOutput<'info> {
    pub user_token: StableSwapAnchorSwapToken<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub fees: AccountInfo<'info>,
}

impl<'info> StableSwapAnchorSwapOutput<'info> {
    pub fn to_cpi_accounts(&self) -> stable_swap_anchor::SwapOutput<'info> {
        stable_swap_anchor::SwapOutput {
            user_token: self.user_token.to_cpi_accounts(),
            fees: self.fees.to_account_info(),
        }
    }
}

#[derive(Accounts)]
pub struct StableSwapAnchorSwapToken<'info> {
    // #[account(
    //     mut,
    //     associated_token::mint = user.mint,
    //     associated_token::authority = user.owner,
    // )]
    pub user: Account<'info, TokenAccount>,
    #[account(mut)]
    /// CHECK: TODO 
    pub reserve: AccountInfo<'info>,
}

impl<'info> StableSwapAnchorSwapToken<'info> {
    pub fn to_cpi_accounts(&self) -> stable_swap_anchor::SwapToken<'info> {
        stable_swap_anchor::SwapToken {
            user: self.user.to_account_info(),
            reserve: self.reserve.to_account_info(),
        }
    }
}

#[derive(Accounts)]
pub struct SunnyStake<'info> {
    #[account(
        seeds = [vault_account.to_account_info().key.as_ref()],
        bump = vault_account.bump
    )]
    /// CHECK: TODO 
    pub vault_signer: AccountInfo<'info>,
    pub vault_account: Account<'info, VaultAccount>,
    #[account(mut)]
    pub vault_lp_saber: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// CHECK: TODO 
    pub vault_sunny: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub sunny_pool: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub sunny_token_mint: AccountInfo<'info>,
    pub quarry_mine_program: Program<'info, QuarryMine>,
    #[account(constraint = sunny_program.key == &sunny::program::ID)]
    /// CHECK: TODO 
    pub sunny_program: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub clock: Sysvar<'info, Clock>,
    #[account(
        constraint = stake_saber.vault_sunny_ata.owner == *vault_sunny.key,
        constraint = stake_saber.vault_sunny_ata.mint == vault_lp_saber.mint
    )]
    pub stake_saber: SunnyStakeInternal<'info>,
    #[account(
        constraint = stake_sunny.vault_sunny_ata.owner == *vault_sunny.key,
    )]
    pub stake_sunny: SunnyStakeInternal<'info>,
}

#[derive(Accounts)]
pub struct SunnyUnstake<'info> {
    #[account(
        seeds = [vault_account.to_account_info().key.as_ref()],
        bump = vault_account.bump
    )]
    /// CHECK: TODO 
    pub vault_signer: AccountInfo<'info>,
    pub vault_account: Account<'info, VaultAccount>,
    #[account(mut)]
    pub vault_lp_saber: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// CHECK: TODO 
    pub vault_sunny: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub sunny_pool: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub sunny_token_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub sunny_fee_destination: AccountInfo<'info>,
    pub quarry_mine_program: Program<'info, QuarryMine>,
    #[account(constraint = sunny_program.key == &sunny::program::ID)]
    /// CHECK: TODO 
    pub sunny_program: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub clock: Sysvar<'info, Clock>,
    #[account(
        constraint = stake_saber.vault_sunny_ata.owner == *vault_sunny.key,
        constraint = stake_saber.vault_sunny_ata.mint == vault_lp_saber.mint
    )]
    pub stake_saber: SunnyStakeInternal<'info>,
    #[account(
        constraint = stake_sunny.vault_sunny_ata.owner == *vault_sunny.key,
    )]
    pub stake_sunny: SunnyStakeInternal<'info>,
}

#[derive(Accounts)]
pub struct SunnyStakeInternal<'info> {
    // #[account(
    //     mut,
    //     associated_token::mint = vault_sunny_ata.mint,
    //     associated_token::authority = vault_sunny_ata.owner,
    // )]
    pub vault_sunny_ata: Box<Account<'info, TokenAccount>>,
    /// CHECK: TODO 
    pub rewarder: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub quarry: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub miner: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub miner_vault: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(
        seeds = [vault_account.to_account_info().key.as_ref()],
        bump = vault_account.bump
    )]
    /// CHECK: TODO 
    pub vault_signer: AccountInfo<'info>,
    pub vault_account: Account<'info, VaultAccount>,
    pub quarry_mine_program_id: Program<'info, QuarryMine>,
    #[account(
        constraint = user_stake.authority.key == vault_signer.key,
        constraint = user_stake.token_account.owner == *vault_signer.key
    )]
    pub user_stake: QuarryMineUserStake<'info>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(
        seeds = [vault_account.to_account_info().key.as_ref()],
        bump = vault_account.bump
    )]
    /// CHECK: TODO 
    pub vault_signer: AccountInfo<'info>,
    pub vault_account: Account<'info, VaultAccount>,
    pub quarry_mine_program_id: Program<'info, QuarryMine>,
    #[account(
        constraint = user_stake.authority.key == vault_signer.key,
        constraint = user_stake.token_account.owner == *vault_signer.key
    )]
    pub user_stake: QuarryMineUserStake<'info>,
}

#[derive(Accounts)]
pub struct QuarryMineUserStake<'info> {
    /// CHECK: TODO 
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub miner: Account<'info, Miner>,
    #[account(mut)]
    /// CHECK: TODO 
    pub quarry: AccountInfo<'info>,
    // #[account(
    //     mut,
    //     associated_token::mint = miner_vault.mint,
    //     associated_token::authority = miner.key(),
    // )]
    pub miner_vault: Account<'info, TokenAccount>,
    // #[account(
    //     mut,
    //     associated_token::mint = miner_vault.mint,
    //     associated_token::authority = authority,
    // )]
    pub token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    /// CHECK: TODO 
    pub rewarder: AccountInfo<'info>,
}

impl<'info> QuarryMineUserStake<'info> {
    pub fn to_user_stake_cpi_accounts(&self) -> quarry_mine::cpi::accounts::UserStake<'info> {
        quarry_mine::cpi::accounts::UserStake {
            authority: self.authority.to_account_info(),
            miner: self.miner.to_account_info(),
            quarry: self.quarry.to_account_info(),
            miner_vault: self.miner_vault.to_account_info(),
            token_account: self.token_account.to_account_info(),
            token_program: self.token_program.to_account_info(),
            rewarder: self.rewarder.to_account_info(),
        }
    }
}

#[derive(Accounts)]
pub struct SunnyClaimRewards<'info> {
    #[account(
        seeds = [vault_account.to_account_info().key.as_ref()],
        bump = vault_account.bump
    )]
    /// CHECK: TODO 
    pub vault_signer: AccountInfo<'info>,
    pub vault_account: Account<'info, VaultAccount>,
    #[account(mut)]
    /// CHECK: TODO 
    pub vault_sunny: AccountInfo<'info>,
    // #[account(
    //     mut,
    //     associated_token::mint = rewards_mint,
    //     associated_token::authority = vault_sunny,
    // )]
    pub vault_sunny_rewards_ata: Box<Account<'info, TokenAccount>>,
    // #[account(
    //     mut,
    //     associated_token::mint = vault_sunny_lp.mint,
    //     associated_token::authority = vault_sunny,
    // )]
    pub vault_sunny_lp: Box<Account<'info, TokenAccount>>,
    // #[account(
    //     mut,
    //     associated_token::mint = rewards_mint,
    //     associated_token::authority = vault_signer,
    // )]
    pub vault_signer_rewards: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// CHECK: TODO 
    pub vault_miner: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub vault_miner_ata: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub mint_wrapper: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub minter: AccountInfo<'info>,
    /// CHECK: TODO 
    pub rewarder: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub quarry: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub rewards_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub claim_fee: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub sunny_fee_destination_rewards: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub sunny_pool: AccountInfo<'info>,
    pub quarry_mine_program: Program<'info, QuarryMine>,
    pub quarry_mint_wrapper_program: Program<'info, QuarryMintWrapper>,
    #[account(constraint = sunny_program.key == &sunny::program::ID)]
    /// CHECK: TODO 
    pub sunny_program: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct ClaimRewardsSaber<'info> {
    #[account(
        seeds = [vault_account.to_account_info().key.as_ref()],
        bump = vault_account.bump
    )]
    /// CHECK: TODO 
    pub vault_signer: AccountInfo<'info>,
    pub vault_account: Account<'info, VaultAccount>,
    pub quarry_mine_program_id: Program<'info, QuarryMine>,
    #[account(
        constraint = claim_rewards.stake.authority.key == vault_signer.key,
        constraint = claim_rewards.rewards_token_account.owner == *vault_signer.key,
    )]
    pub claim_rewards: QuarryMineClaimRewards<'info>,
}

#[derive(Accounts)]
pub struct QuarryMineClaimRewards<'info> {
    #[account(mut)]
    /// CHECK: TODO 
    pub mint_wrapper: AccountInfo<'info>,
    /// CHECK: TODO 
    pub mint_wrapper_program: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub minter: AccountInfo<'info>,
    #[account(mut)]
    pub rewards_token_mint: Account<'info, Mint>,
    // #[account(
    //     mut,
    //     associated_token::mint = rewards_token_mint,
    //     associated_token::authority = stake.authority,
    // )]
    pub rewards_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    /// CHECK: TODO 
    pub claim_fee_token_account: AccountInfo<'info>,
    pub stake: QuarryMineUserClaim<'info>,
}

impl<'info> QuarryMineClaimRewards<'info> {
    pub fn to_cpi_accounts(&self) -> quarry_mine::cpi::accounts::ClaimRewards<'info> {
        quarry_mine::cpi::accounts::ClaimRewards {
            mint_wrapper: self.mint_wrapper.to_account_info(),
            mint_wrapper_program: self.mint_wrapper_program.to_account_info(),
            minter: self.minter.to_account_info(),
            rewards_token_mint: self.rewards_token_mint.to_account_info(),
            rewards_token_account: self.rewards_token_account.to_account_info(),
            claim_fee_token_account: self.claim_fee_token_account.to_account_info(),
            stake: self.stake.to_cpi_accounts(),
        }
    }
}

#[derive(Accounts)]
pub struct QuarryMineUserClaim<'info> {
    /// CHECK: TODO 
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub miner: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub quarry: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub unused_miner_vault: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub unused_token_account: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    /// CHECK: TODO 
    pub rewarder: AccountInfo<'info>,
}

impl<'info> QuarryMineUserClaim<'info> {
    pub fn to_cpi_accounts(&self) -> quarry_mine::cpi::accounts::UserClaim<'info> {
        quarry_mine::cpi::accounts::UserClaim {
            authority: self.authority.to_account_info(),
            miner: self.miner.to_account_info(),
            quarry: self.quarry.to_account_info(),
            unused_miner_vault: self.unused_miner_vault.to_account_info(),
            unused_token_account: self.unused_token_account.to_account_info(),
            token_program: self.token_program.to_account_info(),
            rewarder: self.rewarder.to_account_info(),
        }
    }
}

#[derive(Accounts)]
pub struct SunnyRedeem<'info> {
    // TODO split in two ixs? stack too large...
    #[account(
        seeds = [vault_account.to_account_info().key.as_ref()],
        bump = vault_account.bump
    )]
    /// CHECK: TODO 
    pub vault_signer: AccountInfo<'info>,
    pub vault_account: Box<Account<'info, VaultAccount>>,
    #[account(constraint = sunny_quarry_redeemer_program.key == &sunny::quarry_redeemer_program::ID)]
   /// CHECK: TODO 
    pub sunny_quarry_redeemer_program: AccountInfo<'info>,
    #[account(constraint = redeemer_program_id.key == &redeemer::ID)]
    /// CHECK: TODO 
    pub redeemer_program_id: AccountInfo<'info>,
    #[account(
        constraint = redeem_saber.redeem_ctx.source_authority.key ==  vault_signer.key,
        constraint = redeem_saber.redeem_ctx.iou_source.owner == *vault_signer.key,
        constraint = redeem_saber.redeem_ctx.redemption_destination.owner == *vault_signer.key,
    )]
    pub redeem_saber: RedeemerRedeemTokensFromMintProxy<'info>,
    #[account(
        constraint = redeem_sunny.redeem_ctx.source_authority.key ==  vault_signer.key,
        constraint = redeem_sunny.redeem_ctx.iou_source.owner == *vault_signer.key,
        constraint = redeem_sunny.redeem_ctx.redemption_destination.owner == *vault_signer.key,
    )]
    pub redeem_sunny: SunnyRedeemTokensFromMintWrapper<'info>,
    #[account(
        constraint = dao_treasury_saber_token_account.owner == Pubkey::from_str(ALLOWED_DEPLOYER).unwrap(),
    )]
    pub dao_treasury_saber_token_account: Box<Account<'info, TokenAccount>>,
    #[account(
        constraint = dao_treasury_sunny_token_account.owner == Pubkey::from_str(ALLOWED_DEPLOYER).unwrap(),
    )]
    pub dao_treasury_sunny_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: TODO 
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SaberRedeem<'info> {
    #[account(
        seeds = [vault_account.to_account_info().key.as_ref()],
        bump = vault_account.bump
    )]
    /// CHECK: TODO 
    pub vault_signer: AccountInfo<'info>,
    pub vault_account: Account<'info, VaultAccount>,
    #[account(constraint = redeemer_program_id.key == &redeemer::ID)]
    /// CHECK: TODO 
    pub redeemer_program_id: AccountInfo<'info>,
    #[account(
        constraint = redeem.redeem_ctx.source_authority.key ==  vault_signer.key,
        constraint = redeem.redeem_ctx.iou_source.owner == *vault_signer.key,
        constraint = redeem.redeem_ctx.redemption_destination.owner == *vault_signer.key,
    )]
    pub redeem: RedeemerRedeemTokensFromMintProxy<'info>,
}

#[derive(Accounts)]
pub struct RedeemerRedeemTokensFromMintProxy<'info> {
    pub redeem_ctx: RedeemerRedeemTokens<'info>,
    /// CHECK: TODO 
    pub mint_proxy_state: AccountInfo<'info>,
    /// CHECK: TODO 
    pub proxy_mint_authority: UncheckedAccount<'info>,
    /// CHECK: TODO 
    pub mint_proxy_program: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub minter_info: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SunnyRedeemTokensFromMintWrapper<'info> {
    pub redeem_ctx: RedeemerRedeemTokens<'info>,
    /// CHECK: TODO 
    pub mint_wrapper: AccountInfo<'info>,
    #[account(constraint = mint_wrapper_program.key == &sunny::mint_wrapper_program::ID)]
   /// CHECK: TODO 
   pub mint_wrapper_program: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub minter: AccountInfo<'info>,
}

impl<'info> RedeemerRedeemTokensFromMintProxy<'info> {
    pub fn to_cpi_accounts(&self) -> redeemer::cpi::accounts::RedeemTokensFromMintProxy<'info> {
        redeemer::cpi::accounts::RedeemTokensFromMintProxy {
            redeem_ctx: self.redeem_ctx.to_cpi_accounts(),
            mint_proxy_state: self.mint_proxy_state.to_account_info(),
            proxy_mint_authority: self.proxy_mint_authority.to_account_info(),
            mint_proxy_program: self.mint_proxy_program.to_account_info(),
            minter_info: self.minter_info.to_account_info(),
        }
    }
}

#[derive(Accounts)]
pub struct RedeemerRedeemTokens<'info> {
    /// CHECK: TODO 
    pub redeemer: AccountInfo<'info>,
    pub tokens: RedeemerMutTokenPair<'info>,
    /// CHECK: TODO 
    pub source_authority: AccountInfo<'info>,
    // #[account(
    //     mut,
    //     associated_token::mint = iou_source.mint,
    //     associated_token::authority = source_authority,
    // )]
    pub iou_source: Box<Account<'info, TokenAccount>>,
    // #[account(
    //     mut,
    //     associated_token::mint = redemption_destination.mint,
    //     associated_token::authority = source_authority,
    // )]
    pub redemption_destination: Box<Account<'info, TokenAccount>>,
}

impl<'info> RedeemerRedeemTokens<'info> {
    pub fn to_cpi_accounts(&self) -> redeemer::cpi::accounts::RedeemTokens<'info> {
        redeemer::cpi::accounts::RedeemTokens {
            redeemer: self.redeemer.to_account_info(),
            tokens: self.tokens.to_cpi_accounts(),
            source_authority: self.source_authority.to_account_info(),
            iou_source: self.iou_source.to_account_info(),
            redemption_destination: self.redemption_destination.to_account_info(),
        }
    }
}

#[derive(Accounts)]
pub struct RedeemerMutTokenPair<'info> {
    #[account(mut)]
    /// CHECK: TODO 
    pub iou_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub redemption_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub redemption_vault: AccountInfo<'info>,
    /// CHECK: TODO 
    pub token_program: AccountInfo<'info>,
}

impl<'info> RedeemerMutTokenPair<'info> {
    pub fn to_cpi_accounts(&self) -> redeemer::cpi::accounts::MutTokenPair<'info> {
        redeemer::cpi::accounts::MutTokenPair {
            iou_mint: self.iou_mint.to_account_info(),
            redemption_mint: self.redemption_mint.to_account_info(),
            redemption_vault: self.redemption_vault.to_account_info(),
            token_program: self.token_program.to_account_info(),
        }
    }
}

#[derive(Accounts)]
pub struct RaydiumSwap<'info> {
    #[account(
        seeds = [vault_account.to_account_info().key.as_ref()],
        bump = vault_account.bump
    )]
    /// CHECK: TODO 
    pub vault_signer: AccountInfo<'info>,
    pub vault_account: Account<'info, VaultAccount>,
    // #[account(
    //     mut,
    //     associated_token::mint = vault_input_token_account.mint,
    //     associated_token::authority = vault_signer,
    // )]
    pub vault_input_token_account: Box<Account<'info, TokenAccount>>,
    // #[account(
    //     mut,
    //     associated_token::mint = vault_account.input_mint_pubkey,
    //     associated_token::authority = vault_signer,
    // )]
    pub vault_output_token_account: Box<Account<'info, TokenAccount>>,
    #[account(constraint = raydium_amm_program_id.key == &swap::raydium_amm::ID)]
    /// CHECK: TODO 
    pub raydium_amm_program_id: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub amm_id: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub amm_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub amm_open_orders: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub amm_target_orders: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub amm_pool_coin_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub amm_pool_pc_token_account: AccountInfo<'info>,
    #[account(constraint = serum_program_id.key == &swap::serum::ID)]
    /// CHECK: TODO 
    pub serum_program_id: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub serum_market: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub serum_bids: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub serum_asks: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub serum_event_queue: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub serum_coin_vault_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: TODO 
    pub serum_pc_vault_account: AccountInfo<'info>,
    /// CHECK: TODO 
    pub serum_vault_signer: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

// TODO what should be stored here?
//  - serum swap pool
//  - quarry miner?
//  - rewards tokens?
#[account]
#[derive(Default)]
pub struct VaultAccount {
    /// PDA bump seed
    pub bump: u8,
    /// Strategy input token mint address
    pub input_mint_pubkey: Pubkey,
    /// Strategy LP token mint address
    pub vault_lp_token_mint_pubkey: Pubkey,
    /// Pool id from the stable swap program
    pub stable_swap_pool_id: Pubkey,
    /// Saber Destination fee account
    pub dao_treasury_saber_token_account: Pubkey,
    /// Sunny Destination fee account
    pub dao_treasury_sunny_token_account: Pubkey,
    /// Current TVL deposited in the strategy (considering deposits/withdraws)
    pub current_tvl: u64,
    /// Price of the LP token in the previous interval
    pub previous_lp_price: LpPrice,
}
