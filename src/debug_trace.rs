/// Supervisor-mode context register. scontext(SRW)
pub const SCONTEXT: u16 = 0x5A8;

/// Hypervisor-mode context register
pub const HCONTEXT: u16 = 0x6A8;

/// Debug/Trace trigger register select
pub const TSELECT: u16 = 0x7A0;
/// First Debug/Trace trigger data register
pub const TDATA1: u16 = 0x7A1;
/// Second Debug/Trace trigger data register
pub const TDATA2: u16 = 0x7A2;
/// Third Debug/Trace trigger data register
pub const TDATA3: u16 = 0x7A3;
/// Machine-mode context register
pub const MCONTEXT: u16 = 0x7A8;
