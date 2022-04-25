use crate::instructions::protocol_withdraw_isolated_pool::{self, ProtocolWithdrawIsolatedPool};
use crate::protocols::ProtocolId;
use anchor_lang::prelude::*;

/// Withdraw from the protocol and update protocol data
pub fn handler<'info, T: ProtocolId<'info> + ProtocolWithdrawIsolatedPool<'info>>(
    ctx: Context<T>,
) -> Result<()> {
    let protocol = ctx.accounts.protocol_id();
    protocol_withdraw_isolated_pool::handler(ctx, protocol)
}
