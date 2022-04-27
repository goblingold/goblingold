pub mod francium;
pub mod mango;
pub mod port;
pub mod solend;
pub mod state;
pub mod tulip;

pub const PROTOCOLS_LEN: usize = 6;

/// List of supported protocols
#[repr(usize)]
#[derive(Copy, Clone, Debug)]
pub enum Protocols {
    Mango = 0,
    Solend = 1,
    Port = 2,
    Tulip = 3,
    Francium = 4,
    SolendStablePool = 5,
}

/// Return the protocol id
pub trait ProtocolId<'info> {
    fn protocol_id(&self) -> Protocols;
}
