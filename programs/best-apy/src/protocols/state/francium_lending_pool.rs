use crate::protocols::state::*;
use anchor_lang::solana_program::{
    msg,
    program_error::ProgramError,
    program_option::COption,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::{Pubkey, PUBKEY_BYTES},
};
use arrayref::{array_ref, array_refs};
use solana_maths::{Decimal, Rate, TryAdd, TryDiv, TryMul, WAD};
use std::convert::TryFrom;

const INITIAL_COLLATERAL_RATIO: u64 = 1;
const INITIAL_COLLATERAL_RATE: u64 = INITIAL_COLLATERAL_RATIO * WAD;

const PROGRAM_VERSION: u8 = 1;
const UNINITIALIZED_VERSION: u8 = 0;

const BTC_POOL_VERSION: u8 = 254;

/// Lending market reserve state
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LendingPool {
    /// Version of the struct
    pub version: u8,
    /// Last slot when supply and rates updated
    pub last_update: LastUpdate,
    /// Lending market address
    pub lending_market: Pubkey,
    /// Reserve liquidity
    pub liquidity: ReserveLiquidity,
    /// Reserve collateral
    pub share: ReserveCollateral,

    // Other fields from francium lending_pool layout
    pub credit_mint_pubkey: Pubkey,
    pub credit_mint_total_supply: u64,
    pub credit_supply_pubkey: Pubkey,
    pub threshold_1: u8,
    pub threshold_2: u8,
    pub base_1: u8,
    pub factor_1: u16,
    pub base_2: u8,
    pub factor_2: u16,
    pub base_3: u8,
    pub factor_3: u16,
    pub interest_reverse_rate: u8,
    pub accumulated_interest_reverse: u64,
}

impl LendingPool {
    /// Collateral exchange rate
    pub fn collateral_exchange_rate(&self) -> Result<CollateralExchangeRate, ProgramError> {
        let total_liquidity = self.liquidity.total_supply()?;
        self.share.exchange_rate(total_liquidity)
    }
}

/// Reserve liquidity
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ReserveLiquidity {
    /// Reserve liquidity mint address
    pub mint_pubkey: Pubkey,
    /// Reserve liquidity mint decimals
    pub mint_decimals: u8,
    /// Reserve liquidity supply address
    pub supply_pubkey: Pubkey,
    /// Reserve liquidity fee receiver address
    pub fee_receiver: Pubkey,
    /// Reserve liquidity oracle account
    pub oracle_pubkey: COption<Pubkey>,
    /// Reserve liquidity available
    pub available_amount: u64,
    /// Reserve liquidity borrowed
    pub borrowed_amount_wads: Decimal,
    /// Reserve liquidity cumulative borrow rate
    pub cumulative_borrow_rate_wads: Decimal,
    /// Reserve liquidity market price in quote currency
    pub market_price: u64,
}

impl ReserveLiquidity {
    /// Calculate the total reserve supply including active loans
    pub fn total_supply(&self) -> Result<Decimal, ProgramError> {
        Decimal::from(self.available_amount).try_add(self.borrowed_amount_wads)
    }
}

/// Reserve collateral
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ReserveCollateral {
    /// Reserve collateral mint address
    pub mint_pubkey: Pubkey,
    /// Reserve collateral mint supply, used for exchange rate
    pub mint_total_supply: u64,
    /// Reserve collateral supply address
    pub supply_pubkey: Pubkey,
}

impl ReserveCollateral {
    /// Return the current collateral exchange rate.
    fn exchange_rate(
        &self,
        total_liquidity: Decimal,
    ) -> Result<CollateralExchangeRate, ProgramError> {
        let rate = if self.mint_total_supply == 0 || total_liquidity == Decimal::zero() {
            Rate::from_scaled_val(INITIAL_COLLATERAL_RATE)
        } else {
            let mint_total_supply = Decimal::from(self.mint_total_supply);
            Rate::try_from(mint_total_supply.try_div(total_liquidity)?)?
        };

        Ok(CollateralExchangeRate(rate))
    }
}

/// Collateral exchange rate
#[derive(Clone, Copy, Debug)]
pub struct CollateralExchangeRate(Rate);

impl CollateralExchangeRate {
    /// Convert reserve collateral to liquidity
    pub fn collateral_to_liquidity(&self, collateral_amount: u64) -> Result<u64, ProgramError> {
        self.decimal_collateral_to_liquidity(collateral_amount.into())?
            .try_floor_u64()
    }

    /// Convert reserve collateral to liquidity
    pub fn decimal_collateral_to_liquidity(
        &self,
        collateral_amount: Decimal,
    ) -> Result<Decimal, ProgramError> {
        collateral_amount.try_div(self.0)
    }

    /// Convert reserve liquidity to collateral
    pub fn liquidity_to_collateral(&self, liquidity_amount: u64) -> Result<u64, ProgramError> {
        self.decimal_liquidity_to_collateral(liquidity_amount.into())?
            .try_floor_u64()
    }

    /// Convert reserve liquidity to collateral
    pub fn decimal_liquidity_to_collateral(
        &self,
        liquidity_amount: Decimal,
    ) -> Result<Decimal, ProgramError> {
        liquidity_amount.try_mul(self.0)
    }
}

impl Sealed for LendingPool {}
impl IsInitialized for LendingPool {
    fn is_initialized(&self) -> bool {
        self.version != UNINITIALIZED_VERSION
    }
}

const LENDING_POOL_LEN: usize = 495;
impl Pack for LendingPool {
    const LEN: usize = LENDING_POOL_LEN;
    fn pack_into_slice(&self, _output: &mut [u8]) {}

    /// Unpacks a byte buffer into a [ReserveInfo](struct.ReserveInfo.html).
    fn unpack_from_slice(input: &[u8]) -> Result<Self, ProgramError> {
        let input = array_ref![input, 0, LENDING_POOL_LEN];
        #[allow(clippy::ptr_offset_with_cast)]
        let (
            version,                               // 1
            last_update_slot,                      // 8
            last_update_stale,                     // 1
            lending_market,                        // 32
            liquidity_mint_pubkey,                 // 32
            liquidity_mint_decimals,               // 1
            liquidity_supply_pubkey,               // 32
            liquidity_fee_receiver,                // 32
            liquidity_oracle_pubkey,               // 36
            liquidity_available_amount,            // 8
            liquidity_borrowed_amount_wads,        // 16
            liquidity_cumulative_borrow_rate_wads, // 16
            liquidity_market_price,                // 8
            share_mint_pubkey,                     // 32
            share_mint_total_supply,               // 8
            share_supply_pubkey,                   // 32
            credit_mint_pubkey,                    // 32
            credit_mint_total_supply,              // 8
            credit_supply_pubkey,                  // 32
            threshold_1,                           // 1
            threshold_2,                           // 1
            base_1,                                // 1
            factor_1,                              // 2
            base_2,                                // 1
            factor_2,                              // 2
            base_3,                                // 1
            factor_3,                              // 2
            interest_reverse_rate,                 // 1
            accumulated_interest_reverse,          // 8
            _padding,                              // 108
        ) = array_refs![
            input,
            1,                // 1
            8,                // 8
            1,                // 1
            PUBKEY_BYTES,     // 32
            PUBKEY_BYTES,     // 32
            1,                // 1
            PUBKEY_BYTES,     // 32
            PUBKEY_BYTES,     // 32
            4 + PUBKEY_BYTES, // 36
            8,                // 8
            16,               // 16
            16,               // 16
            8,                // 8
            PUBKEY_BYTES,     // 32
            8,                // 8
            PUBKEY_BYTES,     // 32
            PUBKEY_BYTES,     // 32
            8,                // 8
            PUBKEY_BYTES,     // 32
            1,                // 1
            1,                // 1
            1,                // 1
            2,                // 2
            1,                // 1
            2,                // 2
            1,                // 1
            2,                // 2
            1,                // 1
            8,                // 8
            108               // 108
        ];

        let version = u8::from_le_bytes(*version);

        // dirty hack
        if version > PROGRAM_VERSION && version != BTC_POOL_VERSION {
            msg!("Francium LendingPool version does not match lending program version");
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(Self {
            version,
            last_update: LastUpdate {
                slot: u64::from_le_bytes(*last_update_slot),
                stale: unpack_bool(last_update_stale)?,
            },
            lending_market: Pubkey::new_from_array(*lending_market),
            liquidity: ReserveLiquidity {
                mint_pubkey: Pubkey::new_from_array(*liquidity_mint_pubkey),
                mint_decimals: u8::from_le_bytes(*liquidity_mint_decimals),
                supply_pubkey: Pubkey::new_from_array(*liquidity_supply_pubkey),
                fee_receiver: Pubkey::new_from_array(*liquidity_fee_receiver),
                oracle_pubkey: unpack_coption_key(liquidity_oracle_pubkey)?,
                available_amount: u64::from_le_bytes(*liquidity_available_amount),
                borrowed_amount_wads: unpack_decimal(liquidity_borrowed_amount_wads),
                cumulative_borrow_rate_wads: unpack_decimal(liquidity_cumulative_borrow_rate_wads),
                market_price: u64::from_le_bytes(*liquidity_market_price),
            },
            share: ReserveCollateral {
                mint_pubkey: Pubkey::new_from_array(*share_mint_pubkey),
                mint_total_supply: u64::from_le_bytes(*share_mint_total_supply),
                supply_pubkey: Pubkey::new_from_array(*share_supply_pubkey),
            },
            credit_mint_pubkey: Pubkey::new_from_array(*credit_mint_pubkey),
            credit_mint_total_supply: u64::from_le_bytes(*credit_mint_total_supply),
            credit_supply_pubkey: Pubkey::new_from_array(*credit_supply_pubkey),
            threshold_1: u8::from_le_bytes(*threshold_1),
            threshold_2: u8::from_le_bytes(*threshold_2),
            base_1: u8::from_le_bytes(*base_1),
            factor_1: u16::from_le_bytes(*factor_1),
            base_2: u8::from_le_bytes(*base_2),
            factor_2: u16::from_le_bytes(*factor_2),
            base_3: u8::from_le_bytes(*base_3),
            factor_3: u16::from_le_bytes(*factor_3),
            interest_reverse_rate: u8::from_le_bytes(*interest_reverse_rate),
            accumulated_interest_reverse: u64::from_le_bytes(*accumulated_interest_reverse),
        })
    }
}
