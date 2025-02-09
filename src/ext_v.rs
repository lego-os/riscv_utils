/// 向量指令集拓展

/// Unprivileged Vector CSRs

/// Vector start position. vstart(URW)
pub const VSTART: u16 = 0x008;

/// Fixed-point accrued saturation flag. vxsat(URW)
pub const VXSAT: u16 = 0x009;

/// Fixed-point rounding mode. vxrm(URW)
pub const VXRM: u16 = 0x00A;

/// Vector control and status register. vcsr(URW)
pub const VCSR: u16 = 0x00F;

/// Vector length. vl(URO)
pub const VL: u16 = 0xC20;

/// Vector data type register. vtype(URO)
pub const VTYPE: u16 = 0xC21;

/// Vector register length in bytes. vlenb(URO)
pub const VLENB: u16 = 0xC22;
