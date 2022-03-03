use anchor_lang::solana_program::{
    msg,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::{Pubkey, PUBKEY_BYTES},
};
use arrayref::{array_ref, array_refs};
use spl_token_lending::math::{Decimal, Rate, TryAdd, TryDiv, TryMul, TrySub, WAD};
use spl_token_lending::state::LastUpdate;
use std::convert::TryFrom;

const INITIAL_COLLATERAL_RATIO: u64 = 1;
const INITIAL_COLLATERAL_RATE: u64 = INITIAL_COLLATERAL_RATIO * WAD;

const PROGRAM_VERSION: u8 = 1;
const UNINITIALIZED_VERSION: u8 = 0;

/// Lending market reserve state
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Reserve {
    /// Version of the struct
    pub version: u8,
    /// Last slot when supply and rates updated
    pub last_update: LastUpdate,
    /// Lending market address
    pub lending_market: Pubkey,
    /// Reserve liquidity
    pub liquidity: ReserveLiquidity,
    /// Reserve collateral
    pub collateral: ReserveCollateral,

    // Field from tulip reserve layout
    pub borrow_authorizer: Pubkey,
}

impl Reserve {
    /// Collateral exchange rate
    pub fn collateral_exchange_rate(&self) -> Result<CollateralExchangeRate, ProgramError> {
        let total_liquidity = self.liquidity.total_supply()?;
        self.collateral.exchange_rate(total_liquidity)
    }
}

/// Reserve liquidity
#[derive(Clone, Debug, Default, PartialEq)]
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
    pub oracle_pubkey: Pubkey,
    /// Reserve liquidity available
    pub available_amount: u64,
    /// Reserve liquidity borrowed
    pub borrowed_amount_wads: Decimal,
    /// Reserve liquidity cumulative borrow rate
    pub cumulative_borrow_rate_wads: Decimal,
    /// Reserve liquidity market price in quote currency
    pub market_price: Decimal,

    // Other fields from tulip reserve layout
    pub platform_amount_wads: Decimal,
    pub platform_fees: u8,
}

impl ReserveLiquidity {
    /// Calculate the total reserve supply including active loans
    pub fn total_supply(&self) -> Result<Decimal, ProgramError> {
        // Tulip total_supply formula
        // https://github.com/cryptol0g1c/solfarm-lending/blob/dcb2eb9c314f1fbf8949a96b363b5bab1f883123/utils.js#L177
        Decimal::from(self.available_amount)
            .try_add(self.borrowed_amount_wads)?
            .try_sub(self.platform_amount_wads)
    }
}

/// Reserve collateral
#[derive(Clone, Debug, Default, PartialEq)]
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

impl Sealed for Reserve {}
impl IsInitialized for Reserve {
    fn is_initialized(&self) -> bool {
        self.version != UNINITIALIZED_VERSION
    }
}

const RESERVE_LEN: usize = 622;
impl Pack for Reserve {
    const LEN: usize = RESERVE_LEN;
    fn pack_into_slice(&self, _output: &mut [u8]) {}

    /// Unpacks a byte buffer into a [ReserveInfo](struct.ReserveInfo.html).
    fn unpack_from_slice(input: &[u8]) -> Result<Self, ProgramError> {
        let input = array_ref![input, 0, RESERVE_LEN];
        #[allow(clippy::ptr_offset_with_cast)]
        let (
            version,                               // 1
            last_update_slot,                      // 8
            last_update_stale,                     // 1
            lending_market,                        // 32
            borrow_authorizer,                     // 32
            liquidity_mint_pubkey,                 // 32
            liquidity_mint_decimals,               // 1
            liquidity_supply_pubkey,               // 32
            liquidity_fee_receiver,                // 32
            liquidity_oracle_pubkey,               // 32
            liquidity_available_amount,            // 8
            liquidity_borrowed_amount_wads,        // 16
            liquidity_cumulative_borrow_rate_wads, // 16
            liquidity_market_price,                // 16
            liquidity_platform_amount_wads,        // 16
            liquidity_platform_fees,               // 1
            collateral_mint_pubkey,                // 32
            collateral_mint_total_supply,          // 8
            collateral_supply_pubkey,              // 32
            _padding_and_other_fields,             // 274
        ) = array_refs![
            input,
            1,            // 1
            8,            // 8
            1,            // 1
            PUBKEY_BYTES, // 32
            PUBKEY_BYTES, // 32
            PUBKEY_BYTES, // 32
            1,            // 1
            PUBKEY_BYTES, // 32
            PUBKEY_BYTES, // 32
            PUBKEY_BYTES, // 32
            8,            // 8
            16,           // 16
            16,           // 16
            16,           // 16
            16,           // 16
            1,            // 1
            PUBKEY_BYTES, // 32
            8,            // 8
            PUBKEY_BYTES, // 32
            274           // 274
        ];

        let version = u8::from_le_bytes(*version);
        if version > PROGRAM_VERSION {
            msg!("TulipReserve version does not match lending program version");
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(Self {
            version,
            last_update: LastUpdate {
                slot: u64::from_le_bytes(*last_update_slot),
                stale: unpack_bool(last_update_stale)?,
            },
            lending_market: Pubkey::new_from_array(*lending_market),
            borrow_authorizer: Pubkey::new_from_array(*borrow_authorizer),
            liquidity: ReserveLiquidity {
                mint_pubkey: Pubkey::new_from_array(*liquidity_mint_pubkey),
                mint_decimals: u8::from_le_bytes(*liquidity_mint_decimals),
                supply_pubkey: Pubkey::new_from_array(*liquidity_supply_pubkey),
                fee_receiver: Pubkey::new_from_array(*liquidity_fee_receiver),
                oracle_pubkey: Pubkey::new_from_array(*liquidity_oracle_pubkey),
                available_amount: u64::from_le_bytes(*liquidity_available_amount),
                borrowed_amount_wads: unpack_decimal(liquidity_borrowed_amount_wads),
                cumulative_borrow_rate_wads: unpack_decimal(liquidity_cumulative_borrow_rate_wads),
                market_price: unpack_decimal(liquidity_market_price),
                platform_amount_wads: unpack_decimal(liquidity_platform_amount_wads),
                platform_fees: u8::from_le_bytes(*liquidity_platform_fees),
            },
            collateral: ReserveCollateral {
                mint_pubkey: Pubkey::new_from_array(*collateral_mint_pubkey),
                mint_total_supply: u64::from_le_bytes(*collateral_mint_total_supply),
                supply_pubkey: Pubkey::new_from_array(*collateral_supply_pubkey),
            },
        })
    }
}

// Helpers
fn unpack_decimal(src: &[u8; 16]) -> Decimal {
    Decimal::from_scaled_val(u128::from_le_bytes(*src))
}

fn unpack_bool(src: &[u8; 1]) -> Result<bool, ProgramError> {
    match u8::from_le_bytes(*src) {
        0 => Ok(false),
        1 => Ok(true),
        _ => {
            msg!("Boolean cannot be unpacked");
            Err(ProgramError::InvalidAccountData)
        }
    }
}
