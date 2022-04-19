use crate::error::ErrorCode;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;

/// Check that the current ix is the last of two duplicated instructions
pub fn is_last_of_duplicated_ixs(ixs: AccountInfo) -> Result<bool> {
    let current_index = sysvar::instructions::load_current_index_checked(&ixs)? as usize;
    let current_ix = sysvar::instructions::load_instruction_at_checked(current_index, &ixs)?;

    let mut is_same_as_prev_ix = false;
    if current_index > 0 {
        if let Ok(prev_ix) = sysvar::instructions::load_instruction_at_checked(
            current_index.checked_sub(1).unwrap(),
            &ixs,
        ) {
            is_same_as_prev_ix = prev_ix == current_ix
        }
    };

    let mut is_same_as_next_ix = false;
    if let Ok(next_ix) = sysvar::instructions::load_instruction_at_checked(
        current_index.checked_add(1).unwrap(),
        &ixs,
    ) {
        is_same_as_next_ix = next_ix == current_ix
    };

    require!(
        is_same_as_prev_ix != is_same_as_next_ix,
        ErrorCode::InvalidInstructions
    );

    Ok(is_same_as_prev_ix)
}
