use anchor_lang::solana_program::{
    msg,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::{Pubkey, PUBKEY_BYTES},
};
use arrayref::{array_ref, array_refs};

const PROGRAM_VERSION: u8 = 1;
const UNINITIALIZED_VERSION: u8 = 0;

/// Lending market reserve state
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FarmingUser {
    pub version: u8,
    pub staked_amount: u64,
    pub rewards_debt: u64,
    pub rewards_debt_b: u64,
    pub farming_pool: Pubkey,
    pub user_main: Pubkey,
    pub stake_token_account: Pubkey,
    pub rewards_token_accont: Pubkey,
    pub rewards_token_account_b: Pubkey,
}

impl Sealed for FarmingUser {}
impl IsInitialized for FarmingUser {
    fn is_initialized(&self) -> bool {
        self.version != UNINITIALIZED_VERSION
    }
}

const FARMING_USER_LEN: usize = 313;
impl Pack for FarmingUser {
    const LEN: usize = FARMING_USER_LEN;
    fn pack_into_slice(&self, _output: &mut [u8]) {}

    /// Unpacks a byte buffer into a [ReserveInfo](struct.ReserveInfo.html).
    fn unpack_from_slice(input: &[u8]) -> Result<Self, ProgramError> {
        let input = array_ref![input, 0, FARMING_USER_LEN];
        #[allow(clippy::ptr_offset_with_cast)]
        let (
            version,                 // 1
            staked_amount,           // 8
            rewards_debt,            // 8
            rewards_debt_b,          // 8
            farming_pool,            // 32
            user_main,               // 32
            stake_token_account,     // 32
            rewards_token_accont,    // 32
            rewards_token_account_b, // 32
            _padding,                // 128
        ) = array_refs![
            input,
            1,            // 1
            8,            // 8
            8,            // 8
            8,            // 8
            PUBKEY_BYTES, // 32
            PUBKEY_BYTES, // 32
            PUBKEY_BYTES, // 32
            PUBKEY_BYTES, // 32
            PUBKEY_BYTES, // 32
            128           // 128
        ];

        let version = u8::from_le_bytes(*version);
        if version > PROGRAM_VERSION {
            msg!("Francium FarmingUser version does not match lending program version");
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(Self {
            version,
            staked_amount: u64::from_le_bytes(*staked_amount),
            rewards_debt: u64::from_le_bytes(*rewards_debt),
            rewards_debt_b: u64::from_le_bytes(*rewards_debt_b),
            farming_pool: Pubkey::new_from_array(*farming_pool),
            user_main: Pubkey::new_from_array(*user_main),
            stake_token_account: Pubkey::new_from_array(*stake_token_account),
            rewards_token_accont: Pubkey::new_from_array(*rewards_token_accont),
            rewards_token_account_b: Pubkey::new_from_array(*rewards_token_account_b),
        })
    }
}
