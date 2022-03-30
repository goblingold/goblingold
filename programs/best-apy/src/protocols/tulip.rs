use crate::check_hash::*;
use crate::error::ErrorCode;
use crate::instructions::protocol_deposit::ProtocolDeposit;
use crate::instructions::protocol_rewards::ProtocolRewards;
use crate::macros::generate_seeds;
use crate::protocols::state::tulip_reserve;
use crate::protocols::Protocols;
use crate::{
    generic_accounts_anchor_modules::*, GenericDepositAccounts, GenericTVLAccounts,
    GenericWithdrawAccounts,
};
use anchor_lang::prelude::borsh::{BorshDeserialize, BorshSerialize};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    hash::{hashv, Hash},
    instruction::Instruction,
    program::invoke_signed,
    program_pack::Pack,
};
use anchor_spl::token::TokenAccount;

/// Program id
pub mod tulip_program_id {
    use anchor_lang::declare_id;
    declare_id!("4bcFeLv4nydFrsZqV5CgwCVrPhkQKsXtzfy2KyMz7ozM");
}

/// Deposit instruction data
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct InstructionDepositData {
    pub instruction: u8,
    pub liquidity_amount: u64,
}

/// Withdraw instruction data
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct InstructionWithdrawData {
    pub instruction: u8,
    pub collateral_amount: u64,
}

#[derive(Accounts)]
pub struct TulipDeposit<'info> {
    pub generic_accs: GenericDepositAccounts<'info>,
    #[account(constraint = tulip_program_id.key == &tulip_program_id::ID)]
    /// CHECK: Tulip CPI
    pub tulip_program_id: AccountInfo<'info>,
    #[account(
        mut,
        associated_token::mint = vault_tulip_collateral_token_account.mint,
        associated_token::authority = generic_accs.vault_account,
    )]
    pub vault_tulip_collateral_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    /// CHECK: Tulip CPI
    pub tulip_reserve_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Tulip CPI
    pub tulip_reserve_liquidity_supply_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Tulip CPI
    pub tulip_reserve_collateral_token_mint: AccountInfo<'info>,
    /// CHECK: Tulip CPI
    pub tulip_lending_market_account: AccountInfo<'info>,
    /// CHECK: Tulip CPI
    pub tulip_reserve_authority: AccountInfo<'info>,
}

impl<'info> CheckHash<'info> for TulipDeposit<'info> {
    fn hash(&self) -> Hash {
        hashv(&[
            self.vault_tulip_collateral_token_account.key().as_ref(),
            self.tulip_reserve_account.key.as_ref(),
            self.tulip_reserve_liquidity_supply_token_account
                .key
                .as_ref(),
            self.tulip_reserve_collateral_token_mint.key.as_ref(),
            self.tulip_lending_market_account.key.as_ref(),
            self.tulip_reserve_authority.key.as_ref(),
        ])
    }

    fn target_hash(&self) -> [u8; CHECKHASH_BYTES] {
        self.generic_accs.vault_account.protocols[Protocols::Tulip as usize]
            .hash_pubkey
            .hash_deposit
    }
}

impl<'info> ProtocolDeposit<'info> for TulipDeposit<'info> {
    fn protocol_data_as_mut(&mut self) -> &mut crate::vault::ProtocolData {
        &mut self.generic_accs.vault_account.protocols[Protocols::Tulip as usize]
    }

    fn get_amount(&self) -> Result<u64> {
        self.generic_accs.amount_to_deposit(Protocols::Tulip)
    }

    fn cpi_deposit(&self, amount: u64) -> Result<()> {
        let seeds = generate_seeds!(self.generic_accs.vault_account);
        let signer = &[&seeds[..]];

        // From Tulip team. Instruction #4
        //   0. `[writable]` Source liquidity token account. $authority can transfer $liquidity_amount.
        //   1. `[writable]` Destination collateral token account.
        //   2. `[writable]` Reserve account.
        //   3. `[writable]` Reserve liquidity supply SPL Token account.
        //   4. `[writable]` Reserve collateral SPL Token mint.
        //   5. `[]` Lending market account.
        //   6. `[]` Reserve authority.
        //   7 `[signer]` User transfer authority ($authority).
        //   8 `[]` Clock sysvar.
        //   9 `[]` Token program id.
        let accounts = [
            self.generic_accs
                .vault_input_token_account
                .to_account_info(),
            self.vault_tulip_collateral_token_account.to_account_info(),
            self.tulip_reserve_account.to_account_info(),
            self.tulip_reserve_liquidity_supply_token_account
                .to_account_info(),
            self.tulip_reserve_collateral_token_mint.to_account_info(),
            self.tulip_lending_market_account.to_account_info(),
            self.tulip_reserve_authority.to_account_info(),
            self.generic_accs.vault_account.to_account_info(),
            self.generic_accs.clock.to_account_info(),
            self.generic_accs.token_program.to_account_info(),
        ];
        let account_metas = accounts
            .iter()
            .map(|acc| {
                if acc.key == &self.generic_accs.vault_account.key() {
                    AccountMeta::new_readonly(*acc.key, true)
                } else if acc.is_writable {
                    AccountMeta::new(*acc.key, false)
                } else {
                    AccountMeta::new_readonly(*acc.key, false)
                }
            })
            .collect::<Vec<_>>();
        let ix = Instruction::new_with_borsh(
            tulip_program_id::ID,
            &InstructionDepositData {
                instruction: 4,
                liquidity_amount: amount,
            },
            account_metas,
        );
        invoke_signed(&ix, &accounts, signer)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TulipWithdraw<'info> {
    pub generic_accs: GenericWithdrawAccounts<'info>,
    #[account(constraint = tulip_program_id.key == &tulip_program_id::ID)]
    /// CHECK: Tulip CPI
    pub tulip_program_id: AccountInfo<'info>,
    #[account(
        mut,
        associated_token::mint = vault_tulip_collateral_token_account.mint,
        associated_token::authority = generic_accs.vault_account,
    )]
    pub vault_tulip_collateral_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    /// CHECK: Tulip CPI
    pub tulip_reserve_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Tulip CPI
    pub tulip_reserve_liquidity_supply_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Tulip CPI
    pub tulip_reserve_collateral_token_mint: AccountInfo<'info>,
    /// CHECK: Tulip CPI
    pub tulip_lending_market_account: AccountInfo<'info>,
    /// CHECK: Tulip CPI
    pub tulip_reserve_authority: AccountInfo<'info>,
}

impl<'info> TulipWithdraw<'info> {
    /// Convert reserve liquidity to collateral
    fn liquidity_to_collateral(&self, amount: u64) -> Result<u64> {
        let reserve = tulip_reserve::Reserve::unpack(&self.tulip_reserve_account.data.borrow())?;
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

        // Withdraw from tulip. Instruction #5
        //   0. `[writable]` Source withdraw reserve collateral supply SPL Token account.
        //   1. `[writable]` Destination collateral token account. Minted by withdraw reserve collateral mint.
        //   2. `[writable]*` Withdraw reserve account - refreshed.
        //   3. `[writable]` Reserve liquidity supply SPL Token account.
        //   4. `[writable]` Reserve collateral SPL Token mint.
        //   5. `[]` Lending market account.
        //   6. `[]` Reserve authority.
        //   7 `[signer]` User transfer authority ($authority).
        //   8. `[]` Clock sysvar.
        //   9. `[]` Token program id.
        let accounts = [
            self.vault_tulip_collateral_token_account.to_account_info(),
            self.generic_accs
                .vault_input_token_account
                .to_account_info(),
            self.tulip_reserve_account.to_account_info(),
            self.tulip_reserve_collateral_token_mint.to_account_info(),
            self.tulip_reserve_liquidity_supply_token_account
                .to_account_info(),
            self.tulip_lending_market_account.to_account_info(),
            self.tulip_reserve_authority.to_account_info(),
            self.generic_accs.vault_account.to_account_info(),
            self.generic_accs.clock.to_account_info(),
            self.generic_accs.token_program.to_account_info(),
        ];
        let account_metas = accounts
            .iter()
            .map(|acc| {
                if acc.key == &self.generic_accs.vault_account.key() {
                    AccountMeta::new_readonly(*acc.key, true)
                } else if acc.is_writable {
                    AccountMeta::new(*acc.key, false)
                } else {
                    AccountMeta::new_readonly(*acc.key, false)
                }
            })
            .collect::<Vec<_>>();

        let ix = Instruction::new_with_borsh(
            tulip_program_id::ID,
            &InstructionWithdrawData {
                instruction: 5,
                collateral_amount: amount,
            },
            account_metas,
        );
        invoke_signed(&ix, &accounts, signer)?;

        Ok(())
    }
}

impl<'info> CheckHash<'info> for TulipWithdraw<'info> {
    fn hash(&self) -> Hash {
        hashv(&[
            self.vault_tulip_collateral_token_account.key().as_ref(),
            self.tulip_reserve_account.key.as_ref(),
            self.tulip_reserve_liquidity_supply_token_account
                .key
                .as_ref(),
            self.tulip_reserve_collateral_token_mint.key.as_ref(),
            self.tulip_lending_market_account.key.as_ref(),
            self.tulip_reserve_authority.key.as_ref(),
        ])
    }

    fn target_hash(&self) -> [u8; CHECKHASH_BYTES] {
        self.generic_accs.vault_account.protocols[Protocols::Tulip as usize]
            .hash_pubkey
            .hash_withdraw
    }
}

/// Withdraw from the protocol
pub fn withdraw(ctx: Context<TulipWithdraw>) -> Result<()> {
    let amount = ctx
        .accounts
        .generic_accs
        .amount_to_withdraw(Protocols::Tulip)?;
    let amount_withdrawn = ctx.accounts.withdraw_and_get_balance(amount)?;

    ctx.accounts.generic_accs.vault_account.protocols[Protocols::Tulip as usize]
        .update_after_withdraw(amount_withdrawn)?;

    Ok(())
}

#[derive(Accounts)]
pub struct TulipTVL<'info> {
    pub generic_accs: GenericTVLAccounts<'info>,
    #[account(owner = tulip_program_id::ID)]
    /// CHECK: hash, owner and mint & collateral data fields are checked
    pub reserve: AccountInfo<'info>,
    #[account(
        mut,
        associated_token::mint = vault_tulip_collateral_token_account.mint,
        associated_token::authority = generic_accs.vault_account,
    )]
    pub vault_tulip_collateral_token_account: Account<'info, TokenAccount>,
}

impl<'info> CheckHash<'info> for TulipTVL<'info> {
    fn hash(&self) -> Hash {
        hashv(&[
            self.reserve.key.as_ref(),
            self.vault_tulip_collateral_token_account.key().as_ref(),
        ])
    }

    fn target_hash(&self) -> [u8; CHECKHASH_BYTES] {
        self.generic_accs.vault_account.protocols[Protocols::Tulip as usize]
            .hash_pubkey
            .hash_tvl
    }
}

impl<'info> ProtocolRewards<'info> for TulipTVL<'info> {
    fn protocol_data_as_mut(&mut self) -> &mut crate::vault::ProtocolData {
        &mut self.generic_accs.vault_account.protocols[Protocols::Tulip as usize]
    }

    fn max_withdrawable(&self) -> Result<u64> {
        let reserve = tulip_reserve::Reserve::unpack(&self.reserve.data.borrow())?;

        require!(
            reserve.liquidity.mint_pubkey == self.generic_accs.vault_account.input_mint_pubkey,
            ErrorCode::InvalidMint
        );

        require!(
            reserve.collateral.mint_pubkey == self.vault_tulip_collateral_token_account.key(),
            ErrorCode::InvalidMint
        );

        let lp_amount = self.vault_tulip_collateral_token_account.amount;
        let tvl = reserve
            .collateral_exchange_rate()?
            .collateral_to_liquidity(lp_amount)?;

        Ok(tvl)
    }
}
