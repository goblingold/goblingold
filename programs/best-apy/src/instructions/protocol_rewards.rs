use crate::instructions::protocol_rewards_isolated_pool::{self, ProtocolRewardsIsolatedPool};
use crate::protocols::ProtocolId;
use anchor_lang::prelude::*;

/// Get the rewards produced by the protocol
pub fn handler<'info, T: ProtocolId<'info> + ProtocolRewardsIsolatedPool<'info>>(
    ctx: Context<T>,
) -> Result<()> {
    let protocol = ctx.accounts.protocol_id();
    protocol_rewards_isolated_pool::handler(ctx, protocol)
}
