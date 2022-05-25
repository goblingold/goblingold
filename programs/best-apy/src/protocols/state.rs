use anchor_lang::solana_program::{
    clock::Slot,
    msg,
    program_error::ProgramError,
    program_option::COption,
    pubkey::{Pubkey, PUBKEY_BYTES},
};
use arrayref::array_refs;
use solana_maths::Decimal;

pub mod francium_lending_pool;
pub mod tulip_reserve;

/// Last update state
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LastUpdate {
    /// Last slot when updated
    pub slot: Slot,
    /// True when marked stale, false when slot updated
    pub stale: bool,
}

// Helpers
pub fn unpack_decimal(src: &[u8; 16]) -> Decimal {
    Decimal::from_scaled_val(u128::from_le_bytes(*src))
}

pub fn unpack_bool(src: &[u8; 1]) -> Result<bool, ProgramError> {
    match u8::from_le_bytes(*src) {
        0 => Ok(false),
        1 => Ok(true),
        _ => {
            msg!("Boolean cannot be unpacked");
            Err(ProgramError::InvalidAccountData)
        }
    }
}

pub fn unpack_coption_key(src: &[u8; 4 + PUBKEY_BYTES]) -> Result<COption<Pubkey>, ProgramError> {
    let (tag, body) = array_refs![src, 4, 32];
    match *tag {
        [0, 0, 0, 0] => Ok(COption::None),
        [1, 0, 0, 0] => Ok(COption::Some(Pubkey::new_from_array(*body))),
        _ => Err(ProgramError::InvalidAccountData),
    }
}
