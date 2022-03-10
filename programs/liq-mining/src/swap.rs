use crate::RaydiumSwap;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;

mod raydium_amm_instruction;

pub mod raydium_amm {
    use anchor_lang::declare_id;
    declare_id!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");
}
pub mod serum {
    use anchor_lang::declare_id;
    declare_id!("9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin");
}

impl<'info> RaydiumSwap<'info> {
    pub fn swap_rewards(&self) -> Result<()> {
        // TODO now infinity slipagge. Set the min amount with pyth:
        //   - check oracle is not stale
        //   - check uncertainty in the price is below certain threshold
        //   - ...
        let amount_in = self.vault_input_token_account.amount;
        let min_amount_out = 0;
        self.swap(amount_in, min_amount_out)
    }

    pub fn swap(&self, amount_in: u64, min_amount_out: u64) -> Result<()> {
        let seeds = &[
            self.vault_account.to_account_info().key.as_ref(),
            &[self.vault_account.bump],
        ];
        let signer = &[&seeds[..]];

        let swap_ix = raydium_amm_instruction::swap_base_in(
            &self.raydium_amm_program_id.key(),
            &self.amm_id.key(),
            &self.amm_authority.key(),
            &self.amm_open_orders.key(),
            &self.amm_target_orders.key(),
            &self.amm_pool_coin_token_account.key(),
            &self.amm_pool_pc_token_account.key(),
            &self.serum_program_id.key(),
            &self.serum_market.key(),
            &self.serum_bids.key(),
            &self.serum_asks.key(),
            &self.serum_event_queue.key(),
            &self.serum_coin_vault_account.key(),
            &self.serum_pc_vault_account.key(),
            &self.serum_vault_signer.key(),
            &self.vault_input_token_account.key(),
            &self.vault_output_token_account.key(),
            &self.vault_signer.key(),
            amount_in,
            min_amount_out,
        )
        .unwrap();

        invoke_signed(
            &swap_ix,
            &[
                self.raydium_amm_program_id.to_account_info(),
                self.amm_id.to_account_info(),
                self.amm_authority.to_account_info(),
                self.amm_open_orders.to_account_info(),
                self.amm_target_orders.to_account_info(),
                self.amm_pool_coin_token_account.to_account_info(),
                self.amm_pool_pc_token_account.to_account_info(),
                self.serum_program_id.to_account_info(),
                self.serum_market.to_account_info(),
                self.serum_bids.to_account_info(),
                self.serum_asks.to_account_info(),
                self.serum_event_queue.to_account_info(),
                self.serum_coin_vault_account.to_account_info(),
                self.serum_pc_vault_account.to_account_info(),
                self.serum_vault_signer.to_account_info(),
                self.vault_input_token_account.to_account_info(),
                self.vault_output_token_account.to_account_info(),
                self.vault_signer.to_account_info(),
            ],
            signer,
        )?;

        Ok(())
    }
}
