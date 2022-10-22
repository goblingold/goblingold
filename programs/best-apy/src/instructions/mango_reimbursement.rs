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
    /// CHECK: permisionless
    pub mango_v3_reimbursement: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> MangoReimbursement<'info> {
    fn create_reimbursement_ctx(
        &self,
    ) -> CpiContext<
        '_,
        '_,
        '_,
        'info,
        mango_v3_reimbursement::cpi::accounts::CreateReimbursementAccount<'info>,
    > {
        CpiContext::new(
            self.mango_v3_reimbursement.to_account_info(),
            mango_v3_reimbursement::cpi::accounts::CreateReimbursementAccount {
                group: self.group.to_account_info(),
                reimbursement_account: self.reimbursement_account.to_account_info(),
                mango_account_owner: self.mango_account_owner.to_account_info(),
                payer: self.user_signer.to_account_info(),
                system_program: self.system_program.to_account_info(),
                rent: self.rent.to_account_info(),
            },
        )
    }

    fn reimburse_ctx(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, mango_v3_reimbursement::cpi::accounts::Reimburse<'info>>
    {
        CpiContext::new(
            self.mango_v3_reimbursement.to_account_info(),
            mango_v3_reimbursement::cpi::accounts::Reimburse {
                group: self.group.to_account_info(),
                vault: self.vault_token_account.to_account_info(),
                token_account: self.token_account.to_account_info(),
                reimbursement_account: self.reimbursement_account.to_account_info(),
                mango_account_owner: self.mango_account_owner.to_account_info(),
                signer: self.mango_account_owner.to_account_info(),
                claim_mint_token_account: self.claim_mint_token_account.to_account_info(),
                claim_mint: self.claim_mint.to_account_info(),
                table: self.table.to_account_info(),
                token_program: self.token_program.to_account_info(),
                system_program: self.system_program.to_account_info(),
                rent: self.rent.to_account_info(),
            },
        )
    }
}

pub fn handler(
    ctx: Context<MangoReimbursement>,
    token_index: u8,
    index_into_table: u64,
) -> Result<()> {
    let seeds = generate_seeds!(ctx.accounts.vault_account);
    let signer = &[&seeds[..]];

    mango_v3_reimbursement::cpi::create_reimbursement_account(
        ctx.accounts.create_reimbursement_ctx(),
    )?;
    msg!("{}",index_into_table);
    mango_v3_reimbursement::cpi::reimburse(
        ctx.accounts.reimburse_ctx().with_signer(signer),
        token_index as usize,
        index_into_table as usize,
        true,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateReimbursementAccounts<'info> {
    /// CHECK:
    pub group: AccountInfo<'info>,
    /// CHECK:
    pub reimbursement_account: AccountInfo<'info>,
    /// CHECK:
    pub mango_account_owner: AccountInfo<'info>,
    /// CHECK:
    pub payer: AccountInfo<'info>,
    /// CHECK:
    pub system_program: AccountInfo<'info>,
    /// CHECK:
    pub rent: AccountInfo<'info>,
}
