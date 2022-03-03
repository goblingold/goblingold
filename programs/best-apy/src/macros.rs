/// Generate signed seeds for the vault account
macro_rules! generate_seeds {
    ($account:expr) => {
        &[$account.to_account_info().key.as_ref(), &[$account.bump]]
    };
}
pub(crate) use generate_seeds;
