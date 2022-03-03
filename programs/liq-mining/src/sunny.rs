use crate::{
    InitializeSunny, InitializeSunnyMiner, SunnyClaimRewards, SunnyRedeem, SunnyStake, SunnyUnstake,
};
use anchor_lang::prelude::borsh::{BorshDeserialize, BorshSerialize};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::Instruction,
    program::{invoke, invoke_signed},
};
use anchor_spl::token::{self, Transfer};
use redeemer;

pub mod program {
    use anchor_lang::declare_id;
    declare_id!("SPQR4kT3q2oUKEJes2L6NNSBCiPW9SfuhkuqC9bp6Sx");
}

pub mod quarry_redeemer_program {
    use anchor_lang::declare_id;
    declare_id!("SRDmexy38YTqtCmh7xU2eMFkWweYWF1pqdPyatTF1qP");
}

pub mod mint_wrapper_program {
    use anchor_lang::declare_id;
    declare_id!("MNTerkxkpXetKfRSvSxKp54ty3a7M1ZbGTpL285RtDZ");
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct InitializeSunnyData {
    instruction: [u8; 8],
    bump: u8,
}

impl<'info> InitializeSunny<'info> {
    pub fn initialize_vault(&self, bump: u8) -> ProgramResult {
        let accounts = vec![
            self.sunny_pool.to_account_info(),
            self.vault_signer.to_account_info(),
            self.vault_sunny.to_account_info(),
            self.user_signer.to_account_info(),
            self.system_program.to_account_info(),
        ];
        let accounts_metas = accounts
            .iter()
            .map(|acc| {
                if acc.is_signer {
                    AccountMeta::new(*acc.key, true)
                } else if acc.is_writable {
                    AccountMeta::new(*acc.key, false)
                } else {
                    AccountMeta::new_readonly(*acc.key, false)
                }
            })
            .collect::<Vec<_>>();
        let ix = Instruction {
            program_id: self.sunny_program.key(),
            accounts: accounts_metas,
            data: InitializeSunnyData {
                instruction: [77, 79, 85, 150, 33, 217, 52, 106],
                bump,
            }
            .try_to_vec()
            .unwrap(),
        };
        invoke(&ix, &accounts)?;

        Ok(())
    }
}

impl<'info> InitializeSunnyMiner<'info> {
    pub fn initialize_miner(&self, bump: u8) -> ProgramResult {
        let accounts = vec![
            self.sunny_pool.to_account_info(),
            self.vault_sunny.to_account_info(),
            self.quarry_miner.miner.to_account_info(),
            self.quarry_miner.quarry.to_account_info(),
            self.quarry_miner.rewarder.to_account_info(),
            self.quarry_miner.token_mint.to_account_info(),
            self.quarry_miner.miner_vault.to_account_info(),
            self.user_signer.to_account_info(),
            self.quarry_mine_program_id.to_account_info(),
            self.system_program.to_account_info(),
            self.token_program.to_account_info(),
        ];
        let accounts_metas = accounts
            .iter()
            .map(|acc| {
                if acc.is_signer {
                    AccountMeta::new(*acc.key, true)
                } else if acc.is_writable {
                    AccountMeta::new(*acc.key, false)
                } else {
                    AccountMeta::new_readonly(*acc.key, false)
                }
            })
            .collect::<Vec<_>>();
        let ix = Instruction {
            program_id: self.sunny_program.key(),
            accounts: accounts_metas,
            data: InitializeSunnyData {
                instruction: [144, 159, 202, 208, 234, 154, 242, 55],
                bump,
            }
            .try_to_vec()
            .unwrap(),
        };
        invoke(&ix, &accounts)?;

        Ok(())
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct StakeData {
    instruction: [u8; 8],
}

impl<'info> SunnyStake<'info> {
    pub fn stake(&self) -> ProgramResult {
        let seeds = &[
            self.vault_account.to_account_info().key.as_ref(),
            &[self.vault_account.bump],
        ];
        let signer = &[&seeds[..]];

        // transfer
        {
            let amount = self.vault_lp_saber.amount;
            let cpi_accounts = Transfer {
                from: self.vault_lp_saber.to_account_info(),
                to: self.stake_saber.vault_sunny_ata.to_account_info(),
                authority: self.vault_signer.to_account_info(),
            };
            let cpi_program = self.token_program.to_account_info();
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
            token::transfer(cpi_ctx, amount)?;
        }

        // deposit_vendor
        {
            let accounts = vec![
                self.vault_signer.to_account_info(),
                self.stake_saber.vault_sunny_ata.to_account_info(),
                self.sunny_pool.to_account_info(),
                self.vault_sunny.to_account_info(),
                self.stake_saber.rewarder.to_account_info(),
                self.stake_saber.quarry.to_account_info(),
                self.stake_saber.miner.to_account_info(),
                self.stake_saber.miner_vault.to_account_info(),
                self.token_program.to_account_info(),
                self.quarry_mine_program.to_account_info(),
                self.clock.to_account_info(),
            ];
            let accounts_metas = accounts
                .iter()
                .map(|acc| {
                    if acc.key == self.vault_signer.key {
                        AccountMeta::new_readonly(*acc.key, true)
                    } else if acc.is_writable {
                        AccountMeta::new(*acc.key, false)
                    } else {
                        AccountMeta::new_readonly(*acc.key, false)
                    }
                })
                .collect::<Vec<_>>();
            let ix = Instruction {
                program_id: self.sunny_program.key(),
                accounts: accounts_metas,
                data: StakeData {
                    instruction: [237, 30, 19, 86, 163, 19, 75, 20],
                }
                .try_to_vec()
                .unwrap(),
            };
            invoke_signed(&ix, &accounts, signer)?;
        }

        // stake_internal
        {
            let accounts = vec![
                self.vault_signer.to_account_info(),
                self.sunny_token_mint.to_account_info(),
                self.stake_sunny.vault_sunny_ata.to_account_info(),
                self.sunny_pool.to_account_info(),
                self.vault_sunny.to_account_info(),
                self.stake_sunny.rewarder.to_account_info(),
                self.stake_sunny.quarry.to_account_info(),
                self.stake_sunny.miner.to_account_info(),
                self.stake_sunny.miner_vault.to_account_info(),
                self.token_program.to_account_info(),
                self.quarry_mine_program.to_account_info(),
                self.clock.to_account_info(),
            ];
            let accounts_metas = accounts
                .iter()
                .map(|acc| {
                    if acc.key == self.vault_signer.key {
                        AccountMeta::new_readonly(*acc.key, true)
                    } else if acc.is_writable {
                        AccountMeta::new(*acc.key, false)
                    } else {
                        AccountMeta::new_readonly(*acc.key, false)
                    }
                })
                .collect::<Vec<_>>();
            let ix = Instruction {
                program_id: self.sunny_program.key(),
                accounts: accounts_metas,
                data: StakeData {
                    instruction: [75, 114, 177, 224, 251, 37, 29, 57],
                }
                .try_to_vec()
                .unwrap(),
            };
            invoke_signed(&ix, &accounts, signer)?;
        }

        Ok(())
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct UnstakeData {
    instruction: [u8; 8],
    amount: u64,
}

impl<'info> SunnyUnstake<'info> {
    pub fn unstake(&self, amount: u64) -> ProgramResult {
        let seeds = &[
            self.vault_account.to_account_info().key.as_ref(),
            &[self.vault_account.bump],
        ];
        let signer = &[&seeds[..]];

        // unstake_internal
        {
            let accounts = vec![
                self.vault_signer.to_account_info(),
                self.sunny_token_mint.to_account_info(),
                self.stake_sunny.vault_sunny_ata.to_account_info(),
                self.sunny_pool.to_account_info(),
                self.vault_sunny.to_account_info(),
                self.stake_sunny.rewarder.to_account_info(),
                self.stake_sunny.quarry.to_account_info(),
                self.stake_sunny.miner.to_account_info(),
                self.stake_sunny.miner_vault.to_account_info(),
                self.token_program.to_account_info(),
                self.quarry_mine_program.to_account_info(),
                self.clock.to_account_info(),
            ];
            let accounts_metas = accounts
                .iter()
                .map(|acc| {
                    if acc.key == self.vault_signer.key {
                        AccountMeta::new_readonly(*acc.key, true)
                    } else if acc.is_writable {
                        AccountMeta::new(*acc.key, false)
                    } else {
                        AccountMeta::new_readonly(*acc.key, false)
                    }
                })
                .collect::<Vec<_>>();
            let ix = Instruction {
                program_id: self.sunny_program.key(),
                accounts: accounts_metas,
                data: UnstakeData {
                    instruction: [23, 247, 76, 30, 150, 234, 217, 30],
                    amount,
                }
                .try_to_vec()
                .unwrap(),
            };
            invoke_signed(&ix, &accounts, signer)?;
        }

        // withdraw_vendor
        {
            let accounts = vec![
                self.vault_signer.to_account_info(),
                self.stake_saber.vault_sunny_ata.to_account_info(),
                self.sunny_pool.to_account_info(),
                self.vault_sunny.to_account_info(),
                self.stake_saber.rewarder.to_account_info(),
                self.stake_saber.quarry.to_account_info(),
                self.stake_saber.miner.to_account_info(),
                self.stake_saber.miner_vault.to_account_info(),
                self.token_program.to_account_info(),
                self.quarry_mine_program.to_account_info(),
                self.clock.to_account_info(),
            ];
            let accounts_metas = accounts
                .iter()
                .map(|acc| {
                    if acc.key == self.vault_signer.key {
                        AccountMeta::new_readonly(*acc.key, true)
                    } else if acc.is_writable {
                        AccountMeta::new(*acc.key, false)
                    } else {
                        AccountMeta::new_readonly(*acc.key, false)
                    }
                })
                .collect::<Vec<_>>();
            let ix = Instruction {
                program_id: self.sunny_program.key(),
                accounts: accounts_metas,
                data: UnstakeData {
                    instruction: [57, 234, 188, 83, 92, 233, 44, 199],
                    amount,
                }
                .try_to_vec()
                .unwrap(),
            };
            invoke_signed(&ix, &accounts, signer)?;
        }

        // withdraw_from_vault
        {
            let accounts = vec![
                self.vault_signer.to_account_info(),
                self.sunny_pool.to_account_info(),
                self.vault_sunny.to_account_info(),
                self.stake_saber.vault_sunny_ata.to_account_info(),
                self.vault_lp_saber.to_account_info(),
                self.sunny_fee_destination.to_account_info(),
                self.token_program.to_account_info(),
            ];
            let accounts_metas = accounts
                .iter()
                .map(|acc| {
                    if acc.key == self.vault_signer.key {
                        AccountMeta::new_readonly(*acc.key, true)
                    } else if acc.is_writable {
                        AccountMeta::new(*acc.key, false)
                    } else {
                        AccountMeta::new_readonly(*acc.key, false)
                    }
                })
                .collect::<Vec<_>>();
            let ix = Instruction {
                program_id: self.sunny_program.key(),
                accounts: accounts_metas,
                data: StakeData {
                    instruction: [180, 34, 37, 46, 156, 0, 211, 238],
                }
                .try_to_vec()
                .unwrap(),
            };
            invoke_signed(&ix, &accounts, signer)?;
        }

        Ok(())
    }
}

impl<'info> SunnyClaimRewards<'info> {
    pub fn claim_rewards(&self) -> ProgramResult {
        let seeds = &[
            self.vault_account.to_account_info().key.as_ref(),
            &[self.vault_account.bump],
        ];
        let signer = &[&seeds[..]];

        // claim_rewards
        {
            let accounts = vec![
                self.mint_wrapper.to_account_info(),
                self.quarry_mint_wrapper_program.to_account_info(),
                self.minter.to_account_info(),
                self.rewards_mint.to_account_info(),
                self.vault_sunny_rewards_ata.to_account_info(),
                self.claim_fee.to_account_info(),
                self.vault_sunny_lp.to_account_info(),
                self.sunny_pool.to_account_info(),
                self.vault_sunny.to_account_info(),
                self.rewarder.to_account_info(),
                self.quarry.to_account_info(),
                self.vault_miner.to_account_info(),
                self.vault_miner_ata.to_account_info(),
                self.token_program.to_account_info(),
                self.quarry_mine_program.to_account_info(),
                self.clock.to_account_info(),
            ];
            let accounts_metas = accounts
                .iter()
                .map(|acc| {
                    if acc.key == self.vault_signer.key {
                        AccountMeta::new_readonly(*acc.key, true)
                    } else if acc.is_writable {
                        AccountMeta::new(*acc.key, false)
                    } else {
                        AccountMeta::new_readonly(*acc.key, false)
                    }
                })
                .collect::<Vec<_>>();
            let ix = Instruction {
                program_id: self.sunny_program.key(),
                accounts: accounts_metas,
                data: StakeData {
                    instruction: [4, 144, 132, 71, 116, 23, 151, 80],
                }
                .try_to_vec()
                .unwrap(),
            };
            invoke(&ix, &accounts)?;
        }

        // withdraw_from_vault
        {
            let accounts = vec![
                self.vault_signer.to_account_info(),
                self.sunny_pool.to_account_info(),
                self.vault_sunny.to_account_info(),
                self.vault_sunny_rewards_ata.to_account_info(),
                self.vault_signer_rewards.to_account_info(),
                self.sunny_fee_destination_rewards.to_account_info(),
                self.token_program.to_account_info(),
            ];
            let accounts_metas = accounts
                .iter()
                .map(|acc| {
                    if acc.key == self.vault_signer.key {
                        AccountMeta::new_readonly(*acc.key, true)
                    } else if acc.is_writable {
                        AccountMeta::new(*acc.key, false)
                    } else {
                        AccountMeta::new_readonly(*acc.key, false)
                    }
                })
                .collect::<Vec<_>>();
            let ix = Instruction {
                program_id: self.sunny_program.key(),
                accounts: accounts_metas,
                data: StakeData {
                    instruction: [180, 34, 37, 46, 156, 0, 211, 238],
                }
                .try_to_vec()
                .unwrap(),
            };
            invoke_signed(&ix, &accounts, signer)?;
        }

        Ok(())
    }
}

impl<'info> SunnyRedeem<'info> {
    pub fn redeem(&self) -> ProgramResult {
        let seeds = &[
            self.vault_account.to_account_info().key.as_ref(),
            &[self.vault_account.bump],
        ];
        let signer = &[&seeds[..]];

        // redeem_all_tokens_from_mint_proxy
        {
            let cpi_ctx: CpiContext<redeemer::cpi::accounts::RedeemTokensFromMintProxy> =
                CpiContext::new_with_signer(
                    self.redeemer_program_id.to_account_info(),
                    self.redeem_saber.to_cpi_accounts(),
                    signer,
                );
            redeemer::cpi::redeem_all_tokens_from_mint_proxy(cpi_ctx)?;
        }

        // redeem_all_tokens_from_mint_wrapper
        {
            let accounts = vec![
                self.redeem_sunny.redeem_ctx.redeemer.to_account_info(),
                self.redeem_sunny
                    .redeem_ctx
                    .tokens
                    .iou_mint
                    .to_account_info(),
                self.redeem_sunny
                    .redeem_ctx
                    .tokens
                    .redemption_mint
                    .to_account_info(),
                self.redeem_sunny
                    .redeem_ctx
                    .tokens
                    .redemption_vault
                    .to_account_info(),
                self.redeem_sunny
                    .redeem_ctx
                    .tokens
                    .token_program
                    .to_account_info(),
                self.redeem_sunny
                    .redeem_ctx
                    .source_authority
                    .to_account_info(),
                self.redeem_sunny.redeem_ctx.iou_source.to_account_info(),
                self.redeem_sunny
                    .redeem_ctx
                    .redemption_destination
                    .to_account_info(),
                self.redeem_sunny.mint_wrapper.to_account_info(),
                self.redeem_sunny.mint_wrapper_program.to_account_info(),
                self.redeem_sunny.minter.to_account_info(),
            ];
            let accounts_metas = accounts
                .iter()
                .map(|acc| {
                    if acc.key == self.vault_signer.key {
                        AccountMeta::new_readonly(*acc.key, true)
                    } else if acc.is_writable {
                        AccountMeta::new(*acc.key, false)
                    } else {
                        AccountMeta::new_readonly(*acc.key, false)
                    }
                })
                .collect::<Vec<_>>();
            let ix = Instruction {
                program_id: self.sunny_quarry_redeemer_program.key(),
                accounts: accounts_metas,
                data: StakeData {
                    instruction: [183, 79, 253, 178, 22, 175, 71, 226],
                }
                .try_to_vec()
                .unwrap(),
            };
            invoke_signed(&ix, &accounts, signer)?;
        }

        Ok(())
    }
}
