use anchor_lang::prelude::*;

pub trait ProtocolInitialize<'info> {
    /// Initialize the protocol accounts
    fn cpi_initialize(&self) -> Result<()>;
}

/// Initialize the protocol accounts
pub fn handler<'info, T: ProtocolInitialize<'info>>(ctx: Context<T>) -> Result<()> {
    ctx.accounts.cpi_initialize()
}
