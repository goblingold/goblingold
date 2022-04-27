use crate::instructions::protocol_deposit_isolated_pool::{self, ProtocolDepositIsolatedPool};
use crate::protocols::ProtocolId;
use anchor_lang::prelude::*;

/// Deposit into the protocol and update protocol data
pub fn handler<'info, T: ProtocolId<'info> + ProtocolDepositIsolatedPool<'info>>(
    ctx: Context<T>,
) -> Result<()> {
    let protocol = ctx.accounts.protocol_id();
    protocol_deposit_isolated_pool::handler(ctx, protocol)
}
