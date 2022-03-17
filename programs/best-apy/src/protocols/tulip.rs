use crate::error::ErrorCode;
use crate::macros::generate_seeds;
use crate::protocols::tulip_reserve;
use crate::protocols::Protocols;
use crate::vault::TokenBalances;
use crate::PubkeyWrapper;
use crate::{
    generic_accounts_anchor_modules::*, GenericDepositAccounts, GenericTVLAccounts,
    GenericWithdrawAccounts,
};
use anchor_lang::prelude::borsh::{BorshDeserialize, BorshSerialize};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::Instruction, program::invoke_signed, program_pack::Pack,
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
        associated_token::mint = PubkeyWrapper(vault_tulip_collateral_token_account.mint),
        associated_token::authority = generic_accs.vault_signer,
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

impl<'info> TulipDeposit<'info> {
    /// Deposit into protocol
    pub fn deposit(&mut self) -> Result<()> {
        let amount = self.generic_accs.amount_to_deposit(Protocols::Tulip)?;
        let balances = self.deposit_and_get_balances(amount)?;

        self.generic_accs.vault_account.protocols[Protocols::Tulip as usize]
            .update_after_deposit(self.generic_accs.clock.slot, balances)?;

        Ok(())
    }

    /// Deposit into the protocol and get the true token balances
    fn deposit_and_get_balances(&mut self, amount: u64) -> Result<TokenBalances> {
        let lp_before = self.vault_tulip_collateral_token_account.amount;

        self.cpi_deposit(amount)?;
        self.vault_tulip_collateral_token_account.reload()?;

        let lp_after = self.vault_tulip_collateral_token_account.amount;
        let lp_amount = lp_after
            .checked_sub(lp_before)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

        Ok(TokenBalances {
            base_amount: amount,
            lp_amount,
        })
    }

    /// CPI deposit call
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
            self.generic_accs.vault_signer.to_account_info(),
            self.generic_accs.clock.to_account_info(),
            self.generic_accs.token_program.to_account_info(),
        ];
        let account_metas = accounts
            .iter()
            .map(|acc| {
                if acc.key == self.generic_accs.vault_signer.key {
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
        associated_token::mint = PubkeyWrapper(vault_tulip_collateral_token_account.mint),
        associated_token::authority = generic_accs.vault_signer,
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
    /// Withdraw from the protocol
    pub fn withdraw(&mut self) -> Result<()> {
        let amount = self.generic_accs.amount_to_withdraw(Protocols::Tulip)?;
        let balances = self.withdraw_and_get_balances(amount)?;

        self.generic_accs.vault_account.protocols[Protocols::Tulip as usize]
            .update_after_withdraw(self.generic_accs.clock.slot, balances)?;

        Ok(())
    }

    /// Convert reserve liquidity to collateral
    fn liquidity_to_collateral(&self, amount: u64) -> Result<u64> {
        let reserve = tulip_reserve::Reserve::unpack(&self.tulip_reserve_account.data.borrow())?;
        let lp_amount = reserve
            .collateral_exchange_rate()?
            .liquidity_to_collateral(amount)?;
        Ok(lp_amount)
    }

    /// Withdraw from the protocol and get the true token balances
    fn withdraw_and_get_balances(&mut self, amount: u64) -> Result<TokenBalances> {
        let lp_amount = self.liquidity_to_collateral(amount)?;
        let amount_before = self.generic_accs.vault_input_token_account.amount;

        self.cpi_withdraw(lp_amount)?;

        self.generic_accs.vault_input_token_account.reload()?;

        let amount_after = self.generic_accs.vault_input_token_account.amount;
        let amount_diff = amount_after
            .checked_sub(amount_before)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

        Ok(TokenBalances {
            base_amount: amount_diff,
            lp_amount,
        })
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
            self.generic_accs.vault_signer.to_account_info(),
            self.generic_accs.clock.to_account_info(),
            self.generic_accs.token_program.to_account_info(),
        ];
        let account_metas = accounts
            .iter()
            .map(|acc| {
                if acc.key == self.generic_accs.vault_signer.key {
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

#[derive(Accounts)]
pub struct TulipTVL<'info> {
    pub generic_accs: GenericTVLAccounts<'info>,
    /// CHECK: Tulip CPI
    pub reserve: AccountInfo<'info>,
}

impl<'info> TulipTVL<'info> {
    /// Update the protocol TVL
    pub fn update_rewards(&mut self) -> Result<()> {
        let slot = self.generic_accs.clock.slot;
        let tvl = self.max_withdrawable()?;

        let protocol = &mut self.generic_accs.vault_account.protocols[Protocols::Tulip as usize];
        msg!("Tulip TVL {} and base_amount {}", tvl, protocol.tokens.base_amount);
        let rewards = tvl
            .checked_sub(protocol.tokens.base_amount)
            .ok_or_else(|| error!(ErrorCode::MathOverflow))?;

        protocol.rewards.update(slot, rewards)?;

        Ok(())
    }

    /// Calculate the max native units to withdraw
    fn max_withdrawable(&self) -> Result<u64> {
        let protocol = self.generic_accs.vault_account.protocols[Protocols::Tulip as usize];
        self.lp_to_liquidity(protocol.tokens.lp_amount)
    }

    /// Convert reserve collateral to liquidity
    fn lp_to_liquidity(&self, lp_amount: u64) -> Result<u64> {
        let reserve = tulip_reserve::Reserve::unpack(&self.reserve.data.borrow())?;

        require!(
            reserve.liquidity.mint_pubkey == self.generic_accs.vault_account.input_mint_pubkey,
            ErrorCode::InvalidMint
        );

        let tvl = reserve
            .collateral_exchange_rate()?
            .collateral_to_liquidity(lp_amount)?;

        Ok(tvl)
    }
}
