use crate::check_hash::*;
use crate::error::ErrorCode;
use crate::instructions::{
    protocol_deposit::*, protocol_initialize::*, protocol_rewards::*, protocol_withdraw::*,
};
use crate::macros::generate_seeds;
use crate::protocols::Protocols;
use crate::vault::{ProtocolData, VaultAccount};
use crate::VAULT_ACCOUNT_SEED;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    hash::{hashv, Hash},
    instruction::Instruction,
    program::invoke_signed,
    program_pack::Pack,
    pubkey::Pubkey,
    system_instruction,
};
use anchor_spl::token::{Token, TokenAccount};

/// Program id
pub mod solend_program_id {
    use anchor_lang::declare_id;
    #[cfg(feature = "devnet")]
    declare_id!("ALend7Ketfx5bxh6ghsCDXAoDrhvEmsXT3cynB6aPLgx");
    #[cfg(not(feature = "devnet"))]
    declare_id!("So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo");
}

#[derive(Accounts)]
pub struct SolendInitialize<'info> {
    pub user_signer: Signer<'info>,
    #[account(
        seeds = [VAULT_ACCOUNT_SEED, vault_account.input_mint_pubkey.as_ref()],
        bump = vault_account.bumps.vault
    )]
    pub vault_account: Box<Account<'info, VaultAccount>>,
    #[account(mut)]
    /// CHECK: Solend CPI
    pub vault_solend_obligation_account: AccountInfo<'info>,
    /// CHECK: Solend CPI
    pub solend_lending_market_account: AccountInfo<'info>,
    #[account(constraint = solend_program_id.key == &solend_program_id::ID)]
    /// CHECK: Solend CPI
    pub solend_program_id: AccountInfo<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> ProtocolInitialize<'info> for SolendInitialize<'info> {
    fn cpi_initialize(&self) -> Result<()> {
        let seeds = generate_seeds!(self.vault_account);
        let signer = &[&seeds[..]];

        {
            let account_size = solend_token_lending::state::Obligation::LEN;
            let ix = system_instruction::create_account_with_seed(
                self.user_signer.key,
                self.vault_solend_obligation_account.key,
                &self.vault_account.key(),
                &self.solend_lending_market_account.key.to_string()[..32],
                Rent::default().minimum_balance(account_size),
                account_size as u64,
                self.solend_program_id.key,
            );
            invoke_signed(
                &ix,
                &[
                    self.user_signer.to_account_info(),
                    self.vault_account.to_account_info(),
                    self.vault_solend_obligation_account.to_account_info(),
                ],
                signer,
            )?;
        }

        {
            let ix = solend_token_lending::instruction::init_obligation(
                solend_program_id::ID,
                *self.vault_solend_obligation_account.key,
                *self.solend_lending_market_account.key,
                self.vault_account.key(),
            );
            let accounts = [
                self.vault_solend_obligation_account.to_account_info(),
                self.solend_lending_market_account.to_account_info(),
                self.vault_account.to_account_info(),
                self.clock.to_account_info(),
                self.rent.to_account_info(),
                self.token_program.to_account_info(),
            ];
            invoke_signed(&ix, &accounts, signer)?;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SolendDeposit<'info> {
    pub generic_accs: GenericDepositAccounts<'info>,
    #[account(constraint = solend_program_id.key == &solend_program_id::ID)]
    /// CHECK: Solend CPI
    pub solend_program_id: AccountInfo<'info>,
    #[account(
        mut,
        associated_token::mint = vault_solend_destination_collateral_token_account.mint,
        associated_token::authority = generic_accs.vault_account,
    )]
    pub vault_solend_destination_collateral_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// CHECK: Solend CPI
    pub vault_solend_obligation_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Solend CPI
    pub solend_reserve_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Solend CPI
    pub solend_reserve_liquidity_supply_spl_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Solend CPI
    pub solend_reserve_collateral_spl_token_mint: AccountInfo<'info>,
    /// CHECK: Solend CPI
    pub solend_lending_market_account: AccountInfo<'info>,
    /// CHECK: Solend CPI
    pub solend_derived_lending_market_authority: AccountInfo<'info>,
    #[account(mut)]
    pub solend_destination_deposit_reserve_collateral_supply_spl_token_account:
        Account<'info, TokenAccount>,
    /// CHECK: Solend CPI
    pub solend_pyth_price_oracle_account: AccountInfo<'info>,
    /// CHECK: Solend CPI
    pub solend_switchboard_price_feed_oracle_account: AccountInfo<'info>,
}

impl<'info> CheckHash<'info> for SolendDeposit<'info> {
    fn hash(&self) -> Hash {
        hashv(&[
            self.vault_solend_destination_collateral_token_account
                .key()
                .as_ref(),
            self.vault_solend_obligation_account.key.as_ref(),
            self.solend_reserve_account.key.as_ref(),
            self.solend_reserve_liquidity_supply_spl_token_account
                .key
                .as_ref(),
            self.solend_reserve_collateral_spl_token_mint.key.as_ref(),
            self.solend_lending_market_account.key.as_ref(),
            self.solend_derived_lending_market_authority.key.as_ref(),
            self.solend_destination_deposit_reserve_collateral_supply_spl_token_account
                .key()
                .as_ref(),
            self.solend_pyth_price_oracle_account.key.as_ref(),
            self.solend_switchboard_price_feed_oracle_account
                .key
                .as_ref(),
        ])
    }

    fn target_hash(&self, protocol: Protocols) -> [u8; CHECKHASH_BYTES] {
        let protocol_pos = self
            .generic_accs
            .vault_account
            .protocol_position(protocol)
            .unwrap();
        self.generic_accs.vault_account.protocols[protocol_pos]
            .hash_pubkey
            .hash_deposit
    }
}

impl<'info> ProtocolDeposit<'info> for SolendDeposit<'info> {
    fn protocol_data_as_mut(&mut self, protocol_pos: usize) -> &mut ProtocolData {
        &mut self.generic_accs.vault_account.protocols[protocol_pos]
    }

    fn get_amount(&self, protocol_pos: usize) -> Result<u64> {
        self.generic_accs.amount_to_deposit(protocol_pos)
    }

    fn cpi_deposit(&self, amount: u64) -> Result<()> {
        let seeds = generate_seeds!(self.generic_accs.vault_account);
        let signer = &[&seeds[..]];

        let ix =
            solend_token_lending::instruction::deposit_reserve_liquidity_and_obligation_collateral(
                solend_program_id::ID,
                amount,
                self.generic_accs.vault_input_token_account.key(),
                self.vault_solend_destination_collateral_token_account.key(),
                *self.solend_reserve_account.key,
                *self.solend_reserve_liquidity_supply_spl_token_account.key,
                *self.solend_reserve_collateral_spl_token_mint.key,
                *self.solend_lending_market_account.key,
                self.solend_destination_deposit_reserve_collateral_supply_spl_token_account
                    .key(),
                *self.vault_solend_obligation_account.key,
                self.generic_accs.vault_account.key(),
                *self.solend_pyth_price_oracle_account.key,
                *self.solend_switchboard_price_feed_oracle_account.key,
                self.generic_accs.vault_account.key(),
            );
        let accounts = [
            self.generic_accs
                .vault_input_token_account
                .to_account_info(),
            self.vault_solend_destination_collateral_token_account
                .to_account_info(),
            self.solend_reserve_account.to_account_info(),
            self.solend_reserve_liquidity_supply_spl_token_account
                .to_account_info(),
            self.solend_reserve_collateral_spl_token_mint
                .to_account_info(),
            self.solend_lending_market_account.to_account_info(),
            self.solend_derived_lending_market_authority
                .to_account_info(),
            self.solend_destination_deposit_reserve_collateral_supply_spl_token_account
                .to_account_info(),
            self.vault_solend_obligation_account.to_account_info(),
            self.generic_accs.vault_account.to_account_info(),
            self.solend_pyth_price_oracle_account.to_account_info(),
            self.solend_switchboard_price_feed_oracle_account
                .to_account_info(),
            self.generic_accs.vault_account.to_account_info(),
            self.generic_accs.clock.to_account_info(),
            self.generic_accs.token_program.to_account_info(),
        ];
        invoke_signed(&ix, &accounts, signer)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SolendWithdraw<'info> {
    pub generic_accs: GenericWithdrawAccounts<'info>,
    #[account(constraint = solend_program_id.key == &solend_program_id::ID)]
    /// CHECK: Solend CPI
    pub solend_program_id: AccountInfo<'info>,
    #[account(
        mut,
        associated_token::mint = vault_solend_destination_collateral_token_account.mint,
        associated_token::authority = generic_accs.vault_account,
    )]
    pub vault_solend_destination_collateral_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    /// CHECK: Solend CPI
    pub vault_solend_obligation_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Solend CPI
    pub solend_source_withdraw_reserve_collateral_supply_spl_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Solend CPI
    pub solend_withdraw_reserve_account: AccountInfo<'info>,
    /// CHECK: Solend CPI
    pub solend_lending_market_account: AccountInfo<'info>,
    /// CHECK: Solend CPI
    pub solend_derived_lending_market_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Solend CPI
    pub solend_reserve_collateral_spl_token_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Solend CPI
    pub solend_reserve_liquidity_supply_spl_token_account: AccountInfo<'info>,
}

impl<'info> CheckHash<'info> for SolendWithdraw<'info> {
    fn hash(&self) -> Hash {
        hashv(&[
            self.vault_solend_destination_collateral_token_account
                .key()
                .as_ref(),
            self.vault_solend_obligation_account.key.as_ref(),
            self.solend_source_withdraw_reserve_collateral_supply_spl_token_account
                .key
                .as_ref(),
            self.solend_withdraw_reserve_account.key.as_ref(),
            self.solend_lending_market_account.key.as_ref(),
            self.solend_derived_lending_market_authority.key.as_ref(),
            self.solend_reserve_collateral_spl_token_mint.key.as_ref(),
            self.solend_reserve_liquidity_supply_spl_token_account
                .key
                .as_ref(),
        ])
    }

    fn target_hash(&self, protocol: Protocols) -> [u8; CHECKHASH_BYTES] {
        let protocol_pos = self
            .generic_accs
            .vault_account
            .protocol_position(protocol)
            .unwrap();
        self.generic_accs.vault_account.protocols[protocol_pos]
            .hash_pubkey
            .hash_withdraw
    }
}

impl<'info> ProtocolWithdraw<'info> for SolendWithdraw<'info> {
    fn protocol_data_as_mut(&mut self, protocol_pos: usize) -> &mut ProtocolData {
        &mut self.generic_accs.vault_account.protocols[protocol_pos]
    }

    fn input_token_account_as_mut(&mut self) -> &mut Account<'info, TokenAccount> {
        &mut self.generic_accs.vault_input_token_account
    }

    fn get_amount(&self, protocol_pos: usize) -> Result<u64> {
        self.generic_accs.amount_to_withdraw(protocol_pos)
    }

    fn liquidity_to_collateral(&self, amount: u64) -> Result<u64> {
        let reserve = solend_token_lending::state::Reserve::unpack(
            &self.solend_withdraw_reserve_account.data.borrow(),
        )?;
        let lp_amount = reserve
            .collateral_exchange_rate()?
            .liquidity_to_collateral(amount)?;
        Ok(lp_amount)
    }

    fn cpi_withdraw(&self, amount: u64) -> Result<()> {
        let seeds = generate_seeds!(self.generic_accs.vault_account);
        let signer = &[&seeds[..]];

        // Solend does not provide a function wrapper for
        // WithdrawObligationCollateralAndRedeemReserveCollateral ix
        let accounts = [
            self.solend_source_withdraw_reserve_collateral_supply_spl_token_account
                .to_account_info(),
            self.vault_solend_destination_collateral_token_account
                .to_account_info(),
            self.solend_withdraw_reserve_account.to_account_info(),
            self.vault_solend_obligation_account.to_account_info(),
            self.solend_lending_market_account.to_account_info(),
            self.solend_derived_lending_market_authority
                .to_account_info(),
            self.generic_accs
                .vault_input_token_account
                .to_account_info(),
            self.solend_reserve_collateral_spl_token_mint
                .to_account_info(),
            self.solend_reserve_liquidity_supply_spl_token_account
                .to_account_info(),
            self.generic_accs.vault_account.to_account_info(),
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

        use solend_token_lending::instruction::LendingInstruction;
        let ix = Instruction {
            program_id: solend_program_id::ID,
            accounts: account_metas,
            data: LendingInstruction::WithdrawObligationCollateralAndRedeemReserveCollateral {
                collateral_amount: amount,
            }
            .pack(),
        };
        invoke_signed(&ix, &accounts, signer)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SolendTVL<'info> {
    pub generic_accs: GenericTVLAccounts<'info>,
    #[account(owner = solend_program_id::ID)]
    /// CHECK: hash, owner and mint data field are checked
    pub reserve: AccountInfo<'info>,
    #[account(owner = solend_program_id::ID)]
    /// CHECK: hash, owner and reserve & owner fields are checked
    pub obligation: AccountInfo<'info>,
}

impl<'info> CheckHash<'info> for SolendTVL<'info> {
    fn hash(&self) -> Hash {
        hashv(&[self.reserve.key.as_ref(), self.obligation.key.as_ref()])
    }

    fn target_hash(&self, protocol: Protocols) -> [u8; CHECKHASH_BYTES] {
        let protocol_pos = self
            .generic_accs
            .vault_account
            .protocol_position(protocol)
            .unwrap();
        self.generic_accs.vault_account.protocols[protocol_pos]
            .hash_pubkey
            .hash_tvl
    }
}

impl<'info> ProtocolRewards<'info> for SolendTVL<'info> {
    fn input_mint_pubkey(&self) -> Pubkey {
        self.generic_accs.vault_account.input_mint_pubkey
    }

    fn protocol_data_as_mut(&mut self, protocol_pos: usize) -> &mut ProtocolData {
        &mut self.generic_accs.vault_account.protocols[protocol_pos]
    }

    fn max_withdrawable(&self) -> Result<u64> {
        let reserve = solend_token_lending::state::Reserve::unpack(&self.reserve.data.borrow())?;
        let obligation =
            solend_token_lending::state::Obligation::unpack(&self.obligation.data.borrow())?;

        require!(
            reserve.liquidity.mint_pubkey == self.generic_accs.vault_account.input_mint_pubkey,
            ErrorCode::InvalidMint
        );

        require!(
            obligation.owner == self.generic_accs.vault_account.key(),
            ErrorCode::InvalidObligationOwner
        );

        let tvl = if obligation.deposits.is_empty() {
            0
        } else {
            require!(
                obligation.deposits[0].deposit_reserve == *self.reserve.key,
                ErrorCode::InvalidObligationReserve
            );

            let lp_amount = obligation.deposits[0].deposited_amount;

            reserve
                .collateral_exchange_rate()?
                .collateral_to_liquidity(lp_amount)?
        };

        Ok(tvl)
    }
}
