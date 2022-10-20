use crate::macros::generate_seeds;
use crate::vault::VaultAccount;
use crate::VAULT_ACCOUNT_SEED;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, pubkey::Pubkey};
use anchor_lang::{InstructionData, ToAccountMetas};
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct MangoReimbursement<'info> {
    #[account(mut)]
    pub user_signer: Signer<'info>,
    ///CHECK: Mango checks this
    #[account(mut)]
    pub group: AccountInfo<'info>,

    #[account(mut)]
    pub claim_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: Mango checks this
    #[account(mut)]
    pub reimbursement_account: AccountInfo<'info>,
    /// CHECK: address is part of the ReimbursementAccount PDA
    pub mango_account_owner: UncheckedAccount<'info>,

    #[account(mut)]
    pub claim_mint_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK:
    pub table: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [VAULT_ACCOUNT_SEED, &[vault_account.seed_number][..], vault_account.input_mint_pubkey.as_ref()],
        bump = vault_account.bumps.vault
    )]
    pub vault_account: Box<Account<'info, VaultAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<MangoReimbursement>,
    token_index: usize,
    index_into_table: usize,
) -> Result<()> {
    let seeds = generate_seeds!(ctx.accounts.vault_account);
    let signer = &[&seeds[..]];

    let instr = mango_v3_reimbursement::instruction::CreateReimbursementAccount {};

    let accounts = mango_v3_reimbursement::accounts::CreateReimbursementAccount {
        group: ctx.accounts.group.key(),
        reimbursement_account: ctx.accounts.reimbursement_account.key(),
        mango_account_owner: ctx.accounts.mango_account_owner.key(),
        payer: ctx.accounts.user_signer.key(),
        system_program: ctx.accounts.system_program.key(),
        rent: ctx.accounts.rent.key(),
    };

    let ix = anchor_lang::solana_program::instruction::Instruction {
        program_id: mango_v3_reimbursement::id(),
        accounts: accounts.to_account_metas(None),
        data: instr.data(),
    };

    let accounts_info = [
        ctx.accounts.group.to_account_info(),
        ctx.accounts.reimbursement_account.to_account_info(),
    ];

    invoke_signed(&ix, &accounts_info[..], signer)?;

    let instr = mango_v3_reimbursement::instruction::Reimburse {
        _token_index: token_index,
        _index_into_table: index_into_table,
        _transfer_claim: true,
    };

    let accounts = mango_v3_reimbursement::accounts::Reimburse {
        group: ctx.accounts.group.key(),
        vault: ctx.accounts.vault_token_account.key(),
        token_account: ctx.accounts.token_account.key(),
        reimbursement_account: ctx.accounts.reimbursement_account.key(),
        mango_account_owner: ctx.accounts.mango_account_owner.key(),
        signer: ctx.accounts.user_signer.key(),
        claim_mint_token_account: ctx.accounts.claim_mint_token_account.key(),
        claim_mint: ctx.accounts.claim_mint.key(),
        table: ctx.accounts.table.key(),
        token_program: ctx.accounts.token_program.key(),
        system_program: ctx.accounts.system_program.key(),
        rent: ctx.accounts.rent.key(),
    };
    let ix = anchor_lang::solana_program::instruction::Instruction {
        program_id: mango_v3_reimbursement::id(),
        accounts: accounts.to_account_metas(None),
        data: instr.data(),
    };

    let accounts_info = [
        ctx.accounts.group.to_account_info(),
        ctx.accounts.vault_token_account.to_account_info(),
        ctx.accounts.token_account.to_account_info(),
        ctx.accounts.reimbursement_account.to_account_info(),
        ctx.accounts.mango_account_owner.to_account_info(),
        ctx.accounts.user_signer.to_account_info(),
        ctx.accounts.claim_mint_token_account.to_account_info(),
        ctx.accounts.claim_mint.to_account_info(),
        ctx.accounts.table.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
    ];

    invoke_signed(&ix, &accounts_info[..], signer)?;

    Ok(())
}
