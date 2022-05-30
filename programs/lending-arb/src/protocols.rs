use std::convert::TryFrom;

pub mod francium;
pub mod solend;
pub mod state;

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

impl TryFrom<usize> for Protocols {
    type Error = ();

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            x if x == Protocols::Solend as usize => Ok(Protocols::Solend),
            x if x == Protocols::Francium as usize => Ok(Protocols::Francium),
            _ => Err(()),
        }
    }
}
