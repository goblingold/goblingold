pub mod francium;
pub mod francium_farming_user;
pub mod francium_lending_pool;
pub mod mango;
pub mod port;
pub mod solend;
pub mod tulip;
pub mod tulip_reserve;

pub const PROTOCOLS_LEN: usize = 5;

/// List of supported protocols
#[repr(usize)]
#[derive(Copy, Clone, Debug)]
pub enum Protocols {
    Mango = 0,
    Solend = 1,
    Port = 2,
    Tulip = 3,
    Francium = 4,
}
