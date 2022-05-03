/// Generate signed seeds for the vault account
macro_rules! generate_seeds {
    ($account:expr) => {
        &[
            "vault".as_ref(),
            $account.input_mint_pubkey.as_ref(),
            &[$account.seed_number],
            &[$account.bumps.vault],
        ]
    };
}
pub(crate) use generate_seeds;
