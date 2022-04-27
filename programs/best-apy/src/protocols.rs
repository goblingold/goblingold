pub mod francium;
pub mod mango;
pub mod port;
pub mod solend;
pub mod state;
pub mod tulip;

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
