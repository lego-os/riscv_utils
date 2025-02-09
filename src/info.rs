/// Machine Information Registers

/// Vendor ID
pub const MVENDORID: u16 = 0xF11;
use bitflags::bitflags;
bitflags! {
    #[derive(Debug,Clone, Copy,PartialEq, Eq)]
    pub struct Mvendorid:u32{
        const offset = 0x7F;
        const bank = !0x7F;
    }
}

/// Architecture ID
pub const MARCHID: u16 = 0xF12;

/// Implementation ID
pub const MIMPID: u16 = 0xF13;

/// Hardware thread ID
pub const MHARTID: u16 = 0xF14;

/// Pointer to configuration data structure
pub const MCONFIGPTR: u16 = 0xF15;
