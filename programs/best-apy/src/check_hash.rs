use crate::error::ErrorCode;
use crate::protocols::Protocols;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::Hash;

/// Bytes of the truncated hash to be checked
#[constant]
pub const CHECKHASH_BYTES: usize = 16;

/// Trait to check the validity of a hash of the accounts passed
pub trait CheckHash<'info> {
    /// Hash to be checked
    fn hash(&self) -> Hash;

    /// Target truncated hash
    fn target_hash(&self) -> [u8; CHECKHASH_BYTES];

    /// Check the integrity of the hash
    fn check_hash(&self) -> Result<()> {
        let hash = &self.hash().to_bytes()[..CHECKHASH_BYTES];
        require!(hash == self.target_hash(), ErrorCode::InvalidHash);
        Ok(())
    }
}

/// Trait to check the validity of a hash of the accounts passed
pub trait CheckHashIsolatedPool<'info> {
    /// Hash to be checked
    fn hash(&self) -> Hash;

    /// Target truncated hash
    fn target_hash(&self, protocol: Protocols) -> [u8; CHECKHASH_BYTES];

    /// Check the integrity of the hash
    fn check_hash(&self, protocol: Protocols) -> Result<()> {
        let hash = &self.hash().to_bytes()[..CHECKHASH_BYTES];
        require!(hash == self.target_hash(protocol), ErrorCode::InvalidHash);
        Ok(())
    }
}
