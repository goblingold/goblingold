#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
use error::ErrorCode;
use check_hash::{CheckHash, CHECKHASH_BYTES};
use instructions::*;
use protocols::{francium::*, solend::*, Protocols};
use vault::{RefreshParams, VaultAccount};
mod check_hash {
    use crate::error::ErrorCode;
    use crate::protocols::Protocols;
    use anchor_lang::prelude::*;
    use anchor_lang::solana_program::hash::Hash;
    /// Bytes of the truncated hash to be checked
    pub const CHECKHASH_BYTES: usize = 16;
    /// Trait to check the validity of a hash of the accounts passed
    pub trait CheckHash<'info> {
        /// Hash to be checked
        fn hash(&self) -> Hash;
        /// Target truncated hash
        fn target_hash(&self, protocol: Protocols) -> [u8; CHECKHASH_BYTES];
        /// Check the integrity of the hash
        fn check_hash(&self, protocol: Protocols) -> Result<()> {
            let hash = &self.hash().to_bytes()[..CHECKHASH_BYTES];
            if !(hash == self.target_hash(protocol)) {
                return Err(anchor_lang::error::Error::from(
                    anchor_lang::error::AnchorError {
                        error_name: ErrorCode::InvalidHash.name(),
                        error_code_number: ErrorCode::InvalidHash.into(),
                        error_msg: ErrorCode::InvalidHash.to_string(),
                        error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                            anchor_lang::error::Source {
                                filename: "programs/lending-arb/src/check_hash.rs",
                                line: 21u32,
                            },
                        )),
                        compared_values: None,
                    },
                ));
            };
            Ok(())
        }
    }
}
mod error {
    use anchor_lang::prelude::*;
    #[repr(u32)]
    /// Error codes
    pub enum ErrorCode {
        InvalidZeroWithdraw,
        InvalidLpPrice,
        ForbiddenRefresh,
        StaleProtocolTVL,
        InvalidProtocolDeposit,
        InvalidProtocolWithdraw,
        InvalidDepositAmount,
        InvalidOwner,
        InvalidMint,
        OnPaused,
        InvalidInstructions,
        MathOverflow,
        InvalidWeights,
        InvalidHash,
        InvalidArraySize,
        InvalidObligationOwner,
        InvalidObligationReserve,
        UnauthorizedUser,
        ProtocolNotFoundInVault,
        ProtocolAlreadyExists,
        InvalidProtocolId,
        InvalidBorrow,
        InvalidReapy,
        UnhealthyOperation,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for ErrorCode {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&ErrorCode::InvalidZeroWithdraw,) => {
                    ::core::fmt::Formatter::write_str(f, "InvalidZeroWithdraw")
                }
                (&ErrorCode::InvalidLpPrice,) => {
                    ::core::fmt::Formatter::write_str(f, "InvalidLpPrice")
                }
                (&ErrorCode::ForbiddenRefresh,) => {
                    ::core::fmt::Formatter::write_str(f, "ForbiddenRefresh")
                }
                (&ErrorCode::StaleProtocolTVL,) => {
                    ::core::fmt::Formatter::write_str(f, "StaleProtocolTVL")
                }
                (&ErrorCode::InvalidProtocolDeposit,) => {
                    ::core::fmt::Formatter::write_str(f, "InvalidProtocolDeposit")
                }
                (&ErrorCode::InvalidProtocolWithdraw,) => {
                    ::core::fmt::Formatter::write_str(f, "InvalidProtocolWithdraw")
                }
                (&ErrorCode::InvalidDepositAmount,) => {
                    ::core::fmt::Formatter::write_str(f, "InvalidDepositAmount")
                }
                (&ErrorCode::InvalidOwner,) => ::core::fmt::Formatter::write_str(f, "InvalidOwner"),
                (&ErrorCode::InvalidMint,) => ::core::fmt::Formatter::write_str(f, "InvalidMint"),
                (&ErrorCode::OnPaused,) => ::core::fmt::Formatter::write_str(f, "OnPaused"),
                (&ErrorCode::InvalidInstructions,) => {
                    ::core::fmt::Formatter::write_str(f, "InvalidInstructions")
                }
                (&ErrorCode::MathOverflow,) => ::core::fmt::Formatter::write_str(f, "MathOverflow"),
                (&ErrorCode::InvalidWeights,) => {
                    ::core::fmt::Formatter::write_str(f, "InvalidWeights")
                }
                (&ErrorCode::InvalidHash,) => ::core::fmt::Formatter::write_str(f, "InvalidHash"),
                (&ErrorCode::InvalidArraySize,) => {
                    ::core::fmt::Formatter::write_str(f, "InvalidArraySize")
                }
                (&ErrorCode::InvalidObligationOwner,) => {
                    ::core::fmt::Formatter::write_str(f, "InvalidObligationOwner")
                }
                (&ErrorCode::InvalidObligationReserve,) => {
                    ::core::fmt::Formatter::write_str(f, "InvalidObligationReserve")
                }
                (&ErrorCode::UnauthorizedUser,) => {
                    ::core::fmt::Formatter::write_str(f, "UnauthorizedUser")
                }
                (&ErrorCode::ProtocolNotFoundInVault,) => {
                    ::core::fmt::Formatter::write_str(f, "ProtocolNotFoundInVault")
                }
                (&ErrorCode::ProtocolAlreadyExists,) => {
                    ::core::fmt::Formatter::write_str(f, "ProtocolAlreadyExists")
                }
                (&ErrorCode::InvalidProtocolId,) => {
                    ::core::fmt::Formatter::write_str(f, "InvalidProtocolId")
                }
                (&ErrorCode::InvalidBorrow,) => {
                    ::core::fmt::Formatter::write_str(f, "InvalidBorrow")
                }
                (&ErrorCode::InvalidReapy,) => ::core::fmt::Formatter::write_str(f, "InvalidReapy"),
                (&ErrorCode::UnhealthyOperation,) => {
                    ::core::fmt::Formatter::write_str(f, "UnhealthyOperation")
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for ErrorCode {
        #[inline]
        fn clone(&self) -> ErrorCode {
            {
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for ErrorCode {}
    impl ErrorCode {
        /// Gets the name of this [#enum_name].
        pub fn name(&self) -> String {
            match self {
                ErrorCode::InvalidZeroWithdraw => "InvalidZeroWithdraw".to_string(),
                ErrorCode::InvalidLpPrice => "InvalidLpPrice".to_string(),
                ErrorCode::ForbiddenRefresh => "ForbiddenRefresh".to_string(),
                ErrorCode::StaleProtocolTVL => "StaleProtocolTVL".to_string(),
                ErrorCode::InvalidProtocolDeposit => "InvalidProtocolDeposit".to_string(),
                ErrorCode::InvalidProtocolWithdraw => "InvalidProtocolWithdraw".to_string(),
                ErrorCode::InvalidDepositAmount => "InvalidDepositAmount".to_string(),
                ErrorCode::InvalidOwner => "InvalidOwner".to_string(),
                ErrorCode::InvalidMint => "InvalidMint".to_string(),
                ErrorCode::OnPaused => "OnPaused".to_string(),
                ErrorCode::InvalidInstructions => "InvalidInstructions".to_string(),
                ErrorCode::MathOverflow => "MathOverflow".to_string(),
                ErrorCode::InvalidWeights => "InvalidWeights".to_string(),
                ErrorCode::InvalidHash => "InvalidHash".to_string(),
                ErrorCode::InvalidArraySize => "InvalidArraySize".to_string(),
                ErrorCode::InvalidObligationOwner => "InvalidObligationOwner".to_string(),
                ErrorCode::InvalidObligationReserve => "InvalidObligationReserve".to_string(),
                ErrorCode::UnauthorizedUser => "UnauthorizedUser".to_string(),
                ErrorCode::ProtocolNotFoundInVault => "ProtocolNotFoundInVault".to_string(),
                ErrorCode::ProtocolAlreadyExists => "ProtocolAlreadyExists".to_string(),
                ErrorCode::InvalidProtocolId => "InvalidProtocolId".to_string(),
                ErrorCode::InvalidBorrow => "InvalidBorrow".to_string(),
                ErrorCode::InvalidReapy => "InvalidReapy".to_string(),
                ErrorCode::UnhealthyOperation => "UnhealthyOperation".to_string(),
            }
        }
    }
    impl From<ErrorCode> for u32 {
        fn from(e: ErrorCode) -> u32 {
            e as u32 + anchor_lang::error::ERROR_CODE_OFFSET
        }
    }
    impl From<ErrorCode> for anchor_lang::error::Error {
        fn from(error_code: ErrorCode) -> anchor_lang::error::Error {
            anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                error_name: error_code.name(),
                error_code_number: error_code.into(),
                error_msg: error_code.to_string(),
                error_origin: None,
                compared_values: None,
            })
        }
    }
    impl std::fmt::Display for ErrorCode {
        fn fmt(
            &self,
            fmt: &mut std::fmt::Formatter<'_>,
        ) -> std::result::Result<(), std::fmt::Error> {
            match self {
                ErrorCode::InvalidZeroWithdraw => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Invalid withdraw. Amount cannot be cero"],
                    &[],
                )),
                ErrorCode::InvalidLpPrice => {
                    fmt.write_fmt(::core::fmt::Arguments::new_v1(&["Invalid lp price"], &[]))
                }
                ErrorCode::ForbiddenRefresh => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Can not refresh weights yet, not enough slots since last refresh"],
                    &[],
                )),
                ErrorCode::StaleProtocolTVL => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                    &["A protocol TVL is stale and must be refreshed"],
                    &[],
                )),
                ErrorCode::InvalidProtocolDeposit => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Invalid amount to deposit"],
                    &[],
                )),
                ErrorCode::InvalidProtocolWithdraw => fmt.write_fmt(
                    ::core::fmt::Arguments::new_v1(&["Invalid amount to withdraw"], &[]),
                ),
                ErrorCode::InvalidDepositAmount => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Invalid deposited amount. Please deposit more than 100 lamports"],
                    &[],
                )),
                ErrorCode::InvalidOwner => {
                    fmt.write_fmt(::core::fmt::Arguments::new_v1(&["Invalid owner"], &[]))
                }
                ErrorCode::InvalidMint => {
                    fmt.write_fmt(::core::fmt::Arguments::new_v1(&["Invalid mint"], &[]))
                }
                ErrorCode::OnPaused => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Deposits and withdraws aren\'t allowed on PAUSE"],
                    &[],
                )),
                ErrorCode::InvalidInstructions => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                    &["The instructions provided are invalid"],
                    &[],
                )),
                ErrorCode::MathOverflow => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Math operation overflow"],
                    &[],
                )),
                ErrorCode::InvalidWeights => {
                    fmt.write_fmt(::core::fmt::Arguments::new_v1(&["Non-valid weights"], &[]))
                }
                ErrorCode::InvalidHash => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Invalid hash from provided pubkeys"],
                    &[],
                )),
                ErrorCode::InvalidArraySize => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Invalid size for array"],
                    &[],
                )),
                ErrorCode::InvalidObligationOwner => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Invalid obligation account owner"],
                    &[],
                )),
                ErrorCode::InvalidObligationReserve => fmt.write_fmt(
                    ::core::fmt::Arguments::new_v1(&["Invalid obligation reserve account"], &[]),
                ),
                ErrorCode::UnauthorizedUser => {
                    fmt.write_fmt(::core::fmt::Arguments::new_v1(&["Unauthorized user"], &[]))
                }
                ErrorCode::ProtocolNotFoundInVault => fmt.write_fmt(
                    ::core::fmt::Arguments::new_v1(&["Protocol not found in vault"], &[]),
                ),
                ErrorCode::ProtocolAlreadyExists => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Protocol already exists in vault"],
                    &[],
                )),
                ErrorCode::InvalidProtocolId => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Invalid protocol id"],
                    &[],
                )),
                ErrorCode::InvalidBorrow => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Due to health, borrow is not allowed"],
                    &[],
                )),
                ErrorCode::InvalidReapy => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Due to health, repay is not allowed"],
                    &[],
                )),
                ErrorCode::UnhealthyOperation => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                    &["The operation will remain with unhealthy account"],
                    &[],
                )),
            }
        }
    }
}
mod instructions {
    pub mod deposit {
        use crate::error::ErrorCode;
        use crate::macros::generate_seeds;
        use crate::vault::LpPrice;
        use crate::vault::VaultAccount;
        use crate::{VAULT_ACCOUNT_SEED, VAULT_LP_TOKEN_MINT_SEED};
        use anchor_lang::prelude::*;
        use anchor_lang::solana_program::{program_option::COption, pubkey::Pubkey};
        use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount, Transfer};
        pub struct Deposit<'info> {
            pub user_signer: Signer<'info>,
            # [account (mut , constraint = user_input_token_account . owner == * user_signer . key)]
            pub user_input_token_account: Account<'info, TokenAccount>,
            # [account (mut , constraint = user_lp_token_account . owner == * user_signer . key)]
            pub user_lp_token_account: Account<'info, TokenAccount>,
            # [account (mut , seeds = [VAULT_ACCOUNT_SEED , & [vault_account . seed_number] [..] , vault_account . input_mint_pubkey . as_ref ()] , bump = vault_account . bumps . vault)]
            pub vault_account: Box<Account<'info, VaultAccount>>,
            # [account (mut , constraint = vault_lp_token_mint_pubkey . mint_authority == COption :: Some (vault_account . key ()) , seeds = [VAULT_LP_TOKEN_MINT_SEED , vault_account . key () . as_ref ()] , bump = vault_account . bumps . lp_token_mint)]
            pub vault_lp_token_mint_pubkey: Account<'info, Mint>,
            # [account (mut , associated_token :: mint = vault_account . input_mint_pubkey , associated_token :: authority = vault_account ,)]
            pub vault_input_token_account: Account<'info, TokenAccount>,
            pub token_program: Program<'info, Token>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for Deposit<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
            ) -> anchor_lang::Result<Self> {
                let user_signer: Signer =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("user_signer"))?;
                let user_input_token_account: anchor_lang::accounts::account::Account<
                    TokenAccount,
                > = anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                    .map_err(|e| e.with_account_name("user_input_token_account"))?;
                let user_lp_token_account: anchor_lang::accounts::account::Account<TokenAccount> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("user_lp_token_account"))?;
                let vault_account: Box<anchor_lang::accounts::account::Account<VaultAccount>> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vault_account"))?;
                let vault_lp_token_mint_pubkey: anchor_lang::accounts::account::Account<Mint> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vault_lp_token_mint_pubkey"))?;
                let vault_input_token_account: anchor_lang::accounts::account::Account<
                    TokenAccount,
                > = anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                    .map_err(|e| e.with_account_name("vault_input_token_account"))?;
                let token_program: anchor_lang::accounts::program::Program<Token> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("token_program"))?;
                if !user_input_token_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("user_input_token_account"));
                }
                if !(user_input_token_account.owner == *user_signer.key) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("user_input_token_account"));
                }
                if !user_lp_token_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("user_lp_token_account"));
                }
                if !(user_lp_token_account.owner == *user_signer.key) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("user_lp_token_account"));
                }
                let __pda_address = Pubkey::create_program_address(
                    &[
                        VAULT_ACCOUNT_SEED,
                        &[vault_account.seed_number][..],
                        vault_account.input_mint_pubkey.as_ref(),
                        &[vault_account.bumps.vault][..],
                    ],
                    &program_id,
                )
                .map_err(|_| {
                    anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintSeeds)
                        .with_account_name("vault_account")
                })?;
                if vault_account.key() != __pda_address {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("vault_account")
                    .with_pubkeys((vault_account.key(), __pda_address)));
                }
                if !vault_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_account"));
                }
                let __pda_address = Pubkey::create_program_address(
                    &[
                        VAULT_LP_TOKEN_MINT_SEED,
                        vault_account.key().as_ref(),
                        &[vault_account.bumps.lp_token_mint][..],
                    ],
                    &program_id,
                )
                .map_err(|_| {
                    anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintSeeds)
                        .with_account_name("vault_lp_token_mint_pubkey")
                })?;
                if vault_lp_token_mint_pubkey.key() != __pda_address {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("vault_lp_token_mint_pubkey")
                    .with_pubkeys((vault_lp_token_mint_pubkey.key(), __pda_address)));
                }
                if !vault_lp_token_mint_pubkey.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_lp_token_mint_pubkey"));
                }
                if !(vault_lp_token_mint_pubkey.mint_authority
                    == COption::Some(vault_account.key()))
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("vault_lp_token_mint_pubkey"));
                }
                {
                    let my_owner = vault_input_token_account.owner;
                    let wallet_address = vault_account.key();
                    if my_owner != wallet_address {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintTokenOwner,
                        )
                        .with_account_name("vault_input_token_account")
                        .with_pubkeys((my_owner, wallet_address)));
                    }
                    let __associated_token_address =
                        anchor_spl::associated_token::get_associated_token_address(
                            &wallet_address,
                            &vault_account.input_mint_pubkey.key(),
                        );
                    let my_key = vault_input_token_account.key();
                    if my_key != __associated_token_address {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintAssociated,
                        )
                        .with_account_name("vault_input_token_account")
                        .with_pubkeys((my_key, __associated_token_address)));
                    }
                }
                if !vault_input_token_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_input_token_account"));
                }
                Ok(Deposit {
                    user_signer,
                    user_input_token_account,
                    user_lp_token_account,
                    vault_account,
                    vault_lp_token_mint_pubkey,
                    vault_input_token_account,
                    token_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for Deposit<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.user_signer.to_account_infos());
                account_infos.extend(self.user_input_token_account.to_account_infos());
                account_infos.extend(self.user_lp_token_account.to_account_infos());
                account_infos.extend(self.vault_account.to_account_infos());
                account_infos.extend(self.vault_lp_token_mint_pubkey.to_account_infos());
                account_infos.extend(self.vault_input_token_account.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for Deposit<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.user_signer.to_account_metas(None));
                account_metas.extend(self.user_input_token_account.to_account_metas(None));
                account_metas.extend(self.user_lp_token_account.to_account_metas(None));
                account_metas.extend(self.vault_account.to_account_metas(None));
                account_metas.extend(self.vault_lp_token_mint_pubkey.to_account_metas(None));
                account_metas.extend(self.vault_input_token_account.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for Deposit<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.user_input_token_account, program_id)
                    .map_err(|e| e.with_account_name("user_input_token_account"))?;
                anchor_lang::AccountsExit::exit(&self.user_lp_token_account, program_id)
                    .map_err(|e| e.with_account_name("user_lp_token_account"))?;
                anchor_lang::AccountsExit::exit(&self.vault_account, program_id)
                    .map_err(|e| e.with_account_name("vault_account"))?;
                anchor_lang::AccountsExit::exit(&self.vault_lp_token_mint_pubkey, program_id)
                    .map_err(|e| e.with_account_name("vault_lp_token_mint_pubkey"))?;
                anchor_lang::AccountsExit::exit(&self.vault_input_token_account, program_id)
                    .map_err(|e| e.with_account_name("vault_input_token_account"))?;
                Ok(())
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_deposit {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`Deposit`].
            pub struct Deposit {
                pub user_signer: anchor_lang::solana_program::pubkey::Pubkey,
                pub user_input_token_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub user_lp_token_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_lp_token_mint_pubkey: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_input_token_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for Deposit
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.user_signer, writer)?;
                    borsh::BorshSerialize::serialize(&self.user_input_token_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.user_lp_token_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.vault_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.vault_lp_token_mint_pubkey, writer)?;
                    borsh::BorshSerialize::serialize(&self.vault_input_token_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for Deposit {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.user_signer,
                            true,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.user_input_token_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.user_lp_token_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_lp_token_mint_pubkey,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_input_token_account,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_deposit {
            use super::*;
            /// Generated CPI struct of the accounts for [`Deposit`].
            pub struct Deposit<'info> {
                pub user_signer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub user_input_token_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub user_lp_token_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_lp_token_mint_pubkey:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_input_token_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for Deposit<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.user_signer),
                            true,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.user_input_token_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.user_lp_token_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_lp_token_mint_pubkey),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_input_token_account),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for Deposit<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.user_signer,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.user_input_token_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.user_lp_token_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_lp_token_mint_pubkey,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_input_token_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.token_program,
                    ));
                    account_infos
                }
            }
        }
        impl<'info> Deposit<'info> {
            fn current_lp_price(&self) -> LpPrice {
                LpPrice {
                    total_tokens: self.vault_account.current_tvl,
                    minted_tokens: self.vault_lp_token_mint_pubkey.supply,
                }
            }
            fn transfer_from_user_to_vault_ctx(
                &self,
            ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
                CpiContext::new(
                    self.token_program.to_account_info(),
                    Transfer {
                        from: self.user_input_token_account.to_account_info(),
                        to: self.vault_input_token_account.to_account_info(),
                        authority: self.user_signer.to_account_info(),
                    },
                )
            }
            fn mint_lp_to_user_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
                CpiContext::new(
                    self.token_program.to_account_info(),
                    MintTo {
                        mint: self.vault_lp_token_mint_pubkey.to_account_info(),
                        to: self.user_lp_token_account.to_account_info(),
                        authority: self.vault_account.to_account_info(),
                    },
                )
            }
        }
        /// Deposit user input tokens into the vault account
        pub fn handler(ctx: Context<Deposit>, amount: u64) -> Result<()> {
            let current_price = ctx.accounts.current_lp_price();
            let previous_price = ctx.accounts.vault_account.previous_lp_price;
            if previous_price != LpPrice::default() {
                if !(current_price >= previous_price) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::AnchorError {
                            error_name: ErrorCode::InvalidLpPrice.name(),
                            error_code_number: ErrorCode::InvalidLpPrice.into(),
                            error_msg: ErrorCode::InvalidLpPrice.to_string(),
                            error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                                anchor_lang::error::Source {
                                    filename: "programs/lending-arb/src/instructions/deposit.rs",
                                    line: 76u32,
                                },
                            )),
                            compared_values: None,
                        },
                    ));
                };
            }
            if !(amount >= 100) {
                return Err(anchor_lang::error::Error::from(
                    anchor_lang::error::AnchorError {
                        error_name: ErrorCode::InvalidDepositAmount.name(),
                        error_code_number: ErrorCode::InvalidDepositAmount.into(),
                        error_msg: ErrorCode::InvalidDepositAmount.to_string(),
                        error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                            anchor_lang::error::Source {
                                filename: "programs/lending-arb/src/instructions/deposit.rs",
                                line: 79u32,
                            },
                        )),
                        compared_values: None,
                    },
                ));
            };
            let lp_amount = current_price.token_to_lp(amount)?;
            let seeds = &[
                "vault".as_ref(),
                &[ctx.accounts.vault_account.seed_number][..],
                ctx.accounts.vault_account.input_mint_pubkey.as_ref(),
                &[ctx.accounts.vault_account.bumps.vault],
            ];
            let signer = &[&seeds[..]];
            token::transfer(ctx.accounts.transfer_from_user_to_vault_ctx(), amount)?;
            token::mint_to(
                ctx.accounts.mint_lp_to_user_ctx().with_signer(signer),
                lp_amount,
            )?;
            ctx.accounts.vault_account.current_tvl = ctx
                .accounts
                .vault_account
                .current_tvl
                .checked_add(amount)
                .ok_or_else(|| {
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                        error_name: ErrorCode::MathOverflow.name(),
                        error_code_number: ErrorCode::MathOverflow.into(),
                        error_msg: ErrorCode::MathOverflow.to_string(),
                        error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                            anchor_lang::error::Source {
                                filename: "programs/lending-arb/src/instructions/deposit.rs",
                                line: 98u32,
                            },
                        )),
                        compared_values: None,
                    })
                })?;
            Ok(())
        }
    }
    pub mod withdraw {
        use crate::error::ErrorCode;
        use crate::macros::generate_seeds;
        use crate::vault::{LpPrice, VaultAccount};
        use crate::{VAULT_ACCOUNT_SEED, VAULT_LP_TOKEN_MINT_SEED};
        use anchor_lang::prelude::*;
        use anchor_lang::solana_program::{program_option::COption, pubkey::Pubkey};
        use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount, Transfer};
        pub struct Withdraw<'info> {
            pub user_signer: Signer<'info>,
            # [account (mut , constraint = user_input_token_account . owner == * user_signer . key)]
            pub user_input_token_account: Account<'info, TokenAccount>,
            # [account (mut , constraint = user_lp_token_account . owner == * user_signer . key)]
            pub user_lp_token_account: Account<'info, TokenAccount>,
            # [account (mut , seeds = [VAULT_ACCOUNT_SEED , & [vault_account . seed_number] [..] , vault_account . input_mint_pubkey . as_ref ()] , bump = vault_account . bumps . vault)]
            pub vault_account: Box<Account<'info, VaultAccount>>,
            # [account (mut , constraint = vault_lp_token_mint_pubkey . mint_authority == COption :: Some (vault_account . key ()) , seeds = [VAULT_LP_TOKEN_MINT_SEED , vault_account . key () . as_ref ()] , bump = vault_account . bumps . lp_token_mint)]
            pub vault_lp_token_mint_pubkey: Account<'info, Mint>,
            # [account (mut , associated_token :: mint = vault_account . input_mint_pubkey , associated_token :: authority = vault_account ,)]
            pub vault_input_token_account: Account<'info, TokenAccount>,
            pub token_program: Program<'info, Token>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for Withdraw<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
            ) -> anchor_lang::Result<Self> {
                let user_signer: Signer =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("user_signer"))?;
                let user_input_token_account: anchor_lang::accounts::account::Account<
                    TokenAccount,
                > = anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                    .map_err(|e| e.with_account_name("user_input_token_account"))?;
                let user_lp_token_account: anchor_lang::accounts::account::Account<TokenAccount> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("user_lp_token_account"))?;
                let vault_account: Box<anchor_lang::accounts::account::Account<VaultAccount>> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vault_account"))?;
                let vault_lp_token_mint_pubkey: anchor_lang::accounts::account::Account<Mint> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vault_lp_token_mint_pubkey"))?;
                let vault_input_token_account: anchor_lang::accounts::account::Account<
                    TokenAccount,
                > = anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                    .map_err(|e| e.with_account_name("vault_input_token_account"))?;
                let token_program: anchor_lang::accounts::program::Program<Token> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("token_program"))?;
                if !user_input_token_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("user_input_token_account"));
                }
                if !(user_input_token_account.owner == *user_signer.key) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("user_input_token_account"));
                }
                if !user_lp_token_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("user_lp_token_account"));
                }
                if !(user_lp_token_account.owner == *user_signer.key) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("user_lp_token_account"));
                }
                let __pda_address = Pubkey::create_program_address(
                    &[
                        VAULT_ACCOUNT_SEED,
                        &[vault_account.seed_number][..],
                        vault_account.input_mint_pubkey.as_ref(),
                        &[vault_account.bumps.vault][..],
                    ],
                    &program_id,
                )
                .map_err(|_| {
                    anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintSeeds)
                        .with_account_name("vault_account")
                })?;
                if vault_account.key() != __pda_address {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("vault_account")
                    .with_pubkeys((vault_account.key(), __pda_address)));
                }
                if !vault_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_account"));
                }
                let __pda_address = Pubkey::create_program_address(
                    &[
                        VAULT_LP_TOKEN_MINT_SEED,
                        vault_account.key().as_ref(),
                        &[vault_account.bumps.lp_token_mint][..],
                    ],
                    &program_id,
                )
                .map_err(|_| {
                    anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintSeeds)
                        .with_account_name("vault_lp_token_mint_pubkey")
                })?;
                if vault_lp_token_mint_pubkey.key() != __pda_address {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("vault_lp_token_mint_pubkey")
                    .with_pubkeys((vault_lp_token_mint_pubkey.key(), __pda_address)));
                }
                if !vault_lp_token_mint_pubkey.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_lp_token_mint_pubkey"));
                }
                if !(vault_lp_token_mint_pubkey.mint_authority
                    == COption::Some(vault_account.key()))
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("vault_lp_token_mint_pubkey"));
                }
                {
                    let my_owner = vault_input_token_account.owner;
                    let wallet_address = vault_account.key();
                    if my_owner != wallet_address {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintTokenOwner,
                        )
                        .with_account_name("vault_input_token_account")
                        .with_pubkeys((my_owner, wallet_address)));
                    }
                    let __associated_token_address =
                        anchor_spl::associated_token::get_associated_token_address(
                            &wallet_address,
                            &vault_account.input_mint_pubkey.key(),
                        );
                    let my_key = vault_input_token_account.key();
                    if my_key != __associated_token_address {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintAssociated,
                        )
                        .with_account_name("vault_input_token_account")
                        .with_pubkeys((my_key, __associated_token_address)));
                    }
                }
                if !vault_input_token_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_input_token_account"));
                }
                Ok(Withdraw {
                    user_signer,
                    user_input_token_account,
                    user_lp_token_account,
                    vault_account,
                    vault_lp_token_mint_pubkey,
                    vault_input_token_account,
                    token_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for Withdraw<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.user_signer.to_account_infos());
                account_infos.extend(self.user_input_token_account.to_account_infos());
                account_infos.extend(self.user_lp_token_account.to_account_infos());
                account_infos.extend(self.vault_account.to_account_infos());
                account_infos.extend(self.vault_lp_token_mint_pubkey.to_account_infos());
                account_infos.extend(self.vault_input_token_account.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for Withdraw<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.user_signer.to_account_metas(None));
                account_metas.extend(self.user_input_token_account.to_account_metas(None));
                account_metas.extend(self.user_lp_token_account.to_account_metas(None));
                account_metas.extend(self.vault_account.to_account_metas(None));
                account_metas.extend(self.vault_lp_token_mint_pubkey.to_account_metas(None));
                account_metas.extend(self.vault_input_token_account.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for Withdraw<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.user_input_token_account, program_id)
                    .map_err(|e| e.with_account_name("user_input_token_account"))?;
                anchor_lang::AccountsExit::exit(&self.user_lp_token_account, program_id)
                    .map_err(|e| e.with_account_name("user_lp_token_account"))?;
                anchor_lang::AccountsExit::exit(&self.vault_account, program_id)
                    .map_err(|e| e.with_account_name("vault_account"))?;
                anchor_lang::AccountsExit::exit(&self.vault_lp_token_mint_pubkey, program_id)
                    .map_err(|e| e.with_account_name("vault_lp_token_mint_pubkey"))?;
                anchor_lang::AccountsExit::exit(&self.vault_input_token_account, program_id)
                    .map_err(|e| e.with_account_name("vault_input_token_account"))?;
                Ok(())
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_withdraw {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`Withdraw`].
            pub struct Withdraw {
                pub user_signer: anchor_lang::solana_program::pubkey::Pubkey,
                pub user_input_token_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub user_lp_token_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_lp_token_mint_pubkey: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_input_token_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for Withdraw
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.user_signer, writer)?;
                    borsh::BorshSerialize::serialize(&self.user_input_token_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.user_lp_token_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.vault_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.vault_lp_token_mint_pubkey, writer)?;
                    borsh::BorshSerialize::serialize(&self.vault_input_token_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for Withdraw {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.user_signer,
                            true,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.user_input_token_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.user_lp_token_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_lp_token_mint_pubkey,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_input_token_account,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_withdraw {
            use super::*;
            /// Generated CPI struct of the accounts for [`Withdraw`].
            pub struct Withdraw<'info> {
                pub user_signer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub user_input_token_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub user_lp_token_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_lp_token_mint_pubkey:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_input_token_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for Withdraw<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.user_signer),
                            true,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.user_input_token_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.user_lp_token_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_lp_token_mint_pubkey),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_input_token_account),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for Withdraw<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.user_signer,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.user_input_token_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.user_lp_token_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_lp_token_mint_pubkey,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_input_token_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.token_program,
                    ));
                    account_infos
                }
            }
        }
        impl<'info> Withdraw<'info> {
            fn current_lp_price(&self) -> LpPrice {
                LpPrice {
                    total_tokens: self.vault_account.current_tvl,
                    minted_tokens: self.vault_lp_token_mint_pubkey.supply,
                }
            }
            fn transfer_from_vault_to_user_ctx(
                &self,
            ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
                CpiContext::new(
                    self.token_program.to_account_info(),
                    Transfer {
                        from: self.vault_input_token_account.to_account_info(),
                        to: self.user_input_token_account.to_account_info(),
                        authority: self.vault_account.to_account_info(),
                    },
                )
            }
            fn burn_user_lps_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
                CpiContext::new(
                    self.token_program.to_account_info(),
                    Burn {
                        mint: self.vault_lp_token_mint_pubkey.to_account_info(),
                        to: self.user_lp_token_account.to_account_info(),
                        authority: self.user_signer.to_account_info(),
                    },
                )
            }
        }
        /// Withdraw the required input tokens from the vault and send them back to the user
        pub fn handler(ctx: Context<Withdraw>, lp_amount: u64) -> Result<()> {
            let current_price = ctx.accounts.current_lp_price();
            let previous_price = ctx.accounts.vault_account.previous_lp_price;
            if previous_price != LpPrice::default() {
                if !(current_price >= previous_price) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::AnchorError {
                            error_name: ErrorCode::InvalidLpPrice.name(),
                            error_code_number: ErrorCode::InvalidLpPrice.into(),
                            error_msg: ErrorCode::InvalidLpPrice.to_string(),
                            error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                                anchor_lang::error::Source {
                                    filename: "programs/lending-arb/src/instructions/withdraw.rs",
                                    line: 75u32,
                                },
                            )),
                            compared_values: None,
                        },
                    ));
                };
            }
            let amount = previous_price.lp_to_token(lp_amount)?;
            let amount_conservative = amount.saturating_sub(1);
            if !(amount_conservative > 1) {
                return Err(anchor_lang::error::Error::from(
                    anchor_lang::error::AnchorError {
                        error_name: ErrorCode::InvalidZeroWithdraw.name(),
                        error_code_number: ErrorCode::InvalidZeroWithdraw.into(),
                        error_msg: ErrorCode::InvalidZeroWithdraw.to_string(),
                        error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                            anchor_lang::error::Source {
                                filename: "programs/lending-arb/src/instructions/withdraw.rs",
                                line: 83u32,
                            },
                        )),
                        compared_values: None,
                    },
                ));
            };
            let seeds = &[
                "vault".as_ref(),
                &[ctx.accounts.vault_account.seed_number][..],
                ctx.accounts.vault_account.input_mint_pubkey.as_ref(),
                &[ctx.accounts.vault_account.bumps.vault],
            ];
            let signer = &[&seeds[..]];
            token::burn(ctx.accounts.burn_user_lps_ctx(), lp_amount)?;
            token::transfer(
                ctx.accounts
                    .transfer_from_vault_to_user_ctx()
                    .with_signer(signer),
                amount_conservative,
            )?;
            ctx.accounts.vault_account.current_tvl = ctx
                .accounts
                .vault_account
                .current_tvl
                .checked_sub(amount)
                .ok_or_else(|| {
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                        error_name: ErrorCode::MathOverflow.name(),
                        error_code_number: ErrorCode::MathOverflow.into(),
                        error_msg: ErrorCode::MathOverflow.to_string(),
                        error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                            anchor_lang::error::Source {
                                filename: "programs/lending-arb/src/instructions/withdraw.rs",
                                line: 102u32,
                            },
                        )),
                        compared_values: None,
                    })
                })?;
            Ok(())
        }
    }
    pub mod close_withdraw_ticket {
        use crate::error::ErrorCode;
        use crate::macros::generate_seeds;
        use crate::vault::{LpPrice, VaultAccount};
        use crate::{VAULT_ACCOUNT_SEED, VAULT_LP_TOKEN_MINT_SEED, VAULT_TICKET_MINT_SEED};
        use anchor_lang::prelude::*;
        use anchor_lang::solana_program::{program_option::COption, pubkey::Pubkey};
        use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount, Transfer};
        # [instruction (bump_user : u8)]
        pub struct CloseWithdrawTicket<'info> {
            pub user_signer: Signer<'info>,
            # [account (mut , constraint = user_input_token_account . owner == * user_signer . key)]
            pub user_input_token_account: Account<'info, TokenAccount>,
            # [account (mut , constraint = vault_user_ticket_account . owner == vault_account . key () , seeds = [VAULT_TICKET_MINT_SEED , vault_ticket_mint_pubkey . key () . as_ref () , user_signer . key () . as_ref ()] , bump = bump_user)]
            pub vault_user_ticket_account: Account<'info, TokenAccount>,
            # [account (mut , seeds = [VAULT_ACCOUNT_SEED , & [vault_account . seed_number] [..] , vault_account . input_mint_pubkey . as_ref ()] , bump = vault_account . bumps . vault)]
            pub vault_account: Box<Account<'info, VaultAccount>>,
            # [account (mut , constraint = vault_lp_token_mint_pubkey . mint_authority == COption :: Some (vault_account . key ()) , seeds = [VAULT_LP_TOKEN_MINT_SEED , vault_account . key () . as_ref ()] , bump = vault_account . bumps . lp_token_mint)]
            pub vault_lp_token_mint_pubkey: Account<'info, Mint>,
            # [account (mut , constraint = vault_ticket_mint_pubkey . mint_authority == COption :: Some (vault_account . key ()) , seeds = [VAULT_TICKET_MINT_SEED , vault_account . key () . as_ref ()] , bump = vault_account . bumps . ticket_mint)]
            pub vault_ticket_mint_pubkey: Account<'info, Mint>,
            # [account (mut , associated_token :: mint = vault_account . input_mint_pubkey , associated_token :: authority = vault_account ,)]
            pub vault_input_token_account: Account<'info, TokenAccount>,
            pub token_program: Program<'info, Token>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for CloseWithdrawTicket<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
            ) -> anchor_lang::Result<Self> {
                let mut ix_data = ix_data;
                struct __Args {
                    bump_user: u8,
                }
                impl borsh::ser::BorshSerialize for __Args
                where
                    u8: borsh::ser::BorshSerialize,
                {
                    fn serialize<W: borsh::maybestd::io::Write>(
                        &self,
                        writer: &mut W,
                    ) -> ::core::result::Result<(), borsh::maybestd::io::Error>
                    {
                        borsh::BorshSerialize::serialize(&self.bump_user, writer)?;
                        Ok(())
                    }
                }
                impl borsh::de::BorshDeserialize for __Args
                where
                    u8: borsh::BorshDeserialize,
                {
                    fn deserialize(
                        buf: &mut &[u8],
                    ) -> ::core::result::Result<Self, borsh::maybestd::io::Error>
                    {
                        Ok(Self {
                            bump_user: borsh::BorshDeserialize::deserialize(buf)?,
                        })
                    }
                }
                let __Args { bump_user } = __Args::deserialize(&mut ix_data)
                    .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
                let user_signer: Signer =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("user_signer"))?;
                let user_input_token_account: anchor_lang::accounts::account::Account<
                    TokenAccount,
                > = anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                    .map_err(|e| e.with_account_name("user_input_token_account"))?;
                let vault_user_ticket_account: anchor_lang::accounts::account::Account<
                    TokenAccount,
                > = anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                    .map_err(|e| e.with_account_name("vault_user_ticket_account"))?;
                let vault_account: Box<anchor_lang::accounts::account::Account<VaultAccount>> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vault_account"))?;
                let vault_lp_token_mint_pubkey: anchor_lang::accounts::account::Account<Mint> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vault_lp_token_mint_pubkey"))?;
                let vault_ticket_mint_pubkey: anchor_lang::accounts::account::Account<Mint> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vault_ticket_mint_pubkey"))?;
                let vault_input_token_account: anchor_lang::accounts::account::Account<
                    TokenAccount,
                > = anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                    .map_err(|e| e.with_account_name("vault_input_token_account"))?;
                let token_program: anchor_lang::accounts::program::Program<Token> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("token_program"))?;
                if !user_input_token_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("user_input_token_account"));
                }
                if !(user_input_token_account.owner == *user_signer.key) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("user_input_token_account"));
                }
                let __pda_address = Pubkey::create_program_address(
                    &[
                        VAULT_TICKET_MINT_SEED,
                        vault_ticket_mint_pubkey.key().as_ref(),
                        user_signer.key().as_ref(),
                        &[bump_user][..],
                    ],
                    &program_id,
                )
                .map_err(|_| {
                    anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintSeeds)
                        .with_account_name("vault_user_ticket_account")
                })?;
                if vault_user_ticket_account.key() != __pda_address {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("vault_user_ticket_account")
                    .with_pubkeys((vault_user_ticket_account.key(), __pda_address)));
                }
                if !vault_user_ticket_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_user_ticket_account"));
                }
                if !(vault_user_ticket_account.owner == vault_account.key()) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("vault_user_ticket_account"));
                }
                let __pda_address = Pubkey::create_program_address(
                    &[
                        VAULT_ACCOUNT_SEED,
                        &[vault_account.seed_number][..],
                        vault_account.input_mint_pubkey.as_ref(),
                        &[vault_account.bumps.vault][..],
                    ],
                    &program_id,
                )
                .map_err(|_| {
                    anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintSeeds)
                        .with_account_name("vault_account")
                })?;
                if vault_account.key() != __pda_address {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("vault_account")
                    .with_pubkeys((vault_account.key(), __pda_address)));
                }
                if !vault_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_account"));
                }
                let __pda_address = Pubkey::create_program_address(
                    &[
                        VAULT_LP_TOKEN_MINT_SEED,
                        vault_account.key().as_ref(),
                        &[vault_account.bumps.lp_token_mint][..],
                    ],
                    &program_id,
                )
                .map_err(|_| {
                    anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintSeeds)
                        .with_account_name("vault_lp_token_mint_pubkey")
                })?;
                if vault_lp_token_mint_pubkey.key() != __pda_address {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("vault_lp_token_mint_pubkey")
                    .with_pubkeys((vault_lp_token_mint_pubkey.key(), __pda_address)));
                }
                if !vault_lp_token_mint_pubkey.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_lp_token_mint_pubkey"));
                }
                if !(vault_lp_token_mint_pubkey.mint_authority
                    == COption::Some(vault_account.key()))
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("vault_lp_token_mint_pubkey"));
                }
                let __pda_address = Pubkey::create_program_address(
                    &[
                        VAULT_TICKET_MINT_SEED,
                        vault_account.key().as_ref(),
                        &[vault_account.bumps.ticket_mint][..],
                    ],
                    &program_id,
                )
                .map_err(|_| {
                    anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintSeeds)
                        .with_account_name("vault_ticket_mint_pubkey")
                })?;
                if vault_ticket_mint_pubkey.key() != __pda_address {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("vault_ticket_mint_pubkey")
                    .with_pubkeys((vault_ticket_mint_pubkey.key(), __pda_address)));
                }
                if !vault_ticket_mint_pubkey.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_ticket_mint_pubkey"));
                }
                if !(vault_ticket_mint_pubkey.mint_authority == COption::Some(vault_account.key()))
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("vault_ticket_mint_pubkey"));
                }
                {
                    let my_owner = vault_input_token_account.owner;
                    let wallet_address = vault_account.key();
                    if my_owner != wallet_address {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintTokenOwner,
                        )
                        .with_account_name("vault_input_token_account")
                        .with_pubkeys((my_owner, wallet_address)));
                    }
                    let __associated_token_address =
                        anchor_spl::associated_token::get_associated_token_address(
                            &wallet_address,
                            &vault_account.input_mint_pubkey.key(),
                        );
                    let my_key = vault_input_token_account.key();
                    if my_key != __associated_token_address {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintAssociated,
                        )
                        .with_account_name("vault_input_token_account")
                        .with_pubkeys((my_key, __associated_token_address)));
                    }
                }
                if !vault_input_token_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_input_token_account"));
                }
                Ok(CloseWithdrawTicket {
                    user_signer,
                    user_input_token_account,
                    vault_user_ticket_account,
                    vault_account,
                    vault_lp_token_mint_pubkey,
                    vault_ticket_mint_pubkey,
                    vault_input_token_account,
                    token_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for CloseWithdrawTicket<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.user_signer.to_account_infos());
                account_infos.extend(self.user_input_token_account.to_account_infos());
                account_infos.extend(self.vault_user_ticket_account.to_account_infos());
                account_infos.extend(self.vault_account.to_account_infos());
                account_infos.extend(self.vault_lp_token_mint_pubkey.to_account_infos());
                account_infos.extend(self.vault_ticket_mint_pubkey.to_account_infos());
                account_infos.extend(self.vault_input_token_account.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for CloseWithdrawTicket<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.user_signer.to_account_metas(None));
                account_metas.extend(self.user_input_token_account.to_account_metas(None));
                account_metas.extend(self.vault_user_ticket_account.to_account_metas(None));
                account_metas.extend(self.vault_account.to_account_metas(None));
                account_metas.extend(self.vault_lp_token_mint_pubkey.to_account_metas(None));
                account_metas.extend(self.vault_ticket_mint_pubkey.to_account_metas(None));
                account_metas.extend(self.vault_input_token_account.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for CloseWithdrawTicket<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.user_input_token_account, program_id)
                    .map_err(|e| e.with_account_name("user_input_token_account"))?;
                anchor_lang::AccountsExit::exit(&self.vault_user_ticket_account, program_id)
                    .map_err(|e| e.with_account_name("vault_user_ticket_account"))?;
                anchor_lang::AccountsExit::exit(&self.vault_account, program_id)
                    .map_err(|e| e.with_account_name("vault_account"))?;
                anchor_lang::AccountsExit::exit(&self.vault_lp_token_mint_pubkey, program_id)
                    .map_err(|e| e.with_account_name("vault_lp_token_mint_pubkey"))?;
                anchor_lang::AccountsExit::exit(&self.vault_ticket_mint_pubkey, program_id)
                    .map_err(|e| e.with_account_name("vault_ticket_mint_pubkey"))?;
                anchor_lang::AccountsExit::exit(&self.vault_input_token_account, program_id)
                    .map_err(|e| e.with_account_name("vault_input_token_account"))?;
                Ok(())
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_close_withdraw_ticket {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`CloseWithdrawTicket`].
            pub struct CloseWithdrawTicket {
                pub user_signer: anchor_lang::solana_program::pubkey::Pubkey,
                pub user_input_token_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_user_ticket_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_lp_token_mint_pubkey: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_ticket_mint_pubkey: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_input_token_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for CloseWithdrawTicket
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.user_signer, writer)?;
                    borsh::BorshSerialize::serialize(&self.user_input_token_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.vault_user_ticket_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.vault_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.vault_lp_token_mint_pubkey, writer)?;
                    borsh::BorshSerialize::serialize(&self.vault_ticket_mint_pubkey, writer)?;
                    borsh::BorshSerialize::serialize(&self.vault_input_token_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for CloseWithdrawTicket {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.user_signer,
                            true,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.user_input_token_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_user_ticket_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_lp_token_mint_pubkey,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_ticket_mint_pubkey,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_input_token_account,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_close_withdraw_ticket {
            use super::*;
            /// Generated CPI struct of the accounts for [`CloseWithdrawTicket`].
            pub struct CloseWithdrawTicket<'info> {
                pub user_signer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub user_input_token_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_user_ticket_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_lp_token_mint_pubkey:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_ticket_mint_pubkey:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_input_token_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for CloseWithdrawTicket<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.user_signer),
                            true,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.user_input_token_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_user_ticket_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_lp_token_mint_pubkey),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_ticket_mint_pubkey),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_input_token_account),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for CloseWithdrawTicket<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.user_signer,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.user_input_token_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_user_ticket_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_lp_token_mint_pubkey,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_ticket_mint_pubkey,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_input_token_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.token_program,
                    ));
                    account_infos
                }
            }
        }
        impl<'info> CloseWithdrawTicket<'info> {
            fn current_lp_price(&self) -> LpPrice {
                LpPrice {
                    total_tokens: self.vault_account.current_tvl,
                    minted_tokens: self.vault_lp_token_mint_pubkey.supply,
                }
            }
            fn transfer_from_vault_to_user_ctx(
                &self,
            ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
                CpiContext::new(
                    self.token_program.to_account_info(),
                    Transfer {
                        from: self.vault_input_token_account.to_account_info(),
                        to: self.user_input_token_account.to_account_info(),
                        authority: self.vault_account.to_account_info(),
                    },
                )
            }
            fn burn_ticket_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
                CpiContext::new(
                    self.token_program.to_account_info(),
                    Burn {
                        mint: self.vault_ticket_mint_pubkey.to_account_info(),
                        to: self.vault_user_ticket_account.to_account_info(),
                        authority: self.vault_account.to_account_info(),
                    },
                )
            }
        }
        /// Close a withdrawal ticket
        pub fn handler(
            ctx: Context<CloseWithdrawTicket>,
            lp_amount: u64,
            _bump_user: u8,
        ) -> Result<()> {
            let current_price = ctx.accounts.current_lp_price();
            let previous_price = ctx.accounts.vault_account.previous_lp_price;
            if previous_price != LpPrice::default() {
                if !(current_price >= previous_price) {
                    return Err (anchor_lang :: error :: Error :: from (anchor_lang :: error :: AnchorError { error_name : ErrorCode :: InvalidLpPrice . name () , error_code_number : ErrorCode :: InvalidLpPrice . into () , error_msg : ErrorCode :: InvalidLpPrice . to_string () , error_origin : Some (anchor_lang :: error :: ErrorOrigin :: Source (anchor_lang :: error :: Source { filename : "programs/lending-arb/src/instructions/close_withdraw_ticket.rs" , line : 91u32 , })) , compared_values : None , })) ;
                };
            }
            let amount = previous_price.lp_to_token(lp_amount)?;
            let amount_conservative = amount.saturating_sub(1);
            if !(amount_conservative > 1) {
                return Err(anchor_lang::error::Error::from(
                    anchor_lang::error::AnchorError {
                        error_name: ErrorCode::InvalidZeroWithdraw.name(),
                        error_code_number: ErrorCode::InvalidZeroWithdraw.into(),
                        error_msg: ErrorCode::InvalidZeroWithdraw.to_string(),
                        error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                            anchor_lang::error::Source {
                                filename:
                                    "programs/lending-arb/src/instructions/close_withdraw_ticket.rs",
                                line: 99u32,
                            },
                        )),
                        compared_values: None,
                    },
                ));
            };
            let seeds = &[
                "vault".as_ref(),
                &[ctx.accounts.vault_account.seed_number][..],
                ctx.accounts.vault_account.input_mint_pubkey.as_ref(),
                &[ctx.accounts.vault_account.bumps.vault],
            ];
            let signer = &[&seeds[..]];
            token::burn(ctx.accounts.burn_ticket_ctx().with_signer(signer), amount)?;
            token::transfer(
                ctx.accounts
                    .transfer_from_vault_to_user_ctx()
                    .with_signer(signer),
                amount_conservative,
            )?;
            ctx . accounts . vault_account . current_tvl = ctx . accounts . vault_account . current_tvl . checked_sub (amount) . ok_or_else (| | anchor_lang :: error :: Error :: from (anchor_lang :: error :: AnchorError { error_name : ErrorCode :: MathOverflow . name () , error_code_number : ErrorCode :: MathOverflow . into () , error_msg : ErrorCode :: MathOverflow . to_string () , error_origin : Some (anchor_lang :: error :: ErrorOrigin :: Source (anchor_lang :: error :: Source { filename : "programs/lending-arb/src/instructions/close_withdraw_ticket.rs" , line : 118u32 , })) , compared_values : None , })) ? ;
            Ok(())
        }
    }
    pub mod create_vault_user_ticket_account {
        use crate::vault::VaultAccount;
        use crate::{VAULT_ACCOUNT_SEED, VAULT_TICKET_MINT_SEED};
        use anchor_lang::prelude::*;
        use anchor_lang::solana_program::{program_option::COption, pubkey::Pubkey};
        use anchor_spl::token::{Mint, Token, TokenAccount};
        pub struct CreateVaultUserTicketAccount<'info> {
            #[account(mut)]
            pub user_signer: Signer<'info>,
            /// CHECKED: can create account to anyone
            pub user_ticket_account_owner: AccountInfo<'info>,
            # [account (init , payer = user_signer , token :: mint = vault_ticket_mint_pubkey , token :: authority = vault_account , seeds = [VAULT_TICKET_MINT_SEED , vault_ticket_mint_pubkey . key () . as_ref () , user_ticket_account_owner . key () . as_ref ()] , bump ,)]
            pub vault_user_ticket_account: Account<'info, TokenAccount>,
            # [account (mut , seeds = [VAULT_ACCOUNT_SEED , & [vault_account . seed_number] [..] , vault_account . input_mint_pubkey . as_ref ()] , bump = vault_account . bumps . vault)]
            pub vault_account: Box<Account<'info, VaultAccount>>,
            # [account (mut , constraint = vault_ticket_mint_pubkey . mint_authority == COption :: Some (vault_account . key ()) , seeds = [VAULT_TICKET_MINT_SEED , vault_account . key () . as_ref ()] , bump = vault_account . bumps . ticket_mint)]
            pub vault_ticket_mint_pubkey: Account<'info, Mint>,
            pub system_program: Program<'info, System>,
            pub token_program: Program<'info, Token>,
            pub rent: Sysvar<'info, Rent>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for CreateVaultUserTicketAccount<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
            ) -> anchor_lang::Result<Self> {
                let user_signer: Signer =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("user_signer"))?;
                let user_ticket_account_owner: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("user_ticket_account_owner"))?;
                let vault_user_ticket_account = &accounts[0];
                *accounts = &accounts[1..];
                let vault_account: Box<anchor_lang::accounts::account::Account<VaultAccount>> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vault_account"))?;
                let vault_ticket_mint_pubkey: anchor_lang::accounts::account::Account<Mint> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vault_ticket_mint_pubkey"))?;
                let system_program: anchor_lang::accounts::program::Program<System> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("system_program"))?;
                let token_program: anchor_lang::accounts::program::Program<Token> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("token_program"))?;
                let rent: Sysvar<Rent> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("rent"))?;
                let __anchor_rent = Rent::get()?;
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[
                        VAULT_TICKET_MINT_SEED,
                        vault_ticket_mint_pubkey.key().as_ref(),
                        user_ticket_account_owner.key().as_ref(),
                    ],
                    program_id,
                );
                __bumps.insert("vault_user_ticket_account".to_string(), __bump);
                let vault_user_ticket_account: anchor_lang::accounts::account::Account<
                    TokenAccount,
                > = {
                    if !false
                        || AsRef::<AccountInfo>::as_ref(&vault_user_ticket_account).owner
                            == &anchor_lang::solana_program::system_program::ID
                    {
                        let payer = user_signer.to_account_info();
                        let __current_lamports = vault_user_ticket_account.lamports();
                        if __current_lamports == 0 {
                            let lamports =
                                __anchor_rent.minimum_balance(anchor_spl::token::TokenAccount::LEN);
                            let cpi_accounts = anchor_lang::system_program::CreateAccount {
                                from: payer.to_account_info(),
                                to: vault_user_ticket_account.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::create_account(
                                cpi_context.with_signer(&[&[
                                    VAULT_TICKET_MINT_SEED,
                                    vault_ticket_mint_pubkey.key().as_ref(),
                                    user_ticket_account_owner.key().as_ref(),
                                    &[__bump][..],
                                ][..]]),
                                lamports,
                                anchor_spl::token::TokenAccount::LEN as u64,
                                &token_program.key(),
                            )?;
                        } else {
                            let required_lamports = __anchor_rent
                                .minimum_balance(anchor_spl::token::TokenAccount::LEN)
                                .max(1)
                                .saturating_sub(__current_lamports);
                            if required_lamports > 0 {
                                let cpi_accounts = anchor_lang::system_program::Transfer {
                                    from: payer.to_account_info(),
                                    to: vault_user_ticket_account.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::transfer(
                                    cpi_context,
                                    required_lamports,
                                )?;
                            }
                            let cpi_accounts = anchor_lang::system_program::Allocate {
                                account_to_allocate: vault_user_ticket_account.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::allocate(
                                cpi_context.with_signer(&[&[
                                    VAULT_TICKET_MINT_SEED,
                                    vault_ticket_mint_pubkey.key().as_ref(),
                                    user_ticket_account_owner.key().as_ref(),
                                    &[__bump][..],
                                ][..]]),
                                anchor_spl::token::TokenAccount::LEN as u64,
                            )?;
                            let cpi_accounts = anchor_lang::system_program::Assign {
                                account_to_assign: vault_user_ticket_account.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::assign(
                                cpi_context.with_signer(&[&[
                                    VAULT_TICKET_MINT_SEED,
                                    vault_ticket_mint_pubkey.key().as_ref(),
                                    user_ticket_account_owner.key().as_ref(),
                                    &[__bump][..],
                                ][..]]),
                                &token_program.key(),
                            )?;
                        }
                        let cpi_program = token_program.to_account_info();
                        let accounts = anchor_spl::token::InitializeAccount {
                            account: vault_user_ticket_account.to_account_info(),
                            mint: vault_ticket_mint_pubkey.to_account_info(),
                            authority: vault_account.to_account_info(),
                            rent: rent.to_account_info(),
                        };
                        let cpi_ctx = anchor_lang::context::CpiContext::new(cpi_program, accounts);
                        anchor_spl::token::initialize_account(cpi_ctx)?;
                    }
                    let pa: anchor_lang::accounts::account::Account<TokenAccount> =
                        anchor_lang::accounts::account::Account::try_from_unchecked(
                            &vault_user_ticket_account,
                        )?;
                    if false {
                        if pa.mint != vault_ticket_mint_pubkey.key() {
                            return Err(anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintTokenMint,
                            )
                            .with_account_name("vault_user_ticket_account")
                            .with_pubkeys((pa.mint, vault_ticket_mint_pubkey.key())));
                        }
                        if pa.owner != vault_account.key() {
                            return Err(anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintTokenOwner,
                            )
                            .with_account_name("vault_user_ticket_account")
                            .with_pubkeys((pa.owner, vault_account.key())));
                        }
                    }
                    pa
                };
                if vault_user_ticket_account.key() != __pda_address {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("vault_user_ticket_account")
                    .with_pubkeys((vault_user_ticket_account.key(), __pda_address)));
                }
                if !vault_user_ticket_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_user_ticket_account"));
                }
                if !__anchor_rent.is_exempt(
                    vault_user_ticket_account.to_account_info().lamports(),
                    vault_user_ticket_account.to_account_info().try_data_len()?,
                ) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRentExempt,
                    )
                    .with_account_name("vault_user_ticket_account"));
                }
                if !user_signer.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("user_signer"));
                }
                let __pda_address = Pubkey::create_program_address(
                    &[
                        VAULT_ACCOUNT_SEED,
                        &[vault_account.seed_number][..],
                        vault_account.input_mint_pubkey.as_ref(),
                        &[vault_account.bumps.vault][..],
                    ],
                    &program_id,
                )
                .map_err(|_| {
                    anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintSeeds)
                        .with_account_name("vault_account")
                })?;
                if vault_account.key() != __pda_address {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("vault_account")
                    .with_pubkeys((vault_account.key(), __pda_address)));
                }
                if !vault_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_account"));
                }
                let __pda_address = Pubkey::create_program_address(
                    &[
                        VAULT_TICKET_MINT_SEED,
                        vault_account.key().as_ref(),
                        &[vault_account.bumps.ticket_mint][..],
                    ],
                    &program_id,
                )
                .map_err(|_| {
                    anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintSeeds)
                        .with_account_name("vault_ticket_mint_pubkey")
                })?;
                if vault_ticket_mint_pubkey.key() != __pda_address {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("vault_ticket_mint_pubkey")
                    .with_pubkeys((vault_ticket_mint_pubkey.key(), __pda_address)));
                }
                if !vault_ticket_mint_pubkey.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_ticket_mint_pubkey"));
                }
                if !(vault_ticket_mint_pubkey.mint_authority == COption::Some(vault_account.key()))
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("vault_ticket_mint_pubkey"));
                }
                Ok(CreateVaultUserTicketAccount {
                    user_signer,
                    user_ticket_account_owner,
                    vault_user_ticket_account,
                    vault_account,
                    vault_ticket_mint_pubkey,
                    system_program,
                    token_program,
                    rent,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for CreateVaultUserTicketAccount<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.user_signer.to_account_infos());
                account_infos.extend(self.user_ticket_account_owner.to_account_infos());
                account_infos.extend(self.vault_user_ticket_account.to_account_infos());
                account_infos.extend(self.vault_account.to_account_infos());
                account_infos.extend(self.vault_ticket_mint_pubkey.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos.extend(self.rent.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for CreateVaultUserTicketAccount<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.user_signer.to_account_metas(None));
                account_metas.extend(self.user_ticket_account_owner.to_account_metas(None));
                account_metas.extend(self.vault_user_ticket_account.to_account_metas(None));
                account_metas.extend(self.vault_account.to_account_metas(None));
                account_metas.extend(self.vault_ticket_mint_pubkey.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas.extend(self.rent.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for CreateVaultUserTicketAccount<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.user_signer, program_id)
                    .map_err(|e| e.with_account_name("user_signer"))?;
                anchor_lang::AccountsExit::exit(&self.vault_user_ticket_account, program_id)
                    .map_err(|e| e.with_account_name("vault_user_ticket_account"))?;
                anchor_lang::AccountsExit::exit(&self.vault_account, program_id)
                    .map_err(|e| e.with_account_name("vault_account"))?;
                anchor_lang::AccountsExit::exit(&self.vault_ticket_mint_pubkey, program_id)
                    .map_err(|e| e.with_account_name("vault_ticket_mint_pubkey"))?;
                Ok(())
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_create_vault_user_ticket_account {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`CreateVaultUserTicketAccount`].
            pub struct CreateVaultUserTicketAccount {
                pub user_signer: anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECKED: can create account to anyone
                pub user_ticket_account_owner: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_user_ticket_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_ticket_mint_pubkey: anchor_lang::solana_program::pubkey::Pubkey,
                pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
                pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
                pub rent: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for CreateVaultUserTicketAccount
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.user_signer, writer)?;
                    borsh::BorshSerialize::serialize(&self.user_ticket_account_owner, writer)?;
                    borsh::BorshSerialize::serialize(&self.vault_user_ticket_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.vault_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.vault_ticket_mint_pubkey, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.rent, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for CreateVaultUserTicketAccount {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.user_signer,
                        true,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.user_ticket_account_owner,
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_user_ticket_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_ticket_mint_pubkey,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.system_program,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.rent, false,
                        ),
                    );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_create_vault_user_ticket_account {
            use super::*;
            /// Generated CPI struct of the accounts for [`CreateVaultUserTicketAccount`].
            pub struct CreateVaultUserTicketAccount<'info> {
                pub user_signer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECKED: can create account to anyone
                pub user_ticket_account_owner:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_user_ticket_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_ticket_mint_pubkey:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub rent: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for CreateVaultUserTicketAccount<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.user_signer),
                        true,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.user_ticket_account_owner),
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_user_ticket_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_ticket_mint_pubkey),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.system_program),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.rent),
                            false,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for CreateVaultUserTicketAccount<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.user_signer,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.user_ticket_account_owner,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_user_ticket_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_ticket_mint_pubkey,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.system_program,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.token_program,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.rent));
                    account_infos
                }
            }
        }
        /// Create vault_user_ticket_account
        pub fn handler(_ctx: Context<CreateVaultUserTicketAccount>) -> Result<()> {
            Ok(())
        }
    }
    pub mod initialize_vault {
        use crate::vault::{Bumps, InitVaultAccountParams, VaultAccount};
        use crate::TREASURY_PUBKEY;
        use crate::{VAULT_ACCOUNT_SEED, VAULT_LP_TOKEN_MINT_SEED};
        use anchor_lang::prelude::*;
        use anchor_lang::solana_program::pubkey::Pubkey;
        use anchor_spl::associated_token::AssociatedToken;
        use anchor_spl::token::{Mint, Token, TokenAccount};
        # [instruction (account_number : u8)]
        pub struct InitializeVault<'info> {
            #[account(mut)]
            pub user_signer: Signer<'info>,
            pub input_token_mint_address: Account<'info, Mint>,
            # [account (init , payer = user_signer , space = 8 + VaultAccount :: SIZE , seeds = [VAULT_ACCOUNT_SEED , & [account_number] [..] , input_token_mint_address . key () . as_ref ()] , bump ,)]
            pub vault_account: Box<Account<'info, VaultAccount>>,
            # [account (init , payer = user_signer , associated_token :: mint = input_token_mint_address , associated_token :: authority = vault_account ,)]
            pub vault_input_token_account: Account<'info, TokenAccount>,
            # [account (init , payer = user_signer , mint :: decimals = input_token_mint_address . decimals , mint :: authority = vault_account . key () , seeds = [VAULT_LP_TOKEN_MINT_SEED , vault_account . key () . as_ref ()] , bump ,)]
            pub vault_lp_token_mint_pubkey: Account<'info, Mint>,
            # [account (init , payer = user_signer , associated_token :: mint = vault_lp_token_mint_pubkey , associated_token :: authority = dao_treasury_owner ,)]
            pub dao_treasury_lp_token_account: Account<'info, TokenAccount>,
            # [account (constraint = dao_treasury_owner . key == & TREASURY_PUBKEY)]
            /// CHECKED: address is checked
            pub dao_treasury_owner: AccountInfo<'info>,
            pub system_program: Program<'info, System>,
            pub associated_token_program: Program<'info, AssociatedToken>,
            pub token_program: Program<'info, Token>,
            pub rent: Sysvar<'info, Rent>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for InitializeVault<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
            ) -> anchor_lang::Result<Self> {
                let mut ix_data = ix_data;
                struct __Args {
                    account_number: u8,
                }
                impl borsh::ser::BorshSerialize for __Args
                where
                    u8: borsh::ser::BorshSerialize,
                {
                    fn serialize<W: borsh::maybestd::io::Write>(
                        &self,
                        writer: &mut W,
                    ) -> ::core::result::Result<(), borsh::maybestd::io::Error>
                    {
                        borsh::BorshSerialize::serialize(&self.account_number, writer)?;
                        Ok(())
                    }
                }
                impl borsh::de::BorshDeserialize for __Args
                where
                    u8: borsh::BorshDeserialize,
                {
                    fn deserialize(
                        buf: &mut &[u8],
                    ) -> ::core::result::Result<Self, borsh::maybestd::io::Error>
                    {
                        Ok(Self {
                            account_number: borsh::BorshDeserialize::deserialize(buf)?,
                        })
                    }
                }
                let __Args { account_number } = __Args::deserialize(&mut ix_data)
                    .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
                let user_signer: Signer =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("user_signer"))?;
                let input_token_mint_address: anchor_lang::accounts::account::Account<Mint> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("input_token_mint_address"))?;
                let vault_account = &accounts[0];
                *accounts = &accounts[1..];
                let vault_input_token_account = &accounts[0];
                *accounts = &accounts[1..];
                let vault_lp_token_mint_pubkey = &accounts[0];
                *accounts = &accounts[1..];
                let dao_treasury_lp_token_account = &accounts[0];
                *accounts = &accounts[1..];
                let dao_treasury_owner: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("dao_treasury_owner"))?;
                let system_program: anchor_lang::accounts::program::Program<System> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("system_program"))?;
                let associated_token_program: anchor_lang::accounts::program::Program<
                    AssociatedToken,
                > = anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                    .map_err(|e| e.with_account_name("associated_token_program"))?;
                let token_program: anchor_lang::accounts::program::Program<Token> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("token_program"))?;
                let rent: Sysvar<Rent> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("rent"))?;
                let __anchor_rent = Rent::get()?;
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[
                        VAULT_ACCOUNT_SEED,
                        &[account_number][..],
                        input_token_mint_address.key().as_ref(),
                    ],
                    program_id,
                );
                __bumps.insert("vault_account".to_string(), __bump);
                let vault_account = {
                    let actual_field = vault_account.to_account_info();
                    let actual_owner = actual_field.owner;
                    let space = 8 + VaultAccount::SIZE;
                    if !false || actual_owner == &anchor_lang::solana_program::system_program::ID {
                        let payer = user_signer.to_account_info();
                        let __current_lamports = vault_account.lamports();
                        if __current_lamports == 0 {
                            let lamports = __anchor_rent.minimum_balance(space);
                            let cpi_accounts = anchor_lang::system_program::CreateAccount {
                                from: payer.to_account_info(),
                                to: vault_account.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::create_account(
                                cpi_context.with_signer(&[&[
                                    VAULT_ACCOUNT_SEED,
                                    &[account_number][..],
                                    input_token_mint_address.key().as_ref(),
                                    &[__bump][..],
                                ][..]]),
                                lamports,
                                space as u64,
                                program_id,
                            )?;
                        } else {
                            let required_lamports = __anchor_rent
                                .minimum_balance(space)
                                .max(1)
                                .saturating_sub(__current_lamports);
                            if required_lamports > 0 {
                                let cpi_accounts = anchor_lang::system_program::Transfer {
                                    from: payer.to_account_info(),
                                    to: vault_account.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::transfer(
                                    cpi_context,
                                    required_lamports,
                                )?;
                            }
                            let cpi_accounts = anchor_lang::system_program::Allocate {
                                account_to_allocate: vault_account.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::allocate(
                                cpi_context.with_signer(&[&[
                                    VAULT_ACCOUNT_SEED,
                                    &[account_number][..],
                                    input_token_mint_address.key().as_ref(),
                                    &[__bump][..],
                                ][..]]),
                                space as u64,
                            )?;
                            let cpi_accounts = anchor_lang::system_program::Assign {
                                account_to_assign: vault_account.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::assign(
                                cpi_context.with_signer(&[&[
                                    VAULT_ACCOUNT_SEED,
                                    &[account_number][..],
                                    input_token_mint_address.key().as_ref(),
                                    &[__bump][..],
                                ][..]]),
                                program_id,
                            )?;
                        }
                    }
                    let pa: Box<anchor_lang::accounts::account::Account<VaultAccount>> =
                        Box::new(anchor_lang::accounts::account::Account::try_from_unchecked(
                            &vault_account,
                        )?);
                    if false {
                        if space != actual_field.data_len() {
                            return Err(anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSpace,
                            )
                            .with_account_name("vault_account")
                            .with_values((space, actual_field.data_len())));
                        }
                        if actual_owner != program_id {
                            return Err(anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintOwner,
                            )
                            .with_account_name("vault_account")
                            .with_pubkeys((*actual_owner, *program_id)));
                        }
                        {
                            let required_lamports = __anchor_rent.minimum_balance(space);
                            if pa.to_account_info().lamports() < required_lamports {
                                return Err(anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintRentExempt,
                                )
                                .with_account_name("vault_account"));
                            }
                        }
                    }
                    pa
                };
                if vault_account.key() != __pda_address {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("vault_account")
                    .with_pubkeys((vault_account.key(), __pda_address)));
                }
                if !vault_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_account"));
                }
                if !__anchor_rent.is_exempt(
                    vault_account.to_account_info().lamports(),
                    vault_account.to_account_info().try_data_len()?,
                ) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRentExempt,
                    )
                    .with_account_name("vault_account"));
                }
                let __anchor_rent = Rent::get()?;
                let vault_input_token_account: anchor_lang::accounts::account::Account<
                    TokenAccount,
                > = {
                    if !false
                        || AsRef::<AccountInfo>::as_ref(&vault_input_token_account).owner
                            == &anchor_lang::solana_program::system_program::ID
                    {
                        let payer = user_signer.to_account_info();
                        let cpi_program = associated_token_program.to_account_info();
                        let cpi_accounts = anchor_spl::associated_token::Create {
                            payer: payer.to_account_info(),
                            associated_token: vault_input_token_account.to_account_info(),
                            authority: vault_account.to_account_info(),
                            mint: input_token_mint_address.to_account_info(),
                            system_program: system_program.to_account_info(),
                            token_program: token_program.to_account_info(),
                            rent: rent.to_account_info(),
                        };
                        let cpi_ctx =
                            anchor_lang::context::CpiContext::new(cpi_program, cpi_accounts);
                        anchor_spl::associated_token::create(cpi_ctx)?;
                    }
                    let pa: anchor_lang::accounts::account::Account<TokenAccount> =
                        anchor_lang::accounts::account::Account::try_from_unchecked(
                            &vault_input_token_account,
                        )?;
                    if false {
                        if pa.mint != input_token_mint_address.key() {
                            return Err(anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintTokenMint,
                            )
                            .with_account_name("vault_input_token_account")
                            .with_pubkeys((pa.mint, input_token_mint_address.key())));
                        }
                        if pa.owner != vault_account.key() {
                            return Err(anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintTokenOwner,
                            )
                            .with_account_name("vault_input_token_account")
                            .with_pubkeys((pa.owner, vault_account.key())));
                        }
                        if pa.key()
                            != anchor_spl::associated_token::get_associated_token_address(
                                &vault_account.key(),
                                &input_token_mint_address.key(),
                            )
                        {
                            return Err(anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::AccountNotAssociatedTokenAccount,
                            )
                            .with_account_name("vault_input_token_account"));
                        }
                    }
                    pa
                };
                if !vault_input_token_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_input_token_account"));
                }
                if !__anchor_rent.is_exempt(
                    vault_input_token_account.to_account_info().lamports(),
                    vault_input_token_account.to_account_info().try_data_len()?,
                ) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRentExempt,
                    )
                    .with_account_name("vault_input_token_account"));
                }
                let __anchor_rent = Rent::get()?;
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[VAULT_LP_TOKEN_MINT_SEED, vault_account.key().as_ref()],
                    program_id,
                );
                __bumps.insert("vault_lp_token_mint_pubkey".to_string(), __bump);
                let vault_lp_token_mint_pubkey: anchor_lang::accounts::account::Account<Mint> = {
                    if !false
                        || AsRef::<AccountInfo>::as_ref(&vault_lp_token_mint_pubkey).owner
                            == &anchor_lang::solana_program::system_program::ID
                    {
                        let payer = user_signer.to_account_info();
                        let __current_lamports = vault_lp_token_mint_pubkey.lamports();
                        if __current_lamports == 0 {
                            let lamports =
                                __anchor_rent.minimum_balance(anchor_spl::token::Mint::LEN);
                            let cpi_accounts = anchor_lang::system_program::CreateAccount {
                                from: payer.to_account_info(),
                                to: vault_lp_token_mint_pubkey.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::create_account(
                                cpi_context.with_signer(&[&[
                                    VAULT_LP_TOKEN_MINT_SEED,
                                    vault_account.key().as_ref(),
                                    &[__bump][..],
                                ][..]]),
                                lamports,
                                anchor_spl::token::Mint::LEN as u64,
                                &token_program.key(),
                            )?;
                        } else {
                            let required_lamports = __anchor_rent
                                .minimum_balance(anchor_spl::token::Mint::LEN)
                                .max(1)
                                .saturating_sub(__current_lamports);
                            if required_lamports > 0 {
                                let cpi_accounts = anchor_lang::system_program::Transfer {
                                    from: payer.to_account_info(),
                                    to: vault_lp_token_mint_pubkey.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::transfer(
                                    cpi_context,
                                    required_lamports,
                                )?;
                            }
                            let cpi_accounts = anchor_lang::system_program::Allocate {
                                account_to_allocate: vault_lp_token_mint_pubkey.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::allocate(
                                cpi_context.with_signer(&[&[
                                    VAULT_LP_TOKEN_MINT_SEED,
                                    vault_account.key().as_ref(),
                                    &[__bump][..],
                                ][..]]),
                                anchor_spl::token::Mint::LEN as u64,
                            )?;
                            let cpi_accounts = anchor_lang::system_program::Assign {
                                account_to_assign: vault_lp_token_mint_pubkey.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::assign(
                                cpi_context.with_signer(&[&[
                                    VAULT_LP_TOKEN_MINT_SEED,
                                    vault_account.key().as_ref(),
                                    &[__bump][..],
                                ][..]]),
                                &token_program.key(),
                            )?;
                        }
                        let cpi_program = token_program.to_account_info();
                        let accounts = anchor_spl::token::InitializeMint {
                            mint: vault_lp_token_mint_pubkey.to_account_info(),
                            rent: rent.to_account_info(),
                        };
                        let cpi_ctx = anchor_lang::context::CpiContext::new(cpi_program, accounts);
                        anchor_spl::token::initialize_mint(
                            cpi_ctx,
                            input_token_mint_address.decimals,
                            &vault_account.key().key(),
                            Option::<&anchor_lang::prelude::Pubkey>::None,
                        )?;
                    }
                    let pa: anchor_lang::accounts::account::Account<Mint> =
                        anchor_lang::accounts::account::Account::try_from_unchecked(
                            &vault_lp_token_mint_pubkey,
                        )?;
                    if false {
                        if pa.mint_authority
                            != anchor_lang::solana_program::program_option::COption::Some(
                                vault_account.key().key(),
                            )
                        {
                            return Err(anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMintMintAuthority,
                            )
                            .with_account_name("vault_lp_token_mint_pubkey"));
                        }
                        if pa
                            .freeze_authority
                            .as_ref()
                            .map(|fa| {
                                Option::<&anchor_lang::prelude::Pubkey>::None
                                    .as_ref()
                                    .map(|expected_fa| fa != *expected_fa)
                                    .unwrap_or(true)
                            })
                            .unwrap_or(Option::<&anchor_lang::prelude::Pubkey>::None.is_some())
                        {
                            return Err(anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMintFreezeAuthority,
                            )
                            .with_account_name("vault_lp_token_mint_pubkey"));
                        }
                        if pa.decimals != input_token_mint_address.decimals {
                            return Err(anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMintDecimals,
                            )
                            .with_account_name("vault_lp_token_mint_pubkey")
                            .with_values((pa.decimals, input_token_mint_address.decimals)));
                        }
                    }
                    pa
                };
                if vault_lp_token_mint_pubkey.key() != __pda_address {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("vault_lp_token_mint_pubkey")
                    .with_pubkeys((vault_lp_token_mint_pubkey.key(), __pda_address)));
                }
                if !vault_lp_token_mint_pubkey.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_lp_token_mint_pubkey"));
                }
                if !__anchor_rent.is_exempt(
                    vault_lp_token_mint_pubkey.to_account_info().lamports(),
                    vault_lp_token_mint_pubkey
                        .to_account_info()
                        .try_data_len()?,
                ) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRentExempt,
                    )
                    .with_account_name("vault_lp_token_mint_pubkey"));
                }
                let __anchor_rent = Rent::get()?;
                let dao_treasury_lp_token_account: anchor_lang::accounts::account::Account<
                    TokenAccount,
                > = {
                    if !false
                        || AsRef::<AccountInfo>::as_ref(&dao_treasury_lp_token_account).owner
                            == &anchor_lang::solana_program::system_program::ID
                    {
                        let payer = user_signer.to_account_info();
                        let cpi_program = associated_token_program.to_account_info();
                        let cpi_accounts = anchor_spl::associated_token::Create {
                            payer: payer.to_account_info(),
                            associated_token: dao_treasury_lp_token_account.to_account_info(),
                            authority: dao_treasury_owner.to_account_info(),
                            mint: vault_lp_token_mint_pubkey.to_account_info(),
                            system_program: system_program.to_account_info(),
                            token_program: token_program.to_account_info(),
                            rent: rent.to_account_info(),
                        };
                        let cpi_ctx =
                            anchor_lang::context::CpiContext::new(cpi_program, cpi_accounts);
                        anchor_spl::associated_token::create(cpi_ctx)?;
                    }
                    let pa: anchor_lang::accounts::account::Account<TokenAccount> =
                        anchor_lang::accounts::account::Account::try_from_unchecked(
                            &dao_treasury_lp_token_account,
                        )?;
                    if false {
                        if pa.mint != vault_lp_token_mint_pubkey.key() {
                            return Err(anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintTokenMint,
                            )
                            .with_account_name("dao_treasury_lp_token_account")
                            .with_pubkeys((pa.mint, vault_lp_token_mint_pubkey.key())));
                        }
                        if pa.owner != dao_treasury_owner.key() {
                            return Err(anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintTokenOwner,
                            )
                            .with_account_name("dao_treasury_lp_token_account")
                            .with_pubkeys((pa.owner, dao_treasury_owner.key())));
                        }
                        if pa.key()
                            != anchor_spl::associated_token::get_associated_token_address(
                                &dao_treasury_owner.key(),
                                &vault_lp_token_mint_pubkey.key(),
                            )
                        {
                            return Err(anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::AccountNotAssociatedTokenAccount,
                            )
                            .with_account_name("dao_treasury_lp_token_account"));
                        }
                    }
                    pa
                };
                if !dao_treasury_lp_token_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("dao_treasury_lp_token_account"));
                }
                if !__anchor_rent.is_exempt(
                    dao_treasury_lp_token_account.to_account_info().lamports(),
                    dao_treasury_lp_token_account
                        .to_account_info()
                        .try_data_len()?,
                ) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRentExempt,
                    )
                    .with_account_name("dao_treasury_lp_token_account"));
                }
                if !user_signer.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("user_signer"));
                }
                if !(dao_treasury_owner.key == &TREASURY_PUBKEY) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("dao_treasury_owner"));
                }
                Ok(InitializeVault {
                    user_signer,
                    input_token_mint_address,
                    vault_account,
                    vault_input_token_account,
                    vault_lp_token_mint_pubkey,
                    dao_treasury_lp_token_account,
                    dao_treasury_owner,
                    system_program,
                    associated_token_program,
                    token_program,
                    rent,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeVault<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.user_signer.to_account_infos());
                account_infos.extend(self.input_token_mint_address.to_account_infos());
                account_infos.extend(self.vault_account.to_account_infos());
                account_infos.extend(self.vault_input_token_account.to_account_infos());
                account_infos.extend(self.vault_lp_token_mint_pubkey.to_account_infos());
                account_infos.extend(self.dao_treasury_lp_token_account.to_account_infos());
                account_infos.extend(self.dao_treasury_owner.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos.extend(self.associated_token_program.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos.extend(self.rent.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for InitializeVault<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.user_signer.to_account_metas(None));
                account_metas.extend(self.input_token_mint_address.to_account_metas(None));
                account_metas.extend(self.vault_account.to_account_metas(None));
                account_metas.extend(self.vault_input_token_account.to_account_metas(None));
                account_metas.extend(self.vault_lp_token_mint_pubkey.to_account_metas(None));
                account_metas.extend(self.dao_treasury_lp_token_account.to_account_metas(None));
                account_metas.extend(self.dao_treasury_owner.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas.extend(self.associated_token_program.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas.extend(self.rent.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for InitializeVault<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.user_signer, program_id)
                    .map_err(|e| e.with_account_name("user_signer"))?;
                anchor_lang::AccountsExit::exit(&self.vault_account, program_id)
                    .map_err(|e| e.with_account_name("vault_account"))?;
                anchor_lang::AccountsExit::exit(&self.vault_input_token_account, program_id)
                    .map_err(|e| e.with_account_name("vault_input_token_account"))?;
                anchor_lang::AccountsExit::exit(&self.vault_lp_token_mint_pubkey, program_id)
                    .map_err(|e| e.with_account_name("vault_lp_token_mint_pubkey"))?;
                anchor_lang::AccountsExit::exit(&self.dao_treasury_lp_token_account, program_id)
                    .map_err(|e| e.with_account_name("dao_treasury_lp_token_account"))?;
                Ok(())
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_initialize_vault {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`InitializeVault`].
            pub struct InitializeVault {
                pub user_signer: anchor_lang::solana_program::pubkey::Pubkey,
                pub input_token_mint_address: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_input_token_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_lp_token_mint_pubkey: anchor_lang::solana_program::pubkey::Pubkey,
                pub dao_treasury_lp_token_account: anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECKED: address is checked
                pub dao_treasury_owner: anchor_lang::solana_program::pubkey::Pubkey,
                pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
                pub associated_token_program: anchor_lang::solana_program::pubkey::Pubkey,
                pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
                pub rent: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for InitializeVault
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.user_signer, writer)?;
                    borsh::BorshSerialize::serialize(&self.input_token_mint_address, writer)?;
                    borsh::BorshSerialize::serialize(&self.vault_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.vault_input_token_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.vault_lp_token_mint_pubkey, writer)?;
                    borsh::BorshSerialize::serialize(&self.dao_treasury_lp_token_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.dao_treasury_owner, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.associated_token_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.rent, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for InitializeVault {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.user_signer,
                        true,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.input_token_mint_address,
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_input_token_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_lp_token_mint_pubkey,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.dao_treasury_lp_token_account,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.dao_treasury_owner,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.system_program,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.associated_token_program,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.rent, false,
                        ),
                    );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_initialize_vault {
            use super::*;
            /// Generated CPI struct of the accounts for [`InitializeVault`].
            pub struct InitializeVault<'info> {
                pub user_signer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub input_token_mint_address:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_input_token_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_lp_token_mint_pubkey:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub dao_treasury_lp_token_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECKED: address is checked
                pub dao_treasury_owner:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub associated_token_program:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub rent: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for InitializeVault<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.user_signer),
                        true,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.input_token_mint_address),
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_input_token_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_lp_token_mint_pubkey),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.dao_treasury_lp_token_account),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.dao_treasury_owner),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.system_program),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.associated_token_program),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.rent),
                            false,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeVault<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.user_signer,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.input_token_mint_address,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_input_token_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_lp_token_mint_pubkey,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.dao_treasury_lp_token_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.dao_treasury_owner,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.system_program,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.associated_token_program,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.token_program,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.rent));
                    account_infos
                }
            }
        }
        pub fn handler(ctx: Context<InitializeVault>, account_number: u8) -> Result<()> {
            ctx.accounts
                .vault_account
                .set_inner(VaultAccount::init(InitVaultAccountParams {
                    seed_number: account_number,
                    bumps: Bumps {
                        vault: *ctx.bumps.get("vault_account").unwrap(),
                        lp_token_mint: *ctx.bumps.get("vault_lp_token_mint_pubkey").unwrap(),
                        ticket_mint: *ctx.bumps.get("vault_ticket_mint_pubkey").unwrap(),
                    },
                    input_mint_pubkey: ctx.accounts.input_token_mint_address.key(),
                    dao_treasury_lp_token_account: ctx.accounts.dao_treasury_lp_token_account.key(),
                }));
            Ok(())
        }
    }
    pub mod open_withdraw_ticket {
        use crate::error::ErrorCode;
        use crate::macros::generate_seeds;
        use crate::vault::{LpPrice, VaultAccount};
        use crate::{VAULT_ACCOUNT_SEED, VAULT_LP_TOKEN_MINT_SEED, VAULT_TICKET_MINT_SEED};
        use anchor_lang::prelude::*;
        use anchor_lang::solana_program::{program_option::COption, pubkey::Pubkey};
        use anchor_spl::token::{self, Burn, Mint, MintTo, Token, TokenAccount};
        # [instruction (bump_user : u8)]
        pub struct OpenWithdrawTicket<'info> {
            pub user_signer: Signer<'info>,
            # [account (mut , constraint = user_lp_token_account . owner == * user_signer . key)]
            pub user_lp_token_account: Account<'info, TokenAccount>,
            # [account (mut , constraint = vault_user_ticket_account . owner == vault_account . key () , seeds = [VAULT_TICKET_MINT_SEED , vault_ticket_mint_pubkey . key () . as_ref () , user_signer . key () . as_ref ()] , bump = bump_user)]
            pub vault_user_ticket_account: Account<'info, TokenAccount>,
            # [account (mut , seeds = [VAULT_ACCOUNT_SEED , & [vault_account . seed_number] [..] , vault_account . input_mint_pubkey . as_ref ()] , bump = vault_account . bumps . vault)]
            pub vault_account: Box<Account<'info, VaultAccount>>,
            # [account (mut , constraint = vault_lp_token_mint_pubkey . mint_authority == COption :: Some (vault_account . key ()) , seeds = [VAULT_LP_TOKEN_MINT_SEED , vault_account . key () . as_ref ()] , bump = vault_account . bumps . lp_token_mint)]
            pub vault_lp_token_mint_pubkey: Account<'info, Mint>,
            # [account (mut , constraint = vault_ticket_mint_pubkey . mint_authority == COption :: Some (vault_account . key ()) , seeds = [VAULT_TICKET_MINT_SEED , vault_account . key () . as_ref ()] , bump = vault_account . bumps . ticket_mint)]
            pub vault_ticket_mint_pubkey: Account<'info, Mint>,
            pub token_program: Program<'info, Token>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for OpenWithdrawTicket<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
            ) -> anchor_lang::Result<Self> {
                let mut ix_data = ix_data;
                struct __Args {
                    bump_user: u8,
                }
                impl borsh::ser::BorshSerialize for __Args
                where
                    u8: borsh::ser::BorshSerialize,
                {
                    fn serialize<W: borsh::maybestd::io::Write>(
                        &self,
                        writer: &mut W,
                    ) -> ::core::result::Result<(), borsh::maybestd::io::Error>
                    {
                        borsh::BorshSerialize::serialize(&self.bump_user, writer)?;
                        Ok(())
                    }
                }
                impl borsh::de::BorshDeserialize for __Args
                where
                    u8: borsh::BorshDeserialize,
                {
                    fn deserialize(
                        buf: &mut &[u8],
                    ) -> ::core::result::Result<Self, borsh::maybestd::io::Error>
                    {
                        Ok(Self {
                            bump_user: borsh::BorshDeserialize::deserialize(buf)?,
                        })
                    }
                }
                let __Args { bump_user } = __Args::deserialize(&mut ix_data)
                    .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
                let user_signer: Signer =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("user_signer"))?;
                let user_lp_token_account: anchor_lang::accounts::account::Account<TokenAccount> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("user_lp_token_account"))?;
                let vault_user_ticket_account: anchor_lang::accounts::account::Account<
                    TokenAccount,
                > = anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                    .map_err(|e| e.with_account_name("vault_user_ticket_account"))?;
                let vault_account: Box<anchor_lang::accounts::account::Account<VaultAccount>> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vault_account"))?;
                let vault_lp_token_mint_pubkey: anchor_lang::accounts::account::Account<Mint> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vault_lp_token_mint_pubkey"))?;
                let vault_ticket_mint_pubkey: anchor_lang::accounts::account::Account<Mint> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vault_ticket_mint_pubkey"))?;
                let token_program: anchor_lang::accounts::program::Program<Token> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("token_program"))?;
                if !user_lp_token_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("user_lp_token_account"));
                }
                if !(user_lp_token_account.owner == *user_signer.key) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("user_lp_token_account"));
                }
                let __pda_address = Pubkey::create_program_address(
                    &[
                        VAULT_TICKET_MINT_SEED,
                        vault_ticket_mint_pubkey.key().as_ref(),
                        user_signer.key().as_ref(),
                        &[bump_user][..],
                    ],
                    &program_id,
                )
                .map_err(|_| {
                    anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintSeeds)
                        .with_account_name("vault_user_ticket_account")
                })?;
                if vault_user_ticket_account.key() != __pda_address {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("vault_user_ticket_account")
                    .with_pubkeys((vault_user_ticket_account.key(), __pda_address)));
                }
                if !vault_user_ticket_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_user_ticket_account"));
                }
                if !(vault_user_ticket_account.owner == vault_account.key()) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("vault_user_ticket_account"));
                }
                let __pda_address = Pubkey::create_program_address(
                    &[
                        VAULT_ACCOUNT_SEED,
                        &[vault_account.seed_number][..],
                        vault_account.input_mint_pubkey.as_ref(),
                        &[vault_account.bumps.vault][..],
                    ],
                    &program_id,
                )
                .map_err(|_| {
                    anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintSeeds)
                        .with_account_name("vault_account")
                })?;
                if vault_account.key() != __pda_address {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("vault_account")
                    .with_pubkeys((vault_account.key(), __pda_address)));
                }
                if !vault_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_account"));
                }
                let __pda_address = Pubkey::create_program_address(
                    &[
                        VAULT_LP_TOKEN_MINT_SEED,
                        vault_account.key().as_ref(),
                        &[vault_account.bumps.lp_token_mint][..],
                    ],
                    &program_id,
                )
                .map_err(|_| {
                    anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintSeeds)
                        .with_account_name("vault_lp_token_mint_pubkey")
                })?;
                if vault_lp_token_mint_pubkey.key() != __pda_address {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("vault_lp_token_mint_pubkey")
                    .with_pubkeys((vault_lp_token_mint_pubkey.key(), __pda_address)));
                }
                if !vault_lp_token_mint_pubkey.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_lp_token_mint_pubkey"));
                }
                if !(vault_lp_token_mint_pubkey.mint_authority
                    == COption::Some(vault_account.key()))
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("vault_lp_token_mint_pubkey"));
                }
                let __pda_address = Pubkey::create_program_address(
                    &[
                        VAULT_TICKET_MINT_SEED,
                        vault_account.key().as_ref(),
                        &[vault_account.bumps.ticket_mint][..],
                    ],
                    &program_id,
                )
                .map_err(|_| {
                    anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintSeeds)
                        .with_account_name("vault_ticket_mint_pubkey")
                })?;
                if vault_ticket_mint_pubkey.key() != __pda_address {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("vault_ticket_mint_pubkey")
                    .with_pubkeys((vault_ticket_mint_pubkey.key(), __pda_address)));
                }
                if !vault_ticket_mint_pubkey.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_ticket_mint_pubkey"));
                }
                if !(vault_ticket_mint_pubkey.mint_authority == COption::Some(vault_account.key()))
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("vault_ticket_mint_pubkey"));
                }
                Ok(OpenWithdrawTicket {
                    user_signer,
                    user_lp_token_account,
                    vault_user_ticket_account,
                    vault_account,
                    vault_lp_token_mint_pubkey,
                    vault_ticket_mint_pubkey,
                    token_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for OpenWithdrawTicket<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.user_signer.to_account_infos());
                account_infos.extend(self.user_lp_token_account.to_account_infos());
                account_infos.extend(self.vault_user_ticket_account.to_account_infos());
                account_infos.extend(self.vault_account.to_account_infos());
                account_infos.extend(self.vault_lp_token_mint_pubkey.to_account_infos());
                account_infos.extend(self.vault_ticket_mint_pubkey.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for OpenWithdrawTicket<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.user_signer.to_account_metas(None));
                account_metas.extend(self.user_lp_token_account.to_account_metas(None));
                account_metas.extend(self.vault_user_ticket_account.to_account_metas(None));
                account_metas.extend(self.vault_account.to_account_metas(None));
                account_metas.extend(self.vault_lp_token_mint_pubkey.to_account_metas(None));
                account_metas.extend(self.vault_ticket_mint_pubkey.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for OpenWithdrawTicket<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.user_lp_token_account, program_id)
                    .map_err(|e| e.with_account_name("user_lp_token_account"))?;
                anchor_lang::AccountsExit::exit(&self.vault_user_ticket_account, program_id)
                    .map_err(|e| e.with_account_name("vault_user_ticket_account"))?;
                anchor_lang::AccountsExit::exit(&self.vault_account, program_id)
                    .map_err(|e| e.with_account_name("vault_account"))?;
                anchor_lang::AccountsExit::exit(&self.vault_lp_token_mint_pubkey, program_id)
                    .map_err(|e| e.with_account_name("vault_lp_token_mint_pubkey"))?;
                anchor_lang::AccountsExit::exit(&self.vault_ticket_mint_pubkey, program_id)
                    .map_err(|e| e.with_account_name("vault_ticket_mint_pubkey"))?;
                Ok(())
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_open_withdraw_ticket {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`OpenWithdrawTicket`].
            pub struct OpenWithdrawTicket {
                pub user_signer: anchor_lang::solana_program::pubkey::Pubkey,
                pub user_lp_token_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_user_ticket_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_lp_token_mint_pubkey: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_ticket_mint_pubkey: anchor_lang::solana_program::pubkey::Pubkey,
                pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for OpenWithdrawTicket
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.user_signer, writer)?;
                    borsh::BorshSerialize::serialize(&self.user_lp_token_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.vault_user_ticket_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.vault_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.vault_lp_token_mint_pubkey, writer)?;
                    borsh::BorshSerialize::serialize(&self.vault_ticket_mint_pubkey, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for OpenWithdrawTicket {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.user_signer,
                            true,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.user_lp_token_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_user_ticket_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_lp_token_mint_pubkey,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_ticket_mint_pubkey,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_open_withdraw_ticket {
            use super::*;
            /// Generated CPI struct of the accounts for [`OpenWithdrawTicket`].
            pub struct OpenWithdrawTicket<'info> {
                pub user_signer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub user_lp_token_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_user_ticket_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_lp_token_mint_pubkey:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_ticket_mint_pubkey:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for OpenWithdrawTicket<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.user_signer),
                            true,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.user_lp_token_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_user_ticket_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_lp_token_mint_pubkey),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_ticket_mint_pubkey),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for OpenWithdrawTicket<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.user_signer,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.user_lp_token_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_user_ticket_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_lp_token_mint_pubkey,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_ticket_mint_pubkey,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.token_program,
                    ));
                    account_infos
                }
            }
        }
        impl<'info> OpenWithdrawTicket<'info> {
            fn current_lp_price(&self) -> LpPrice {
                LpPrice {
                    total_tokens: self.vault_account.current_tvl,
                    minted_tokens: self.vault_lp_token_mint_pubkey.supply,
                }
            }
            fn burn_user_lps_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
                CpiContext::new(
                    self.token_program.to_account_info(),
                    Burn {
                        mint: self.vault_lp_token_mint_pubkey.to_account_info(),
                        to: self.user_lp_token_account.to_account_info(),
                        authority: self.user_signer.to_account_info(),
                    },
                )
            }
            fn mint_ticket_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
                CpiContext::new(
                    self.token_program.to_account_info(),
                    MintTo {
                        mint: self.vault_ticket_mint_pubkey.to_account_info(),
                        to: self.vault_user_ticket_account.to_account_info(),
                        authority: self.vault_account.to_account_info(),
                    },
                )
            }
        }
        /// Open a withdrawal ticket (for delayed withdrawals)
        pub fn handler(
            ctx: Context<OpenWithdrawTicket>,
            lp_amount: u64,
            _bump_user: u8,
        ) -> Result<()> {
            let current_price = ctx.accounts.current_lp_price();
            let previous_price = ctx.accounts.vault_account.previous_lp_price;
            if previous_price != LpPrice::default() {
                if !(current_price >= previous_price) {
                    return Err (anchor_lang :: error :: Error :: from (anchor_lang :: error :: AnchorError { error_name : ErrorCode :: InvalidLpPrice . name () , error_code_number : ErrorCode :: InvalidLpPrice . into () , error_msg : ErrorCode :: InvalidLpPrice . to_string () , error_origin : Some (anchor_lang :: error :: ErrorOrigin :: Source (anchor_lang :: error :: Source { filename : "programs/lending-arb/src/instructions/open_withdraw_ticket.rs" , line : 85u32 , })) , compared_values : None , })) ;
                };
            }
            let seeds = &[
                "vault".as_ref(),
                &[ctx.accounts.vault_account.seed_number][..],
                ctx.accounts.vault_account.input_mint_pubkey.as_ref(),
                &[ctx.accounts.vault_account.bumps.vault],
            ];
            let signer = &[&seeds[..]];
            token::burn(ctx.accounts.burn_user_lps_ctx(), lp_amount)?;
            token::mint_to(
                ctx.accounts.mint_ticket_ctx().with_signer(signer),
                lp_amount,
            )?;
            Ok(())
        }
    }
    pub mod set_hashes {
        use crate::check_hash::CHECKHASH_BYTES;
        use crate::error::ErrorCode;
        use crate::protocols::Protocols;
        use crate::vault::VaultAccount;
        use crate::VAULT_ACCOUNT_SEED;
        use anchor_lang::prelude::*;
        use anchor_lang::solana_program::pubkey::Pubkey;
        use std::convert::TryInto;
        pub struct SetHashes<'info> {
            pub user_signer: Signer<'info>,
            # [account (mut , seeds = [VAULT_ACCOUNT_SEED , & [vault_account . seed_number] [..] , vault_account . input_mint_pubkey . as_ref ()] , bump = vault_account . bumps . vault)]
            pub vault_account: Box<Account<'info, VaultAccount>>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for SetHashes<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
            ) -> anchor_lang::Result<Self> {
                let user_signer: Signer =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("user_signer"))?;
                let vault_account: Box<anchor_lang::accounts::account::Account<VaultAccount>> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vault_account"))?;
                let __pda_address = Pubkey::create_program_address(
                    &[
                        VAULT_ACCOUNT_SEED,
                        &[vault_account.seed_number][..],
                        vault_account.input_mint_pubkey.as_ref(),
                        &[vault_account.bumps.vault][..],
                    ],
                    &program_id,
                )
                .map_err(|_| {
                    anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintSeeds)
                        .with_account_name("vault_account")
                })?;
                if vault_account.key() != __pda_address {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("vault_account")
                    .with_pubkeys((vault_account.key(), __pda_address)));
                }
                if !vault_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_account"));
                }
                Ok(SetHashes {
                    user_signer,
                    vault_account,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for SetHashes<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.user_signer.to_account_infos());
                account_infos.extend(self.vault_account.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for SetHashes<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.user_signer.to_account_metas(None));
                account_metas.extend(self.vault_account.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for SetHashes<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.vault_account, program_id)
                    .map_err(|e| e.with_account_name("vault_account"))?;
                Ok(())
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_set_hashes {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`SetHashes`].
            pub struct SetHashes {
                pub user_signer: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_account: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for SetHashes
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.user_signer, writer)?;
                    borsh::BorshSerialize::serialize(&self.vault_account, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for SetHashes {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.user_signer,
                            true,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_account,
                        false,
                    ));
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_set_hashes {
            use super::*;
            /// Generated CPI struct of the accounts for [`SetHashes`].
            pub struct SetHashes<'info> {
                pub user_signer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for SetHashes<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.user_signer),
                            true,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_account),
                        false,
                    ));
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for SetHashes<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.user_signer,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_account,
                    ));
                    account_infos
                }
            }
        }
        /// Set hash of a protocol for a specific action
        pub fn handler(
            ctx: Context<SetHashes>,
            protocol_id: u8,
            hashes: [[u8; CHECKHASH_BYTES]; 3],
        ) -> Result<()> {
            let protocol: Protocols = usize::from(protocol_id).try_into().map_err(|_| {
                anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                    error_name: ErrorCode::InvalidProtocolId.name(),
                    error_code_number: ErrorCode::InvalidProtocolId.into(),
                    error_msg: ErrorCode::InvalidProtocolId.to_string(),
                    error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                        anchor_lang::error::Source {
                            filename: "programs/lending-arb/src/instructions/set_hashes.rs",
                            line: 29u32,
                        },
                    )),
                    compared_values: None,
                })
            })?;
            let protocol_idx = ctx.accounts.vault_account.protocol_position(protocol)?;
            ctx.accounts.vault_account.protocols[protocol_idx].set_hashes(hashes);
            Ok(())
        }
    }
    pub use deposit::*;
    pub use withdraw::*;
    pub use close_withdraw_ticket::*;
    pub use create_vault_user_ticket_account::*;
    pub use initialize_vault::*;
    pub use open_withdraw_ticket::*;
    pub use set_hashes::*;
    pub mod protocol_deposit {
        use crate::protocols::Protocols;
        use crate::vault::{ProtocolData, VaultAccount};
        use crate::VAULT_ACCOUNT_SEED;
        use anchor_lang::prelude::*;
        use anchor_spl::token::{Token, TokenAccount};
        /// Deposit into the protocol
        pub trait ProtocolDeposit<'info> {
            /// Return the protcol position in the vector
            fn protocol_position(&self, protocol: Protocols) -> Result<usize>;
            /// Return a mutable refrence of the data
            fn protocol_data_as_mut(&mut self, protocol_idx: usize) -> &mut ProtocolData;
            /// Compute the amount to deposit
            fn get_amount(&self, protocol_idx: usize) -> Result<u64>;
            /// Deposit into the protocol
            fn cpi_deposit(&self, amount: u64) -> Result<()>;
        }
        /// Deposit into the protocol and update protocol data
        pub fn handler<'info, T: ProtocolDeposit<'info>>(
            ctx: Context<T>,
            protocol: Protocols,
        ) -> Result<()> {
            let protocol_idx = ctx.accounts.protocol_position(protocol)?;
            let amount = ctx.accounts.get_amount(protocol_idx)?;
            ctx.accounts.cpi_deposit(amount)?;
            ctx.accounts
                .protocol_data_as_mut(protocol_idx)
                .update_after_deposit(amount)?;
            Ok(())
        }
        pub struct GenericDepositAccounts<'info> {
            # [account (mut , seeds = [VAULT_ACCOUNT_SEED , & [vault_account . seed_number] [..] , vault_account . input_mint_pubkey . as_ref ()] , bump = vault_account . bumps . vault)]
            pub vault_account: Box<Account<'info, VaultAccount>>,
            # [account (mut , associated_token :: mint = vault_account . input_mint_pubkey , associated_token :: authority = vault_account ,)]
            pub vault_input_token_account: Account<'info, TokenAccount>,
            pub token_program: Program<'info, Token>,
            pub clock: Sysvar<'info, Clock>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for GenericDepositAccounts<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
            ) -> anchor_lang::Result<Self> {
                let vault_account: Box<anchor_lang::accounts::account::Account<VaultAccount>> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vault_account"))?;
                let vault_input_token_account: anchor_lang::accounts::account::Account<
                    TokenAccount,
                > = anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                    .map_err(|e| e.with_account_name("vault_input_token_account"))?;
                let token_program: anchor_lang::accounts::program::Program<Token> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("token_program"))?;
                let clock: Sysvar<Clock> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("clock"))?;
                let __pda_address = Pubkey::create_program_address(
                    &[
                        VAULT_ACCOUNT_SEED,
                        &[vault_account.seed_number][..],
                        vault_account.input_mint_pubkey.as_ref(),
                        &[vault_account.bumps.vault][..],
                    ],
                    &program_id,
                )
                .map_err(|_| {
                    anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintSeeds)
                        .with_account_name("vault_account")
                })?;
                if vault_account.key() != __pda_address {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("vault_account")
                    .with_pubkeys((vault_account.key(), __pda_address)));
                }
                if !vault_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_account"));
                }
                {
                    let my_owner = vault_input_token_account.owner;
                    let wallet_address = vault_account.key();
                    if my_owner != wallet_address {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintTokenOwner,
                        )
                        .with_account_name("vault_input_token_account")
                        .with_pubkeys((my_owner, wallet_address)));
                    }
                    let __associated_token_address =
                        anchor_spl::associated_token::get_associated_token_address(
                            &wallet_address,
                            &vault_account.input_mint_pubkey.key(),
                        );
                    let my_key = vault_input_token_account.key();
                    if my_key != __associated_token_address {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintAssociated,
                        )
                        .with_account_name("vault_input_token_account")
                        .with_pubkeys((my_key, __associated_token_address)));
                    }
                }
                if !vault_input_token_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_input_token_account"));
                }
                Ok(GenericDepositAccounts {
                    vault_account,
                    vault_input_token_account,
                    token_program,
                    clock,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for GenericDepositAccounts<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.vault_account.to_account_infos());
                account_infos.extend(self.vault_input_token_account.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos.extend(self.clock.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for GenericDepositAccounts<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.vault_account.to_account_metas(None));
                account_metas.extend(self.vault_input_token_account.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas.extend(self.clock.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for GenericDepositAccounts<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.vault_account, program_id)
                    .map_err(|e| e.with_account_name("vault_account"))?;
                anchor_lang::AccountsExit::exit(&self.vault_input_token_account, program_id)
                    .map_err(|e| e.with_account_name("vault_input_token_account"))?;
                Ok(())
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_generic_deposit_accounts {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`GenericDepositAccounts`].
            pub struct GenericDepositAccounts {
                pub vault_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_input_token_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
                pub clock: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for GenericDepositAccounts
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.vault_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.vault_input_token_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.clock, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for GenericDepositAccounts {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_input_token_account,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.clock, false,
                        ),
                    );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_generic_deposit_accounts {
            use super::*;
            /// Generated CPI struct of the accounts for [`GenericDepositAccounts`].
            pub struct GenericDepositAccounts<'info> {
                pub vault_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_input_token_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub clock: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for GenericDepositAccounts<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_input_token_account),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.clock),
                            false,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for GenericDepositAccounts<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_input_token_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.token_program,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.clock));
                    account_infos
                }
            }
        }
        impl<'info> GenericDepositAccounts<'info> {
            /// Compute the amount to deposit into the protocol
            pub fn amount_to_deposit(&self, protocol_idx: usize) -> Result<u64> {
                self.vault_account
                    .calculate_deposit(protocol_idx, self.vault_input_token_account.amount)
            }
        }
    }
    pub mod protocol_initialize {
        use anchor_lang::prelude::*;
        pub trait ProtocolInitialize<'info> {
            /// Initialize the protocol accounts
            fn cpi_initialize(&self) -> Result<()>;
        }
        /// Initialize the protocol accounts
        pub fn handler<'info, T: ProtocolInitialize<'info>>(ctx: Context<T>) -> Result<()> {
            ctx.accounts.cpi_initialize()
        }
    }
    pub mod protocol_withdraw {
        use crate::error::ErrorCode;
        use crate::protocols::Protocols;
        use crate::vault::{ProtocolData, VaultAccount};
        use crate::VAULT_ACCOUNT_SEED;
        use anchor_lang::prelude::*;
        use anchor_lang::solana_program::sysvar;
        use anchor_spl::token::{Token, TokenAccount};
        /// Withdraw from the protocol
        pub trait ProtocolWithdraw<'info> {
            /// Return the protcol position in the vector
            fn protocol_position(&self, protocol: Protocols) -> Result<usize>;
            /// Return a mutable refrence of the data
            fn protocol_data_as_mut(&mut self, protocol_idx: usize) -> &mut ProtocolData;
            /// Return the input token account
            fn input_token_account_as_mut(&mut self) -> &mut Account<'info, TokenAccount>;
            /// Compute the amount to deposit
            fn get_amount(&self, protocol_idx: usize) -> Result<u64>;
            /// Convert reserve liquidity to collateral (if any)
            fn liquidity_to_collateral(&self, amount: u64) -> Result<u64> {
                Ok(amount)
            }
            /// Withdraw from the protocol
            fn cpi_withdraw(&self, amount: u64) -> Result<()>;
        }
        /// Withdraw from the protocol and update protocol data
        pub fn handler<'info, T: ProtocolWithdraw<'info>>(
            ctx: Context<T>,
            protocol: Protocols,
        ) -> Result<()> {
            let protocol_idx = ctx.accounts.protocol_position(protocol)?;
            let amount = ctx.accounts.get_amount(protocol_idx)?;
            let mut lp_amount = ctx.accounts.liquidity_to_collateral(amount)?;
            if amount < ctx.accounts.protocol_data_as_mut(protocol_idx).amount {
                lp_amount = lp_amount.checked_add(1).ok_or_else(|| {
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                        error_name: ErrorCode::MathOverflow.name(),
                        error_code_number: ErrorCode::MathOverflow.into(),
                        error_msg: ErrorCode::MathOverflow.to_string(),
                        error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                            anchor_lang::error::Source {
                                filename:
                                    "programs/lending-arb/src/instructions/protocol_withdraw.rs",
                                line: 46u32,
                            },
                        )),
                        compared_values: None,
                    })
                })?;
            }
            let amount_before = {
                let input_token_account = ctx.accounts.input_token_account_as_mut();
                input_token_account.amount
            };
            ctx.accounts.cpi_withdraw(lp_amount)?;
            let amount_after = {
                let input_token_account = ctx.accounts.input_token_account_as_mut();
                input_token_account.reload()?;
                input_token_account.amount
            };
            let amount_diff = amount_after.checked_sub(amount_before).ok_or_else(|| {
                anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                    error_name: ErrorCode::MathOverflow.name(),
                    error_code_number: ErrorCode::MathOverflow.into(),
                    error_msg: ErrorCode::MathOverflow.to_string(),
                    error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                        anchor_lang::error::Source {
                            filename: "programs/lending-arb/src/instructions/protocol_withdraw.rs",
                            line: 64u32,
                        },
                    )),
                    compared_values: None,
                })
            })?;
            ctx.accounts
                .protocol_data_as_mut(protocol_idx)
                .update_after_withdraw(amount_diff)?;
            Ok(())
        }
        pub struct GenericWithdrawAccounts<'info> {
            # [account (mut , seeds = [VAULT_ACCOUNT_SEED , & [vault_account . seed_number] [..] , vault_account . input_mint_pubkey . as_ref ()] , bump = vault_account . bumps . vault)]
            pub vault_account: Box<Account<'info, VaultAccount>>,
            # [account (mut , associated_token :: mint = vault_account . input_mint_pubkey , associated_token :: authority = vault_account ,)]
            pub vault_input_token_account: Account<'info, TokenAccount>,
            pub token_program: Program<'info, Token>,
            pub clock: Sysvar<'info, Clock>,
            # [account (address = sysvar :: instructions :: ID)]
            /// CHECK: address is checked
            pub instructions: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for GenericWithdrawAccounts<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
            ) -> anchor_lang::Result<Self> {
                let vault_account: Box<anchor_lang::accounts::account::Account<VaultAccount>> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vault_account"))?;
                let vault_input_token_account: anchor_lang::accounts::account::Account<
                    TokenAccount,
                > = anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                    .map_err(|e| e.with_account_name("vault_input_token_account"))?;
                let token_program: anchor_lang::accounts::program::Program<Token> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("token_program"))?;
                let clock: Sysvar<Clock> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("clock"))?;
                let instructions: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("instructions"))?;
                let __pda_address = Pubkey::create_program_address(
                    &[
                        VAULT_ACCOUNT_SEED,
                        &[vault_account.seed_number][..],
                        vault_account.input_mint_pubkey.as_ref(),
                        &[vault_account.bumps.vault][..],
                    ],
                    &program_id,
                )
                .map_err(|_| {
                    anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintSeeds)
                        .with_account_name("vault_account")
                })?;
                if vault_account.key() != __pda_address {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("vault_account")
                    .with_pubkeys((vault_account.key(), __pda_address)));
                }
                if !vault_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_account"));
                }
                {
                    let my_owner = vault_input_token_account.owner;
                    let wallet_address = vault_account.key();
                    if my_owner != wallet_address {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintTokenOwner,
                        )
                        .with_account_name("vault_input_token_account")
                        .with_pubkeys((my_owner, wallet_address)));
                    }
                    let __associated_token_address =
                        anchor_spl::associated_token::get_associated_token_address(
                            &wallet_address,
                            &vault_account.input_mint_pubkey.key(),
                        );
                    let my_key = vault_input_token_account.key();
                    if my_key != __associated_token_address {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintAssociated,
                        )
                        .with_account_name("vault_input_token_account")
                        .with_pubkeys((my_key, __associated_token_address)));
                    }
                }
                if !vault_input_token_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_input_token_account"));
                }
                {
                    let actual = instructions.key();
                    let expected = sysvar::instructions::ID;
                    if actual != expected {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintAddress,
                        )
                        .with_account_name("instructions")
                        .with_pubkeys((actual, expected)));
                    }
                }
                Ok(GenericWithdrawAccounts {
                    vault_account,
                    vault_input_token_account,
                    token_program,
                    clock,
                    instructions,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for GenericWithdrawAccounts<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.vault_account.to_account_infos());
                account_infos.extend(self.vault_input_token_account.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos.extend(self.clock.to_account_infos());
                account_infos.extend(self.instructions.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for GenericWithdrawAccounts<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.vault_account.to_account_metas(None));
                account_metas.extend(self.vault_input_token_account.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas.extend(self.clock.to_account_metas(None));
                account_metas.extend(self.instructions.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for GenericWithdrawAccounts<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.vault_account, program_id)
                    .map_err(|e| e.with_account_name("vault_account"))?;
                anchor_lang::AccountsExit::exit(&self.vault_input_token_account, program_id)
                    .map_err(|e| e.with_account_name("vault_input_token_account"))?;
                Ok(())
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_generic_withdraw_accounts {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`GenericWithdrawAccounts`].
            pub struct GenericWithdrawAccounts {
                pub vault_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_input_token_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
                pub clock: anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: address is checked
                pub instructions: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for GenericWithdrawAccounts
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.vault_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.vault_input_token_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.clock, writer)?;
                    borsh::BorshSerialize::serialize(&self.instructions, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for GenericWithdrawAccounts {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_input_token_account,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.clock, false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.instructions,
                            false,
                        ),
                    );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_generic_withdraw_accounts {
            use super::*;
            /// Generated CPI struct of the accounts for [`GenericWithdrawAccounts`].
            pub struct GenericWithdrawAccounts<'info> {
                pub vault_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_input_token_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub clock: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: address is checked
                pub instructions: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for GenericWithdrawAccounts<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_input_token_account),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.clock),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.instructions),
                            false,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for GenericWithdrawAccounts<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_input_token_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.token_program,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.clock));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.instructions,
                    ));
                    account_infos
                }
            }
        }
        /// Anchor generated sighash
        const IX_WITHDRAW_SIGHASH: [u8; 8] = [183, 18, 70, 156, 148, 109, 161, 34];
        /// Instruction data length (sighash + u64)
        const IX_WITHDRAW_DATA_LEN: usize = 16;
        impl<'info> GenericWithdrawAccounts<'info> {
            /// Compute the amount to withdraw from the protocol depending on whether the instruction comes
            /// from the bot or from a user, assuming for the latter that the following ix corresponds to
            /// the `withdraw` one
            pub fn amount_to_withdraw(&self, protocol_idx: usize) -> Result<u64> {
                self.amount_to_withdraw_in_n_txs(protocol_idx, 1)
            }
            pub fn amount_to_withdraw_in_n_txs(
                &self,
                protocol_idx: usize,
                ix_offset: usize,
            ) -> Result<u64> {
                if let Some(amount) = self.read_amount_from_withdraw_ix(ix_offset)? {
                    Ok(amount)
                } else {
                    Ok(self.vault_account.calculate_withdraw(0, 0)?)
                }
            }
            /// Read the amount to withdraw from the target `withdraw` instruction
            fn read_amount_from_withdraw_ix(&self, target_ix: usize) -> Result<Option<u64>> {
                let current_index =
                    sysvar::instructions::load_current_index_checked(&self.instructions)? as usize;
                if let Ok(next_ix) = sysvar::instructions::load_instruction_at_checked(
                    current_index.checked_add(target_ix).unwrap(),
                    &self.instructions,
                ) {
                    let ix_data: &[u8] = &next_ix.data;
                    if !(next_ix.data.len() == IX_WITHDRAW_DATA_LEN
                        && ix_data[..8] == IX_WITHDRAW_SIGHASH)
                    {
                        return Err (anchor_lang :: error :: Error :: from (anchor_lang :: error :: AnchorError { error_name : ErrorCode :: InvalidInstructions . name () , error_code_number : ErrorCode :: InvalidInstructions . into () , error_msg : ErrorCode :: InvalidInstructions . to_string () , error_origin : Some (anchor_lang :: error :: ErrorOrigin :: Source (anchor_lang :: error :: Source { filename : "programs/lending-arb/src/instructions/protocol_withdraw.rs" , line : 129u32 , })) , compared_values : None , })) ;
                    };
                    use crate::instruction;
                    let ix = instruction::Withdraw::deserialize(&mut &ix_data[8..])
                        .map_err(|_| ErrorCode::InvalidInstructions)?;
                    let instruction::Withdraw { lp_amount } = ix;
                    let amount = self
                        .vault_account
                        .previous_lp_price
                        .lp_to_token(lp_amount)?;
                    let vault_token_amount = self.vault_input_token_account.amount;
                    if !(amount > vault_token_amount) {
                        return Err (anchor_lang :: error :: Error :: from (anchor_lang :: error :: AnchorError { error_name : ErrorCode :: InvalidInstructions . name () , error_code_number : ErrorCode :: InvalidInstructions . into () , error_msg : ErrorCode :: InvalidInstructions . to_string () , error_origin : Some (anchor_lang :: error :: ErrorOrigin :: Source (anchor_lang :: error :: Source { filename : "programs/lending-arb/src/instructions/protocol_withdraw.rs" , line : 145u32 , })) , compared_values : None , })) ;
                    };
                    Ok (Some (amount . checked_sub (vault_token_amount) . ok_or_else (| | anchor_lang :: error :: Error :: from (anchor_lang :: error :: AnchorError { error_name : ErrorCode :: MathOverflow . name () , error_code_number : ErrorCode :: MathOverflow . into () , error_msg : ErrorCode :: MathOverflow . to_string () , error_origin : Some (anchor_lang :: error :: ErrorOrigin :: Source (anchor_lang :: error :: Source { filename : "programs/lending-arb/src/instructions/protocol_withdraw.rs" , line : 150u32 , })) , compared_values : None , })) ?))
                } else {
                    Ok(None)
                }
            }
        }
    }
    pub mod protocol_borrow {
        use crate::protocols::Protocols;
        use crate::vault::{ProtocolData, VaultAccount};
        use crate::VAULT_ACCOUNT_SEED;
        use crate::health::Health;
        use anchor_lang::prelude::*;
        use anchor_spl::token::TokenAccount;
        use pyth_sdk_solana::{load_price_feed_from_account_info, PriceFeed, Price};
        /// Borrow from the protocol
        pub trait ProtocolBorrow<'info> {
            /// Return the protcol position in the vector
            fn protocol_position(&self, protocol: Protocols) -> Result<usize>;
            /// Return a mutable refrence of the data
            fn protocol_data_as_mut(&mut self, protocol_idx: usize) -> &mut ProtocolData;
            /// Compute the amount to borrow
            fn amount_to_borrow(&self) -> Result<u64>;
            /// Borrow from the protocol
            fn cpi_borrow(&self, amount: u64) -> Result<()>;
        }
        /// Borrow from protocol and update protocol data
        pub fn handler<'info, T: ProtocolBorrow<'info>>(
            ctx: Context<T>,
            protocol: Protocols,
        ) -> Result<()> {
            let protocol_idx = ctx.accounts.protocol_position(protocol)?;
            let amount = ctx.accounts.amount_to_borrow()?;
            ctx.accounts.cpi_borrow(amount)?;
            ctx.accounts
                .protocol_data_as_mut(protocol_idx)
                .update_after_borrow(amount)?;
            Ok(())
        }
        pub struct GenericBorrowAccounts<'info> {
            # [account (mut , seeds = [VAULT_ACCOUNT_SEED , & [vault_account . seed_number] [..] , vault_account . borrow_mint_pubkey . as_ref ()] , bump = vault_account . bumps . vault)]
            pub vault_account: Box<Account<'info, VaultAccount>>,
            # [account (mut , associated_token :: mint = vault_account . borrow_mint_pubkey , associated_token :: authority = vault_account ,)]
            pub vault_borrow_token_account: Account<'info, TokenAccount>,
            #[account()]
            pub price_account_info: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for GenericBorrowAccounts<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
            ) -> anchor_lang::Result<Self> {
                let vault_account: Box<anchor_lang::accounts::account::Account<VaultAccount>> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vault_account"))?;
                let vault_borrow_token_account: anchor_lang::accounts::account::Account<
                    TokenAccount,
                > = anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                    .map_err(|e| e.with_account_name("vault_borrow_token_account"))?;
                let price_account_info: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("price_account_info"))?;
                let __pda_address = Pubkey::create_program_address(
                    &[
                        VAULT_ACCOUNT_SEED,
                        &[vault_account.seed_number][..],
                        vault_account.borrow_mint_pubkey.as_ref(),
                        &[vault_account.bumps.vault][..],
                    ],
                    &program_id,
                )
                .map_err(|_| {
                    anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintSeeds)
                        .with_account_name("vault_account")
                })?;
                if vault_account.key() != __pda_address {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("vault_account")
                    .with_pubkeys((vault_account.key(), __pda_address)));
                }
                if !vault_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_account"));
                }
                {
                    let my_owner = vault_borrow_token_account.owner;
                    let wallet_address = vault_account.key();
                    if my_owner != wallet_address {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintTokenOwner,
                        )
                        .with_account_name("vault_borrow_token_account")
                        .with_pubkeys((my_owner, wallet_address)));
                    }
                    let __associated_token_address =
                        anchor_spl::associated_token::get_associated_token_address(
                            &wallet_address,
                            &vault_account.borrow_mint_pubkey.key(),
                        );
                    let my_key = vault_borrow_token_account.key();
                    if my_key != __associated_token_address {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintAssociated,
                        )
                        .with_account_name("vault_borrow_token_account")
                        .with_pubkeys((my_key, __associated_token_address)));
                    }
                }
                if !vault_borrow_token_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_borrow_token_account"));
                }
                Ok(GenericBorrowAccounts {
                    vault_account,
                    vault_borrow_token_account,
                    price_account_info,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for GenericBorrowAccounts<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.vault_account.to_account_infos());
                account_infos.extend(self.vault_borrow_token_account.to_account_infos());
                account_infos.extend(self.price_account_info.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for GenericBorrowAccounts<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.vault_account.to_account_metas(None));
                account_metas.extend(self.vault_borrow_token_account.to_account_metas(None));
                account_metas.extend(self.price_account_info.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for GenericBorrowAccounts<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.vault_account, program_id)
                    .map_err(|e| e.with_account_name("vault_account"))?;
                anchor_lang::AccountsExit::exit(&self.vault_borrow_token_account, program_id)
                    .map_err(|e| e.with_account_name("vault_borrow_token_account"))?;
                Ok(())
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_generic_borrow_accounts {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`GenericBorrowAccounts`].
            pub struct GenericBorrowAccounts {
                pub vault_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_borrow_token_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub price_account_info: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for GenericBorrowAccounts
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.vault_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.vault_borrow_token_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.price_account_info, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for GenericBorrowAccounts {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_borrow_token_account,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.price_account_info,
                            false,
                        ),
                    );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_generic_borrow_accounts {
            use super::*;
            /// Generated CPI struct of the accounts for [`GenericBorrowAccounts`].
            pub struct GenericBorrowAccounts<'info> {
                pub vault_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_borrow_token_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub price_account_info:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for GenericBorrowAccounts<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_borrow_token_account),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.price_account_info),
                            false,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for GenericBorrowAccounts<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_borrow_token_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.price_account_info,
                    ));
                    account_infos
                }
            }
        }
        impl<'info> GenericBorrowAccounts<'info> {
            fn price_feed(&self) -> Result<Price> {
                let price_feed: PriceFeed =
                    load_price_feed_from_account_info(&self.price_account_info.to_account_info())
                        .unwrap();
                let current_price: Price = price_feed.get_current_price().unwrap();
                Ok(current_price)
            }
        }
    }
    pub mod protocol_repay {
        use crate::protocols::Protocols;
        use crate::vault::{ProtocolData, VaultAccount};
        use crate::VAULT_ACCOUNT_SEED;
        use anchor_lang::prelude::*;
        use anchor_spl::token::{Token, TokenAccount};
        use crate::health::Health;
        /// Repay back to the protocol
        pub trait ProtocolRepay<'info> {
            /// Return the protcol position in the vector
            fn protocol_position(&self, protocol: Protocols) -> Result<usize>;
            /// Return a mutable refrence of the data
            fn protocol_data_as_mut(&mut self, protocol_idx: usize) -> &mut ProtocolData;
            /// Compute the amount to repay
            fn amount_to_repay(&self) -> Result<u64>;
            /// Borrow from the protocol
            fn cpi_repay(&self, amount: u64) -> Result<()>;
        }
        /// Borrow from protocol and update protocol data
        pub fn handler<'info, T: ProtocolRepay<'info>>(
            ctx: Context<T>,
            protocol: Protocols,
        ) -> Result<()> {
            let protocol_idx = ctx.accounts.protocol_position(protocol)?;
            let amount = ctx.accounts.amount_to_repay()?;
            ctx.accounts.cpi_repay(amount)?;
            ctx.accounts
                .protocol_data_as_mut(protocol_idx)
                .update_after_repay(amount)?;
            Ok(())
        }
        pub struct GenericRepayAccounts<'info> {
            # [account (mut , seeds = [VAULT_ACCOUNT_SEED , & [vault_account . seed_number] [..] , vault_account . borrow_mint_pubkey . as_ref ()] , bump = vault_account . bumps . vault)]
            pub vault_account: Box<Account<'info, VaultAccount>>,
            # [account (mut , associated_token :: mint = vault_account . borrow_mint_pubkey , associated_token :: authority = vault_account ,)]
            pub vault_borrow_token_account: Account<'info, TokenAccount>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for GenericRepayAccounts<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
            ) -> anchor_lang::Result<Self> {
                let vault_account: Box<anchor_lang::accounts::account::Account<VaultAccount>> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vault_account"))?;
                let vault_borrow_token_account: anchor_lang::accounts::account::Account<
                    TokenAccount,
                > = anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                    .map_err(|e| e.with_account_name("vault_borrow_token_account"))?;
                let __pda_address = Pubkey::create_program_address(
                    &[
                        VAULT_ACCOUNT_SEED,
                        &[vault_account.seed_number][..],
                        vault_account.borrow_mint_pubkey.as_ref(),
                        &[vault_account.bumps.vault][..],
                    ],
                    &program_id,
                )
                .map_err(|_| {
                    anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintSeeds)
                        .with_account_name("vault_account")
                })?;
                if vault_account.key() != __pda_address {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("vault_account")
                    .with_pubkeys((vault_account.key(), __pda_address)));
                }
                if !vault_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_account"));
                }
                {
                    let my_owner = vault_borrow_token_account.owner;
                    let wallet_address = vault_account.key();
                    if my_owner != wallet_address {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintTokenOwner,
                        )
                        .with_account_name("vault_borrow_token_account")
                        .with_pubkeys((my_owner, wallet_address)));
                    }
                    let __associated_token_address =
                        anchor_spl::associated_token::get_associated_token_address(
                            &wallet_address,
                            &vault_account.borrow_mint_pubkey.key(),
                        );
                    let my_key = vault_borrow_token_account.key();
                    if my_key != __associated_token_address {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintAssociated,
                        )
                        .with_account_name("vault_borrow_token_account")
                        .with_pubkeys((my_key, __associated_token_address)));
                    }
                }
                if !vault_borrow_token_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_borrow_token_account"));
                }
                Ok(GenericRepayAccounts {
                    vault_account,
                    vault_borrow_token_account,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for GenericRepayAccounts<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.vault_account.to_account_infos());
                account_infos.extend(self.vault_borrow_token_account.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for GenericRepayAccounts<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.vault_account.to_account_metas(None));
                account_metas.extend(self.vault_borrow_token_account.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for GenericRepayAccounts<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.vault_account, program_id)
                    .map_err(|e| e.with_account_name("vault_account"))?;
                anchor_lang::AccountsExit::exit(&self.vault_borrow_token_account, program_id)
                    .map_err(|e| e.with_account_name("vault_borrow_token_account"))?;
                Ok(())
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_generic_repay_accounts {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`GenericRepayAccounts`].
            pub struct GenericRepayAccounts {
                pub vault_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_borrow_token_account: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for GenericRepayAccounts
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.vault_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.vault_borrow_token_account, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for GenericRepayAccounts {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_borrow_token_account,
                        false,
                    ));
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_generic_repay_accounts {
            use super::*;
            /// Generated CPI struct of the accounts for [`GenericRepayAccounts`].
            pub struct GenericRepayAccounts<'info> {
                pub vault_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_borrow_token_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for GenericRepayAccounts<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_borrow_token_account),
                        false,
                    ));
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for GenericRepayAccounts<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_borrow_token_account,
                    ));
                    account_infos
                }
            }
        }
    }
    pub mod protocol_rewards {
        use crate::error::ErrorCode;
        use crate::protocols::Protocols;
        use crate::vault::ProtocolData;
        use crate::VaultAccount;
        use crate::VAULT_ACCOUNT_SEED;
        use anchor_lang::prelude::*;
        use solana_maths::WAD;
        use std::convert::TryFrom;
        use std::convert::TryInto;
        pub struct ProtocolRewardsEvent {
            protocol_id: u8,
            token: Pubkey,
            rewards: i64,
            lamports: u64,
            initial_slot: u64,
        }
        impl borsh::ser::BorshSerialize for ProtocolRewardsEvent
        where
            u8: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            i64: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.protocol_id, writer)?;
                borsh::BorshSerialize::serialize(&self.token, writer)?;
                borsh::BorshSerialize::serialize(&self.rewards, writer)?;
                borsh::BorshSerialize::serialize(&self.lamports, writer)?;
                borsh::BorshSerialize::serialize(&self.initial_slot, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for ProtocolRewardsEvent
        where
            u8: borsh::BorshDeserialize,
            Pubkey: borsh::BorshDeserialize,
            i64: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    protocol_id: borsh::BorshDeserialize::deserialize(buf)?,
                    token: borsh::BorshDeserialize::deserialize(buf)?,
                    rewards: borsh::BorshDeserialize::deserialize(buf)?,
                    lamports: borsh::BorshDeserialize::deserialize(buf)?,
                    initial_slot: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        impl anchor_lang::Event for ProtocolRewardsEvent {
            fn data(&self) -> Vec<u8> {
                let mut d = [254, 182, 197, 139, 244, 190, 213, 126].to_vec();
                d.append(&mut self.try_to_vec().unwrap());
                d
            }
        }
        impl anchor_lang::Discriminator for ProtocolRewardsEvent {
            fn discriminator() -> [u8; 8] {
                [254, 182, 197, 139, 244, 190, 213, 126]
            }
        }
        /// Get the rewards produced by the protocol
        pub trait ProtocolRewards<'info> {
            /// Return the protcol position in the vector
            fn protocol_position(&self, protocol: Protocols) -> Result<usize>;
            /// Get the input token mint pubkey
            fn input_mint_pubkey(&self) -> Pubkey;
            /// Return a mutable refrence of the data
            fn protocol_data_as_mut(&mut self, protocol_idx: usize) -> &mut ProtocolData;
            /// Compute the maximam withdrawable units
            fn max_withdrawable(&self) -> Result<u64>;
        }
        /// Update the rewards
        pub fn handler<'info, T: ProtocolRewards<'info>>(
            ctx: Context<T>,
            protocol: Protocols,
        ) -> Result<()> {
            let protocol_idx = ctx.accounts.protocol_position(protocol)?;
            let protocol_id: u8 = (protocol as usize).try_into().unwrap();
            let token = ctx.accounts.input_mint_pubkey();
            let tvl = ctx.accounts.max_withdrawable()?;
            let protocol_data = ctx.accounts.protocol_data_as_mut(protocol_idx);
            let rewards: i64 = i64::try_from(tvl)
                .unwrap()
                .checked_sub(i64::try_from(protocol_data.amount).unwrap())
                .unwrap();
            protocol_data
                .rewards
                .update(rewards, protocol_data.amount)?;
            let deposited_lamports: u64 = protocol_data
                .rewards
                .deposited_avg_wad
                .checked_div(WAD as u128)
                .ok_or_else(|| {
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                        error_name: ErrorCode::MathOverflow.name(),
                        error_code_number: ErrorCode::MathOverflow.into(),
                        error_msg: ErrorCode::MathOverflow.to_string(),
                        error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                            anchor_lang::error::Source {
                                filename:
                                    "programs/lending-arb/src/instructions/protocol_rewards.rs",
                                line: 60u32,
                            },
                        )),
                        compared_values: None,
                    })
                })?
                .try_into()
                .map_err(|_| ErrorCode::MathOverflow)?;
            {
                anchor_lang::solana_program::log::sol_log_data(&[&anchor_lang::Event::data(
                    &ProtocolRewardsEvent {
                        protocol_id,
                        token,
                        rewards: protocol_data.rewards.amount,
                        lamports: deposited_lamports,
                        initial_slot: protocol_data.rewards.deposited_integral.initial_slot,
                    },
                )]);
            };
            Ok(())
        }
        pub struct GenericTVLAccounts<'info> {
            # [account (mut , seeds = [VAULT_ACCOUNT_SEED , & [vault_account . seed_number] [..] , vault_account . input_mint_pubkey . as_ref ()] , bump = vault_account . bumps . vault)]
            pub vault_account: Box<Account<'info, VaultAccount>>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for GenericTVLAccounts<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
            ) -> anchor_lang::Result<Self> {
                let vault_account: Box<anchor_lang::accounts::account::Account<VaultAccount>> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vault_account"))?;
                let __pda_address = Pubkey::create_program_address(
                    &[
                        VAULT_ACCOUNT_SEED,
                        &[vault_account.seed_number][..],
                        vault_account.input_mint_pubkey.as_ref(),
                        &[vault_account.bumps.vault][..],
                    ],
                    &program_id,
                )
                .map_err(|_| {
                    anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintSeeds)
                        .with_account_name("vault_account")
                })?;
                if vault_account.key() != __pda_address {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("vault_account")
                    .with_pubkeys((vault_account.key(), __pda_address)));
                }
                if !vault_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_account"));
                }
                Ok(GenericTVLAccounts { vault_account })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for GenericTVLAccounts<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.vault_account.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for GenericTVLAccounts<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.vault_account.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for GenericTVLAccounts<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.vault_account, program_id)
                    .map_err(|e| e.with_account_name("vault_account"))?;
                Ok(())
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_generic_tvl_accounts {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`GenericTVLAccounts`].
            pub struct GenericTVLAccounts {
                pub vault_account: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for GenericTVLAccounts
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.vault_account, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for GenericTVLAccounts {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_account,
                        false,
                    ));
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_generic_tvl_accounts {
            use super::*;
            /// Generated CPI struct of the accounts for [`GenericTVLAccounts`].
            pub struct GenericTVLAccounts<'info> {
                pub vault_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for GenericTVLAccounts<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_account),
                        false,
                    ));
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for GenericTVLAccounts<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_account,
                    ));
                    account_infos
                }
            }
        }
    }
    pub use protocol_deposit::*;
    pub use protocol_initialize::*;
    pub use protocol_withdraw::*;
    pub use protocol_borrow::*;
    pub use protocol_repay::*;
    pub use protocol_rewards::*;
}
mod macros {
    pub(crate) use generate_seeds;
}
mod protocols {
    use std::convert::TryFrom;
    pub mod francium {
        use crate::check_hash::*;
        use crate::error::ErrorCode;
        use crate::instructions::{protocol_deposit::*, protocol_rewards::*, protocol_withdraw::*};
        use crate::macros::generate_seeds;
        use crate::protocols::{state::francium_lending_pool, Protocols};
        use crate::vault::ProtocolData;
        use anchor_lang::prelude::borsh::{BorshDeserialize, BorshSerialize};
        use anchor_lang::prelude::*;
        use anchor_lang::solana_program::{
            hash::{hashv, Hash},
            instruction::Instruction,
            program::invoke_signed,
            program_pack::Pack,
        };
        use anchor_spl::token::TokenAccount;
        /// Program ids
        pub mod francium_lending_program_id {
            use anchor_lang::declare_id;
            /// The static program ID
            pub static ID: anchor_lang::solana_program::pubkey::Pubkey =
                anchor_lang::solana_program::pubkey::Pubkey::new_from_array([
                    210u8, 220u8, 240u8, 41u8, 152u8, 7u8, 165u8, 233u8, 236u8, 32u8, 104u8, 56u8,
                    223u8, 125u8, 21u8, 70u8, 117u8, 132u8, 140u8, 128u8, 153u8, 214u8, 239u8,
                    220u8, 29u8, 7u8, 56u8, 74u8, 153u8, 55u8, 197u8, 120u8,
                ]);
            /// Confirms that a given pubkey is equivalent to the program ID
            pub fn check_id(id: &anchor_lang::solana_program::pubkey::Pubkey) -> bool {
                id == &ID
            }
            /// Returns the program ID
            pub fn id() -> anchor_lang::solana_program::pubkey::Pubkey {
                ID
            }
        }
        /// Instruction data
        pub struct InstructionData {
            pub instruction: u8,
        }
        impl borsh::de::BorshDeserialize for InstructionData
        where
            u8: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    instruction: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        impl borsh::ser::BorshSerialize for InstructionData
        where
            u8: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.instruction, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for InstructionData {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    InstructionData {
                        instruction: ref __self_0_0,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "InstructionData");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "instruction",
                            &&(*__self_0_0),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        /// Instruction data with amount
        pub struct InstructionAmountData {
            pub instruction: u8,
            pub amount: u64,
        }
        impl borsh::de::BorshDeserialize for InstructionAmountData
        where
            u8: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    instruction: borsh::BorshDeserialize::deserialize(buf)?,
                    amount: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        impl borsh::ser::BorshSerialize for InstructionAmountData
        where
            u8: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.instruction, writer)?;
                borsh::BorshSerialize::serialize(&self.amount, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for InstructionAmountData {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    InstructionAmountData {
                        instruction: ref __self_0_0,
                        amount: ref __self_0_1,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "InstructionAmountData");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "instruction",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "amount",
                            &&(*__self_0_1),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        pub struct FranciumDeposit<'info> {
            pub generic_accs: GenericDepositAccounts<'info>,
            # [account (constraint = francium_lending_program_id . key == & francium_lending_program_id :: ID)]
            /// CHECK: Francium CPI
            pub francium_lending_program_id: AccountInfo<'info>,
            # [account (mut , associated_token :: mint = vault_francium_collateral_token_account . mint , associated_token :: authority = generic_accs . vault_account ,)]
            pub vault_francium_collateral_token_account: Box<Account<'info, TokenAccount>>,
            #[account(mut)]
            /// CHECK: Francium CPI
            pub francium_lending_pool_info_account: AccountInfo<'info>,
            #[account(mut)]
            /// CHECK: Francium CPI
            pub francium_lending_pool_token_account: AccountInfo<'info>,
            #[account(mut)]
            /// CHECK: Francium CPI
            pub francium_farming_pool_stake_token_mint: AccountInfo<'info>,
            #[account(mut)]
            /// CHECK: Francium CPI
            pub francium_market_info_account: AccountInfo<'info>,
            #[account(mut)]
            /// CHECK: Francium CPI
            pub francium_lending_market_authority: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for FranciumDeposit<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
            ) -> anchor_lang::Result<Self> {
                let generic_accs: GenericDepositAccounts<'info> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)?;
                let francium_lending_program_id: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("francium_lending_program_id"))?;
                let vault_francium_collateral_token_account: Box<
                    anchor_lang::accounts::account::Account<TokenAccount>,
                > = anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                    .map_err(|e| e.with_account_name("vault_francium_collateral_token_account"))?;
                let francium_lending_pool_info_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("francium_lending_pool_info_account"))?;
                let francium_lending_pool_token_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("francium_lending_pool_token_account"))?;
                let francium_farming_pool_stake_token_mint: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| {
                            e.with_account_name("francium_farming_pool_stake_token_mint")
                        })?;
                let francium_market_info_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("francium_market_info_account"))?;
                let francium_lending_market_authority: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("francium_lending_market_authority"))?;
                if !(francium_lending_program_id.key == &francium_lending_program_id::ID) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("francium_lending_program_id"));
                }
                {
                    let my_owner = vault_francium_collateral_token_account.owner;
                    let wallet_address = generic_accs.vault_account.key();
                    if my_owner != wallet_address {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintTokenOwner,
                        )
                        .with_account_name("vault_francium_collateral_token_account")
                        .with_pubkeys((my_owner, wallet_address)));
                    }
                    let __associated_token_address =
                        anchor_spl::associated_token::get_associated_token_address(
                            &wallet_address,
                            &vault_francium_collateral_token_account.mint.key(),
                        );
                    let my_key = vault_francium_collateral_token_account.key();
                    if my_key != __associated_token_address {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintAssociated,
                        )
                        .with_account_name("vault_francium_collateral_token_account")
                        .with_pubkeys((my_key, __associated_token_address)));
                    }
                }
                if !vault_francium_collateral_token_account
                    .to_account_info()
                    .is_writable
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_francium_collateral_token_account"));
                }
                if !francium_lending_pool_info_account
                    .to_account_info()
                    .is_writable
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("francium_lending_pool_info_account"));
                }
                if !francium_lending_pool_token_account
                    .to_account_info()
                    .is_writable
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("francium_lending_pool_token_account"));
                }
                if !francium_farming_pool_stake_token_mint
                    .to_account_info()
                    .is_writable
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("francium_farming_pool_stake_token_mint"));
                }
                if !francium_market_info_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("francium_market_info_account"));
                }
                if !francium_lending_market_authority
                    .to_account_info()
                    .is_writable
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("francium_lending_market_authority"));
                }
                Ok(FranciumDeposit {
                    generic_accs,
                    francium_lending_program_id,
                    vault_francium_collateral_token_account,
                    francium_lending_pool_info_account,
                    francium_lending_pool_token_account,
                    francium_farming_pool_stake_token_mint,
                    francium_market_info_account,
                    francium_lending_market_authority,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for FranciumDeposit<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.generic_accs.to_account_infos());
                account_infos.extend(self.francium_lending_program_id.to_account_infos());
                account_infos.extend(
                    self.vault_francium_collateral_token_account
                        .to_account_infos(),
                );
                account_infos.extend(self.francium_lending_pool_info_account.to_account_infos());
                account_infos.extend(self.francium_lending_pool_token_account.to_account_infos());
                account_infos.extend(
                    self.francium_farming_pool_stake_token_mint
                        .to_account_infos(),
                );
                account_infos.extend(self.francium_market_info_account.to_account_infos());
                account_infos.extend(self.francium_lending_market_authority.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for FranciumDeposit<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.generic_accs.to_account_metas(None));
                account_metas.extend(self.francium_lending_program_id.to_account_metas(None));
                account_metas.extend(
                    self.vault_francium_collateral_token_account
                        .to_account_metas(None),
                );
                account_metas.extend(
                    self.francium_lending_pool_info_account
                        .to_account_metas(None),
                );
                account_metas.extend(
                    self.francium_lending_pool_token_account
                        .to_account_metas(None),
                );
                account_metas.extend(
                    self.francium_farming_pool_stake_token_mint
                        .to_account_metas(None),
                );
                account_metas.extend(self.francium_market_info_account.to_account_metas(None));
                account_metas.extend(
                    self.francium_lending_market_authority
                        .to_account_metas(None),
                );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for FranciumDeposit<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.generic_accs, program_id)
                    .map_err(|e| e.with_account_name("generic_accs"))?;
                anchor_lang::AccountsExit::exit(
                    &self.vault_francium_collateral_token_account,
                    program_id,
                )
                .map_err(|e| e.with_account_name("vault_francium_collateral_token_account"))?;
                anchor_lang::AccountsExit::exit(
                    &self.francium_lending_pool_info_account,
                    program_id,
                )
                .map_err(|e| e.with_account_name("francium_lending_pool_info_account"))?;
                anchor_lang::AccountsExit::exit(
                    &self.francium_lending_pool_token_account,
                    program_id,
                )
                .map_err(|e| e.with_account_name("francium_lending_pool_token_account"))?;
                anchor_lang::AccountsExit::exit(
                    &self.francium_farming_pool_stake_token_mint,
                    program_id,
                )
                .map_err(|e| e.with_account_name("francium_farming_pool_stake_token_mint"))?;
                anchor_lang::AccountsExit::exit(&self.francium_market_info_account, program_id)
                    .map_err(|e| e.with_account_name("francium_market_info_account"))?;
                anchor_lang::AccountsExit::exit(
                    &self.francium_lending_market_authority,
                    program_id,
                )
                .map_err(|e| e.with_account_name("francium_lending_market_authority"))?;
                Ok(())
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_francium_deposit {
            use super::*;
            use anchor_lang::prelude::borsh;
            pub use __client_accounts_generic_deposit_accounts::GenericDepositAccounts;
            /// Generated client accounts for [`FranciumDeposit`].
            pub struct FranciumDeposit {
                pub generic_accs:
                    __client_accounts_generic_deposit_accounts::GenericDepositAccounts,
                /// CHECK: Francium CPI
                pub francium_lending_program_id: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_francium_collateral_token_account:
                    anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: Francium CPI
                pub francium_lending_pool_info_account: anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: Francium CPI
                pub francium_lending_pool_token_account:
                    anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: Francium CPI
                pub francium_farming_pool_stake_token_mint:
                    anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: Francium CPI
                pub francium_market_info_account: anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: Francium CPI
                pub francium_lending_market_authority: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for FranciumDeposit
            where
                __client_accounts_generic_deposit_accounts::GenericDepositAccounts:
                    borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.generic_accs, writer)?;
                    borsh::BorshSerialize::serialize(&self.francium_lending_program_id, writer)?;
                    borsh::BorshSerialize::serialize(
                        &self.vault_francium_collateral_token_account,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(
                        &self.francium_lending_pool_info_account,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(
                        &self.francium_lending_pool_token_account,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(
                        &self.francium_farming_pool_stake_token_mint,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(&self.francium_market_info_account, writer)?;
                    borsh::BorshSerialize::serialize(
                        &self.francium_lending_market_authority,
                        writer,
                    )?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for FranciumDeposit {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.extend(self.generic_accs.to_account_metas(None));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.francium_lending_program_id,
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_francium_collateral_token_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.francium_lending_pool_info_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.francium_lending_pool_token_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.francium_farming_pool_stake_token_mint,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.francium_market_info_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.francium_lending_market_authority,
                        false,
                    ));
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_francium_deposit {
            use super::*;
            pub use __cpi_client_accounts_generic_deposit_accounts::GenericDepositAccounts;
            /// Generated CPI struct of the accounts for [`FranciumDeposit`].
            pub struct FranciumDeposit<'info> {
                pub generic_accs:
                    __cpi_client_accounts_generic_deposit_accounts::GenericDepositAccounts<'info>,
                /// CHECK: Francium CPI
                pub francium_lending_program_id:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_francium_collateral_token_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: Francium CPI
                pub francium_lending_pool_info_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: Francium CPI
                pub francium_lending_pool_token_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: Francium CPI
                pub francium_farming_pool_stake_token_mint:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: Francium CPI
                pub francium_market_info_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: Francium CPI
                pub francium_lending_market_authority:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for FranciumDeposit<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.extend(self.generic_accs.to_account_metas(None));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.francium_lending_program_id),
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_francium_collateral_token_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.francium_lending_pool_info_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.francium_lending_pool_token_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.francium_farming_pool_stake_token_mint),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.francium_market_info_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.francium_lending_market_authority),
                        false,
                    ));
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for FranciumDeposit<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(
                        &self.generic_accs,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.francium_lending_program_id,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_francium_collateral_token_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.francium_lending_pool_info_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.francium_lending_pool_token_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.francium_farming_pool_stake_token_mint,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.francium_market_info_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.francium_lending_market_authority,
                    ));
                    account_infos
                }
            }
        }
        impl<'info> CheckHash<'info> for FranciumDeposit<'info> {
            fn hash(&self) -> Hash {
                hashv(&[
                    self.vault_francium_collateral_token_account.key().as_ref(),
                    self.francium_lending_pool_info_account.key.as_ref(),
                    self.francium_lending_pool_token_account.key.as_ref(),
                    self.francium_farming_pool_stake_token_mint.key.as_ref(),
                    self.francium_market_info_account.key.as_ref(),
                    self.francium_lending_market_authority.key.as_ref(),
                ])
            }
            fn target_hash(&self, protocol: Protocols) -> [u8; CHECKHASH_BYTES] {
                let protocol_idx = self
                    .generic_accs
                    .vault_account
                    .protocol_position(protocol)
                    .unwrap();
                self.generic_accs.vault_account.protocols[protocol_idx]
                    .hash_pubkey
                    .hash_deposit
            }
        }
        impl<'info> ProtocolDeposit<'info> for FranciumDeposit<'info> {
            fn protocol_position(&self, protocol: Protocols) -> Result<usize> {
                self.generic_accs.vault_account.protocol_position(protocol)
            }
            fn protocol_data_as_mut(&mut self, protocol_idx: usize) -> &mut ProtocolData {
                &mut self.generic_accs.vault_account.protocols[protocol_idx]
            }
            fn get_amount(&self, protocol_idx: usize) -> Result<u64> {
                self.generic_accs.amount_to_deposit(protocol_idx)
            }
            fn cpi_deposit(&self, amount: u64) -> Result<()> {
                let seeds = &[
                    "vault".as_ref(),
                    &[self.generic_accs.vault_account.seed_number][..],
                    self.generic_accs.vault_account.input_mint_pubkey.as_ref(),
                    &[self.generic_accs.vault_account.bumps.vault],
                ];
                let signer = &[&seeds[..]];
                let accounts = [
                    self.generic_accs
                        .vault_input_token_account
                        .to_account_info(),
                    self.vault_francium_collateral_token_account
                        .to_account_info(),
                    self.francium_lending_pool_info_account.to_account_info(),
                    self.francium_lending_pool_token_account.to_account_info(),
                    self.francium_farming_pool_stake_token_mint
                        .to_account_info(),
                    self.francium_market_info_account.to_account_info(),
                    self.francium_lending_market_authority.to_account_info(),
                    self.generic_accs.vault_account.to_account_info(),
                    self.generic_accs.clock.to_account_info(),
                    self.generic_accs.token_program.to_account_info(),
                ];
                let account_metas = accounts
                    .iter()
                    .map(|acc| {
                        if acc.key == &self.generic_accs.vault_account.key() {
                            AccountMeta::new(*acc.key, true)
                        } else if acc.is_writable {
                            AccountMeta::new(*acc.key, false)
                        } else {
                            AccountMeta::new_readonly(*acc.key, false)
                        }
                    })
                    .collect::<Vec<_>>();
                let ix = Instruction::new_with_borsh(
                    francium_lending_program_id::ID,
                    &InstructionAmountData {
                        instruction: 4,
                        amount,
                    },
                    account_metas,
                );
                invoke_signed(&ix, &accounts, signer)?;
                Ok(())
            }
        }
        pub struct FranciumWithdraw<'info> {
            pub generic_accs: GenericWithdrawAccounts<'info>,
            # [account (constraint = francium_lending_program_id . key == & francium_lending_program_id :: ID)]
            /// CHECK: Francium CPI
            pub francium_lending_program_id: AccountInfo<'info>,
            # [account (mut , associated_token :: mint = vault_francium_collateral_token_account . mint , associated_token :: authority = generic_accs . vault_account ,)]
            pub vault_francium_collateral_token_account: Box<Account<'info, TokenAccount>>,
            #[account(mut)]
            /// CHECK: Francium CPI
            pub francium_lending_pool_info_account: AccountInfo<'info>,
            #[account(mut)]
            /// CHECK: Francium CPI
            pub francium_lending_pool_token_account: AccountInfo<'info>,
            #[account(mut)]
            /// CHECK: Francium CPI
            pub francium_farming_pool_stake_token_mint: AccountInfo<'info>,
            #[account(mut)]
            /// CHECK: Francium CPI
            pub francium_market_info_account: AccountInfo<'info>,
            #[account(mut)]
            /// CHECK: Francium CPI
            pub francium_lending_market_authority: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for FranciumWithdraw<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
            ) -> anchor_lang::Result<Self> {
                let generic_accs: GenericWithdrawAccounts<'info> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)?;
                let francium_lending_program_id: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("francium_lending_program_id"))?;
                let vault_francium_collateral_token_account: Box<
                    anchor_lang::accounts::account::Account<TokenAccount>,
                > = anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                    .map_err(|e| e.with_account_name("vault_francium_collateral_token_account"))?;
                let francium_lending_pool_info_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("francium_lending_pool_info_account"))?;
                let francium_lending_pool_token_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("francium_lending_pool_token_account"))?;
                let francium_farming_pool_stake_token_mint: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| {
                            e.with_account_name("francium_farming_pool_stake_token_mint")
                        })?;
                let francium_market_info_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("francium_market_info_account"))?;
                let francium_lending_market_authority: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("francium_lending_market_authority"))?;
                if !(francium_lending_program_id.key == &francium_lending_program_id::ID) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("francium_lending_program_id"));
                }
                {
                    let my_owner = vault_francium_collateral_token_account.owner;
                    let wallet_address = generic_accs.vault_account.key();
                    if my_owner != wallet_address {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintTokenOwner,
                        )
                        .with_account_name("vault_francium_collateral_token_account")
                        .with_pubkeys((my_owner, wallet_address)));
                    }
                    let __associated_token_address =
                        anchor_spl::associated_token::get_associated_token_address(
                            &wallet_address,
                            &vault_francium_collateral_token_account.mint.key(),
                        );
                    let my_key = vault_francium_collateral_token_account.key();
                    if my_key != __associated_token_address {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintAssociated,
                        )
                        .with_account_name("vault_francium_collateral_token_account")
                        .with_pubkeys((my_key, __associated_token_address)));
                    }
                }
                if !vault_francium_collateral_token_account
                    .to_account_info()
                    .is_writable
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_francium_collateral_token_account"));
                }
                if !francium_lending_pool_info_account
                    .to_account_info()
                    .is_writable
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("francium_lending_pool_info_account"));
                }
                if !francium_lending_pool_token_account
                    .to_account_info()
                    .is_writable
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("francium_lending_pool_token_account"));
                }
                if !francium_farming_pool_stake_token_mint
                    .to_account_info()
                    .is_writable
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("francium_farming_pool_stake_token_mint"));
                }
                if !francium_market_info_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("francium_market_info_account"));
                }
                if !francium_lending_market_authority
                    .to_account_info()
                    .is_writable
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("francium_lending_market_authority"));
                }
                Ok(FranciumWithdraw {
                    generic_accs,
                    francium_lending_program_id,
                    vault_francium_collateral_token_account,
                    francium_lending_pool_info_account,
                    francium_lending_pool_token_account,
                    francium_farming_pool_stake_token_mint,
                    francium_market_info_account,
                    francium_lending_market_authority,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for FranciumWithdraw<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.generic_accs.to_account_infos());
                account_infos.extend(self.francium_lending_program_id.to_account_infos());
                account_infos.extend(
                    self.vault_francium_collateral_token_account
                        .to_account_infos(),
                );
                account_infos.extend(self.francium_lending_pool_info_account.to_account_infos());
                account_infos.extend(self.francium_lending_pool_token_account.to_account_infos());
                account_infos.extend(
                    self.francium_farming_pool_stake_token_mint
                        .to_account_infos(),
                );
                account_infos.extend(self.francium_market_info_account.to_account_infos());
                account_infos.extend(self.francium_lending_market_authority.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for FranciumWithdraw<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.generic_accs.to_account_metas(None));
                account_metas.extend(self.francium_lending_program_id.to_account_metas(None));
                account_metas.extend(
                    self.vault_francium_collateral_token_account
                        .to_account_metas(None),
                );
                account_metas.extend(
                    self.francium_lending_pool_info_account
                        .to_account_metas(None),
                );
                account_metas.extend(
                    self.francium_lending_pool_token_account
                        .to_account_metas(None),
                );
                account_metas.extend(
                    self.francium_farming_pool_stake_token_mint
                        .to_account_metas(None),
                );
                account_metas.extend(self.francium_market_info_account.to_account_metas(None));
                account_metas.extend(
                    self.francium_lending_market_authority
                        .to_account_metas(None),
                );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for FranciumWithdraw<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.generic_accs, program_id)
                    .map_err(|e| e.with_account_name("generic_accs"))?;
                anchor_lang::AccountsExit::exit(
                    &self.vault_francium_collateral_token_account,
                    program_id,
                )
                .map_err(|e| e.with_account_name("vault_francium_collateral_token_account"))?;
                anchor_lang::AccountsExit::exit(
                    &self.francium_lending_pool_info_account,
                    program_id,
                )
                .map_err(|e| e.with_account_name("francium_lending_pool_info_account"))?;
                anchor_lang::AccountsExit::exit(
                    &self.francium_lending_pool_token_account,
                    program_id,
                )
                .map_err(|e| e.with_account_name("francium_lending_pool_token_account"))?;
                anchor_lang::AccountsExit::exit(
                    &self.francium_farming_pool_stake_token_mint,
                    program_id,
                )
                .map_err(|e| e.with_account_name("francium_farming_pool_stake_token_mint"))?;
                anchor_lang::AccountsExit::exit(&self.francium_market_info_account, program_id)
                    .map_err(|e| e.with_account_name("francium_market_info_account"))?;
                anchor_lang::AccountsExit::exit(
                    &self.francium_lending_market_authority,
                    program_id,
                )
                .map_err(|e| e.with_account_name("francium_lending_market_authority"))?;
                Ok(())
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_francium_withdraw {
            use super::*;
            use anchor_lang::prelude::borsh;
            pub use __client_accounts_generic_withdraw_accounts::GenericWithdrawAccounts;
            /// Generated client accounts for [`FranciumWithdraw`].
            pub struct FranciumWithdraw {
                pub generic_accs:
                    __client_accounts_generic_withdraw_accounts::GenericWithdrawAccounts,
                /// CHECK: Francium CPI
                pub francium_lending_program_id: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_francium_collateral_token_account:
                    anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: Francium CPI
                pub francium_lending_pool_info_account: anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: Francium CPI
                pub francium_lending_pool_token_account:
                    anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: Francium CPI
                pub francium_farming_pool_stake_token_mint:
                    anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: Francium CPI
                pub francium_market_info_account: anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: Francium CPI
                pub francium_lending_market_authority: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for FranciumWithdraw
            where
                __client_accounts_generic_withdraw_accounts::GenericWithdrawAccounts:
                    borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.generic_accs, writer)?;
                    borsh::BorshSerialize::serialize(&self.francium_lending_program_id, writer)?;
                    borsh::BorshSerialize::serialize(
                        &self.vault_francium_collateral_token_account,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(
                        &self.francium_lending_pool_info_account,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(
                        &self.francium_lending_pool_token_account,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(
                        &self.francium_farming_pool_stake_token_mint,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(&self.francium_market_info_account, writer)?;
                    borsh::BorshSerialize::serialize(
                        &self.francium_lending_market_authority,
                        writer,
                    )?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for FranciumWithdraw {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.extend(self.generic_accs.to_account_metas(None));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.francium_lending_program_id,
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_francium_collateral_token_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.francium_lending_pool_info_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.francium_lending_pool_token_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.francium_farming_pool_stake_token_mint,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.francium_market_info_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.francium_lending_market_authority,
                        false,
                    ));
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_francium_withdraw {
            use super::*;
            pub use __cpi_client_accounts_generic_withdraw_accounts::GenericWithdrawAccounts;
            /// Generated CPI struct of the accounts for [`FranciumWithdraw`].
            pub struct FranciumWithdraw<'info> {
                pub generic_accs:
                    __cpi_client_accounts_generic_withdraw_accounts::GenericWithdrawAccounts<'info>,
                /// CHECK: Francium CPI
                pub francium_lending_program_id:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_francium_collateral_token_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: Francium CPI
                pub francium_lending_pool_info_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: Francium CPI
                pub francium_lending_pool_token_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: Francium CPI
                pub francium_farming_pool_stake_token_mint:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: Francium CPI
                pub francium_market_info_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: Francium CPI
                pub francium_lending_market_authority:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for FranciumWithdraw<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.extend(self.generic_accs.to_account_metas(None));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.francium_lending_program_id),
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_francium_collateral_token_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.francium_lending_pool_info_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.francium_lending_pool_token_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.francium_farming_pool_stake_token_mint),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.francium_market_info_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.francium_lending_market_authority),
                        false,
                    ));
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for FranciumWithdraw<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(
                        &self.generic_accs,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.francium_lending_program_id,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_francium_collateral_token_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.francium_lending_pool_info_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.francium_lending_pool_token_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.francium_farming_pool_stake_token_mint,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.francium_market_info_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.francium_lending_market_authority,
                    ));
                    account_infos
                }
            }
        }
        impl<'info> CheckHash<'info> for FranciumWithdraw<'info> {
            fn hash(&self) -> Hash {
                hashv(&[
                    self.vault_francium_collateral_token_account.key().as_ref(),
                    self.francium_lending_pool_info_account.key.as_ref(),
                    self.francium_lending_pool_token_account.key.as_ref(),
                    self.francium_farming_pool_stake_token_mint.key.as_ref(),
                    self.francium_market_info_account.key.as_ref(),
                    self.francium_lending_market_authority.key.as_ref(),
                ])
            }
            fn target_hash(&self, protocol: Protocols) -> [u8; CHECKHASH_BYTES] {
                let protocol_idx = self
                    .generic_accs
                    .vault_account
                    .protocol_position(protocol)
                    .unwrap();
                self.generic_accs.vault_account.protocols[protocol_idx]
                    .hash_pubkey
                    .hash_withdraw
            }
        }
        impl<'info> ProtocolWithdraw<'info> for FranciumWithdraw<'info> {
            fn protocol_position(&self, protocol: Protocols) -> Result<usize> {
                self.generic_accs.vault_account.protocol_position(protocol)
            }
            fn protocol_data_as_mut(&mut self, protocol_idx: usize) -> &mut ProtocolData {
                &mut self.generic_accs.vault_account.protocols[protocol_idx]
            }
            fn input_token_account_as_mut(&mut self) -> &mut Account<'info, TokenAccount> {
                &mut self.generic_accs.vault_input_token_account
            }
            fn get_amount(&self, protocol_idx: usize) -> Result<u64> {
                self.generic_accs.amount_to_withdraw(protocol_idx)
            }
            fn liquidity_to_collateral(&self, amount: u64) -> Result<u64> {
                let lending_pool = francium_lending_pool::LendingPool::unpack(
                    &self.francium_lending_pool_info_account.data.borrow(),
                )?;
                let lp_amount = lending_pool
                    .collateral_exchange_rate()?
                    .liquidity_to_collateral(amount)?;
                Ok(lp_amount)
            }
            fn cpi_withdraw(&self, amount: u64) -> Result<()> {
                let seeds = &[
                    "vault".as_ref(),
                    &[self.generic_accs.vault_account.seed_number][..],
                    self.generic_accs.vault_account.input_mint_pubkey.as_ref(),
                    &[self.generic_accs.vault_account.bumps.vault],
                ];
                let signer = &[&seeds[..]];
                let accounts = [
                    self.vault_francium_collateral_token_account
                        .to_account_info(),
                    self.generic_accs
                        .vault_input_token_account
                        .to_account_info(),
                    self.francium_lending_pool_info_account.to_account_info(),
                    self.francium_farming_pool_stake_token_mint
                        .to_account_info(),
                    self.francium_lending_pool_token_account.to_account_info(),
                    self.francium_market_info_account.to_account_info(),
                    self.francium_lending_market_authority.to_account_info(),
                    self.generic_accs.vault_account.to_account_info(),
                    self.generic_accs.clock.to_account_info(),
                    self.generic_accs.token_program.to_account_info(),
                ];
                let account_metas = accounts
                    .iter()
                    .map(|acc| {
                        if acc.key == &self.generic_accs.vault_account.key() {
                            AccountMeta::new(*acc.key, true)
                        } else if acc.is_writable {
                            AccountMeta::new(*acc.key, false)
                        } else {
                            AccountMeta::new_readonly(*acc.key, false)
                        }
                    })
                    .collect::<Vec<_>>();
                let ix = Instruction::new_with_borsh(
                    francium_lending_program_id::ID,
                    &InstructionAmountData {
                        instruction: 5,
                        amount,
                    },
                    account_metas,
                );
                invoke_signed(&ix, &accounts, signer)?;
                Ok(())
            }
        }
        pub struct FranciumTVL<'info> {
            pub generic_accs: GenericTVLAccounts<'info>,
            # [account (owner = francium_lending_program_id :: ID)]
            /// CHECK: hash, owner and mint & collateral data fields are checked
            pub lending_pool: AccountInfo<'info>,
            # [account (associated_token :: mint = vault_francium_collateral_token_account . mint , associated_token :: authority = generic_accs . vault_account ,)]
            pub vault_francium_collateral_token_account: Account<'info, TokenAccount>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for FranciumTVL<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
            ) -> anchor_lang::Result<Self> {
                let generic_accs: GenericTVLAccounts<'info> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)?;
                let lending_pool: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("lending_pool"))?;
                let vault_francium_collateral_token_account : anchor_lang :: accounts :: account :: Account < TokenAccount > = anchor_lang :: Accounts :: try_accounts (program_id , accounts , ix_data , __bumps) . map_err (| e | e . with_account_name ("vault_francium_collateral_token_account")) ? ;
                {
                    let my_owner = AsRef::<AccountInfo>::as_ref(&lending_pool).owner;
                    let owner_address = francium_lending_program_id::ID;
                    if my_owner != &owner_address {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintOwner,
                        )
                        .with_account_name("lending_pool")
                        .with_pubkeys((*my_owner, owner_address)));
                    }
                }
                {
                    let my_owner = vault_francium_collateral_token_account.owner;
                    let wallet_address = generic_accs.vault_account.key();
                    if my_owner != wallet_address {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintTokenOwner,
                        )
                        .with_account_name("vault_francium_collateral_token_account")
                        .with_pubkeys((my_owner, wallet_address)));
                    }
                    let __associated_token_address =
                        anchor_spl::associated_token::get_associated_token_address(
                            &wallet_address,
                            &vault_francium_collateral_token_account.mint.key(),
                        );
                    let my_key = vault_francium_collateral_token_account.key();
                    if my_key != __associated_token_address {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintAssociated,
                        )
                        .with_account_name("vault_francium_collateral_token_account")
                        .with_pubkeys((my_key, __associated_token_address)));
                    }
                }
                Ok(FranciumTVL {
                    generic_accs,
                    lending_pool,
                    vault_francium_collateral_token_account,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for FranciumTVL<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.generic_accs.to_account_infos());
                account_infos.extend(self.lending_pool.to_account_infos());
                account_infos.extend(
                    self.vault_francium_collateral_token_account
                        .to_account_infos(),
                );
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for FranciumTVL<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.generic_accs.to_account_metas(None));
                account_metas.extend(self.lending_pool.to_account_metas(None));
                account_metas.extend(
                    self.vault_francium_collateral_token_account
                        .to_account_metas(None),
                );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for FranciumTVL<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.generic_accs, program_id)
                    .map_err(|e| e.with_account_name("generic_accs"))?;
                Ok(())
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_francium_tvl {
            use super::*;
            use anchor_lang::prelude::borsh;
            pub use __client_accounts_generic_tvl_accounts::GenericTVLAccounts;
            /// Generated client accounts for [`FranciumTVL`].
            pub struct FranciumTVL {
                pub generic_accs: __client_accounts_generic_tvl_accounts::GenericTVLAccounts,
                /// CHECK: hash, owner and mint & collateral data fields are checked
                pub lending_pool: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_francium_collateral_token_account:
                    anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for FranciumTVL
            where
                __client_accounts_generic_tvl_accounts::GenericTVLAccounts:
                    borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.generic_accs, writer)?;
                    borsh::BorshSerialize::serialize(&self.lending_pool, writer)?;
                    borsh::BorshSerialize::serialize(
                        &self.vault_francium_collateral_token_account,
                        writer,
                    )?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for FranciumTVL {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.extend(self.generic_accs.to_account_metas(None));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.lending_pool,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.vault_francium_collateral_token_account,
                            false,
                        ),
                    );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_francium_tvl {
            use super::*;
            pub use __cpi_client_accounts_generic_tvl_accounts::GenericTVLAccounts;
            /// Generated CPI struct of the accounts for [`FranciumTVL`].
            pub struct FranciumTVL<'info> {
                pub generic_accs:
                    __cpi_client_accounts_generic_tvl_accounts::GenericTVLAccounts<'info>,
                /// CHECK: hash, owner and mint & collateral data fields are checked
                pub lending_pool: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_francium_collateral_token_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for FranciumTVL<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.extend(self.generic_accs.to_account_metas(None));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.lending_pool),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.vault_francium_collateral_token_account),
                            false,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for FranciumTVL<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(
                        &self.generic_accs,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.lending_pool,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_francium_collateral_token_account,
                    ));
                    account_infos
                }
            }
        }
        impl<'info> CheckHash<'info> for FranciumTVL<'info> {
            fn hash(&self) -> Hash {
                hashv(&[
                    self.lending_pool.key.as_ref(),
                    self.vault_francium_collateral_token_account.key().as_ref(),
                ])
            }
            fn target_hash(&self, protocol: Protocols) -> [u8; CHECKHASH_BYTES] {
                let protocol_idx = self
                    .generic_accs
                    .vault_account
                    .protocol_position(protocol)
                    .unwrap();
                self.generic_accs.vault_account.protocols[protocol_idx]
                    .hash_pubkey
                    .hash_tvl
            }
        }
        impl<'info> ProtocolRewards<'info> for FranciumTVL<'info> {
            fn protocol_position(&self, protocol: Protocols) -> Result<usize> {
                self.generic_accs.vault_account.protocol_position(protocol)
            }
            fn input_mint_pubkey(&self) -> Pubkey {
                self.generic_accs.vault_account.input_mint_pubkey
            }
            fn protocol_data_as_mut(&mut self, protocol_idx: usize) -> &mut ProtocolData {
                &mut self.generic_accs.vault_account.protocols[protocol_idx]
            }
            fn max_withdrawable(&self) -> Result<u64> {
                let lending =
                    francium_lending_pool::LendingPool::unpack(&self.lending_pool.data.borrow())?;
                if !(lending.liquidity.mint_pubkey
                    == self.generic_accs.vault_account.input_mint_pubkey)
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::AnchorError {
                            error_name: ErrorCode::InvalidMint.name(),
                            error_code_number: ErrorCode::InvalidMint.into(),
                            error_msg: ErrorCode::InvalidMint.to_string(),
                            error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                                anchor_lang::error::Source {
                                    filename: "programs/lending-arb/src/protocols/francium.rs",
                                    line: 325u32,
                                },
                            )),
                            compared_values: None,
                        },
                    ));
                };
                if !(lending.share.mint_pubkey == self.vault_francium_collateral_token_account.mint)
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::AnchorError {
                            error_name: ErrorCode::InvalidMint.name(),
                            error_code_number: ErrorCode::InvalidMint.into(),
                            error_msg: ErrorCode::InvalidMint.to_string(),
                            error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                                anchor_lang::error::Source {
                                    filename: "programs/lending-arb/src/protocols/francium.rs",
                                    line: 330u32,
                                },
                            )),
                            compared_values: None,
                        },
                    ));
                };
                let lp_amount = self.vault_francium_collateral_token_account.amount;
                let tvl = lending
                    .collateral_exchange_rate()?
                    .collateral_to_liquidity(lp_amount)?;
                Ok(tvl)
            }
        }
    }
    pub mod solend {
        use crate::check_hash::*;
        use crate::error::ErrorCode;
        use crate::instructions::{
            protocol_initialize::*, protocol_deposit::*, protocol_withdraw::*, protocol_borrow::*,
            protocol_repay::*, protocol_rewards::*,
        };
        use crate::macros::generate_seeds;
        use crate::protocols::Protocols;
        use crate::vault::{ProtocolData, VaultAccount};
        use crate::VAULT_ACCOUNT_SEED;
        use crate::health::{MAX_HEALTH_FACTOR, MIN_HEALTH_FACTOR, OPTIMAL_HEALTH_FACTOR, Health};
        use anchor_lang::prelude::*;
        use anchor_lang::solana_program::{
            hash::{hashv, Hash},
            program::invoke_signed,
            program_pack::Pack,
            pubkey::Pubkey,
            instruction::Instruction,
        };
        use anchor_lang::solana_program::system_instruction;
        use anchor_spl::token::{Token, TokenAccount};
        use pyth_sdk_solana::{load_price_feed_from_account_info, PriceFeed, Price};
        use solend_token_lending::math::{TryMul, TrySub};
        /// Program id
        pub mod solend_program_id {
            use anchor_lang::declare_id;
            /// The static program ID
            pub static ID: anchor_lang::solana_program::pubkey::Pubkey =
                anchor_lang::solana_program::pubkey::Pubkey::new_from_array([
                    6u8, 155u8, 139u8, 152u8, 90u8, 171u8, 83u8, 42u8, 69u8, 9u8, 13u8, 232u8,
                    85u8, 127u8, 205u8, 220u8, 190u8, 108u8, 183u8, 239u8, 199u8, 58u8, 10u8,
                    101u8, 176u8, 111u8, 146u8, 3u8, 93u8, 183u8, 62u8, 236u8,
                ]);
            /// Confirms that a given pubkey is equivalent to the program ID
            pub fn check_id(id: &anchor_lang::solana_program::pubkey::Pubkey) -> bool {
                id == &ID
            }
            /// Returns the program ID
            pub fn id() -> anchor_lang::solana_program::pubkey::Pubkey {
                ID
            }
        }
        pub struct SolendInitialize<'info> {
            pub user_signer: Signer<'info>,
            # [account (seeds = [VAULT_ACCOUNT_SEED , vault_account . input_mint_pubkey . as_ref ()] , bump = vault_account . bumps . vault)]
            pub vault_account: Box<Account<'info, VaultAccount>>,
            #[account(mut)]
            /// CHECK: Solend CPI
            pub vault_solend_obligation_account: AccountInfo<'info>,
            /// CHECK: Solend CPI
            pub solend_lending_market_account: AccountInfo<'info>,
            # [account (constraint = solend_program_id . key == & solend_program_id :: ID)]
            /// CHECK: Solend CPI
            pub solend_program_id: AccountInfo<'info>,
            pub clock: Sysvar<'info, Clock>,
            pub rent: Sysvar<'info, Rent>,
            pub token_program: Program<'info, Token>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for SolendInitialize<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
            ) -> anchor_lang::Result<Self> {
                let user_signer: Signer =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("user_signer"))?;
                let vault_account: Box<anchor_lang::accounts::account::Account<VaultAccount>> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vault_account"))?;
                let vault_solend_obligation_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vault_solend_obligation_account"))?;
                let solend_lending_market_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("solend_lending_market_account"))?;
                let solend_program_id: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("solend_program_id"))?;
                let clock: Sysvar<Clock> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("clock"))?;
                let rent: Sysvar<Rent> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("rent"))?;
                let token_program: anchor_lang::accounts::program::Program<Token> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("token_program"))?;
                let system_program: anchor_lang::accounts::program::Program<System> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("system_program"))?;
                let __pda_address = Pubkey::create_program_address(
                    &[
                        VAULT_ACCOUNT_SEED,
                        vault_account.input_mint_pubkey.as_ref(),
                        &[vault_account.bumps.vault][..],
                    ],
                    &program_id,
                )
                .map_err(|_| {
                    anchor_lang::error::Error::from(anchor_lang::error::ErrorCode::ConstraintSeeds)
                        .with_account_name("vault_account")
                })?;
                if vault_account.key() != __pda_address {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("vault_account")
                    .with_pubkeys((vault_account.key(), __pda_address)));
                }
                if !vault_solend_obligation_account
                    .to_account_info()
                    .is_writable
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_solend_obligation_account"));
                }
                if !(solend_program_id.key == &solend_program_id::ID) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("solend_program_id"));
                }
                Ok(SolendInitialize {
                    user_signer,
                    vault_account,
                    vault_solend_obligation_account,
                    solend_lending_market_account,
                    solend_program_id,
                    clock,
                    rent,
                    token_program,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for SolendInitialize<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.user_signer.to_account_infos());
                account_infos.extend(self.vault_account.to_account_infos());
                account_infos.extend(self.vault_solend_obligation_account.to_account_infos());
                account_infos.extend(self.solend_lending_market_account.to_account_infos());
                account_infos.extend(self.solend_program_id.to_account_infos());
                account_infos.extend(self.clock.to_account_infos());
                account_infos.extend(self.rent.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for SolendInitialize<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.user_signer.to_account_metas(None));
                account_metas.extend(self.vault_account.to_account_metas(None));
                account_metas.extend(self.vault_solend_obligation_account.to_account_metas(None));
                account_metas.extend(self.solend_lending_market_account.to_account_metas(None));
                account_metas.extend(self.solend_program_id.to_account_metas(None));
                account_metas.extend(self.clock.to_account_metas(None));
                account_metas.extend(self.rent.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for SolendInitialize<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.vault_solend_obligation_account, program_id)
                    .map_err(|e| e.with_account_name("vault_solend_obligation_account"))?;
                Ok(())
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_solend_initialize {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`SolendInitialize`].
            pub struct SolendInitialize {
                pub user_signer: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_account: anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: Solend CPI
                pub vault_solend_obligation_account: anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: Solend CPI
                pub solend_lending_market_account: anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: Solend CPI
                pub solend_program_id: anchor_lang::solana_program::pubkey::Pubkey,
                pub clock: anchor_lang::solana_program::pubkey::Pubkey,
                pub rent: anchor_lang::solana_program::pubkey::Pubkey,
                pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
                pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for SolendInitialize
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.user_signer, writer)?;
                    borsh::BorshSerialize::serialize(&self.vault_account, writer)?;
                    borsh::BorshSerialize::serialize(
                        &self.vault_solend_obligation_account,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(&self.solend_lending_market_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.solend_program_id, writer)?;
                    borsh::BorshSerialize::serialize(&self.clock, writer)?;
                    borsh::BorshSerialize::serialize(&self.rent, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for SolendInitialize {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.user_signer,
                            true,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.vault_account,
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_solend_obligation_account,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.solend_lending_market_account,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.solend_program_id,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.clock, false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.rent, false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.system_program,
                            false,
                        ),
                    );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_solend_initialize {
            use super::*;
            /// Generated CPI struct of the accounts for [`SolendInitialize`].
            pub struct SolendInitialize<'info> {
                pub user_signer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: Solend CPI
                pub vault_solend_obligation_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: Solend CPI
                pub solend_lending_market_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: Solend CPI
                pub solend_program_id:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub clock: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub rent: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for SolendInitialize<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.user_signer),
                            true,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.vault_account),
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_solend_obligation_account),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.solend_lending_market_account),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.solend_program_id),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.clock),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.rent),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.system_program),
                            false,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for SolendInitialize<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.user_signer,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_solend_obligation_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.solend_lending_market_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.solend_program_id,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.clock));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.rent));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.token_program,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.system_program,
                    ));
                    account_infos
                }
            }
        }
        impl<'info> ProtocolInitialize<'info> for SolendInitialize<'info> {
            fn cpi_initialize(&self) -> Result<()> {
                let seeds = &[
                    "vault".as_ref(),
                    &[self.vault_account.seed_number][..],
                    self.vault_account.input_mint_pubkey.as_ref(),
                    &[self.vault_account.bumps.vault],
                ];
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
        pub struct SolendDeposit<'info> {
            pub generic_accs: GenericDepositAccounts<'info>,
            # [account (constraint = solend_program_id . key == & solend_program_id :: ID)]
            /// CHECK: Solend CPI
            pub solend_program_id: AccountInfo<'info>,
            # [account (mut , associated_token :: mint = vault_solend_destination_collateral_token_account . mint , associated_token :: authority = generic_accs . vault_account ,)]
            pub vault_solend_destination_collateral_token_account:
                Box<Account<'info, TokenAccount>>,
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
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for SolendDeposit<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
            ) -> anchor_lang::Result<Self> {
                let generic_accs: GenericDepositAccounts<'info> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)?;
                let solend_program_id: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("solend_program_id"))?;
                let vault_solend_destination_collateral_token_account: Box<
                    anchor_lang::accounts::account::Account<TokenAccount>,
                > = anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                    .map_err(|e| {
                        e.with_account_name("vault_solend_destination_collateral_token_account")
                    })?;
                let vault_solend_obligation_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vault_solend_obligation_account"))?;
                let solend_reserve_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("solend_reserve_account"))?;
                let solend_reserve_liquidity_supply_spl_token_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| {
                            e.with_account_name("solend_reserve_liquidity_supply_spl_token_account")
                        })?;
                let solend_reserve_collateral_spl_token_mint: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| {
                            e.with_account_name("solend_reserve_collateral_spl_token_mint")
                        })?;
                let solend_lending_market_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("solend_lending_market_account"))?;
                let solend_derived_lending_market_authority: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| {
                            e.with_account_name("solend_derived_lending_market_authority")
                        })?;
                let solend_destination_deposit_reserve_collateral_supply_spl_token_account : anchor_lang :: accounts :: account :: Account < TokenAccount > = anchor_lang :: Accounts :: try_accounts (program_id , accounts , ix_data , __bumps) . map_err (| e | e . with_account_name ("solend_destination_deposit_reserve_collateral_supply_spl_token_account")) ? ;
                let solend_pyth_price_oracle_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("solend_pyth_price_oracle_account"))?;
                let solend_switchboard_price_feed_oracle_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| {
                            e.with_account_name("solend_switchboard_price_feed_oracle_account")
                        })?;
                if !(solend_program_id.key == &solend_program_id::ID) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("solend_program_id"));
                }
                {
                    let my_owner = vault_solend_destination_collateral_token_account.owner;
                    let wallet_address = generic_accs.vault_account.key();
                    if my_owner != wallet_address {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintTokenOwner,
                        )
                        .with_account_name("vault_solend_destination_collateral_token_account")
                        .with_pubkeys((my_owner, wallet_address)));
                    }
                    let __associated_token_address =
                        anchor_spl::associated_token::get_associated_token_address(
                            &wallet_address,
                            &vault_solend_destination_collateral_token_account.mint.key(),
                        );
                    let my_key = vault_solend_destination_collateral_token_account.key();
                    if my_key != __associated_token_address {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintAssociated,
                        )
                        .with_account_name("vault_solend_destination_collateral_token_account")
                        .with_pubkeys((my_key, __associated_token_address)));
                    }
                }
                if !vault_solend_destination_collateral_token_account
                    .to_account_info()
                    .is_writable
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_solend_destination_collateral_token_account"));
                }
                if !vault_solend_obligation_account
                    .to_account_info()
                    .is_writable
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_solend_obligation_account"));
                }
                if !solend_reserve_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("solend_reserve_account"));
                }
                if !solend_reserve_liquidity_supply_spl_token_account
                    .to_account_info()
                    .is_writable
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("solend_reserve_liquidity_supply_spl_token_account"));
                }
                if !solend_reserve_collateral_spl_token_mint
                    .to_account_info()
                    .is_writable
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("solend_reserve_collateral_spl_token_mint"));
                }
                if !solend_destination_deposit_reserve_collateral_supply_spl_token_account
                    .to_account_info()
                    .is_writable
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name(
                        "solend_destination_deposit_reserve_collateral_supply_spl_token_account",
                    ));
                }
                Ok(SolendDeposit {
                    generic_accs,
                    solend_program_id,
                    vault_solend_destination_collateral_token_account,
                    vault_solend_obligation_account,
                    solend_reserve_account,
                    solend_reserve_liquidity_supply_spl_token_account,
                    solend_reserve_collateral_spl_token_mint,
                    solend_lending_market_account,
                    solend_derived_lending_market_authority,
                    solend_destination_deposit_reserve_collateral_supply_spl_token_account,
                    solend_pyth_price_oracle_account,
                    solend_switchboard_price_feed_oracle_account,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for SolendDeposit<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.generic_accs.to_account_infos());
                account_infos.extend(self.solend_program_id.to_account_infos());
                account_infos.extend(
                    self.vault_solend_destination_collateral_token_account
                        .to_account_infos(),
                );
                account_infos.extend(self.vault_solend_obligation_account.to_account_infos());
                account_infos.extend(self.solend_reserve_account.to_account_infos());
                account_infos.extend(
                    self.solend_reserve_liquidity_supply_spl_token_account
                        .to_account_infos(),
                );
                account_infos.extend(
                    self.solend_reserve_collateral_spl_token_mint
                        .to_account_infos(),
                );
                account_infos.extend(self.solend_lending_market_account.to_account_infos());
                account_infos.extend(
                    self.solend_derived_lending_market_authority
                        .to_account_infos(),
                );
                account_infos.extend(
                    self.solend_destination_deposit_reserve_collateral_supply_spl_token_account
                        .to_account_infos(),
                );
                account_infos.extend(self.solend_pyth_price_oracle_account.to_account_infos());
                account_infos.extend(
                    self.solend_switchboard_price_feed_oracle_account
                        .to_account_infos(),
                );
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for SolendDeposit<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.generic_accs.to_account_metas(None));
                account_metas.extend(self.solend_program_id.to_account_metas(None));
                account_metas.extend(
                    self.vault_solend_destination_collateral_token_account
                        .to_account_metas(None),
                );
                account_metas.extend(self.vault_solend_obligation_account.to_account_metas(None));
                account_metas.extend(self.solend_reserve_account.to_account_metas(None));
                account_metas.extend(
                    self.solend_reserve_liquidity_supply_spl_token_account
                        .to_account_metas(None),
                );
                account_metas.extend(
                    self.solend_reserve_collateral_spl_token_mint
                        .to_account_metas(None),
                );
                account_metas.extend(self.solend_lending_market_account.to_account_metas(None));
                account_metas.extend(
                    self.solend_derived_lending_market_authority
                        .to_account_metas(None),
                );
                account_metas.extend(
                    self.solend_destination_deposit_reserve_collateral_supply_spl_token_account
                        .to_account_metas(None),
                );
                account_metas.extend(self.solend_pyth_price_oracle_account.to_account_metas(None));
                account_metas.extend(
                    self.solend_switchboard_price_feed_oracle_account
                        .to_account_metas(None),
                );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for SolendDeposit<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.generic_accs, program_id)
                    .map_err(|e| e.with_account_name("generic_accs"))?;
                anchor_lang::AccountsExit::exit(
                    &self.vault_solend_destination_collateral_token_account,
                    program_id,
                )
                .map_err(|e| {
                    e.with_account_name("vault_solend_destination_collateral_token_account")
                })?;
                anchor_lang::AccountsExit::exit(&self.vault_solend_obligation_account, program_id)
                    .map_err(|e| e.with_account_name("vault_solend_obligation_account"))?;
                anchor_lang::AccountsExit::exit(&self.solend_reserve_account, program_id)
                    .map_err(|e| e.with_account_name("solend_reserve_account"))?;
                anchor_lang::AccountsExit::exit(
                    &self.solend_reserve_liquidity_supply_spl_token_account,
                    program_id,
                )
                .map_err(|e| {
                    e.with_account_name("solend_reserve_liquidity_supply_spl_token_account")
                })?;
                anchor_lang::AccountsExit::exit(
                    &self.solend_reserve_collateral_spl_token_mint,
                    program_id,
                )
                .map_err(|e| e.with_account_name("solend_reserve_collateral_spl_token_mint"))?;
                anchor_lang::AccountsExit::exit(
                    &self.solend_destination_deposit_reserve_collateral_supply_spl_token_account,
                    program_id,
                )
                .map_err(|e| {
                    e.with_account_name(
                        "solend_destination_deposit_reserve_collateral_supply_spl_token_account",
                    )
                })?;
                Ok(())
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_solend_deposit {
            use super::*;
            use anchor_lang::prelude::borsh;
            pub use __client_accounts_generic_deposit_accounts::GenericDepositAccounts;
            /// Generated client accounts for [`SolendDeposit`].
            pub struct SolendDeposit {
                pub generic_accs:
                    __client_accounts_generic_deposit_accounts::GenericDepositAccounts,
                /// CHECK: Solend CPI
                pub solend_program_id: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_solend_destination_collateral_token_account:
                    anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: Solend CPI
                pub vault_solend_obligation_account: anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: Solend CPI
                pub solend_reserve_account: anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: Solend CPI
                pub solend_reserve_liquidity_supply_spl_token_account:
                    anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: Solend CPI
                pub solend_reserve_collateral_spl_token_mint:
                    anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: Solend CPI
                pub solend_lending_market_account: anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: Solend CPI
                pub solend_derived_lending_market_authority:
                    anchor_lang::solana_program::pubkey::Pubkey,
                pub solend_destination_deposit_reserve_collateral_supply_spl_token_account:
                    anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: Solend CPI
                pub solend_pyth_price_oracle_account: anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: Solend CPI
                pub solend_switchboard_price_feed_oracle_account:
                    anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for SolendDeposit
            where
                __client_accounts_generic_deposit_accounts::GenericDepositAccounts:
                    borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.generic_accs, writer)?;
                    borsh::BorshSerialize::serialize(&self.solend_program_id, writer)?;
                    borsh::BorshSerialize::serialize(
                        &self.vault_solend_destination_collateral_token_account,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(
                        &self.vault_solend_obligation_account,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(&self.solend_reserve_account, writer)?;
                    borsh::BorshSerialize::serialize(
                        &self.solend_reserve_liquidity_supply_spl_token_account,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(
                        &self.solend_reserve_collateral_spl_token_mint,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(&self.solend_lending_market_account, writer)?;
                    borsh::BorshSerialize::serialize(
                        &self.solend_derived_lending_market_authority,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(
                        &self
                            .solend_destination_deposit_reserve_collateral_supply_spl_token_account,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(
                        &self.solend_pyth_price_oracle_account,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(
                        &self.solend_switchboard_price_feed_oracle_account,
                        writer,
                    )?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for SolendDeposit {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.extend(self.generic_accs.to_account_metas(None));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.solend_program_id,
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_solend_destination_collateral_token_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_solend_obligation_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.solend_reserve_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.solend_reserve_liquidity_supply_spl_token_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.solend_reserve_collateral_spl_token_mint,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.solend_lending_market_account,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.solend_derived_lending_market_authority,
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.solend_destination_deposit_reserve_collateral_supply_spl_token_account,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.solend_pyth_price_oracle_account,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.solend_switchboard_price_feed_oracle_account,
                            false,
                        ),
                    );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_solend_deposit {
            use super::*;
            pub use __cpi_client_accounts_generic_deposit_accounts::GenericDepositAccounts;
            /// Generated CPI struct of the accounts for [`SolendDeposit`].
            pub struct SolendDeposit<'info> {
                pub generic_accs:
                    __cpi_client_accounts_generic_deposit_accounts::GenericDepositAccounts<'info>,
                /// CHECK: Solend CPI
                pub solend_program_id:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_solend_destination_collateral_token_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: Solend CPI
                pub vault_solend_obligation_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: Solend CPI
                pub solend_reserve_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: Solend CPI
                pub solend_reserve_liquidity_supply_spl_token_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: Solend CPI
                pub solend_reserve_collateral_spl_token_mint:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: Solend CPI
                pub solend_lending_market_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: Solend CPI
                pub solend_derived_lending_market_authority:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub solend_destination_deposit_reserve_collateral_supply_spl_token_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: Solend CPI
                pub solend_pyth_price_oracle_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: Solend CPI
                pub solend_switchboard_price_feed_oracle_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for SolendDeposit<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.extend(self.generic_accs.to_account_metas(None));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.solend_program_id),
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(
                            &self.vault_solend_destination_collateral_token_account,
                        ),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_solend_obligation_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.solend_reserve_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(
                            &self.solend_reserve_liquidity_supply_spl_token_account,
                        ),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.solend_reserve_collateral_spl_token_mint),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.solend_lending_market_account),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.solend_derived_lending_market_authority),
                            false,
                        ),
                    );
                    account_metas . push (anchor_lang :: solana_program :: instruction :: AccountMeta :: new (anchor_lang :: Key :: key (& self . solend_destination_deposit_reserve_collateral_supply_spl_token_account) , false)) ;
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.solend_pyth_price_oracle_account),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(
                                &self.solend_switchboard_price_feed_oracle_account,
                            ),
                            false,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for SolendDeposit<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(
                        &self.generic_accs,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.solend_program_id,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_solend_destination_collateral_token_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_solend_obligation_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.solend_reserve_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.solend_reserve_liquidity_supply_spl_token_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.solend_reserve_collateral_spl_token_mint,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.solend_lending_market_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.solend_derived_lending_market_authority,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self
                            .solend_destination_deposit_reserve_collateral_supply_spl_token_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.solend_pyth_price_oracle_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.solend_switchboard_price_feed_oracle_account,
                    ));
                    account_infos
                }
            }
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
                let protocol_idx = self
                    .generic_accs
                    .vault_account
                    .protocol_position(protocol)
                    .unwrap();
                self.generic_accs.vault_account.protocols[protocol_idx]
                    .hash_pubkey
                    .hash_deposit
            }
        }
        impl<'info> ProtocolDeposit<'info> for SolendDeposit<'info> {
            fn protocol_position(&self, protocol: Protocols) -> Result<usize> {
                self.generic_accs.vault_account.protocol_position(protocol)
            }
            fn protocol_data_as_mut(&mut self, protocol_idx: usize) -> &mut ProtocolData {
                &mut self.generic_accs.vault_account.protocols[protocol_idx]
            }
            fn get_amount(&self, protocol_idx: usize) -> Result<u64> {
                self.generic_accs.amount_to_deposit(protocol_idx)
            }
            fn cpi_deposit(&self, amount: u64) -> Result<()> {
                let seeds = &[
                    "vault".as_ref(),
                    &[self.generic_accs.vault_account.seed_number][..],
                    self.generic_accs.vault_account.input_mint_pubkey.as_ref(),
                    &[self.generic_accs.vault_account.bumps.vault],
                ];
                let signer = &[&seeds[..]];
                let ix = solend_token_lending :: instruction :: deposit_reserve_liquidity_and_obligation_collateral (solend_program_id :: ID , amount , self . generic_accs . vault_input_token_account . key () , self . vault_solend_destination_collateral_token_account . key () , * self . solend_reserve_account . key , * self . solend_reserve_liquidity_supply_spl_token_account . key , * self . solend_reserve_collateral_spl_token_mint . key , * self . solend_lending_market_account . key , self . solend_destination_deposit_reserve_collateral_supply_spl_token_account . key () , * self . vault_solend_obligation_account . key , self . generic_accs . vault_account . key () , * self . solend_pyth_price_oracle_account . key , * self . solend_switchboard_price_feed_oracle_account . key , self . generic_accs . vault_account . key ()) ;
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
        pub struct SolendWithdraw<'info> {
            pub generic_accs: GenericWithdrawAccounts<'info>,
            # [account (constraint = solend_program_id . key == & solend_program_id :: ID)]
            /// CHECK: Solend CPI
            pub solend_program_id: AccountInfo<'info>,
            # [account (mut , associated_token :: mint = vault_solend_destination_collateral_token_account . mint , associated_token :: authority = generic_accs . vault_account ,)]
            pub vault_solend_destination_collateral_token_account: Account<'info, TokenAccount>,
            #[account(mut)]
            /// CHECK: Solend CPI
            pub vault_solend_obligation_account: AccountInfo<'info>,
            #[account(mut)]
            /// CHECK: Solend CPI
            pub solend_source_withdraw_reserve_collateral_supply_spl_token_account:
                AccountInfo<'info>,
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
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for SolendWithdraw<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
            ) -> anchor_lang::Result<Self> {
                let generic_accs: GenericWithdrawAccounts<'info> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)?;
                let solend_program_id: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("solend_program_id"))?;
                let vault_solend_destination_collateral_token_account : anchor_lang :: accounts :: account :: Account < TokenAccount > = anchor_lang :: Accounts :: try_accounts (program_id , accounts , ix_data , __bumps) . map_err (| e | e . with_account_name ("vault_solend_destination_collateral_token_account")) ? ;
                let vault_solend_obligation_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vault_solend_obligation_account"))?;
                let solend_source_withdraw_reserve_collateral_supply_spl_token_account : AccountInfo = anchor_lang :: Accounts :: try_accounts (program_id , accounts , ix_data , __bumps) . map_err (| e | e . with_account_name ("solend_source_withdraw_reserve_collateral_supply_spl_token_account")) ? ;
                let solend_withdraw_reserve_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("solend_withdraw_reserve_account"))?;
                let solend_lending_market_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("solend_lending_market_account"))?;
                let solend_derived_lending_market_authority: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| {
                            e.with_account_name("solend_derived_lending_market_authority")
                        })?;
                let solend_reserve_collateral_spl_token_mint: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| {
                            e.with_account_name("solend_reserve_collateral_spl_token_mint")
                        })?;
                let solend_reserve_liquidity_supply_spl_token_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| {
                            e.with_account_name("solend_reserve_liquidity_supply_spl_token_account")
                        })?;
                if !(solend_program_id.key == &solend_program_id::ID) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("solend_program_id"));
                }
                {
                    let my_owner = vault_solend_destination_collateral_token_account.owner;
                    let wallet_address = generic_accs.vault_account.key();
                    if my_owner != wallet_address {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintTokenOwner,
                        )
                        .with_account_name("vault_solend_destination_collateral_token_account")
                        .with_pubkeys((my_owner, wallet_address)));
                    }
                    let __associated_token_address =
                        anchor_spl::associated_token::get_associated_token_address(
                            &wallet_address,
                            &vault_solend_destination_collateral_token_account.mint.key(),
                        );
                    let my_key = vault_solend_destination_collateral_token_account.key();
                    if my_key != __associated_token_address {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintAssociated,
                        )
                        .with_account_name("vault_solend_destination_collateral_token_account")
                        .with_pubkeys((my_key, __associated_token_address)));
                    }
                }
                if !vault_solend_destination_collateral_token_account
                    .to_account_info()
                    .is_writable
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_solend_destination_collateral_token_account"));
                }
                if !vault_solend_obligation_account
                    .to_account_info()
                    .is_writable
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_solend_obligation_account"));
                }
                if !solend_source_withdraw_reserve_collateral_supply_spl_token_account
                    .to_account_info()
                    .is_writable
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name(
                        "solend_source_withdraw_reserve_collateral_supply_spl_token_account",
                    ));
                }
                if !solend_withdraw_reserve_account
                    .to_account_info()
                    .is_writable
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("solend_withdraw_reserve_account"));
                }
                if !solend_reserve_collateral_spl_token_mint
                    .to_account_info()
                    .is_writable
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("solend_reserve_collateral_spl_token_mint"));
                }
                if !solend_reserve_liquidity_supply_spl_token_account
                    .to_account_info()
                    .is_writable
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("solend_reserve_liquidity_supply_spl_token_account"));
                }
                Ok(SolendWithdraw {
                    generic_accs,
                    solend_program_id,
                    vault_solend_destination_collateral_token_account,
                    vault_solend_obligation_account,
                    solend_source_withdraw_reserve_collateral_supply_spl_token_account,
                    solend_withdraw_reserve_account,
                    solend_lending_market_account,
                    solend_derived_lending_market_authority,
                    solend_reserve_collateral_spl_token_mint,
                    solend_reserve_liquidity_supply_spl_token_account,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for SolendWithdraw<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.generic_accs.to_account_infos());
                account_infos.extend(self.solend_program_id.to_account_infos());
                account_infos.extend(
                    self.vault_solend_destination_collateral_token_account
                        .to_account_infos(),
                );
                account_infos.extend(self.vault_solend_obligation_account.to_account_infos());
                account_infos.extend(
                    self.solend_source_withdraw_reserve_collateral_supply_spl_token_account
                        .to_account_infos(),
                );
                account_infos.extend(self.solend_withdraw_reserve_account.to_account_infos());
                account_infos.extend(self.solend_lending_market_account.to_account_infos());
                account_infos.extend(
                    self.solend_derived_lending_market_authority
                        .to_account_infos(),
                );
                account_infos.extend(
                    self.solend_reserve_collateral_spl_token_mint
                        .to_account_infos(),
                );
                account_infos.extend(
                    self.solend_reserve_liquidity_supply_spl_token_account
                        .to_account_infos(),
                );
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for SolendWithdraw<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.generic_accs.to_account_metas(None));
                account_metas.extend(self.solend_program_id.to_account_metas(None));
                account_metas.extend(
                    self.vault_solend_destination_collateral_token_account
                        .to_account_metas(None),
                );
                account_metas.extend(self.vault_solend_obligation_account.to_account_metas(None));
                account_metas.extend(
                    self.solend_source_withdraw_reserve_collateral_supply_spl_token_account
                        .to_account_metas(None),
                );
                account_metas.extend(self.solend_withdraw_reserve_account.to_account_metas(None));
                account_metas.extend(self.solend_lending_market_account.to_account_metas(None));
                account_metas.extend(
                    self.solend_derived_lending_market_authority
                        .to_account_metas(None),
                );
                account_metas.extend(
                    self.solend_reserve_collateral_spl_token_mint
                        .to_account_metas(None),
                );
                account_metas.extend(
                    self.solend_reserve_liquidity_supply_spl_token_account
                        .to_account_metas(None),
                );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for SolendWithdraw<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.generic_accs, program_id)
                    .map_err(|e| e.with_account_name("generic_accs"))?;
                anchor_lang::AccountsExit::exit(
                    &self.vault_solend_destination_collateral_token_account,
                    program_id,
                )
                .map_err(|e| {
                    e.with_account_name("vault_solend_destination_collateral_token_account")
                })?;
                anchor_lang::AccountsExit::exit(&self.vault_solend_obligation_account, program_id)
                    .map_err(|e| e.with_account_name("vault_solend_obligation_account"))?;
                anchor_lang::AccountsExit::exit(
                    &self.solend_source_withdraw_reserve_collateral_supply_spl_token_account,
                    program_id,
                )
                .map_err(|e| {
                    e.with_account_name(
                        "solend_source_withdraw_reserve_collateral_supply_spl_token_account",
                    )
                })?;
                anchor_lang::AccountsExit::exit(&self.solend_withdraw_reserve_account, program_id)
                    .map_err(|e| e.with_account_name("solend_withdraw_reserve_account"))?;
                anchor_lang::AccountsExit::exit(
                    &self.solend_reserve_collateral_spl_token_mint,
                    program_id,
                )
                .map_err(|e| e.with_account_name("solend_reserve_collateral_spl_token_mint"))?;
                anchor_lang::AccountsExit::exit(
                    &self.solend_reserve_liquidity_supply_spl_token_account,
                    program_id,
                )
                .map_err(|e| {
                    e.with_account_name("solend_reserve_liquidity_supply_spl_token_account")
                })?;
                Ok(())
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_solend_withdraw {
            use super::*;
            use anchor_lang::prelude::borsh;
            pub use __client_accounts_generic_withdraw_accounts::GenericWithdrawAccounts;
            /// Generated client accounts for [`SolendWithdraw`].
            pub struct SolendWithdraw {
                pub generic_accs:
                    __client_accounts_generic_withdraw_accounts::GenericWithdrawAccounts,
                /// CHECK: Solend CPI
                pub solend_program_id: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_solend_destination_collateral_token_account:
                    anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: Solend CPI
                pub vault_solend_obligation_account: anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: Solend CPI
                pub solend_source_withdraw_reserve_collateral_supply_spl_token_account:
                    anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: Solend CPI
                pub solend_withdraw_reserve_account: anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: Solend CPI
                pub solend_lending_market_account: anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: Solend CPI
                pub solend_derived_lending_market_authority:
                    anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: Solend CPI
                pub solend_reserve_collateral_spl_token_mint:
                    anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: Solend CPI
                pub solend_reserve_liquidity_supply_spl_token_account:
                    anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for SolendWithdraw
            where
                __client_accounts_generic_withdraw_accounts::GenericWithdrawAccounts:
                    borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.generic_accs, writer)?;
                    borsh::BorshSerialize::serialize(&self.solend_program_id, writer)?;
                    borsh::BorshSerialize::serialize(
                        &self.vault_solend_destination_collateral_token_account,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(
                        &self.vault_solend_obligation_account,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(
                        &self.solend_source_withdraw_reserve_collateral_supply_spl_token_account,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(
                        &self.solend_withdraw_reserve_account,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(&self.solend_lending_market_account, writer)?;
                    borsh::BorshSerialize::serialize(
                        &self.solend_derived_lending_market_authority,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(
                        &self.solend_reserve_collateral_spl_token_mint,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(
                        &self.solend_reserve_liquidity_supply_spl_token_account,
                        writer,
                    )?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for SolendWithdraw {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.extend(self.generic_accs.to_account_metas(None));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.solend_program_id,
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_solend_destination_collateral_token_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_solend_obligation_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.solend_source_withdraw_reserve_collateral_supply_spl_token_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.solend_withdraw_reserve_account,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.solend_lending_market_account,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.solend_derived_lending_market_authority,
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.solend_reserve_collateral_spl_token_mint,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.solend_reserve_liquidity_supply_spl_token_account,
                        false,
                    ));
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_solend_withdraw {
            use super::*;
            pub use __cpi_client_accounts_generic_withdraw_accounts::GenericWithdrawAccounts;
            /// Generated CPI struct of the accounts for [`SolendWithdraw`].
            pub struct SolendWithdraw<'info> {
                pub generic_accs:
                    __cpi_client_accounts_generic_withdraw_accounts::GenericWithdrawAccounts<'info>,
                /// CHECK: Solend CPI
                pub solend_program_id:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_solend_destination_collateral_token_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: Solend CPI
                pub vault_solend_obligation_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: Solend CPI
                pub solend_source_withdraw_reserve_collateral_supply_spl_token_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: Solend CPI
                pub solend_withdraw_reserve_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: Solend CPI
                pub solend_lending_market_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: Solend CPI
                pub solend_derived_lending_market_authority:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: Solend CPI
                pub solend_reserve_collateral_spl_token_mint:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: Solend CPI
                pub solend_reserve_liquidity_supply_spl_token_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for SolendWithdraw<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.extend(self.generic_accs.to_account_metas(None));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.solend_program_id),
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(
                            &self.vault_solend_destination_collateral_token_account,
                        ),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_solend_obligation_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(
                            &self
                                .solend_source_withdraw_reserve_collateral_supply_spl_token_account,
                        ),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.solend_withdraw_reserve_account),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.solend_lending_market_account),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.solend_derived_lending_market_authority),
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.solend_reserve_collateral_spl_token_mint),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(
                            &self.solend_reserve_liquidity_supply_spl_token_account,
                        ),
                        false,
                    ));
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for SolendWithdraw<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(
                        &self.generic_accs,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.solend_program_id,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_solend_destination_collateral_token_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_solend_obligation_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.solend_source_withdraw_reserve_collateral_supply_spl_token_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.solend_withdraw_reserve_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.solend_lending_market_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.solend_derived_lending_market_authority,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.solend_reserve_collateral_spl_token_mint,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.solend_reserve_liquidity_supply_spl_token_account,
                    ));
                    account_infos
                }
            }
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
                let protocol_idx = self
                    .generic_accs
                    .vault_account
                    .protocol_position(protocol)
                    .unwrap();
                self.generic_accs.vault_account.protocols[protocol_idx]
                    .hash_pubkey
                    .hash_withdraw
            }
        }
        impl<'info> ProtocolWithdraw<'info> for SolendWithdraw<'info> {
            fn protocol_position(&self, protocol: Protocols) -> Result<usize> {
                self.generic_accs.vault_account.protocol_position(protocol)
            }
            fn protocol_data_as_mut(&mut self, protocol_idx: usize) -> &mut ProtocolData {
                &mut self.generic_accs.vault_account.protocols[protocol_idx]
            }
            fn input_token_account_as_mut(&mut self) -> &mut Account<'info, TokenAccount> {
                &mut self.generic_accs.vault_input_token_account
            }
            fn get_amount(&self, protocol_idx: usize) -> Result<u64> {
                self.generic_accs.amount_to_withdraw(protocol_idx)
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
                let seeds = &[
                    "vault".as_ref(),
                    &[self.generic_accs.vault_account.seed_number][..],
                    self.generic_accs.vault_account.input_mint_pubkey.as_ref(),
                    &[self.generic_accs.vault_account.bumps.vault],
                ];
                let signer = &[&seeds[..]];
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
                    data:
                        LendingInstruction::WithdrawObligationCollateralAndRedeemReserveCollateral {
                            collateral_amount: amount,
                        }
                        .pack(),
                };
                invoke_signed(&ix, &accounts, signer)?;
                Ok(())
            }
        }
        pub struct SolendBorrow<'info> {
            pub generic_accs: GenericBorrowAccounts<'info>,
            # [account (constraint = solend_program_id . key == & solend_program_id :: ID)]
            /// CHECK: Solend CPI
            pub solend_program_id: AccountInfo<'info>,
            pub vault_solend_borrow_token_account: Account<'info, TokenAccount>,
            #[account(mut)]
            /// CHECK: Solend CPI
            pub solend_reserve_account: AccountInfo<'info>,
            #[account(mut)]
            /// CHECK: Solend CPI
            pub borrow_reserve_liquidity_fee_receiver_pubkey: AccountInfo<'info>,
            #[account(mut)]
            /// CHECK: Solend CPI
            pub vault_solend_obligation_account: AccountInfo<'info>,
            /// CHECK: Solend CPI
            pub solend_lending_market_account: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for SolendBorrow<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
            ) -> anchor_lang::Result<Self> {
                let generic_accs: GenericBorrowAccounts<'info> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)?;
                let solend_program_id: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("solend_program_id"))?;
                let vault_solend_borrow_token_account: anchor_lang::accounts::account::Account<
                    TokenAccount,
                > = anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                    .map_err(|e| e.with_account_name("vault_solend_borrow_token_account"))?;
                let solend_reserve_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("solend_reserve_account"))?;
                let borrow_reserve_liquidity_fee_receiver_pubkey: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| {
                            e.with_account_name("borrow_reserve_liquidity_fee_receiver_pubkey")
                        })?;
                let vault_solend_obligation_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vault_solend_obligation_account"))?;
                let solend_lending_market_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("solend_lending_market_account"))?;
                if !(solend_program_id.key == &solend_program_id::ID) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("solend_program_id"));
                }
                if !solend_reserve_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("solend_reserve_account"));
                }
                if !borrow_reserve_liquidity_fee_receiver_pubkey
                    .to_account_info()
                    .is_writable
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("borrow_reserve_liquidity_fee_receiver_pubkey"));
                }
                if !vault_solend_obligation_account
                    .to_account_info()
                    .is_writable
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_solend_obligation_account"));
                }
                Ok(SolendBorrow {
                    generic_accs,
                    solend_program_id,
                    vault_solend_borrow_token_account,
                    solend_reserve_account,
                    borrow_reserve_liquidity_fee_receiver_pubkey,
                    vault_solend_obligation_account,
                    solend_lending_market_account,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for SolendBorrow<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.generic_accs.to_account_infos());
                account_infos.extend(self.solend_program_id.to_account_infos());
                account_infos.extend(self.vault_solend_borrow_token_account.to_account_infos());
                account_infos.extend(self.solend_reserve_account.to_account_infos());
                account_infos.extend(
                    self.borrow_reserve_liquidity_fee_receiver_pubkey
                        .to_account_infos(),
                );
                account_infos.extend(self.vault_solend_obligation_account.to_account_infos());
                account_infos.extend(self.solend_lending_market_account.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for SolendBorrow<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.generic_accs.to_account_metas(None));
                account_metas.extend(self.solend_program_id.to_account_metas(None));
                account_metas.extend(
                    self.vault_solend_borrow_token_account
                        .to_account_metas(None),
                );
                account_metas.extend(self.solend_reserve_account.to_account_metas(None));
                account_metas.extend(
                    self.borrow_reserve_liquidity_fee_receiver_pubkey
                        .to_account_metas(None),
                );
                account_metas.extend(self.vault_solend_obligation_account.to_account_metas(None));
                account_metas.extend(self.solend_lending_market_account.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for SolendBorrow<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.generic_accs, program_id)
                    .map_err(|e| e.with_account_name("generic_accs"))?;
                anchor_lang::AccountsExit::exit(&self.solend_reserve_account, program_id)
                    .map_err(|e| e.with_account_name("solend_reserve_account"))?;
                anchor_lang::AccountsExit::exit(
                    &self.borrow_reserve_liquidity_fee_receiver_pubkey,
                    program_id,
                )
                .map_err(|e| e.with_account_name("borrow_reserve_liquidity_fee_receiver_pubkey"))?;
                anchor_lang::AccountsExit::exit(&self.vault_solend_obligation_account, program_id)
                    .map_err(|e| e.with_account_name("vault_solend_obligation_account"))?;
                Ok(())
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_solend_borrow {
            use super::*;
            use anchor_lang::prelude::borsh;
            pub use __client_accounts_generic_borrow_accounts::GenericBorrowAccounts;
            /// Generated client accounts for [`SolendBorrow`].
            pub struct SolendBorrow {
                pub generic_accs: __client_accounts_generic_borrow_accounts::GenericBorrowAccounts,
                /// CHECK: Solend CPI
                pub solend_program_id: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_solend_borrow_token_account: anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: Solend CPI
                pub solend_reserve_account: anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: Solend CPI
                pub borrow_reserve_liquidity_fee_receiver_pubkey:
                    anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: Solend CPI
                pub vault_solend_obligation_account: anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: Solend CPI
                pub solend_lending_market_account: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for SolendBorrow
            where
                __client_accounts_generic_borrow_accounts::GenericBorrowAccounts:
                    borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.generic_accs, writer)?;
                    borsh::BorshSerialize::serialize(&self.solend_program_id, writer)?;
                    borsh::BorshSerialize::serialize(
                        &self.vault_solend_borrow_token_account,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(&self.solend_reserve_account, writer)?;
                    borsh::BorshSerialize::serialize(
                        &self.borrow_reserve_liquidity_fee_receiver_pubkey,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(
                        &self.vault_solend_obligation_account,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(&self.solend_lending_market_account, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for SolendBorrow {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.extend(self.generic_accs.to_account_metas(None));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.solend_program_id,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.vault_solend_borrow_token_account,
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.solend_reserve_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.borrow_reserve_liquidity_fee_receiver_pubkey,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_solend_obligation_account,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.solend_lending_market_account,
                            false,
                        ),
                    );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_solend_borrow {
            use super::*;
            pub use __cpi_client_accounts_generic_borrow_accounts::GenericBorrowAccounts;
            /// Generated CPI struct of the accounts for [`SolendBorrow`].
            pub struct SolendBorrow<'info> {
                pub generic_accs:
                    __cpi_client_accounts_generic_borrow_accounts::GenericBorrowAccounts<'info>,
                /// CHECK: Solend CPI
                pub solend_program_id:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_solend_borrow_token_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: Solend CPI
                pub solend_reserve_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: Solend CPI
                pub borrow_reserve_liquidity_fee_receiver_pubkey:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: Solend CPI
                pub vault_solend_obligation_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: Solend CPI
                pub solend_lending_market_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for SolendBorrow<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.extend(self.generic_accs.to_account_metas(None));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.solend_program_id),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.vault_solend_borrow_token_account),
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.solend_reserve_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.borrow_reserve_liquidity_fee_receiver_pubkey),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_solend_obligation_account),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.solend_lending_market_account),
                            false,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for SolendBorrow<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(
                        &self.generic_accs,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.solend_program_id,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_solend_borrow_token_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.solend_reserve_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.borrow_reserve_liquidity_fee_receiver_pubkey,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_solend_obligation_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.solend_lending_market_account,
                    ));
                    account_infos
                }
            }
        }
        impl<'info> CheckHash<'info> for SolendBorrow<'info> {
            fn hash(&self) -> Hash {
                hashv(&[
                    self.vault_solend_borrow_token_account.key().as_ref(),
                    self.solend_reserve_account.key.as_ref(),
                    self.borrow_reserve_liquidity_fee_receiver_pubkey
                        .key
                        .as_ref(),
                    self.vault_solend_obligation_account.key.as_ref(),
                    self.solend_lending_market_account.key.as_ref(),
                    self.solend_lending_market_account.key.as_ref(),
                ])
            }
            fn target_hash(&self, protocol: Protocols) -> [u8; CHECKHASH_BYTES] {
                let protocol_idx = self
                    .generic_accs
                    .vault_account
                    .protocol_position(protocol)
                    .unwrap();
                self.generic_accs.vault_account.protocols[protocol_idx]
                    .hash_pubkey
                    .hash_deposit
            }
        }
        impl<'info> ProtocolBorrow<'info> for SolendBorrow<'info> {
            fn protocol_position(&self, protocol: Protocols) -> Result<usize> {
                self.generic_accs.vault_account.protocol_position(protocol)
            }
            fn protocol_data_as_mut(&mut self, protocol_idx: usize) -> &mut ProtocolData {
                &mut self.generic_accs.vault_account.protocols[protocol_idx]
            }
            fn amount_to_borrow(&self) -> Result<u64> {
                let obligation = solend_token_lending::state::Obligation::unpack(
                    &self.vault_solend_obligation_account.data.borrow(),
                )?;
                get_health(obligation, Health::Keto);
                let optimal_amount = obligation
                    .allowed_borrow_value
                    .try_mul(OPTIMAL_HEALTH_FACTOR as u64)?;
                if !(optimal_amount.gt(&obligation.unhealthy_borrow_value)) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::AnchorError {
                            error_name: ErrorCode::UnhealthyOperation.name(),
                            error_code_number: ErrorCode::UnhealthyOperation.into(),
                            error_msg: ErrorCode::UnhealthyOperation.to_string(),
                            error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                                anchor_lang::error::Source {
                                    filename: "programs/lending-arb/src/protocols/solend.rs",
                                    line: 463u32,
                                },
                            )),
                            compared_values: None,
                        },
                    ));
                };
                let amount_to_borrow = optimal_amount.try_sub(obligation.borrowed_value)?;
                Ok(amount_to_borrow.try_ceil_u64()?)
            }
            fn cpi_borrow(&self, amount: u64) -> Result<()> {
                let seeds = &[
                    "vault".as_ref(),
                    &[self.generic_accs.vault_account.seed_number][..],
                    self.generic_accs.vault_account.input_mint_pubkey.as_ref(),
                    &[self.generic_accs.vault_account.bumps.vault],
                ];
                let signer = &[&seeds[..]];
                let ix = solend_token_lending::instruction::borrow_obligation_liquidity(
                    solend_program_id::ID,
                    amount,
                    self.vault_solend_borrow_token_account.key(),
                    self.generic_accs.vault_borrow_token_account.key(),
                    *self.solend_reserve_account.key,
                    *self.borrow_reserve_liquidity_fee_receiver_pubkey.key,
                    *self.vault_solend_obligation_account.key,
                    *self.solend_lending_market_account.key,
                    self.generic_accs.vault_account.key(),
                    Option::None,
                );
                let accounts = [
                    self.vault_solend_borrow_token_account.to_account_info(),
                    self.generic_accs
                        .vault_borrow_token_account
                        .to_account_info(),
                    self.solend_reserve_account.to_account_info(),
                    self.borrow_reserve_liquidity_fee_receiver_pubkey
                        .to_account_info(),
                    self.vault_solend_obligation_account.to_account_info(),
                    self.solend_lending_market_account.to_account_info(),
                    self.generic_accs.vault_account.to_account_info(),
                ];
                invoke_signed(&ix, &accounts, signer)?;
                Ok(())
            }
        }
        pub struct SolendRepay<'info> {
            pub generic_accs: GenericRepayAccounts<'info>,
            # [account (constraint = solend_program_id . key == & solend_program_id :: ID)]
            /// CHECK: Solend CPI
            pub solend_program_id: AccountInfo<'info>,
            pub vault_solend_borrow_token_account: Account<'info, TokenAccount>,
            #[account(mut)]
            /// CHECK: Solend CPI
            pub solend_reserve_account: AccountInfo<'info>,
            #[account(mut)]
            /// CHECK: Solend CPI
            pub vault_solend_obligation_account: AccountInfo<'info>,
            /// CHECK: Solend CPI
            pub solend_lending_market_account: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for SolendRepay<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
            ) -> anchor_lang::Result<Self> {
                let generic_accs: GenericRepayAccounts<'info> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)?;
                let solend_program_id: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("solend_program_id"))?;
                let vault_solend_borrow_token_account: anchor_lang::accounts::account::Account<
                    TokenAccount,
                > = anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                    .map_err(|e| e.with_account_name("vault_solend_borrow_token_account"))?;
                let solend_reserve_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("solend_reserve_account"))?;
                let vault_solend_obligation_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vault_solend_obligation_account"))?;
                let solend_lending_market_account: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("solend_lending_market_account"))?;
                if !(solend_program_id.key == &solend_program_id::ID) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("solend_program_id"));
                }
                if !solend_reserve_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("solend_reserve_account"));
                }
                if !vault_solend_obligation_account
                    .to_account_info()
                    .is_writable
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault_solend_obligation_account"));
                }
                Ok(SolendRepay {
                    generic_accs,
                    solend_program_id,
                    vault_solend_borrow_token_account,
                    solend_reserve_account,
                    vault_solend_obligation_account,
                    solend_lending_market_account,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for SolendRepay<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.generic_accs.to_account_infos());
                account_infos.extend(self.solend_program_id.to_account_infos());
                account_infos.extend(self.vault_solend_borrow_token_account.to_account_infos());
                account_infos.extend(self.solend_reserve_account.to_account_infos());
                account_infos.extend(self.vault_solend_obligation_account.to_account_infos());
                account_infos.extend(self.solend_lending_market_account.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for SolendRepay<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.generic_accs.to_account_metas(None));
                account_metas.extend(self.solend_program_id.to_account_metas(None));
                account_metas.extend(
                    self.vault_solend_borrow_token_account
                        .to_account_metas(None),
                );
                account_metas.extend(self.solend_reserve_account.to_account_metas(None));
                account_metas.extend(self.vault_solend_obligation_account.to_account_metas(None));
                account_metas.extend(self.solend_lending_market_account.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for SolendRepay<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.generic_accs, program_id)
                    .map_err(|e| e.with_account_name("generic_accs"))?;
                anchor_lang::AccountsExit::exit(&self.solend_reserve_account, program_id)
                    .map_err(|e| e.with_account_name("solend_reserve_account"))?;
                anchor_lang::AccountsExit::exit(&self.vault_solend_obligation_account, program_id)
                    .map_err(|e| e.with_account_name("vault_solend_obligation_account"))?;
                Ok(())
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_solend_repay {
            use super::*;
            use anchor_lang::prelude::borsh;
            pub use __client_accounts_generic_repay_accounts::GenericRepayAccounts;
            /// Generated client accounts for [`SolendRepay`].
            pub struct SolendRepay {
                pub generic_accs: __client_accounts_generic_repay_accounts::GenericRepayAccounts,
                /// CHECK: Solend CPI
                pub solend_program_id: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_solend_borrow_token_account: anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: Solend CPI
                pub solend_reserve_account: anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: Solend CPI
                pub vault_solend_obligation_account: anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: Solend CPI
                pub solend_lending_market_account: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for SolendRepay
            where
                __client_accounts_generic_repay_accounts::GenericRepayAccounts:
                    borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.generic_accs, writer)?;
                    borsh::BorshSerialize::serialize(&self.solend_program_id, writer)?;
                    borsh::BorshSerialize::serialize(
                        &self.vault_solend_borrow_token_account,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(&self.solend_reserve_account, writer)?;
                    borsh::BorshSerialize::serialize(
                        &self.vault_solend_obligation_account,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(&self.solend_lending_market_account, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for SolendRepay {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.extend(self.generic_accs.to_account_metas(None));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.solend_program_id,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.vault_solend_borrow_token_account,
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.solend_reserve_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault_solend_obligation_account,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.solend_lending_market_account,
                            false,
                        ),
                    );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_solend_repay {
            use super::*;
            pub use __cpi_client_accounts_generic_repay_accounts::GenericRepayAccounts;
            /// Generated CPI struct of the accounts for [`SolendRepay`].
            pub struct SolendRepay<'info> {
                pub generic_accs:
                    __cpi_client_accounts_generic_repay_accounts::GenericRepayAccounts<'info>,
                /// CHECK: Solend CPI
                pub solend_program_id:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_solend_borrow_token_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: Solend CPI
                pub solend_reserve_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: Solend CPI
                pub vault_solend_obligation_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: Solend CPI
                pub solend_lending_market_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for SolendRepay<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.extend(self.generic_accs.to_account_metas(None));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.solend_program_id),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.vault_solend_borrow_token_account),
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.solend_reserve_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault_solend_obligation_account),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.solend_lending_market_account),
                            false,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for SolendRepay<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(
                        &self.generic_accs,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.solend_program_id,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_solend_borrow_token_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.solend_reserve_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_solend_obligation_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.solend_lending_market_account,
                    ));
                    account_infos
                }
            }
        }
        impl<'info> CheckHash<'info> for SolendRepay<'info> {
            fn hash(&self) -> Hash {
                hashv(&[
                    self.vault_solend_borrow_token_account.key().as_ref(),
                    self.solend_reserve_account.key.as_ref(),
                    self.vault_solend_obligation_account.key.as_ref(),
                    self.solend_lending_market_account.key.as_ref(),
                ])
            }
            fn target_hash(&self, protocol: Protocols) -> [u8; CHECKHASH_BYTES] {
                let protocol_idx = self
                    .generic_accs
                    .vault_account
                    .protocol_position(protocol)
                    .unwrap();
                self.generic_accs.vault_account.protocols[protocol_idx]
                    .hash_pubkey
                    .hash_deposit
            }
        }
        impl<'info> ProtocolRepay<'info> for SolendRepay<'info> {
            fn protocol_position(&self, protocol: Protocols) -> Result<usize> {
                self.generic_accs.vault_account.protocol_position(protocol)
            }
            fn protocol_data_as_mut(&mut self, protocol_idx: usize) -> &mut ProtocolData {
                &mut self.generic_accs.vault_account.protocols[protocol_idx]
            }
            fn amount_to_repay(&self) -> Result<u64> {
                let obligation = solend_token_lending::state::Obligation::unpack(
                    &self.vault_solend_obligation_account.data.borrow(),
                )?;
                get_health(obligation, Health::Keto);
                Ok(0)
            }
            fn cpi_repay(&self, amount: u64) -> Result<()> {
                let seeds = &[
                    "vault".as_ref(),
                    &[self.generic_accs.vault_account.seed_number][..],
                    self.generic_accs.vault_account.input_mint_pubkey.as_ref(),
                    &[self.generic_accs.vault_account.bumps.vault],
                ];
                let signer = &[&seeds[..]];
                let ix = solend_token_lending::instruction::repay_obligation_liquidity(
                    solend_program_id::ID,
                    amount,
                    self.generic_accs.vault_borrow_token_account.key(),
                    self.vault_solend_borrow_token_account.key(),
                    *self.solend_reserve_account.key,
                    *self.vault_solend_obligation_account.key,
                    *self.solend_lending_market_account.key,
                    self.generic_accs.vault_account.key(),
                );
                let accounts = [
                    self.generic_accs
                        .vault_borrow_token_account
                        .to_account_info(),
                    self.vault_solend_borrow_token_account.to_account_info(),
                    self.solend_reserve_account.to_account_info(),
                    self.vault_solend_obligation_account.to_account_info(),
                    self.solend_lending_market_account.to_account_info(),
                    self.generic_accs.vault_account.to_account_info(),
                ];
                invoke_signed(&ix, &accounts, signer)?;
                Ok(())
            }
        }
        pub struct SolendTVL<'info> {
            pub generic_accs: GenericTVLAccounts<'info>,
            # [account (owner = solend_program_id :: ID)]
            /// CHECK: hash, owner and mint & collateral data fields are checked
            pub reserve: AccountInfo<'info>,
            # [account (associated_token :: mint = vault_solend_collateral_token_account . mint , associated_token :: authority = generic_accs . vault_account ,)]
            pub vault_solend_collateral_token_account: Account<'info, TokenAccount>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for SolendTVL<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
            ) -> anchor_lang::Result<Self> {
                let generic_accs: GenericTVLAccounts<'info> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)?;
                let reserve: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("reserve"))?;
                let vault_solend_collateral_token_account: anchor_lang::accounts::account::Account<
                    TokenAccount,
                > = anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                    .map_err(|e| e.with_account_name("vault_solend_collateral_token_account"))?;
                {
                    let my_owner = AsRef::<AccountInfo>::as_ref(&reserve).owner;
                    let owner_address = solend_program_id::ID;
                    if my_owner != &owner_address {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintOwner,
                        )
                        .with_account_name("reserve")
                        .with_pubkeys((*my_owner, owner_address)));
                    }
                }
                {
                    let my_owner = vault_solend_collateral_token_account.owner;
                    let wallet_address = generic_accs.vault_account.key();
                    if my_owner != wallet_address {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintTokenOwner,
                        )
                        .with_account_name("vault_solend_collateral_token_account")
                        .with_pubkeys((my_owner, wallet_address)));
                    }
                    let __associated_token_address =
                        anchor_spl::associated_token::get_associated_token_address(
                            &wallet_address,
                            &vault_solend_collateral_token_account.mint.key(),
                        );
                    let my_key = vault_solend_collateral_token_account.key();
                    if my_key != __associated_token_address {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintAssociated,
                        )
                        .with_account_name("vault_solend_collateral_token_account")
                        .with_pubkeys((my_key, __associated_token_address)));
                    }
                }
                Ok(SolendTVL {
                    generic_accs,
                    reserve,
                    vault_solend_collateral_token_account,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for SolendTVL<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.generic_accs.to_account_infos());
                account_infos.extend(self.reserve.to_account_infos());
                account_infos.extend(
                    self.vault_solend_collateral_token_account
                        .to_account_infos(),
                );
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for SolendTVL<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.generic_accs.to_account_metas(None));
                account_metas.extend(self.reserve.to_account_metas(None));
                account_metas.extend(
                    self.vault_solend_collateral_token_account
                        .to_account_metas(None),
                );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for SolendTVL<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.generic_accs, program_id)
                    .map_err(|e| e.with_account_name("generic_accs"))?;
                Ok(())
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_solend_tvl {
            use super::*;
            use anchor_lang::prelude::borsh;
            pub use __client_accounts_generic_tvl_accounts::GenericTVLAccounts;
            /// Generated client accounts for [`SolendTVL`].
            pub struct SolendTVL {
                pub generic_accs: __client_accounts_generic_tvl_accounts::GenericTVLAccounts,
                /// CHECK: hash, owner and mint & collateral data fields are checked
                pub reserve: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault_solend_collateral_token_account:
                    anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for SolendTVL
            where
                __client_accounts_generic_tvl_accounts::GenericTVLAccounts:
                    borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.generic_accs, writer)?;
                    borsh::BorshSerialize::serialize(&self.reserve, writer)?;
                    borsh::BorshSerialize::serialize(
                        &self.vault_solend_collateral_token_account,
                        writer,
                    )?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for SolendTVL {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.extend(self.generic_accs.to_account_metas(None));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.reserve,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.vault_solend_collateral_token_account,
                            false,
                        ),
                    );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_solend_tvl {
            use super::*;
            pub use __cpi_client_accounts_generic_tvl_accounts::GenericTVLAccounts;
            /// Generated CPI struct of the accounts for [`SolendTVL`].
            pub struct SolendTVL<'info> {
                pub generic_accs:
                    __cpi_client_accounts_generic_tvl_accounts::GenericTVLAccounts<'info>,
                /// CHECK: hash, owner and mint & collateral data fields are checked
                pub reserve: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault_solend_collateral_token_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for SolendTVL<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.extend(self.generic_accs.to_account_metas(None));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.reserve),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.vault_solend_collateral_token_account),
                            false,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for SolendTVL<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(
                        &self.generic_accs,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.reserve));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vault_solend_collateral_token_account,
                    ));
                    account_infos
                }
            }
        }
        impl<'info> CheckHash<'info> for SolendTVL<'info> {
            fn hash(&self) -> Hash {
                hashv(&[
                    self.reserve.key.as_ref(),
                    self.vault_solend_collateral_token_account.key().as_ref(),
                ])
            }
            fn target_hash(&self, protocol: Protocols) -> [u8; CHECKHASH_BYTES] {
                let protocol_idx = self
                    .generic_accs
                    .vault_account
                    .protocol_position(protocol)
                    .unwrap();
                self.generic_accs.vault_account.protocols[protocol_idx]
                    .hash_pubkey
                    .hash_tvl
            }
        }
        impl<'info> ProtocolRewards<'info> for SolendTVL<'info> {
            fn protocol_position(&self, protocol: Protocols) -> Result<usize> {
                self.generic_accs.vault_account.protocol_position(protocol)
            }
            fn input_mint_pubkey(&self) -> Pubkey {
                self.generic_accs.vault_account.input_mint_pubkey
            }
            fn protocol_data_as_mut(&mut self, protocol_idx: usize) -> &mut ProtocolData {
                &mut self.generic_accs.vault_account.protocols[protocol_idx]
            }
            fn max_withdrawable(&self) -> Result<u64> {
                let reserve =
                    solend_token_lending::state::Reserve::unpack(&self.reserve.data.borrow())?;
                if !(reserve.liquidity.mint_pubkey
                    == self.generic_accs.vault_account.input_mint_pubkey)
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::AnchorError {
                            error_name: ErrorCode::InvalidMint.name(),
                            error_code_number: ErrorCode::InvalidMint.into(),
                            error_msg: ErrorCode::InvalidMint.to_string(),
                            error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                                anchor_lang::error::Source {
                                    filename: "programs/lending-arb/src/protocols/solend.rs",
                                    line: 649u32,
                                },
                            )),
                            compared_values: None,
                        },
                    ));
                };
                if !(reserve.collateral.mint_pubkey
                    == self.vault_solend_collateral_token_account.mint)
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::AnchorError {
                            error_name: ErrorCode::InvalidMint.name(),
                            error_code_number: ErrorCode::InvalidMint.into(),
                            error_msg: ErrorCode::InvalidMint.to_string(),
                            error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                                anchor_lang::error::Source {
                                    filename: "programs/lending-arb/src/protocols/solend.rs",
                                    line: 654u32,
                                },
                            )),
                            compared_values: None,
                        },
                    ));
                };
                let lp_amount = self.vault_solend_collateral_token_account.amount;
                let tvl = reserve
                    .collateral_exchange_rate()?
                    .collateral_to_liquidity(lp_amount)?;
                Ok(tvl)
            }
        }
        fn get_health(
            obligation: solend_token_lending::state::Obligation,
            required_health: Health,
        ) -> Result<Health> {
            if obligation.allowed_borrow_value.le(
                &solend_token_lending::math::Decimal::from_scaled_val(MIN_HEALTH_FACTOR),
            ) {
                Ok(Health::Keto)
            } else if obligation.allowed_borrow_value.ge(
                &solend_token_lending::math::Decimal::from_scaled_val(MAX_HEALTH_FACTOR),
            ) {
                Ok(Health::Vegetarian)
            } else {
                Ok(Health::Vegan)
            }
        }
    }
    pub mod state {
        use anchor_lang::solana_program::{
            clock::Slot,
            msg,
            program_error::ProgramError,
            program_option::COption,
            pubkey::{Pubkey, PUBKEY_BYTES},
        };
        use arrayref::array_refs;
        use solana_maths::Decimal;
        pub mod francium_lending_pool {
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
            /// Lending market reserve state
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
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::clone::Clone for LendingPool {
                #[inline]
                fn clone(&self) -> LendingPool {
                    match *self {
                        LendingPool {
                            version: ref __self_0_0,
                            last_update: ref __self_0_1,
                            lending_market: ref __self_0_2,
                            liquidity: ref __self_0_3,
                            share: ref __self_0_4,
                            credit_mint_pubkey: ref __self_0_5,
                            credit_mint_total_supply: ref __self_0_6,
                            credit_supply_pubkey: ref __self_0_7,
                            threshold_1: ref __self_0_8,
                            threshold_2: ref __self_0_9,
                            base_1: ref __self_0_10,
                            factor_1: ref __self_0_11,
                            base_2: ref __self_0_12,
                            factor_2: ref __self_0_13,
                            base_3: ref __self_0_14,
                            factor_3: ref __self_0_15,
                            interest_reverse_rate: ref __self_0_16,
                            accumulated_interest_reverse: ref __self_0_17,
                        } => LendingPool {
                            version: ::core::clone::Clone::clone(&(*__self_0_0)),
                            last_update: ::core::clone::Clone::clone(&(*__self_0_1)),
                            lending_market: ::core::clone::Clone::clone(&(*__self_0_2)),
                            liquidity: ::core::clone::Clone::clone(&(*__self_0_3)),
                            share: ::core::clone::Clone::clone(&(*__self_0_4)),
                            credit_mint_pubkey: ::core::clone::Clone::clone(&(*__self_0_5)),
                            credit_mint_total_supply: ::core::clone::Clone::clone(&(*__self_0_6)),
                            credit_supply_pubkey: ::core::clone::Clone::clone(&(*__self_0_7)),
                            threshold_1: ::core::clone::Clone::clone(&(*__self_0_8)),
                            threshold_2: ::core::clone::Clone::clone(&(*__self_0_9)),
                            base_1: ::core::clone::Clone::clone(&(*__self_0_10)),
                            factor_1: ::core::clone::Clone::clone(&(*__self_0_11)),
                            base_2: ::core::clone::Clone::clone(&(*__self_0_12)),
                            factor_2: ::core::clone::Clone::clone(&(*__self_0_13)),
                            base_3: ::core::clone::Clone::clone(&(*__self_0_14)),
                            factor_3: ::core::clone::Clone::clone(&(*__self_0_15)),
                            interest_reverse_rate: ::core::clone::Clone::clone(&(*__self_0_16)),
                            accumulated_interest_reverse: ::core::clone::Clone::clone(
                                &(*__self_0_17),
                            ),
                        },
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for LendingPool {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        LendingPool {
                            version: ref __self_0_0,
                            last_update: ref __self_0_1,
                            lending_market: ref __self_0_2,
                            liquidity: ref __self_0_3,
                            share: ref __self_0_4,
                            credit_mint_pubkey: ref __self_0_5,
                            credit_mint_total_supply: ref __self_0_6,
                            credit_supply_pubkey: ref __self_0_7,
                            threshold_1: ref __self_0_8,
                            threshold_2: ref __self_0_9,
                            base_1: ref __self_0_10,
                            factor_1: ref __self_0_11,
                            base_2: ref __self_0_12,
                            factor_2: ref __self_0_13,
                            base_3: ref __self_0_14,
                            factor_3: ref __self_0_15,
                            interest_reverse_rate: ref __self_0_16,
                            accumulated_interest_reverse: ref __self_0_17,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "LendingPool");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "version",
                                &&(*__self_0_0),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "last_update",
                                &&(*__self_0_1),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "lending_market",
                                &&(*__self_0_2),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "liquidity",
                                &&(*__self_0_3),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "share",
                                &&(*__self_0_4),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "credit_mint_pubkey",
                                &&(*__self_0_5),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "credit_mint_total_supply",
                                &&(*__self_0_6),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "credit_supply_pubkey",
                                &&(*__self_0_7),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "threshold_1",
                                &&(*__self_0_8),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "threshold_2",
                                &&(*__self_0_9),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "base_1",
                                &&(*__self_0_10),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "factor_1",
                                &&(*__self_0_11),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "base_2",
                                &&(*__self_0_12),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "factor_2",
                                &&(*__self_0_13),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "base_3",
                                &&(*__self_0_14),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "factor_3",
                                &&(*__self_0_15),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "interest_reverse_rate",
                                &&(*__self_0_16),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "accumulated_interest_reverse",
                                &&(*__self_0_17),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::default::Default for LendingPool {
                #[inline]
                fn default() -> LendingPool {
                    LendingPool {
                        version: ::core::default::Default::default(),
                        last_update: ::core::default::Default::default(),
                        lending_market: ::core::default::Default::default(),
                        liquidity: ::core::default::Default::default(),
                        share: ::core::default::Default::default(),
                        credit_mint_pubkey: ::core::default::Default::default(),
                        credit_mint_total_supply: ::core::default::Default::default(),
                        credit_supply_pubkey: ::core::default::Default::default(),
                        threshold_1: ::core::default::Default::default(),
                        threshold_2: ::core::default::Default::default(),
                        base_1: ::core::default::Default::default(),
                        factor_1: ::core::default::Default::default(),
                        base_2: ::core::default::Default::default(),
                        factor_2: ::core::default::Default::default(),
                        base_3: ::core::default::Default::default(),
                        factor_3: ::core::default::Default::default(),
                        interest_reverse_rate: ::core::default::Default::default(),
                        accumulated_interest_reverse: ::core::default::Default::default(),
                    }
                }
            }
            impl ::core::marker::StructuralPartialEq for LendingPool {}
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::cmp::PartialEq for LendingPool {
                #[inline]
                fn eq(&self, other: &LendingPool) -> bool {
                    match *other {
                        LendingPool {
                            version: ref __self_1_0,
                            last_update: ref __self_1_1,
                            lending_market: ref __self_1_2,
                            liquidity: ref __self_1_3,
                            share: ref __self_1_4,
                            credit_mint_pubkey: ref __self_1_5,
                            credit_mint_total_supply: ref __self_1_6,
                            credit_supply_pubkey: ref __self_1_7,
                            threshold_1: ref __self_1_8,
                            threshold_2: ref __self_1_9,
                            base_1: ref __self_1_10,
                            factor_1: ref __self_1_11,
                            base_2: ref __self_1_12,
                            factor_2: ref __self_1_13,
                            base_3: ref __self_1_14,
                            factor_3: ref __self_1_15,
                            interest_reverse_rate: ref __self_1_16,
                            accumulated_interest_reverse: ref __self_1_17,
                        } => match *self {
                            LendingPool {
                                version: ref __self_0_0,
                                last_update: ref __self_0_1,
                                lending_market: ref __self_0_2,
                                liquidity: ref __self_0_3,
                                share: ref __self_0_4,
                                credit_mint_pubkey: ref __self_0_5,
                                credit_mint_total_supply: ref __self_0_6,
                                credit_supply_pubkey: ref __self_0_7,
                                threshold_1: ref __self_0_8,
                                threshold_2: ref __self_0_9,
                                base_1: ref __self_0_10,
                                factor_1: ref __self_0_11,
                                base_2: ref __self_0_12,
                                factor_2: ref __self_0_13,
                                base_3: ref __self_0_14,
                                factor_3: ref __self_0_15,
                                interest_reverse_rate: ref __self_0_16,
                                accumulated_interest_reverse: ref __self_0_17,
                            } => {
                                (*__self_0_0) == (*__self_1_0)
                                    && (*__self_0_1) == (*__self_1_1)
                                    && (*__self_0_2) == (*__self_1_2)
                                    && (*__self_0_3) == (*__self_1_3)
                                    && (*__self_0_4) == (*__self_1_4)
                                    && (*__self_0_5) == (*__self_1_5)
                                    && (*__self_0_6) == (*__self_1_6)
                                    && (*__self_0_7) == (*__self_1_7)
                                    && (*__self_0_8) == (*__self_1_8)
                                    && (*__self_0_9) == (*__self_1_9)
                                    && (*__self_0_10) == (*__self_1_10)
                                    && (*__self_0_11) == (*__self_1_11)
                                    && (*__self_0_12) == (*__self_1_12)
                                    && (*__self_0_13) == (*__self_1_13)
                                    && (*__self_0_14) == (*__self_1_14)
                                    && (*__self_0_15) == (*__self_1_15)
                                    && (*__self_0_16) == (*__self_1_16)
                                    && (*__self_0_17) == (*__self_1_17)
                            }
                        },
                    }
                }
                #[inline]
                fn ne(&self, other: &LendingPool) -> bool {
                    match *other {
                        LendingPool {
                            version: ref __self_1_0,
                            last_update: ref __self_1_1,
                            lending_market: ref __self_1_2,
                            liquidity: ref __self_1_3,
                            share: ref __self_1_4,
                            credit_mint_pubkey: ref __self_1_5,
                            credit_mint_total_supply: ref __self_1_6,
                            credit_supply_pubkey: ref __self_1_7,
                            threshold_1: ref __self_1_8,
                            threshold_2: ref __self_1_9,
                            base_1: ref __self_1_10,
                            factor_1: ref __self_1_11,
                            base_2: ref __self_1_12,
                            factor_2: ref __self_1_13,
                            base_3: ref __self_1_14,
                            factor_3: ref __self_1_15,
                            interest_reverse_rate: ref __self_1_16,
                            accumulated_interest_reverse: ref __self_1_17,
                        } => match *self {
                            LendingPool {
                                version: ref __self_0_0,
                                last_update: ref __self_0_1,
                                lending_market: ref __self_0_2,
                                liquidity: ref __self_0_3,
                                share: ref __self_0_4,
                                credit_mint_pubkey: ref __self_0_5,
                                credit_mint_total_supply: ref __self_0_6,
                                credit_supply_pubkey: ref __self_0_7,
                                threshold_1: ref __self_0_8,
                                threshold_2: ref __self_0_9,
                                base_1: ref __self_0_10,
                                factor_1: ref __self_0_11,
                                base_2: ref __self_0_12,
                                factor_2: ref __self_0_13,
                                base_3: ref __self_0_14,
                                factor_3: ref __self_0_15,
                                interest_reverse_rate: ref __self_0_16,
                                accumulated_interest_reverse: ref __self_0_17,
                            } => {
                                (*__self_0_0) != (*__self_1_0)
                                    || (*__self_0_1) != (*__self_1_1)
                                    || (*__self_0_2) != (*__self_1_2)
                                    || (*__self_0_3) != (*__self_1_3)
                                    || (*__self_0_4) != (*__self_1_4)
                                    || (*__self_0_5) != (*__self_1_5)
                                    || (*__self_0_6) != (*__self_1_6)
                                    || (*__self_0_7) != (*__self_1_7)
                                    || (*__self_0_8) != (*__self_1_8)
                                    || (*__self_0_9) != (*__self_1_9)
                                    || (*__self_0_10) != (*__self_1_10)
                                    || (*__self_0_11) != (*__self_1_11)
                                    || (*__self_0_12) != (*__self_1_12)
                                    || (*__self_0_13) != (*__self_1_13)
                                    || (*__self_0_14) != (*__self_1_14)
                                    || (*__self_0_15) != (*__self_1_15)
                                    || (*__self_0_16) != (*__self_1_16)
                                    || (*__self_0_17) != (*__self_1_17)
                            }
                        },
                    }
                }
            }
            impl ::core::marker::StructuralEq for LendingPool {}
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::cmp::Eq for LendingPool {
                #[inline]
                #[doc(hidden)]
                #[no_coverage]
                fn assert_receiver_is_total_eq(&self) -> () {
                    {
                        let _: ::core::cmp::AssertParamIsEq<u8>;
                        let _: ::core::cmp::AssertParamIsEq<LastUpdate>;
                        let _: ::core::cmp::AssertParamIsEq<Pubkey>;
                        let _: ::core::cmp::AssertParamIsEq<ReserveLiquidity>;
                        let _: ::core::cmp::AssertParamIsEq<ReserveCollateral>;
                        let _: ::core::cmp::AssertParamIsEq<Pubkey>;
                        let _: ::core::cmp::AssertParamIsEq<u64>;
                        let _: ::core::cmp::AssertParamIsEq<Pubkey>;
                        let _: ::core::cmp::AssertParamIsEq<u8>;
                        let _: ::core::cmp::AssertParamIsEq<u8>;
                        let _: ::core::cmp::AssertParamIsEq<u8>;
                        let _: ::core::cmp::AssertParamIsEq<u16>;
                        let _: ::core::cmp::AssertParamIsEq<u8>;
                        let _: ::core::cmp::AssertParamIsEq<u16>;
                        let _: ::core::cmp::AssertParamIsEq<u8>;
                        let _: ::core::cmp::AssertParamIsEq<u16>;
                        let _: ::core::cmp::AssertParamIsEq<u8>;
                        let _: ::core::cmp::AssertParamIsEq<u64>;
                    }
                }
            }
            impl LendingPool {
                /// Collateral exchange rate
                pub fn collateral_exchange_rate(
                    &self,
                ) -> Result<CollateralExchangeRate, ProgramError> {
                    let total_liquidity = self.liquidity.total_supply()?;
                    self.share.exchange_rate(total_liquidity)
                }
            }
            /// Reserve liquidity
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
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::clone::Clone for ReserveLiquidity {
                #[inline]
                fn clone(&self) -> ReserveLiquidity {
                    match *self {
                        ReserveLiquidity {
                            mint_pubkey: ref __self_0_0,
                            mint_decimals: ref __self_0_1,
                            supply_pubkey: ref __self_0_2,
                            fee_receiver: ref __self_0_3,
                            oracle_pubkey: ref __self_0_4,
                            available_amount: ref __self_0_5,
                            borrowed_amount_wads: ref __self_0_6,
                            cumulative_borrow_rate_wads: ref __self_0_7,
                            market_price: ref __self_0_8,
                        } => ReserveLiquidity {
                            mint_pubkey: ::core::clone::Clone::clone(&(*__self_0_0)),
                            mint_decimals: ::core::clone::Clone::clone(&(*__self_0_1)),
                            supply_pubkey: ::core::clone::Clone::clone(&(*__self_0_2)),
                            fee_receiver: ::core::clone::Clone::clone(&(*__self_0_3)),
                            oracle_pubkey: ::core::clone::Clone::clone(&(*__self_0_4)),
                            available_amount: ::core::clone::Clone::clone(&(*__self_0_5)),
                            borrowed_amount_wads: ::core::clone::Clone::clone(&(*__self_0_6)),
                            cumulative_borrow_rate_wads: ::core::clone::Clone::clone(
                                &(*__self_0_7),
                            ),
                            market_price: ::core::clone::Clone::clone(&(*__self_0_8)),
                        },
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for ReserveLiquidity {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        ReserveLiquidity {
                            mint_pubkey: ref __self_0_0,
                            mint_decimals: ref __self_0_1,
                            supply_pubkey: ref __self_0_2,
                            fee_receiver: ref __self_0_3,
                            oracle_pubkey: ref __self_0_4,
                            available_amount: ref __self_0_5,
                            borrowed_amount_wads: ref __self_0_6,
                            cumulative_borrow_rate_wads: ref __self_0_7,
                            market_price: ref __self_0_8,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "ReserveLiquidity");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "mint_pubkey",
                                &&(*__self_0_0),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "mint_decimals",
                                &&(*__self_0_1),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "supply_pubkey",
                                &&(*__self_0_2),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "fee_receiver",
                                &&(*__self_0_3),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "oracle_pubkey",
                                &&(*__self_0_4),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "available_amount",
                                &&(*__self_0_5),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "borrowed_amount_wads",
                                &&(*__self_0_6),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "cumulative_borrow_rate_wads",
                                &&(*__self_0_7),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "market_price",
                                &&(*__self_0_8),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::default::Default for ReserveLiquidity {
                #[inline]
                fn default() -> ReserveLiquidity {
                    ReserveLiquidity {
                        mint_pubkey: ::core::default::Default::default(),
                        mint_decimals: ::core::default::Default::default(),
                        supply_pubkey: ::core::default::Default::default(),
                        fee_receiver: ::core::default::Default::default(),
                        oracle_pubkey: ::core::default::Default::default(),
                        available_amount: ::core::default::Default::default(),
                        borrowed_amount_wads: ::core::default::Default::default(),
                        cumulative_borrow_rate_wads: ::core::default::Default::default(),
                        market_price: ::core::default::Default::default(),
                    }
                }
            }
            impl ::core::marker::StructuralPartialEq for ReserveLiquidity {}
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::cmp::PartialEq for ReserveLiquidity {
                #[inline]
                fn eq(&self, other: &ReserveLiquidity) -> bool {
                    match *other {
                        ReserveLiquidity {
                            mint_pubkey: ref __self_1_0,
                            mint_decimals: ref __self_1_1,
                            supply_pubkey: ref __self_1_2,
                            fee_receiver: ref __self_1_3,
                            oracle_pubkey: ref __self_1_4,
                            available_amount: ref __self_1_5,
                            borrowed_amount_wads: ref __self_1_6,
                            cumulative_borrow_rate_wads: ref __self_1_7,
                            market_price: ref __self_1_8,
                        } => match *self {
                            ReserveLiquidity {
                                mint_pubkey: ref __self_0_0,
                                mint_decimals: ref __self_0_1,
                                supply_pubkey: ref __self_0_2,
                                fee_receiver: ref __self_0_3,
                                oracle_pubkey: ref __self_0_4,
                                available_amount: ref __self_0_5,
                                borrowed_amount_wads: ref __self_0_6,
                                cumulative_borrow_rate_wads: ref __self_0_7,
                                market_price: ref __self_0_8,
                            } => {
                                (*__self_0_0) == (*__self_1_0)
                                    && (*__self_0_1) == (*__self_1_1)
                                    && (*__self_0_2) == (*__self_1_2)
                                    && (*__self_0_3) == (*__self_1_3)
                                    && (*__self_0_4) == (*__self_1_4)
                                    && (*__self_0_5) == (*__self_1_5)
                                    && (*__self_0_6) == (*__self_1_6)
                                    && (*__self_0_7) == (*__self_1_7)
                                    && (*__self_0_8) == (*__self_1_8)
                            }
                        },
                    }
                }
                #[inline]
                fn ne(&self, other: &ReserveLiquidity) -> bool {
                    match *other {
                        ReserveLiquidity {
                            mint_pubkey: ref __self_1_0,
                            mint_decimals: ref __self_1_1,
                            supply_pubkey: ref __self_1_2,
                            fee_receiver: ref __self_1_3,
                            oracle_pubkey: ref __self_1_4,
                            available_amount: ref __self_1_5,
                            borrowed_amount_wads: ref __self_1_6,
                            cumulative_borrow_rate_wads: ref __self_1_7,
                            market_price: ref __self_1_8,
                        } => match *self {
                            ReserveLiquidity {
                                mint_pubkey: ref __self_0_0,
                                mint_decimals: ref __self_0_1,
                                supply_pubkey: ref __self_0_2,
                                fee_receiver: ref __self_0_3,
                                oracle_pubkey: ref __self_0_4,
                                available_amount: ref __self_0_5,
                                borrowed_amount_wads: ref __self_0_6,
                                cumulative_borrow_rate_wads: ref __self_0_7,
                                market_price: ref __self_0_8,
                            } => {
                                (*__self_0_0) != (*__self_1_0)
                                    || (*__self_0_1) != (*__self_1_1)
                                    || (*__self_0_2) != (*__self_1_2)
                                    || (*__self_0_3) != (*__self_1_3)
                                    || (*__self_0_4) != (*__self_1_4)
                                    || (*__self_0_5) != (*__self_1_5)
                                    || (*__self_0_6) != (*__self_1_6)
                                    || (*__self_0_7) != (*__self_1_7)
                                    || (*__self_0_8) != (*__self_1_8)
                            }
                        },
                    }
                }
            }
            impl ::core::marker::StructuralEq for ReserveLiquidity {}
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::cmp::Eq for ReserveLiquidity {
                #[inline]
                #[doc(hidden)]
                #[no_coverage]
                fn assert_receiver_is_total_eq(&self) -> () {
                    {
                        let _: ::core::cmp::AssertParamIsEq<Pubkey>;
                        let _: ::core::cmp::AssertParamIsEq<u8>;
                        let _: ::core::cmp::AssertParamIsEq<Pubkey>;
                        let _: ::core::cmp::AssertParamIsEq<Pubkey>;
                        let _: ::core::cmp::AssertParamIsEq<COption<Pubkey>>;
                        let _: ::core::cmp::AssertParamIsEq<u64>;
                        let _: ::core::cmp::AssertParamIsEq<Decimal>;
                        let _: ::core::cmp::AssertParamIsEq<Decimal>;
                        let _: ::core::cmp::AssertParamIsEq<u64>;
                    }
                }
            }
            impl ReserveLiquidity {
                /// Calculate the total reserve supply including active loans
                pub fn total_supply(&self) -> Result<Decimal, ProgramError> {
                    Decimal::from(self.available_amount).try_add(self.borrowed_amount_wads)
                }
            }
            /// Reserve collateral
            pub struct ReserveCollateral {
                /// Reserve collateral mint address
                pub mint_pubkey: Pubkey,
                /// Reserve collateral mint supply, used for exchange rate
                pub mint_total_supply: u64,
                /// Reserve collateral supply address
                pub supply_pubkey: Pubkey,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::clone::Clone for ReserveCollateral {
                #[inline]
                fn clone(&self) -> ReserveCollateral {
                    match *self {
                        ReserveCollateral {
                            mint_pubkey: ref __self_0_0,
                            mint_total_supply: ref __self_0_1,
                            supply_pubkey: ref __self_0_2,
                        } => ReserveCollateral {
                            mint_pubkey: ::core::clone::Clone::clone(&(*__self_0_0)),
                            mint_total_supply: ::core::clone::Clone::clone(&(*__self_0_1)),
                            supply_pubkey: ::core::clone::Clone::clone(&(*__self_0_2)),
                        },
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for ReserveCollateral {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        ReserveCollateral {
                            mint_pubkey: ref __self_0_0,
                            mint_total_supply: ref __self_0_1,
                            supply_pubkey: ref __self_0_2,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "ReserveCollateral");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "mint_pubkey",
                                &&(*__self_0_0),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "mint_total_supply",
                                &&(*__self_0_1),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "supply_pubkey",
                                &&(*__self_0_2),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::default::Default for ReserveCollateral {
                #[inline]
                fn default() -> ReserveCollateral {
                    ReserveCollateral {
                        mint_pubkey: ::core::default::Default::default(),
                        mint_total_supply: ::core::default::Default::default(),
                        supply_pubkey: ::core::default::Default::default(),
                    }
                }
            }
            impl ::core::marker::StructuralPartialEq for ReserveCollateral {}
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::cmp::PartialEq for ReserveCollateral {
                #[inline]
                fn eq(&self, other: &ReserveCollateral) -> bool {
                    match *other {
                        ReserveCollateral {
                            mint_pubkey: ref __self_1_0,
                            mint_total_supply: ref __self_1_1,
                            supply_pubkey: ref __self_1_2,
                        } => match *self {
                            ReserveCollateral {
                                mint_pubkey: ref __self_0_0,
                                mint_total_supply: ref __self_0_1,
                                supply_pubkey: ref __self_0_2,
                            } => {
                                (*__self_0_0) == (*__self_1_0)
                                    && (*__self_0_1) == (*__self_1_1)
                                    && (*__self_0_2) == (*__self_1_2)
                            }
                        },
                    }
                }
                #[inline]
                fn ne(&self, other: &ReserveCollateral) -> bool {
                    match *other {
                        ReserveCollateral {
                            mint_pubkey: ref __self_1_0,
                            mint_total_supply: ref __self_1_1,
                            supply_pubkey: ref __self_1_2,
                        } => match *self {
                            ReserveCollateral {
                                mint_pubkey: ref __self_0_0,
                                mint_total_supply: ref __self_0_1,
                                supply_pubkey: ref __self_0_2,
                            } => {
                                (*__self_0_0) != (*__self_1_0)
                                    || (*__self_0_1) != (*__self_1_1)
                                    || (*__self_0_2) != (*__self_1_2)
                            }
                        },
                    }
                }
            }
            impl ::core::marker::StructuralEq for ReserveCollateral {}
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::cmp::Eq for ReserveCollateral {
                #[inline]
                #[doc(hidden)]
                #[no_coverage]
                fn assert_receiver_is_total_eq(&self) -> () {
                    {
                        let _: ::core::cmp::AssertParamIsEq<Pubkey>;
                        let _: ::core::cmp::AssertParamIsEq<u64>;
                        let _: ::core::cmp::AssertParamIsEq<Pubkey>;
                    }
                }
            }
            impl ReserveCollateral {
                /// Return the current collateral exchange rate.
                fn exchange_rate(
                    &self,
                    total_liquidity: Decimal,
                ) -> Result<CollateralExchangeRate, ProgramError> {
                    let rate = if self.mint_total_supply == 0 || total_liquidity == Decimal::zero()
                    {
                        Rate::from_scaled_val(INITIAL_COLLATERAL_RATE)
                    } else {
                        let mint_total_supply = Decimal::from(self.mint_total_supply);
                        Rate::try_from(mint_total_supply.try_div(total_liquidity)?)?
                    };
                    Ok(CollateralExchangeRate(rate))
                }
            }
            /// Collateral exchange rate
            pub struct CollateralExchangeRate(Rate);
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::clone::Clone for CollateralExchangeRate {
                #[inline]
                fn clone(&self) -> CollateralExchangeRate {
                    {
                        let _: ::core::clone::AssertParamIsClone<Rate>;
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::marker::Copy for CollateralExchangeRate {}
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for CollateralExchangeRate {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        CollateralExchangeRate(ref __self_0_0) => {
                            let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(
                                f,
                                "CollateralExchangeRate",
                            );
                            let _ = ::core::fmt::DebugTuple::field(
                                debug_trait_builder,
                                &&(*__self_0_0),
                            );
                            ::core::fmt::DebugTuple::finish(debug_trait_builder)
                        }
                    }
                }
            }
            impl CollateralExchangeRate {
                /// Convert reserve collateral to liquidity
                pub fn collateral_to_liquidity(
                    &self,
                    collateral_amount: u64,
                ) -> Result<u64, ProgramError> {
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
                pub fn liquidity_to_collateral(
                    &self,
                    liquidity_amount: u64,
                ) -> Result<u64, ProgramError> {
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
                    let input = {
                        {
                            #[inline]
                            unsafe fn as_array<T>(slice: &[T]) -> &[T; LENDING_POOL_LEN] {
                                &*(slice.as_ptr() as *const [_; LENDING_POOL_LEN])
                            }
                            let offset = 0;
                            let slice = &input[offset..offset + LENDING_POOL_LEN];
                            #[allow(unused_unsafe)]
                            unsafe {
                                as_array(slice)
                            }
                        }
                    };
                    #[allow(clippy::ptr_offset_with_cast)]
                    let (
                        version,
                        last_update_slot,
                        last_update_stale,
                        lending_market,
                        liquidity_mint_pubkey,
                        liquidity_mint_decimals,
                        liquidity_supply_pubkey,
                        liquidity_fee_receiver,
                        liquidity_oracle_pubkey,
                        liquidity_available_amount,
                        liquidity_borrowed_amount_wads,
                        liquidity_cumulative_borrow_rate_wads,
                        liquidity_market_price,
                        share_mint_pubkey,
                        share_mint_total_supply,
                        share_supply_pubkey,
                        credit_mint_pubkey,
                        credit_mint_total_supply,
                        credit_supply_pubkey,
                        threshold_1,
                        threshold_2,
                        base_1,
                        factor_1,
                        base_2,
                        factor_2,
                        base_3,
                        factor_3,
                        interest_reverse_rate,
                        accumulated_interest_reverse,
                        _padding,
                    ) = {
                        {
                            #[inline]
                            #[allow(unused_assignments)]
                            #[allow(eval_order_dependence)]
                            unsafe fn as_arrays<T>(
                                a: &[T; 1
                                     + 8
                                     + 1
                                     + PUBKEY_BYTES
                                     + PUBKEY_BYTES
                                     + 1
                                     + PUBKEY_BYTES
                                     + PUBKEY_BYTES
                                     + (4 + PUBKEY_BYTES)
                                     + 8
                                     + 16
                                     + 16
                                     + 8
                                     + PUBKEY_BYTES
                                     + 8
                                     + PUBKEY_BYTES
                                     + PUBKEY_BYTES
                                     + 8
                                     + PUBKEY_BYTES
                                     + 1
                                     + 1
                                     + 1
                                     + 2
                                     + 1
                                     + 2
                                     + 1
                                     + 2
                                     + 1
                                     + 8
                                     + 108
                                     + 0],
                            ) -> (
                                &[T; 1],
                                &[T; 8],
                                &[T; 1],
                                &[T; PUBKEY_BYTES],
                                &[T; PUBKEY_BYTES],
                                &[T; 1],
                                &[T; PUBKEY_BYTES],
                                &[T; PUBKEY_BYTES],
                                &[T; 4 + PUBKEY_BYTES],
                                &[T; 8],
                                &[T; 16],
                                &[T; 16],
                                &[T; 8],
                                &[T; PUBKEY_BYTES],
                                &[T; 8],
                                &[T; PUBKEY_BYTES],
                                &[T; PUBKEY_BYTES],
                                &[T; 8],
                                &[T; PUBKEY_BYTES],
                                &[T; 1],
                                &[T; 1],
                                &[T; 1],
                                &[T; 2],
                                &[T; 1],
                                &[T; 2],
                                &[T; 1],
                                &[T; 2],
                                &[T; 1],
                                &[T; 8],
                                &[T; 108],
                            ) {
                                let mut p = a.as_ptr();
                                (
                                    {
                                        let aref = &*(p as *const [T; 1]);
                                        p = p.offset(1 as isize);
                                        aref
                                    },
                                    {
                                        let aref = &*(p as *const [T; 8]);
                                        p = p.offset(8 as isize);
                                        aref
                                    },
                                    {
                                        let aref = &*(p as *const [T; 1]);
                                        p = p.offset(1 as isize);
                                        aref
                                    },
                                    {
                                        let aref = &*(p as *const [T; PUBKEY_BYTES]);
                                        p = p.offset(PUBKEY_BYTES as isize);
                                        aref
                                    },
                                    {
                                        let aref = &*(p as *const [T; PUBKEY_BYTES]);
                                        p = p.offset(PUBKEY_BYTES as isize);
                                        aref
                                    },
                                    {
                                        let aref = &*(p as *const [T; 1]);
                                        p = p.offset(1 as isize);
                                        aref
                                    },
                                    {
                                        let aref = &*(p as *const [T; PUBKEY_BYTES]);
                                        p = p.offset(PUBKEY_BYTES as isize);
                                        aref
                                    },
                                    {
                                        let aref = &*(p as *const [T; PUBKEY_BYTES]);
                                        p = p.offset(PUBKEY_BYTES as isize);
                                        aref
                                    },
                                    {
                                        let aref = &*(p as *const [T; 4 + PUBKEY_BYTES]);
                                        p = p.offset((4 + PUBKEY_BYTES) as isize);
                                        aref
                                    },
                                    {
                                        let aref = &*(p as *const [T; 8]);
                                        p = p.offset(8 as isize);
                                        aref
                                    },
                                    {
                                        let aref = &*(p as *const [T; 16]);
                                        p = p.offset(16 as isize);
                                        aref
                                    },
                                    {
                                        let aref = &*(p as *const [T; 16]);
                                        p = p.offset(16 as isize);
                                        aref
                                    },
                                    {
                                        let aref = &*(p as *const [T; 8]);
                                        p = p.offset(8 as isize);
                                        aref
                                    },
                                    {
                                        let aref = &*(p as *const [T; PUBKEY_BYTES]);
                                        p = p.offset(PUBKEY_BYTES as isize);
                                        aref
                                    },
                                    {
                                        let aref = &*(p as *const [T; 8]);
                                        p = p.offset(8 as isize);
                                        aref
                                    },
                                    {
                                        let aref = &*(p as *const [T; PUBKEY_BYTES]);
                                        p = p.offset(PUBKEY_BYTES as isize);
                                        aref
                                    },
                                    {
                                        let aref = &*(p as *const [T; PUBKEY_BYTES]);
                                        p = p.offset(PUBKEY_BYTES as isize);
                                        aref
                                    },
                                    {
                                        let aref = &*(p as *const [T; 8]);
                                        p = p.offset(8 as isize);
                                        aref
                                    },
                                    {
                                        let aref = &*(p as *const [T; PUBKEY_BYTES]);
                                        p = p.offset(PUBKEY_BYTES as isize);
                                        aref
                                    },
                                    {
                                        let aref = &*(p as *const [T; 1]);
                                        p = p.offset(1 as isize);
                                        aref
                                    },
                                    {
                                        let aref = &*(p as *const [T; 1]);
                                        p = p.offset(1 as isize);
                                        aref
                                    },
                                    {
                                        let aref = &*(p as *const [T; 1]);
                                        p = p.offset(1 as isize);
                                        aref
                                    },
                                    {
                                        let aref = &*(p as *const [T; 2]);
                                        p = p.offset(2 as isize);
                                        aref
                                    },
                                    {
                                        let aref = &*(p as *const [T; 1]);
                                        p = p.offset(1 as isize);
                                        aref
                                    },
                                    {
                                        let aref = &*(p as *const [T; 2]);
                                        p = p.offset(2 as isize);
                                        aref
                                    },
                                    {
                                        let aref = &*(p as *const [T; 1]);
                                        p = p.offset(1 as isize);
                                        aref
                                    },
                                    {
                                        let aref = &*(p as *const [T; 2]);
                                        p = p.offset(2 as isize);
                                        aref
                                    },
                                    {
                                        let aref = &*(p as *const [T; 1]);
                                        p = p.offset(1 as isize);
                                        aref
                                    },
                                    {
                                        let aref = &*(p as *const [T; 8]);
                                        p = p.offset(8 as isize);
                                        aref
                                    },
                                    {
                                        let aref = &*(p as *const [T; 108]);
                                        p = p.offset(108 as isize);
                                        aref
                                    },
                                )
                            }
                            let input = input;
                            #[allow(unused_unsafe)]
                            unsafe {
                                as_arrays(input)
                            }
                        }
                    };
                    let version = u8::from_le_bytes(*version);
                    if version > PROGRAM_VERSION {
                        ::solana_program::log::sol_log(
                            "Francium LendingPool version does not match lending program version",
                        );
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
                            cumulative_borrow_rate_wads: unpack_decimal(
                                liquidity_cumulative_borrow_rate_wads,
                            ),
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
                        accumulated_interest_reverse: u64::from_le_bytes(
                            *accumulated_interest_reverse,
                        ),
                    })
                }
            }
        }
        /// Last update state
        pub struct LastUpdate {
            /// Last slot when updated
            pub slot: Slot,
            /// True when marked stale, false when slot updated
            pub stale: bool,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for LastUpdate {
            #[inline]
            fn clone(&self) -> LastUpdate {
                match *self {
                    LastUpdate {
                        slot: ref __self_0_0,
                        stale: ref __self_0_1,
                    } => LastUpdate {
                        slot: ::core::clone::Clone::clone(&(*__self_0_0)),
                        stale: ::core::clone::Clone::clone(&(*__self_0_1)),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for LastUpdate {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    LastUpdate {
                        slot: ref __self_0_0,
                        stale: ref __self_0_1,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "LastUpdate");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "slot",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "stale",
                            &&(*__self_0_1),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for LastUpdate {
            #[inline]
            fn default() -> LastUpdate {
                LastUpdate {
                    slot: ::core::default::Default::default(),
                    stale: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for LastUpdate {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for LastUpdate {
            #[inline]
            fn eq(&self, other: &LastUpdate) -> bool {
                match *other {
                    LastUpdate {
                        slot: ref __self_1_0,
                        stale: ref __self_1_1,
                    } => match *self {
                        LastUpdate {
                            slot: ref __self_0_0,
                            stale: ref __self_0_1,
                        } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &LastUpdate) -> bool {
                match *other {
                    LastUpdate {
                        slot: ref __self_1_0,
                        stale: ref __self_1_1,
                    } => match *self {
                        LastUpdate {
                            slot: ref __self_0_0,
                            stale: ref __self_0_1,
                        } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                    },
                }
            }
        }
        impl ::core::marker::StructuralEq for LastUpdate {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for LastUpdate {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<Slot>;
                    let _: ::core::cmp::AssertParamIsEq<bool>;
                }
            }
        }
        pub fn unpack_decimal(src: &[u8; 16]) -> Decimal {
            Decimal::from_scaled_val(u128::from_le_bytes(*src))
        }
        pub fn unpack_bool(src: &[u8; 1]) -> Result<bool, ProgramError> {
            match u8::from_le_bytes(*src) {
                0 => Ok(false),
                1 => Ok(true),
                _ => {
                    ::solana_program::log::sol_log("Boolean cannot be unpacked");
                    Err(ProgramError::InvalidAccountData)
                }
            }
        }
        pub fn unpack_coption_key(
            src: &[u8; 4 + PUBKEY_BYTES],
        ) -> Result<COption<Pubkey>, ProgramError> {
            let (tag, body) = {
                {
                    #[inline]
                    #[allow(unused_assignments)]
                    #[allow(eval_order_dependence)]
                    unsafe fn as_arrays<T>(a: &[T; 4 + 32 + 0]) -> (&[T; 4], &[T; 32]) {
                        let mut p = a.as_ptr();
                        (
                            {
                                let aref = &*(p as *const [T; 4]);
                                p = p.offset(4 as isize);
                                aref
                            },
                            {
                                let aref = &*(p as *const [T; 32]);
                                p = p.offset(32 as isize);
                                aref
                            },
                        )
                    }
                    let input = src;
                    #[allow(unused_unsafe)]
                    unsafe {
                        as_arrays(input)
                    }
                }
            };
            match *tag {
                [0, 0, 0, 0] => Ok(COption::None),
                [1, 0, 0, 0] => Ok(COption::Some(Pubkey::new_from_array(*body))),
                _ => Err(ProgramError::InvalidAccountData),
            }
        }
    }
    /// List of supported protocols
    #[repr(usize)]
    pub enum Protocols {
        Mango = 0,
        Solend = 1,
        Port = 2,
        Tulip = 3,
        Francium = 4,
        SolendStablePool = 5,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for Protocols {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Protocols {
        #[inline]
        fn clone(&self) -> Protocols {
            {
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Protocols {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&Protocols::Mango,) => ::core::fmt::Formatter::write_str(f, "Mango"),
                (&Protocols::Solend,) => ::core::fmt::Formatter::write_str(f, "Solend"),
                (&Protocols::Port,) => ::core::fmt::Formatter::write_str(f, "Port"),
                (&Protocols::Tulip,) => ::core::fmt::Formatter::write_str(f, "Tulip"),
                (&Protocols::Francium,) => ::core::fmt::Formatter::write_str(f, "Francium"),
                (&Protocols::SolendStablePool,) => {
                    ::core::fmt::Formatter::write_str(f, "SolendStablePool")
                }
            }
        }
    }
    impl TryFrom<usize> for Protocols {
        type Error = ();
        fn try_from(v: usize) -> Result<Self, Self::Error> {
            match v {
                x if x == Protocols::Solend as usize => Ok(Protocols::Solend),
                x if x == Protocols::Francium as usize => Ok(Protocols::Francium),
                _ => Err(()),
            }
        }
    }
}
mod vault {
    use crate::check_hash::CHECKHASH_BYTES;
    use crate::error::ErrorCode;
    use crate::protocols::Protocols;
    use anchor_lang::prelude::*;
    use solana_maths::{U192, WAD};
    use std::{
        cmp::{self, Ordering},
        convert::{TryFrom, TryInto},
    };
    pub const _VAULT_VERSION: u8 = 1;
    pub const WEIGHTS_SCALE: u32 = 10_000;
    const VAULT_TICKET_MINT_SEED: &[u8; 11] = b"ticket_mint";
    /// Strategy vault account
    pub struct VaultAccount {
        /// Vault version
        pub version: u8,
        /// This vault is paused
        pub is_paused: bool,
        /// Account seed number
        pub seed_number: u8,
        /// PDA bump seeds
        pub bumps: Bumps,
        /// Strategy input token mint address
        pub input_mint_pubkey: Pubkey,
        /// Destination fee account
        pub dao_treasury_lp_token_account: Pubkey,
        /// Strategy borrow token mint address
        pub borrow_mint_pubkey: Pubkey,
        /// Last refresh slot in which protocol weights were updated
        pub last_refresh_time: i64,
        /// Strategy refresh parameters
        pub refresh: RefreshParams,
        /// Current TVL deposited in the strategy (considering deposits/withdraws)
        pub current_tvl: u64,
        /// Accumulated rewards until fee is minted (not accounted in current_tvl)
        pub rewards_sum: u64,
        /// Price of the LP token in the previous interval
        pub previous_lp_price: LpPrice,
        /// Additional padding
        pub _padding: [u64; 8],
        /// Protocol data (maximum = 10)
        pub protocols: Vec<ProtocolData>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for VaultAccount {
        #[inline]
        fn default() -> VaultAccount {
            VaultAccount {
                version: ::core::default::Default::default(),
                is_paused: ::core::default::Default::default(),
                seed_number: ::core::default::Default::default(),
                bumps: ::core::default::Default::default(),
                input_mint_pubkey: ::core::default::Default::default(),
                dao_treasury_lp_token_account: ::core::default::Default::default(),
                borrow_mint_pubkey: ::core::default::Default::default(),
                last_refresh_time: ::core::default::Default::default(),
                refresh: ::core::default::Default::default(),
                current_tvl: ::core::default::Default::default(),
                rewards_sum: ::core::default::Default::default(),
                previous_lp_price: ::core::default::Default::default(),
                _padding: ::core::default::Default::default(),
                protocols: ::core::default::Default::default(),
            }
        }
    }
    impl borsh::ser::BorshSerialize for VaultAccount
    where
        u8: borsh::ser::BorshSerialize,
        bool: borsh::ser::BorshSerialize,
        u8: borsh::ser::BorshSerialize,
        Bumps: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        i64: borsh::ser::BorshSerialize,
        RefreshParams: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        LpPrice: borsh::ser::BorshSerialize,
        [u64; 8]: borsh::ser::BorshSerialize,
        Vec<ProtocolData>: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.version, writer)?;
            borsh::BorshSerialize::serialize(&self.is_paused, writer)?;
            borsh::BorshSerialize::serialize(&self.seed_number, writer)?;
            borsh::BorshSerialize::serialize(&self.bumps, writer)?;
            borsh::BorshSerialize::serialize(&self.input_mint_pubkey, writer)?;
            borsh::BorshSerialize::serialize(&self.dao_treasury_lp_token_account, writer)?;
            borsh::BorshSerialize::serialize(&self.borrow_mint_pubkey, writer)?;
            borsh::BorshSerialize::serialize(&self.last_refresh_time, writer)?;
            borsh::BorshSerialize::serialize(&self.refresh, writer)?;
            borsh::BorshSerialize::serialize(&self.current_tvl, writer)?;
            borsh::BorshSerialize::serialize(&self.rewards_sum, writer)?;
            borsh::BorshSerialize::serialize(&self.previous_lp_price, writer)?;
            borsh::BorshSerialize::serialize(&self._padding, writer)?;
            borsh::BorshSerialize::serialize(&self.protocols, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for VaultAccount
    where
        u8: borsh::BorshDeserialize,
        bool: borsh::BorshDeserialize,
        u8: borsh::BorshDeserialize,
        Bumps: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        i64: borsh::BorshDeserialize,
        RefreshParams: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        LpPrice: borsh::BorshDeserialize,
        [u64; 8]: borsh::BorshDeserialize,
        Vec<ProtocolData>: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                version: borsh::BorshDeserialize::deserialize(buf)?,
                is_paused: borsh::BorshDeserialize::deserialize(buf)?,
                seed_number: borsh::BorshDeserialize::deserialize(buf)?,
                bumps: borsh::BorshDeserialize::deserialize(buf)?,
                input_mint_pubkey: borsh::BorshDeserialize::deserialize(buf)?,
                dao_treasury_lp_token_account: borsh::BorshDeserialize::deserialize(buf)?,
                borrow_mint_pubkey: borsh::BorshDeserialize::deserialize(buf)?,
                last_refresh_time: borsh::BorshDeserialize::deserialize(buf)?,
                refresh: borsh::BorshDeserialize::deserialize(buf)?,
                current_tvl: borsh::BorshDeserialize::deserialize(buf)?,
                rewards_sum: borsh::BorshDeserialize::deserialize(buf)?,
                previous_lp_price: borsh::BorshDeserialize::deserialize(buf)?,
                _padding: borsh::BorshDeserialize::deserialize(buf)?,
                protocols: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for VaultAccount {
        #[inline]
        fn clone(&self) -> VaultAccount {
            match *self {
                VaultAccount {
                    version: ref __self_0_0,
                    is_paused: ref __self_0_1,
                    seed_number: ref __self_0_2,
                    bumps: ref __self_0_3,
                    input_mint_pubkey: ref __self_0_4,
                    dao_treasury_lp_token_account: ref __self_0_5,
                    borrow_mint_pubkey: ref __self_0_6,
                    last_refresh_time: ref __self_0_7,
                    refresh: ref __self_0_8,
                    current_tvl: ref __self_0_9,
                    rewards_sum: ref __self_0_10,
                    previous_lp_price: ref __self_0_11,
                    _padding: ref __self_0_12,
                    protocols: ref __self_0_13,
                } => VaultAccount {
                    version: ::core::clone::Clone::clone(&(*__self_0_0)),
                    is_paused: ::core::clone::Clone::clone(&(*__self_0_1)),
                    seed_number: ::core::clone::Clone::clone(&(*__self_0_2)),
                    bumps: ::core::clone::Clone::clone(&(*__self_0_3)),
                    input_mint_pubkey: ::core::clone::Clone::clone(&(*__self_0_4)),
                    dao_treasury_lp_token_account: ::core::clone::Clone::clone(&(*__self_0_5)),
                    borrow_mint_pubkey: ::core::clone::Clone::clone(&(*__self_0_6)),
                    last_refresh_time: ::core::clone::Clone::clone(&(*__self_0_7)),
                    refresh: ::core::clone::Clone::clone(&(*__self_0_8)),
                    current_tvl: ::core::clone::Clone::clone(&(*__self_0_9)),
                    rewards_sum: ::core::clone::Clone::clone(&(*__self_0_10)),
                    previous_lp_price: ::core::clone::Clone::clone(&(*__self_0_11)),
                    _padding: ::core::clone::Clone::clone(&(*__self_0_12)),
                    protocols: ::core::clone::Clone::clone(&(*__self_0_13)),
                },
            }
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for VaultAccount {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer
                .write_all(&[230, 251, 241, 83, 139, 202, 93, 28])
                .is_err()
            {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for VaultAccount {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            if buf.len() < [230, 251, 241, 83, 139, 202, 93, 28].len() {
                return Err(anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into());
            }
            let given_disc = &buf[..8];
            if &[230, 251, 241, 83, 139, 202, 93, 28] != given_disc {
                return Err(anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch.into());
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for VaultAccount {
        fn discriminator() -> [u8; 8] {
            [230, 251, 241, 83, 139, 202, 93, 28]
        }
    }
    #[automatically_derived]
    impl anchor_lang::Owner for VaultAccount {
        fn owner() -> Pubkey {
            crate::ID
        }
    }
    impl VaultAccount {
        pub const SIZE: usize = 1
            + 1
            + 1
            + Bumps::SIZE
            + 32
            + 32
            + 8
            + RefreshParams::SIZE
            + 8
            + 8
            + LpPrice::SIZE
            + 8 * 8
            + 4
            + ProtocolData::SIZE * 10;
        /// Initialize a new vault
        pub fn init(params: InitVaultAccountParams) -> Self {
            Self {
                bumps: params.bumps,
                input_mint_pubkey: params.input_mint_pubkey,
                dao_treasury_lp_token_account: params.dao_treasury_lp_token_account,
                refresh: RefreshParams {
                    min_elapsed_time: 3000,
                    min_deposit_lamports: 0,
                },
                ..Self::default()
            }
        }
        /// Find the position of the protocol in the protocol_data vector
        pub fn protocol_position(&self, protocol: Protocols) -> Result<usize> {
            let protocol_id: u8 = (protocol as usize).try_into().unwrap();
            self.protocols
                .iter()
                .position(|protocol| protocol.protocol_id == protocol_id)
                .ok_or_else(|| {
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                        error_name: ErrorCode::ProtocolNotFoundInVault.name(),
                        error_code_number: ErrorCode::ProtocolNotFoundInVault.into(),
                        error_msg: ErrorCode::ProtocolNotFoundInVault.to_string(),
                        error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                            anchor_lang::error::Source {
                                filename: "programs/lending-arb/src/vault.rs",
                                line: 98u32,
                            },
                        )),
                        compared_values: None,
                    })
                })
        }
        /// Calculate amount to deposit in the given protocol
        pub fn calculate_deposit(&self, protocol_idx: usize, available_amount: u64) -> Result<u64> {
            Ok(100)
        }
        /// Calculate amount to withdraw in the given protocol
        pub fn calculate_withdraw(
            &self,
            protocol_idx: usize,
            available_amount: u64,
        ) -> Result<u64> {
            Ok(100)
        }
        /// Calculate amount to borrow in the given protocol
        pub fn calculate_borrow(&self, protocol_idx: usize) -> Result<u64> {
            Ok(100)
        }
        /// Calculate amount to repay in the given protocol
        pub fn calculate_repay(&self, protocol_idx: usize) -> Result<u64> {
            Ok(100)
        }
    }
    /// Initialize a new vault
    pub struct InitVaultAccountParams {
        /// Account seed number
        pub seed_number: u8,
        /// PDA bump seeds
        pub bumps: Bumps,
        /// Strategy input token mint address
        pub input_mint_pubkey: Pubkey,
        /// Destination fee account
        pub dao_treasury_lp_token_account: Pubkey,
    }
    /// PDA bump seeds
    pub struct Bumps {
        pub vault: u8,
        pub lp_token_mint: u8,
        pub ticket_mint: u8,
    }
    impl borsh::ser::BorshSerialize for Bumps
    where
        u8: borsh::ser::BorshSerialize,
        u8: borsh::ser::BorshSerialize,
        u8: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.vault, writer)?;
            borsh::BorshSerialize::serialize(&self.lp_token_mint, writer)?;
            borsh::BorshSerialize::serialize(&self.ticket_mint, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Bumps
    where
        u8: borsh::BorshDeserialize,
        u8: borsh::BorshDeserialize,
        u8: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                vault: borsh::BorshDeserialize::deserialize(buf)?,
                lp_token_mint: borsh::BorshDeserialize::deserialize(buf)?,
                ticket_mint: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for Bumps {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Bumps {
        #[inline]
        fn clone(&self) -> Bumps {
            {
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for Bumps {
        #[inline]
        fn default() -> Bumps {
            Bumps {
                vault: ::core::default::Default::default(),
                lp_token_mint: ::core::default::Default::default(),
                ticket_mint: ::core::default::Default::default(),
            }
        }
    }
    impl Bumps {
        pub const SIZE: usize = 1 + 1 + 1;
    }
    /// Strategy refresh parameters
    pub struct RefreshParams {
        /// Minimum elapsed slots for updating the protocol weights
        pub min_elapsed_time: i64,
        /// Minimum amount of lamports to deposit in each protocol
        pub min_deposit_lamports: u64,
    }
    impl borsh::ser::BorshSerialize for RefreshParams
    where
        i64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.min_elapsed_time, writer)?;
            borsh::BorshSerialize::serialize(&self.min_deposit_lamports, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for RefreshParams
    where
        i64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                min_elapsed_time: borsh::BorshDeserialize::deserialize(buf)?,
                min_deposit_lamports: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for RefreshParams {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for RefreshParams {
        #[inline]
        fn clone(&self) -> RefreshParams {
            {
                let _: ::core::clone::AssertParamIsClone<i64>;
                let _: ::core::clone::AssertParamIsClone<u64>;
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for RefreshParams {
        #[inline]
        fn default() -> RefreshParams {
            RefreshParams {
                min_elapsed_time: ::core::default::Default::default(),
                min_deposit_lamports: ::core::default::Default::default(),
            }
        }
    }
    impl RefreshParams {
        pub const SIZE: usize = 8 + 8;
    }
    /// Protocol data
    pub struct ProtocolData {
        /// Protocol ID
        pub protocol_id: u8,
        /// Hashes of Pubkey
        pub hash_pubkey: HashPubkey,
        /// Percentage of the TVL that should be deposited in the protocol
        pub weight: u32,
        /// Deposited token amount in the protocol
        pub amount: u64,
        /// Accumulated rewards
        pub rewards: AccumulatedRewards,
        /// Padding for other future field
        pub _padding: [u64; 5],
    }
    impl borsh::ser::BorshSerialize for ProtocolData
    where
        u8: borsh::ser::BorshSerialize,
        HashPubkey: borsh::ser::BorshSerialize,
        u32: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        AccumulatedRewards: borsh::ser::BorshSerialize,
        [u64; 5]: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.protocol_id, writer)?;
            borsh::BorshSerialize::serialize(&self.hash_pubkey, writer)?;
            borsh::BorshSerialize::serialize(&self.weight, writer)?;
            borsh::BorshSerialize::serialize(&self.amount, writer)?;
            borsh::BorshSerialize::serialize(&self.rewards, writer)?;
            borsh::BorshSerialize::serialize(&self._padding, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for ProtocolData
    where
        u8: borsh::BorshDeserialize,
        HashPubkey: borsh::BorshDeserialize,
        u32: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        AccumulatedRewards: borsh::BorshDeserialize,
        [u64; 5]: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                protocol_id: borsh::BorshDeserialize::deserialize(buf)?,
                hash_pubkey: borsh::BorshDeserialize::deserialize(buf)?,
                weight: borsh::BorshDeserialize::deserialize(buf)?,
                amount: borsh::BorshDeserialize::deserialize(buf)?,
                rewards: borsh::BorshDeserialize::deserialize(buf)?,
                _padding: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for ProtocolData {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for ProtocolData {
        #[inline]
        fn clone(&self) -> ProtocolData {
            {
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<HashPubkey>;
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<u64>;
                let _: ::core::clone::AssertParamIsClone<AccumulatedRewards>;
                let _: ::core::clone::AssertParamIsClone<[u64; 5]>;
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for ProtocolData {
        #[inline]
        fn default() -> ProtocolData {
            ProtocolData {
                protocol_id: ::core::default::Default::default(),
                hash_pubkey: ::core::default::Default::default(),
                weight: ::core::default::Default::default(),
                amount: ::core::default::Default::default(),
                rewards: ::core::default::Default::default(),
                _padding: ::core::default::Default::default(),
            }
        }
    }
    impl ProtocolData {
        pub const SIZE: usize = 1 + HashPubkey::SIZE + 4 + 8 + AccumulatedRewards::SIZE + 8 * 5;
        /// Check the protocol is active
        pub fn is_active(&self) -> bool {
            self.weight != u32::default()
        }
        /// Set the protocol pubkey hashes
        pub fn set_hashes(&mut self, hashes: [[u8; CHECKHASH_BYTES]; 3]) {
            self.hash_pubkey.hash_deposit = hashes[0];
            self.hash_pubkey.hash_withdraw = hashes[1];
            self.hash_pubkey.hash_tvl = hashes[2];
        }
        /// Amount that should be deposited according to the weight
        fn amount_should_be_deposited(&self, total_amount: u64) -> Result<u64> {
            let amount: u64 = (total_amount as u128)
                .checked_mul(self.weight as u128)
                .ok_or_else(|| {
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                        error_name: ErrorCode::MathOverflow.name(),
                        error_code_number: ErrorCode::MathOverflow.into(),
                        error_msg: ErrorCode::MathOverflow.to_string(),
                        error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                            anchor_lang::error::Source {
                                filename: "programs/lending-arb/src/vault.rs",
                                line: 203u32,
                            },
                        )),
                        compared_values: None,
                    })
                })?
                .checked_div(WEIGHTS_SCALE.into())
                .ok_or_else(|| {
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                        error_name: ErrorCode::MathOverflow.name(),
                        error_code_number: ErrorCode::MathOverflow.into(),
                        error_msg: ErrorCode::MathOverflow.to_string(),
                        error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                            anchor_lang::error::Source {
                                filename: "programs/lending-arb/src/vault.rs",
                                line: 205u32,
                            },
                        )),
                        compared_values: None,
                    })
                })?
                .try_into()
                .map_err(|_| ErrorCode::MathOverflow)?;
            Ok(amount)
        }
        /// Update the protocol tvl with the generated rewards
        pub fn update_tvl(&mut self) -> Result<()> {
            self.amount = i64::try_from(self.amount)
                .unwrap()
                .checked_add(self.rewards.amount)
                .ok_or_else(|| {
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                        error_name: ErrorCode::MathOverflow.name(),
                        error_code_number: ErrorCode::MathOverflow.into(),
                        error_msg: ErrorCode::MathOverflow.to_string(),
                        error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                            anchor_lang::error::Source {
                                filename: "programs/lending-arb/src/vault.rs",
                                line: 216u32,
                            },
                        )),
                        compared_values: None,
                    })
                })?
                .try_into()
                .map_err(|_| ErrorCode::MathOverflow)?;
            self.rewards.amount = 0_i64;
            Ok(())
        }
        /// Update token amount after depositing in the protocol
        pub fn update_after_deposit(&mut self, amount: u64) -> Result<()> {
            self.rewards.deposited_integral.accumulate(self.amount)?;
            self.amount = self.amount.checked_add(amount).ok_or_else(|| {
                anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                    error_name: ErrorCode::MathOverflow.name(),
                    error_code_number: ErrorCode::MathOverflow.into(),
                    error_msg: ErrorCode::MathOverflow.to_string(),
                    error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                        anchor_lang::error::Source {
                            filename: "programs/lending-arb/src/vault.rs",
                            line: 229u32,
                        },
                    )),
                    compared_values: None,
                })
            })?;
            Ok(())
        }
        /// Update token amount after withdrawing from the protocol
        pub fn update_after_withdraw(&mut self, amount: u64) -> Result<()> {
            self.rewards.deposited_integral.accumulate(self.amount)?;
            self.amount = self.amount.checked_sub(amount).ok_or_else(|| {
                anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                    error_name: ErrorCode::MathOverflow.name(),
                    error_code_number: ErrorCode::MathOverflow.into(),
                    error_msg: ErrorCode::MathOverflow.to_string(),
                    error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                        anchor_lang::error::Source {
                            filename: "programs/lending-arb/src/vault.rs",
                            line: 240u32,
                        },
                    )),
                    compared_values: None,
                })
            })?;
            Ok(())
        }
        /// Update token amount after borrow from the protocol
        pub fn update_after_borrow(&mut self, amount: u64) -> Result<()> {
            self.rewards.borrowed_integral.accumulate(self.amount)?;
            self.amount = self.amount.checked_add(amount).ok_or_else(|| {
                anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                    error_name: ErrorCode::MathOverflow.name(),
                    error_code_number: ErrorCode::MathOverflow.into(),
                    error_msg: ErrorCode::MathOverflow.to_string(),
                    error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                        anchor_lang::error::Source {
                            filename: "programs/lending-arb/src/vault.rs",
                            line: 251u32,
                        },
                    )),
                    compared_values: None,
                })
            })?;
            Ok(())
        }
        /// Update token amount after borrow from the protocol
        pub fn update_after_repay(&mut self, amount: u64) -> Result<()> {
            self.rewards.borrowed_integral.accumulate(self.amount)?;
            self.amount = self.amount.checked_sub(amount).ok_or_else(|| {
                anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                    error_name: ErrorCode::MathOverflow.name(),
                    error_code_number: ErrorCode::MathOverflow.into(),
                    error_msg: ErrorCode::MathOverflow.to_string(),
                    error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                        anchor_lang::error::Source {
                            filename: "programs/lending-arb/src/vault.rs",
                            line: 262u32,
                        },
                    )),
                    compared_values: None,
                })
            })?;
            Ok(())
        }
    }
    pub struct HashPubkey {
        /// Hash of important accounts for each protocol on deposit
        pub hash_deposit: [u8; CHECKHASH_BYTES],
        /// Hash of important accounts for each protocol on withdraw
        pub hash_withdraw: [u8; CHECKHASH_BYTES],
        /// Hash of important accounts for each protocol on tvl
        pub hash_tvl: [u8; CHECKHASH_BYTES],
    }
    impl borsh::ser::BorshSerialize for HashPubkey
    where
        [u8; CHECKHASH_BYTES]: borsh::ser::BorshSerialize,
        [u8; CHECKHASH_BYTES]: borsh::ser::BorshSerialize,
        [u8; CHECKHASH_BYTES]: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.hash_deposit, writer)?;
            borsh::BorshSerialize::serialize(&self.hash_withdraw, writer)?;
            borsh::BorshSerialize::serialize(&self.hash_tvl, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for HashPubkey
    where
        [u8; CHECKHASH_BYTES]: borsh::BorshDeserialize,
        [u8; CHECKHASH_BYTES]: borsh::BorshDeserialize,
        [u8; CHECKHASH_BYTES]: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                hash_deposit: borsh::BorshDeserialize::deserialize(buf)?,
                hash_withdraw: borsh::BorshDeserialize::deserialize(buf)?,
                hash_tvl: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for HashPubkey {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for HashPubkey {
        #[inline]
        fn clone(&self) -> HashPubkey {
            {
                let _: ::core::clone::AssertParamIsClone<[u8; CHECKHASH_BYTES]>;
                let _: ::core::clone::AssertParamIsClone<[u8; CHECKHASH_BYTES]>;
                let _: ::core::clone::AssertParamIsClone<[u8; CHECKHASH_BYTES]>;
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for HashPubkey {
        #[inline]
        fn default() -> HashPubkey {
            HashPubkey {
                hash_deposit: ::core::default::Default::default(),
                hash_withdraw: ::core::default::Default::default(),
                hash_tvl: ::core::default::Default::default(),
            }
        }
    }
    impl HashPubkey {
        pub const SIZE: usize = CHECKHASH_BYTES * 3;
    }
    /// Generated rewards
    pub struct AccumulatedRewards {
        /// Last slot the rewards were accumulated
        pub last_slot: u64,
        /// Last accumulated rewards
        pub amount: i64,
        /// Slot-average deposited amount that generates these rewards
        pub deposited_avg_wad: u128,
        /// Slot-integrated deposited amount
        pub deposited_integral: SlotIntegrated,
        /// Slot-integrated deposited amount
        pub borrowed_integral: SlotIntegrated,
    }
    impl borsh::ser::BorshSerialize for AccumulatedRewards
    where
        u64: borsh::ser::BorshSerialize,
        i64: borsh::ser::BorshSerialize,
        u128: borsh::ser::BorshSerialize,
        SlotIntegrated: borsh::ser::BorshSerialize,
        SlotIntegrated: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.last_slot, writer)?;
            borsh::BorshSerialize::serialize(&self.amount, writer)?;
            borsh::BorshSerialize::serialize(&self.deposited_avg_wad, writer)?;
            borsh::BorshSerialize::serialize(&self.deposited_integral, writer)?;
            borsh::BorshSerialize::serialize(&self.borrowed_integral, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for AccumulatedRewards
    where
        u64: borsh::BorshDeserialize,
        i64: borsh::BorshDeserialize,
        u128: borsh::BorshDeserialize,
        SlotIntegrated: borsh::BorshDeserialize,
        SlotIntegrated: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                last_slot: borsh::BorshDeserialize::deserialize(buf)?,
                amount: borsh::BorshDeserialize::deserialize(buf)?,
                deposited_avg_wad: borsh::BorshDeserialize::deserialize(buf)?,
                deposited_integral: borsh::BorshDeserialize::deserialize(buf)?,
                borrowed_integral: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for AccumulatedRewards {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for AccumulatedRewards {
        #[inline]
        fn clone(&self) -> AccumulatedRewards {
            {
                let _: ::core::clone::AssertParamIsClone<u64>;
                let _: ::core::clone::AssertParamIsClone<i64>;
                let _: ::core::clone::AssertParamIsClone<u128>;
                let _: ::core::clone::AssertParamIsClone<SlotIntegrated>;
                let _: ::core::clone::AssertParamIsClone<SlotIntegrated>;
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for AccumulatedRewards {
        #[inline]
        fn default() -> AccumulatedRewards {
            AccumulatedRewards {
                last_slot: ::core::default::Default::default(),
                amount: ::core::default::Default::default(),
                deposited_avg_wad: ::core::default::Default::default(),
                deposited_integral: ::core::default::Default::default(),
                borrowed_integral: ::core::default::Default::default(),
            }
        }
    }
    impl AccumulatedRewards {
        pub const SIZE: usize = 8 + 8 + 16 + SlotIntegrated::SIZE + SlotIntegrated::SIZE;
        /// Update the rewards
        pub fn update(&mut self, rewards: i64, deposited_amount: u64) -> Result<()> {
            let current_slot = Clock::get()?.slot;
            self.last_slot = current_slot;
            self.amount = rewards;
            self.deposited_avg_wad = self
                .deposited_integral
                .get_average_wad(current_slot, deposited_amount)?;
            Ok(())
        }
        /// Reset the initegral from the last rewards values
        pub fn reset_integral(&mut self) -> Result<()> {
            let elapsed_slots_while_rewards = self
                .last_slot
                .checked_sub(self.deposited_integral.initial_slot)
                .ok_or_else(|| {
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                        error_name: ErrorCode::MathOverflow.name(),
                        error_code_number: ErrorCode::MathOverflow.into(),
                        error_msg: ErrorCode::MathOverflow.to_string(),
                        error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                            anchor_lang::error::Source {
                                filename: "programs/lending-arb/src/vault.rs",
                                line: 317u32,
                            },
                        )),
                        compared_values: None,
                    })
                })?;
            let acc_at_rewards: u128 = (U192::from(self.deposited_avg_wad))
                .checked_mul(U192::from(elapsed_slots_while_rewards))
                .ok_or_else(|| {
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                        error_name: ErrorCode::MathOverflow.name(),
                        error_code_number: ErrorCode::MathOverflow.into(),
                        error_msg: ErrorCode::MathOverflow.to_string(),
                        error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                            anchor_lang::error::Source {
                                filename: "programs/lending-arb/src/vault.rs",
                                line: 321u32,
                            },
                        )),
                        compared_values: None,
                    })
                })?
                .checked_div(U192::from(WAD))
                .ok_or_else(|| {
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                        error_name: ErrorCode::MathOverflow.name(),
                        error_code_number: ErrorCode::MathOverflow.into(),
                        error_msg: ErrorCode::MathOverflow.to_string(),
                        error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                            anchor_lang::error::Source {
                                filename: "programs/lending-arb/src/vault.rs",
                                line: 323u32,
                            },
                        )),
                        compared_values: None,
                    })
                })?
                .as_u128();
            let acc_since_last_rewards = self
                .deposited_integral
                .accumulator
                .checked_sub(acc_at_rewards)
                .ok_or_else(|| {
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                        error_name: ErrorCode::MathOverflow.name(),
                        error_code_number: ErrorCode::MathOverflow.into(),
                        error_msg: ErrorCode::MathOverflow.to_string(),
                        error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                            anchor_lang::error::Source {
                                filename: "programs/lending-arb/src/vault.rs",
                                line: 330u32,
                            },
                        )),
                        compared_values: None,
                    })
                })?;
            self.deposited_integral.accumulator = acc_since_last_rewards;
            self.deposited_integral.initial_slot = self.last_slot;
            Ok(())
        }
    }
    /// Slot-integrated quantities
    pub struct SlotIntegrated {
        /// Initial slot from which the integral starts
        pub initial_slot: u64,
        /// Last slot the integral was updated
        pub last_slot: u64,
        /// Summation accumulator
        pub accumulator: u128,
    }
    impl borsh::ser::BorshSerialize for SlotIntegrated
    where
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u128: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.initial_slot, writer)?;
            borsh::BorshSerialize::serialize(&self.last_slot, writer)?;
            borsh::BorshSerialize::serialize(&self.accumulator, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for SlotIntegrated
    where
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u128: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                initial_slot: borsh::BorshDeserialize::deserialize(buf)?,
                last_slot: borsh::BorshDeserialize::deserialize(buf)?,
                accumulator: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for SlotIntegrated {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for SlotIntegrated {
        #[inline]
        fn clone(&self) -> SlotIntegrated {
            {
                let _: ::core::clone::AssertParamIsClone<u64>;
                let _: ::core::clone::AssertParamIsClone<u64>;
                let _: ::core::clone::AssertParamIsClone<u128>;
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for SlotIntegrated {
        #[inline]
        fn default() -> SlotIntegrated {
            SlotIntegrated {
                initial_slot: ::core::default::Default::default(),
                last_slot: ::core::default::Default::default(),
                accumulator: ::core::default::Default::default(),
            }
        }
    }
    impl SlotIntegrated {
        pub const SIZE: usize = 8 + 8 + 16;
        /// Update the summation accumulator
        pub fn accumulate(&mut self, amount: u64) -> Result<()> {
            let current_slot = Clock::get()?.slot;
            let elapsed_slots = current_slot.checked_sub(self.last_slot).ok_or_else(|| {
                anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                    error_name: ErrorCode::MathOverflow.name(),
                    error_code_number: ErrorCode::MathOverflow.into(),
                    error_msg: ErrorCode::MathOverflow.to_string(),
                    error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                        anchor_lang::error::Source {
                            filename: "programs/lending-arb/src/vault.rs",
                            line: 358u32,
                        },
                    )),
                    compared_values: None,
                })
            })?;
            let interval_avg: u128 = (elapsed_slots as u128)
                .checked_mul(amount as u128)
                .ok_or_else(|| {
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                        error_name: ErrorCode::MathOverflow.name(),
                        error_code_number: ErrorCode::MathOverflow.into(),
                        error_msg: ErrorCode::MathOverflow.to_string(),
                        error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                            anchor_lang::error::Source {
                                filename: "programs/lending-arb/src/vault.rs",
                                line: 362u32,
                            },
                        )),
                        compared_values: None,
                    })
                })?;
            self.accumulator = self.accumulator.checked_add(interval_avg).ok_or_else(|| {
                anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                    error_name: ErrorCode::MathOverflow.name(),
                    error_code_number: ErrorCode::MathOverflow.into(),
                    error_msg: ErrorCode::MathOverflow.to_string(),
                    error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                        anchor_lang::error::Source {
                            filename: "programs/lending-arb/src/vault.rs",
                            line: 367u32,
                        },
                    )),
                    compared_values: None,
                })
            })?;
            self.last_slot = current_slot;
            Ok(())
        }
        /// Compute the average value scaled by WAD
        pub fn get_average_wad(
            &mut self,
            current_slot: u64,
            deposited_amount: u64,
        ) -> Result<u128> {
            self.accumulate(deposited_amount)?;
            let elapsed_slots = current_slot.checked_sub(self.initial_slot).ok_or_else(|| {
                anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                    error_name: ErrorCode::MathOverflow.name(),
                    error_code_number: ErrorCode::MathOverflow.into(),
                    error_msg: ErrorCode::MathOverflow.to_string(),
                    error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                        anchor_lang::error::Source {
                            filename: "programs/lending-arb/src/vault.rs",
                            line: 379u32,
                        },
                    )),
                    compared_values: None,
                })
            })?;
            let avg: u128 = (U192::from(self.accumulator))
                .checked_mul(U192::from(WAD))
                .ok_or_else(|| {
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                        error_name: ErrorCode::MathOverflow.name(),
                        error_code_number: ErrorCode::MathOverflow.into(),
                        error_msg: ErrorCode::MathOverflow.to_string(),
                        error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                            anchor_lang::error::Source {
                                filename: "programs/lending-arb/src/vault.rs",
                                line: 383u32,
                            },
                        )),
                        compared_values: None,
                    })
                })?
                .checked_div(U192::from(elapsed_slots))
                .ok_or_else(|| {
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                        error_name: ErrorCode::MathOverflow.name(),
                        error_code_number: ErrorCode::MathOverflow.into(),
                        error_msg: ErrorCode::MathOverflow.to_string(),
                        error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                            anchor_lang::error::Source {
                                filename: "programs/lending-arb/src/vault.rs",
                                line: 385u32,
                            },
                        )),
                        compared_values: None,
                    })
                })?
                .as_u128();
            Ok(avg)
        }
    }
    /// Strategy LP token price
    pub struct LpPrice {
        /// Total amount of tokens to be distributed
        pub total_tokens: u64,
        /// Supply of strategy LP tokens
        pub minted_tokens: u64,
    }
    impl borsh::ser::BorshSerialize for LpPrice
    where
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.total_tokens, writer)?;
            borsh::BorshSerialize::serialize(&self.minted_tokens, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for LpPrice
    where
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                total_tokens: borsh::BorshDeserialize::deserialize(buf)?,
                minted_tokens: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for LpPrice {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for LpPrice {
        #[inline]
        fn clone(&self) -> LpPrice {
            {
                let _: ::core::clone::AssertParamIsClone<u64>;
                let _: ::core::clone::AssertParamIsClone<u64>;
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for LpPrice {
        #[inline]
        fn default() -> LpPrice {
            LpPrice {
                total_tokens: ::core::default::Default::default(),
                minted_tokens: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for LpPrice {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                LpPrice {
                    total_tokens: ref __self_0_0,
                    minted_tokens: ref __self_0_1,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "LpPrice");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "total_tokens",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "minted_tokens",
                        &&(*__self_0_1),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl LpPrice {
        pub const SIZE: usize = 8 + 8;
        /// Transform input token amount to LP amount
        pub fn token_to_lp(&self, amount: u64) -> Result<u64> {
            if self.minted_tokens == 0 {
                Ok(amount)
            } else {
                Ok((amount as u128)
                    .checked_mul(self.minted_tokens as u128)
                    .ok_or_else(|| {
                        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                            error_name: ErrorCode::MathOverflow.name(),
                            error_code_number: ErrorCode::MathOverflow.into(),
                            error_msg: ErrorCode::MathOverflow.to_string(),
                            error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                                anchor_lang::error::Source {
                                    filename: "programs/lending-arb/src/vault.rs",
                                    line: 411u32,
                                },
                            )),
                            compared_values: None,
                        })
                    })?
                    .checked_div(self.total_tokens as u128)
                    .ok_or_else(|| {
                        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                            error_name: ErrorCode::MathOverflow.name(),
                            error_code_number: ErrorCode::MathOverflow.into(),
                            error_msg: ErrorCode::MathOverflow.to_string(),
                            error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                                anchor_lang::error::Source {
                                    filename: "programs/lending-arb/src/vault.rs",
                                    line: 413u32,
                                },
                            )),
                            compared_values: None,
                        })
                    })?
                    .try_into()
                    .map_err(|_| ErrorCode::MathOverflow)?)
            }
        }
        /// Transform LP amount to input token amount
        pub fn lp_to_token(&self, lp_amount: u64) -> Result<u64> {
            if self.minted_tokens == 0 {
                Ok(lp_amount)
            } else {
                Ok((lp_amount as u128)
                    .checked_mul(self.total_tokens as u128)
                    .ok_or_else(|| {
                        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                            error_name: ErrorCode::MathOverflow.name(),
                            error_code_number: ErrorCode::MathOverflow.into(),
                            error_msg: ErrorCode::MathOverflow.to_string(),
                            error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                                anchor_lang::error::Source {
                                    filename: "programs/lending-arb/src/vault.rs",
                                    line: 426u32,
                                },
                            )),
                            compared_values: None,
                        })
                    })?
                    .checked_div(self.minted_tokens as u128)
                    .ok_or_else(|| {
                        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                            error_name: ErrorCode::MathOverflow.name(),
                            error_code_number: ErrorCode::MathOverflow.into(),
                            error_msg: ErrorCode::MathOverflow.to_string(),
                            error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                                anchor_lang::error::Source {
                                    filename: "programs/lending-arb/src/vault.rs",
                                    line: 428u32,
                                },
                            )),
                            compared_values: None,
                        })
                    })?
                    .try_into()
                    .map_err(|_| ErrorCode::MathOverflow)?)
            }
        }
    }
    impl PartialEq for LpPrice {
        fn eq(&self, other: &Self) -> bool {
            let lhs = (self.total_tokens as u128)
                .checked_mul(other.minted_tokens as u128)
                .unwrap();
            let rhs = (other.total_tokens as u128)
                .checked_mul(self.minted_tokens as u128)
                .unwrap();
            lhs == rhs
        }
    }
    impl PartialOrd for LpPrice {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            let lhs = (self.total_tokens as u128)
                .checked_mul(other.minted_tokens as u128)
                .unwrap();
            let rhs = (other.total_tokens as u128)
                .checked_mul(self.minted_tokens as u128)
                .unwrap();
            lhs.partial_cmp(&rhs)
        }
    }
}
mod health {
    use crate::error::ErrorCode;
    use pyth_sdk_solana::{load_price_feed_from_account_info, PriceFeed, Price};
    use solana_maths::Decimal;
    pub const MAX_HEALTH_FACTOR: u128 = 75;
    pub const MIN_HEALTH_FACTOR: u128 = 70;
    pub const OPTIMAL_HEALTH_FACTOR: u128 = 70;
    pub enum Health {
        Vegan = 0,
        Vegetarian = 1,
        Keto = 2,
    }
    impl ::core::marker::StructuralPartialEq for Health {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for Health {
        #[inline]
        fn eq(&self, other: &Health) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
    }
}
/// The static program ID
pub static ID: anchor_lang::solana_program::pubkey::Pubkey =
    anchor_lang::solana_program::pubkey::Pubkey::new_from_array([
        0u8, 51u8, 144u8, 114u8, 141u8, 52u8, 17u8, 96u8, 121u8, 189u8, 201u8, 17u8, 191u8, 255u8,
        0u8, 219u8, 212u8, 77u8, 46u8, 205u8, 204u8, 247u8, 156u8, 166u8, 225u8, 0u8, 56u8, 225u8,
        0u8, 0u8, 0u8, 0u8,
    ]);
/// Confirms that a given pubkey is equivalent to the program ID
pub fn check_id(id: &anchor_lang::solana_program::pubkey::Pubkey) -> bool {
    id == &ID
}
/// Returns the program ID
pub fn id() -> anchor_lang::solana_program::pubkey::Pubkey {
    ID
}
const PAUSED_DEPOSIT: bool = false;
const PAUSED_WITHDRAW: bool = false;
const VAULT_ACCOUNT_SEED: &[u8; 5] = b"vault";
const VAULT_LP_TOKEN_MINT_SEED: &[u8; 4] = b"mint";
const VAULT_TICKET_MINT_SEED: &[u8; 11] = b"ticket_mint";
const ADMIN_PUBKEY: Pubkey = Pubkey::new_from_array([
    191, 17, 77, 109, 253, 243, 16, 188, 64, 67, 249, 18, 51, 62, 173, 81, 128, 208, 121, 29, 74,
    57, 94, 247, 114, 4, 114, 88, 209, 115, 147, 136,
]);
const TREASURY_PUBKEY: Pubkey = Pubkey::new_from_array([
    111, 222, 226, 197, 174, 64, 51, 181, 235, 205, 56, 138, 76, 105, 173, 158, 191, 43, 143, 141,
    91, 145, 78, 45, 130, 86, 102, 175, 146, 188, 82, 152,
]);
use self::lending_arb::*;
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
    let (program_id, accounts, instruction_data) =
        unsafe { ::solana_program::entrypoint::deserialize(input) };
    match entry(&program_id, &accounts, &instruction_data) {
        Ok(()) => ::solana_program::entrypoint::SUCCESS,
        Err(error) => error.into(),
    }
}
/// The Anchor codegen exposes a programming model where a user defines
/// a set of methods inside of a `#[program]` module in a way similar
/// to writing RPC request handlers. The macro then generates a bunch of
/// code wrapping these user defined methods into something that can be
/// executed on Solana.
///
/// These methods fall into one of three categories, each of which
/// can be considered a different "namespace" of the program.
///
/// 1) Global methods - regular methods inside of the `#[program]`.
/// 2) State methods - associated methods inside a `#[state]` struct.
/// 3) Interface methods - methods inside a strait struct's
///    implementation of an `#[interface]` trait.
///
/// Care must be taken by the codegen to prevent collisions between
/// methods in these different namespaces. For this reason, Anchor uses
/// a variant of sighash to perform method dispatch, rather than
/// something like a simple enum variant discriminator.
///
/// The execution flow of the generated code can be roughly outlined:
///
/// * Start program via the entrypoint.
/// * Strip method identifier off the first 8 bytes of the instruction
///   data and invoke the identified method. The method identifier
///   is a variant of sighash. See docs.rs for `anchor_lang` for details.
/// * If the method identifier is an IDL identifier, execute the IDL
///   instructions, which are a special set of hardcoded instructions
///   baked into every Anchor program. Then exit.
/// * Otherwise, the method identifier is for a user defined
///   instruction, i.e., one of the methods in the user defined
///   `#[program]` module. Perform method dispatch, i.e., execute the
///   big match statement mapping method identifier to method handler
///   wrapper.
/// * Run the method handler wrapper. This wraps the code the user
///   actually wrote, deserializing the accounts, constructing the
///   context, invoking the user's code, and finally running the exit
///   routine, which typically persists account changes.
///
/// The `entry` function here, defines the standard entry to a Solana
/// program, where execution begins.
pub fn entry(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> anchor_lang::solana_program::entrypoint::ProgramResult {
    try_entry(program_id, accounts, data).map_err(|e| {
        e.log();
        e.into()
    })
}
fn try_entry(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> anchor_lang::Result<()> {
    if *program_id != ID {
        return Err(anchor_lang::error::ErrorCode::DeclaredProgramIdMismatch.into());
    }
    if data.len() < 8 {
        return Err(anchor_lang::error::ErrorCode::InstructionMissing.into());
    }
    dispatch(program_id, accounts, data)
}
/// Module representing the program.
pub mod program {
    use super::*;
    /// Type representing the program.
    pub struct LendingArb;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for LendingArb {
        #[inline]
        fn clone(&self) -> LendingArb {
            match *self {
                LendingArb => LendingArb,
            }
        }
    }
    impl anchor_lang::Id for LendingArb {
        fn id() -> Pubkey {
            ID
        }
    }
}
/// Performs method dispatch.
///
/// Each method in an anchor program is uniquely defined by a namespace
/// and a rust identifier (i.e., the name given to the method). These
/// two pieces can be combined to creater a method identifier,
/// specifically, Anchor uses
///
/// Sha256("<namespace>::<rust-identifier>")[..8],
///
/// where the namespace can be one of three types. 1) "global" for a
/// regular instruction, 2) "state" for a state struct instruction
/// handler and 3) a trait namespace (used in combination with the
/// `#[interface]` attribute), which is defined by the trait name, e..
/// `MyTrait`.
///
/// With this 8 byte identifier, Anchor performs method dispatch,
/// matching the given 8 byte identifier to the associated method
/// handler, which leads to user defined code being eventually invoked.
fn dispatch(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> anchor_lang::Result<()> {
    let mut ix_data: &[u8] = data;
    let sighash: [u8; 8] = {
        let mut sighash: [u8; 8] = [0; 8];
        sighash.copy_from_slice(&ix_data[..8]);
        ix_data = &ix_data[8..];
        sighash
    };
    if true {
        if sighash == anchor_lang::idl::IDL_IX_TAG.to_le_bytes() {
            return __private::__idl::__idl_dispatch(program_id, accounts, &ix_data);
        }
    }
    match sighash {
        [48, 191, 163, 44, 71, 129, 63, 164] => {
            __private::__global::initialize_vault(program_id, accounts, ix_data)
        }
        [56, 127, 176, 148, 12, 25, 3, 24] => {
            __private::__global::solend_deposit(program_id, accounts, ix_data)
        }
        [238, 144, 170, 199, 21, 72, 155, 36] => {
            __private::__global::solend_withdraw(program_id, accounts, ix_data)
        }
        [200, 133, 72, 39, 12, 233, 241, 54] => {
            __private::__global::solend_borrow(program_id, accounts, ix_data)
        }
        [180, 181, 88, 114, 68, 133, 85, 97] => {
            __private::__global::solend_repay(program_id, accounts, ix_data)
        }
        [30, 196, 238, 181, 151, 73, 213, 198] => {
            __private::__global::francium_deposit(program_id, accounts, ix_data)
        }
        [145, 193, 80, 186, 173, 38, 60, 185] => {
            __private::__global::francium_withdraw(program_id, accounts, ix_data)
        }
        [150, 47, 186, 46, 53, 152, 185, 208] => {
            __private::__global::create_vault_user_ticket_account(program_id, accounts, ix_data)
        }
        [242, 35, 198, 137, 82, 225, 242, 182] => {
            __private::__global::deposit(program_id, accounts, ix_data)
        }
        [235, 63, 23, 100, 111, 206, 72, 11] => {
            __private::__global::open_withdraw_ticket(program_id, accounts, ix_data)
        }
        [59, 115, 209, 162, 26, 58, 153, 83] => {
            __private::__global::close_withdraw_ticket(program_id, accounts, ix_data)
        }
        _ => Err(anchor_lang::error::ErrorCode::InstructionFallbackNotFound.into()),
    }
}
/// Create a private module to not clutter the program's namespace.
/// Defines an entrypoint for each individual instruction handler
/// wrapper.
mod __private {
    use super::*;
    /// __idl mod defines handlers for injected Anchor IDL instructions.
    pub mod __idl {
        use super::*;
        #[inline(never)]
        #[cfg(not(feature = "no-idl"))]
        pub fn __idl_dispatch(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            idl_ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            let mut accounts = accounts;
            let mut data: &[u8] = idl_ix_data;
            let ix = anchor_lang::idl::IdlInstruction::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            match ix {
                anchor_lang::idl::IdlInstruction::Create { data_len } => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut accounts = anchor_lang::idl::IdlCreateAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                    )?;
                    __idl_create_account(program_id, &mut accounts, data_len)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::CreateBuffer => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut accounts = anchor_lang::idl::IdlCreateBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                    )?;
                    __idl_create_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Write { data } => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut accounts = anchor_lang::idl::IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                    )?;
                    __idl_write(program_id, &mut accounts, data)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetAuthority { new_authority } => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut accounts = anchor_lang::idl::IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                    )?;
                    __idl_set_authority(program_id, &mut accounts, new_authority)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetBuffer => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut accounts = anchor_lang::idl::IdlSetBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                    )?;
                    __idl_set_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
            }
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_create_account(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlCreateAccounts,
            data_len: u64,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlCreateAccount");
            if program_id != accounts.program.key {
                return Err(anchor_lang::error::ErrorCode::IdlInstructionInvalidProgram.into());
            }
            let from = accounts.from.key;
            let (base, nonce) = Pubkey::find_program_address(&[], program_id);
            let seed = anchor_lang::idl::IdlAccount::seed();
            let owner = accounts.program.key;
            let to = Pubkey::create_with_seed(&base, seed, owner).unwrap();
            let space = 8 + 32 + 4 + data_len as usize;
            let rent = Rent::get()?;
            let lamports = rent.minimum_balance(space);
            let seeds = &[&[nonce][..]];
            let ix = anchor_lang::solana_program::system_instruction::create_account_with_seed(
                from,
                &to,
                &base,
                seed,
                lamports,
                space as u64,
                owner,
            );
            anchor_lang::solana_program::program::invoke_signed(
                &ix,
                &[
                    accounts.from.clone(),
                    accounts.to.clone(),
                    accounts.base.clone(),
                    accounts.system_program.clone(),
                ],
                &[seeds],
            )?;
            let mut idl_account = {
                let mut account_data = accounts.to.try_borrow_data()?;
                let mut account_data_slice: &[u8] = &account_data;
                anchor_lang::idl::IdlAccount::try_deserialize_unchecked(&mut account_data_slice)?
            };
            idl_account.authority = *accounts.from.key;
            let mut data = accounts.to.try_borrow_mut_data()?;
            let dst: &mut [u8] = &mut data;
            let mut cursor = std::io::Cursor::new(dst);
            idl_account.try_serialize(&mut cursor)?;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_create_buffer(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlCreateBuffer,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlCreateBuffer");
            let mut buffer = &mut accounts.buffer;
            buffer.authority = *accounts.authority.key;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_write(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlAccounts,
            idl_data: Vec<u8>,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlWrite");
            let mut idl = &mut accounts.idl;
            idl.data.extend(idl_data);
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_authority(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlAccounts,
            new_authority: Pubkey,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlSetAuthority");
            accounts.idl.authority = new_authority;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_buffer(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlSetBuffer,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlSetBuffer");
            accounts.idl.data = accounts.buffer.data.clone();
            Ok(())
        }
    }
    /// __state mod defines wrapped handlers for state instructions.
    pub mod __state {
        use super::*;
    }
    /// __interface mod defines wrapped handlers for `#[interface]` trait
    /// implementations.
    pub mod __interface {
        use super::*;
    }
    /// __global mod defines wrapped handlers for global instructions.
    pub mod __global {
        use super::*;
        #[inline(never)]
        pub fn initialize_vault(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: InitializeVault");
            let ix = instruction::InitializeVault::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::InitializeVault { account_number } = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = InitializeVault::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
            )?;
            lending_arb::initialize_vault(
                anchor_lang::context::Context::new(
                    program_id,
                    &mut accounts,
                    remaining_accounts,
                    __bumps,
                ),
                account_number,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn solend_deposit(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: SolendDeposit");
            let ix = instruction::SolendDeposit::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::SolendDeposit = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = SolendDeposit::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
            )?;
            lending_arb::solend_deposit(anchor_lang::context::Context::new(
                program_id,
                &mut accounts,
                remaining_accounts,
                __bumps,
            ))?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn solend_withdraw(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: SolendWithdraw");
            let ix = instruction::SolendWithdraw::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::SolendWithdraw = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = SolendWithdraw::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
            )?;
            lending_arb::solend_withdraw(anchor_lang::context::Context::new(
                program_id,
                &mut accounts,
                remaining_accounts,
                __bumps,
            ))?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn solend_borrow(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: SolendBorrow");
            let ix = instruction::SolendBorrow::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::SolendBorrow = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = SolendBorrow::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
            )?;
            lending_arb::solend_borrow(anchor_lang::context::Context::new(
                program_id,
                &mut accounts,
                remaining_accounts,
                __bumps,
            ))?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn solend_repay(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: SolendRepay");
            let ix = instruction::SolendRepay::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::SolendRepay = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = SolendRepay::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
            )?;
            lending_arb::solend_repay(anchor_lang::context::Context::new(
                program_id,
                &mut accounts,
                remaining_accounts,
                __bumps,
            ))?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn francium_deposit(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: FranciumDeposit");
            let ix = instruction::FranciumDeposit::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::FranciumDeposit = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = FranciumDeposit::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
            )?;
            lending_arb::francium_deposit(anchor_lang::context::Context::new(
                program_id,
                &mut accounts,
                remaining_accounts,
                __bumps,
            ))?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn francium_withdraw(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: FranciumWithdraw");
            let ix = instruction::FranciumWithdraw::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::FranciumWithdraw = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = FranciumWithdraw::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
            )?;
            lending_arb::francium_withdraw(anchor_lang::context::Context::new(
                program_id,
                &mut accounts,
                remaining_accounts,
                __bumps,
            ))?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn create_vault_user_ticket_account(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: CreateVaultUserTicketAccount");
            let ix = instruction::CreateVaultUserTicketAccount::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::CreateVaultUserTicketAccount = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = CreateVaultUserTicketAccount::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
            )?;
            lending_arb::create_vault_user_ticket_account(anchor_lang::context::Context::new(
                program_id,
                &mut accounts,
                remaining_accounts,
                __bumps,
            ))?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn deposit(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: Deposit");
            let ix = instruction::Deposit::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::Deposit { amount } = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                Deposit::try_accounts(program_id, &mut remaining_accounts, ix_data, &mut __bumps)?;
            lending_arb::deposit(
                anchor_lang::context::Context::new(
                    program_id,
                    &mut accounts,
                    remaining_accounts,
                    __bumps,
                ),
                amount,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn open_withdraw_ticket(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: OpenWithdrawTicket");
            let ix = instruction::OpenWithdrawTicket::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::OpenWithdrawTicket {
                lp_amount,
                bump_user,
            } = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = OpenWithdrawTicket::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
            )?;
            lending_arb::open_withdraw_ticket(
                anchor_lang::context::Context::new(
                    program_id,
                    &mut accounts,
                    remaining_accounts,
                    __bumps,
                ),
                lp_amount,
                bump_user,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn close_withdraw_ticket(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: CloseWithdrawTicket");
            let ix = instruction::CloseWithdrawTicket::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::CloseWithdrawTicket {
                lp_amount,
                bump_user,
            } = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = CloseWithdrawTicket::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
            )?;
            lending_arb::close_withdraw_ticket(
                anchor_lang::context::Context::new(
                    program_id,
                    &mut accounts,
                    remaining_accounts,
                    __bumps,
                ),
                lp_amount,
                bump_user,
            )?;
            accounts.exit(program_id)
        }
    }
}
pub mod lending_arb {
    use super::*;
    /// Initialize the vault account and its fields
    pub fn initialize_vault(ctx: Context<InitializeVault>, account_number: u8) -> Result<()> {
        is_admin(ctx.accounts.user_signer.key)?;
        instructions::initialize_vault::handler(ctx, account_number)
    }
    /// Solend: Deposit from the vault account
    pub fn solend_deposit(ctx: Context<SolendDeposit>) -> Result<()> {
        ctx.accounts.check_hash(Protocols::Solend)?;
        instructions::protocol_deposit::handler(ctx, Protocols::Solend)
    }
    /// Solend: Withdraw to the vault account
    pub fn solend_withdraw(ctx: Context<SolendWithdraw>) -> Result<()> {
        ctx.accounts.check_hash(Protocols::Solend)?;
        instructions::protocol_withdraw::handler(ctx, Protocols::Solend)
    }
    /// Solend: Withdraw to the vault account
    pub fn solend_borrow(ctx: Context<SolendBorrow>) -> Result<()> {
        ctx.accounts.check_hash(Protocols::Solend)?;
        instructions::protocol_borrow::handler(ctx, Protocols::Solend)
    }
    /// Solend: Withdraw to the vault account
    pub fn solend_repay(ctx: Context<SolendRepay>) -> Result<()> {
        ctx.accounts.check_hash(Protocols::Solend)?;
        instructions::protocol_repay::handler(ctx, Protocols::Solend)
    }
    /// Francium: Deposit from the vault account
    pub fn francium_deposit(ctx: Context<FranciumDeposit>) -> Result<()> {
        ctx.accounts.check_hash(Protocols::Francium)?;
        instructions::protocol_deposit::handler(ctx, Protocols::Francium)
    }
    /// Francium: Withdraw to the vault account
    pub fn francium_withdraw(ctx: Context<FranciumWithdraw>) -> Result<()> {
        ctx.accounts.check_hash(Protocols::Francium)?;
        instructions::protocol_withdraw::handler(ctx, Protocols::Francium)
    }
    /// Creates a vault_user_ticket_account
    pub fn create_vault_user_ticket_account(
        ctx: Context<CreateVaultUserTicketAccount>,
    ) -> Result<()> {
        instructions::create_vault_user_ticket_account::handler(ctx)
    }
    /// Deposit user input tokens into the vault account
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        deposit_not_paused()?;
        instructions::deposit::handler(ctx, amount)
    }
    /// Open a withdrawal ticket (for delayed withdrawals)
    pub fn open_withdraw_ticket(
        ctx: Context<OpenWithdrawTicket>,
        lp_amount: u64,
        bump_user: u8,
    ) -> Result<()> {
        withdraw_not_paused()?;
        instructions::open_withdraw_ticket::handler(ctx, lp_amount, bump_user)
    }
    /// Close a withdrawal ticket
    pub fn close_withdraw_ticket(
        ctx: Context<CloseWithdrawTicket>,
        lp_amount: u64,
        bump_user: u8,
    ) -> Result<()> {
        withdraw_not_paused()?;
        instructions::close_withdraw_ticket::handler(ctx, lp_amount, bump_user)
    }
}
/// An Anchor generated module containing the program's set of
/// instructions, where each method handler in the `#[program]` mod is
/// associated with a struct defining the input arguments to the
/// method. These should be used directly, when one wants to serialize
/// Anchor instruction data, for example, when speciying
/// instructions on a client.
pub mod instruction {
    use super::*;
    /// Instruction struct definitions for `#[state]` methods.
    pub mod state {
        use super::*;
    }
    /// Instruction.
    pub struct InitializeVault {
        pub account_number: u8,
    }
    impl borsh::ser::BorshSerialize for InitializeVault
    where
        u8: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.account_number, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for InitializeVault
    where
        u8: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                account_number: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for InitializeVault {
        fn data(&self) -> Vec<u8> {
            let mut d = [48, 191, 163, 44, 71, 129, 63, 164].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct SolendDeposit;
    impl borsh::ser::BorshSerialize for SolendDeposit {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for SolendDeposit {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for SolendDeposit {
        fn data(&self) -> Vec<u8> {
            let mut d = [56, 127, 176, 148, 12, 25, 3, 24].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct SolendWithdraw;
    impl borsh::ser::BorshSerialize for SolendWithdraw {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for SolendWithdraw {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for SolendWithdraw {
        fn data(&self) -> Vec<u8> {
            let mut d = [238, 144, 170, 199, 21, 72, 155, 36].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct SolendBorrow;
    impl borsh::ser::BorshSerialize for SolendBorrow {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for SolendBorrow {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for SolendBorrow {
        fn data(&self) -> Vec<u8> {
            let mut d = [200, 133, 72, 39, 12, 233, 241, 54].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct SolendRepay;
    impl borsh::ser::BorshSerialize for SolendRepay {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for SolendRepay {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for SolendRepay {
        fn data(&self) -> Vec<u8> {
            let mut d = [180, 181, 88, 114, 68, 133, 85, 97].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct FranciumDeposit;
    impl borsh::ser::BorshSerialize for FranciumDeposit {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for FranciumDeposit {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for FranciumDeposit {
        fn data(&self) -> Vec<u8> {
            let mut d = [30, 196, 238, 181, 151, 73, 213, 198].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct FranciumWithdraw;
    impl borsh::ser::BorshSerialize for FranciumWithdraw {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for FranciumWithdraw {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for FranciumWithdraw {
        fn data(&self) -> Vec<u8> {
            let mut d = [145, 193, 80, 186, 173, 38, 60, 185].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct CreateVaultUserTicketAccount;
    impl borsh::ser::BorshSerialize for CreateVaultUserTicketAccount {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for CreateVaultUserTicketAccount {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for CreateVaultUserTicketAccount {
        fn data(&self) -> Vec<u8> {
            let mut d = [150, 47, 186, 46, 53, 152, 185, 208].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct Deposit {
        pub amount: u64,
    }
    impl borsh::ser::BorshSerialize for Deposit
    where
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.amount, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Deposit
    where
        u64: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                amount: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for Deposit {
        fn data(&self) -> Vec<u8> {
            let mut d = [242, 35, 198, 137, 82, 225, 242, 182].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct OpenWithdrawTicket {
        pub lp_amount: u64,
        pub bump_user: u8,
    }
    impl borsh::ser::BorshSerialize for OpenWithdrawTicket
    where
        u64: borsh::ser::BorshSerialize,
        u8: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.lp_amount, writer)?;
            borsh::BorshSerialize::serialize(&self.bump_user, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for OpenWithdrawTicket
    where
        u64: borsh::BorshDeserialize,
        u8: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                lp_amount: borsh::BorshDeserialize::deserialize(buf)?,
                bump_user: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for OpenWithdrawTicket {
        fn data(&self) -> Vec<u8> {
            let mut d = [235, 63, 23, 100, 111, 206, 72, 11].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct CloseWithdrawTicket {
        pub lp_amount: u64,
        pub bump_user: u8,
    }
    impl borsh::ser::BorshSerialize for CloseWithdrawTicket
    where
        u64: borsh::ser::BorshSerialize,
        u8: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.lp_amount, writer)?;
            borsh::BorshSerialize::serialize(&self.bump_user, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for CloseWithdrawTicket
    where
        u64: borsh::BorshDeserialize,
        u8: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                lp_amount: borsh::BorshDeserialize::deserialize(buf)?,
                bump_user: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for CloseWithdrawTicket {
        fn data(&self) -> Vec<u8> {
            let mut d = [59, 115, 209, 162, 26, 58, 153, 83].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
}
/// An Anchor generated module, providing a set of structs
/// mirroring the structs deriving `Accounts`, where each field is
/// a `Pubkey`. This is useful for specifying accounts for a client.
pub mod accounts {
    pub use crate::__client_accounts_solend_deposit::*;
    pub use crate::__client_accounts_francium_withdraw::*;
    pub use crate::__client_accounts_solend_repay::*;
    pub use crate::__client_accounts_solend_withdraw::*;
    pub use crate::__client_accounts_francium_deposit::*;
    pub use crate::__client_accounts_close_withdraw_ticket::*;
    pub use crate::__client_accounts_create_vault_user_ticket_account::*;
    pub use crate::__client_accounts_initialize_vault::*;
    pub use crate::__client_accounts_solend_borrow::*;
    pub use crate::__client_accounts_open_withdraw_ticket::*;
    pub use crate::__client_accounts_deposit::*;
}
/// Check if target key is authorized
fn is_admin(key: &Pubkey) -> Result<()> {
    if !(key == &ADMIN_PUBKEY) {
        return Err(anchor_lang::error::Error::from(
            anchor_lang::error::AnchorError {
                error_name: crate::ErrorCode::UnauthorizedUser.name(),
                error_code_number: crate::ErrorCode::UnauthorizedUser.into(),
                error_msg: crate::ErrorCode::UnauthorizedUser.to_string(),
                error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                    anchor_lang::error::Source {
                        filename: "programs/lending-arb/src/lib.rs",
                        line: 124u32,
                    },
                )),
                compared_values: None,
            },
        ));
    };
    Ok(())
}
/// Check if the deposit is paused
fn deposit_not_paused() -> Result<()> {
    if !(!PAUSED_DEPOSIT) {
        return Err(anchor_lang::error::Error::from(
            anchor_lang::error::AnchorError {
                error_name: ErrorCode::OnPaused.name(),
                error_code_number: ErrorCode::OnPaused.into(),
                error_msg: ErrorCode::OnPaused.to_string(),
                error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                    anchor_lang::error::Source {
                        filename: "programs/lending-arb/src/lib.rs",
                        line: 130u32,
                    },
                )),
                compared_values: None,
            },
        ));
    };
    Ok(())
}
/// Check if the withdraw is paused
fn withdraw_not_paused() -> Result<()> {
    if !(!PAUSED_WITHDRAW) {
        return Err(anchor_lang::error::Error::from(
            anchor_lang::error::AnchorError {
                error_name: ErrorCode::OnPaused.name(),
                error_code_number: ErrorCode::OnPaused.into(),
                error_msg: ErrorCode::OnPaused.to_string(),
                error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                    anchor_lang::error::Source {
                        filename: "programs/lending-arb/src/lib.rs",
                        line: 136u32,
                    },
                )),
                compared_values: None,
            },
        ));
    };
    Ok(())
}
