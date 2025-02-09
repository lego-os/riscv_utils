use bitflags::bitflags;

/// Supervisor Counter Setup

/// Supervisor counter-inhibit register. (SRW)
#[cfg(feature = "superv")]
pub const SCOUNTINHIBIT: u16 = 0x120;

/// Machine Counter Setup

/// Machine counter-inhibit register
pub const MCOUNTINHIBIT: u16 = 0x320;
bitflags! {
    /// Machine counter-inhibit register field.
    /// ```text
    ///    31      30     29      5    4      3     2    1   0
    /// | HPM31 | HPM30 |    ...    | HPM4 | HPM3 | IR | 0 | CY |
    /// ```
    #[derive(Debug,Clone, Copy,PartialEq, Eq)]
    pub struct Mcountinhibit:u32{
        const cy = 1;
        const ir = 1 << 2;
        const hpm3 = 1 << 3;
        const hpm4 = 1 << 4;
        const hpm5 = 1 << 5;
        const hpm6 = 1 << 6;
        const hpm7 = 1 << 7;
        const hpm8 = 1 << 8;
        const hpm9 = 1 << 9;
        const hpm10 = 1 << 10;
        const hpm11 = 1 << 11;
        const hpm12 = 1 << 12;
        const hpm13 = 1 << 13;
        const hpm14 = 1 << 14;
        const hpm15 = 1 << 15;
        const hpm16 = 1 << 16;
        const hpm17 = 1 << 17;
        const hpm18 = 1 << 18;
        const hpm19 = 1 << 19;
        const hpm20 = 1 << 20;
        const hpm21 = 1 << 21;
        const hpm22 = 1 << 22;
        const hpm23 = 1 << 23;
        const hpm24 = 1 << 24;
        const hpm25 = 1 << 25;
        const hpm26 = 1 << 26;
        const hpm27 = 1 << 27;
        const hpm28 = 1 << 28;
        const hpm29 = 1 << 29;
        const hpm30 = 1 << 30;
        const hpm31 = 1 << 31;
    }
}

pub const MHPMEVENT3: u16 = 0x323;
pub const MHPMEVENT4: u16 = 0x324;
pub const MHPMEVENT5: u16 = 0x325;
pub const MHPMEVENT6: u16 = 0x326;
pub const MHPMEVENT7: u16 = 0x327;
pub const MHPMEVENT8: u16 = 0x328;
pub const MHPMEVENT9: u16 = 0x329;
pub const MHPMEVENT10: u16 = 0x32A;
pub const MHPMEVENT11: u16 = 0x32B;
pub const MHPMEVENT12: u16 = 0x32C;
pub const MHPMEVENT13: u16 = 0x32D;
pub const MHPMEVENT14: u16 = 0x32E;
pub const MHPMEVENT15: u16 = 0x32F;
pub const MHPMEVENT16: u16 = 0x330;
pub const MHPMEVENT17: u16 = 0x331;
pub const MHPMEVENT18: u16 = 0x332;
pub const MHPMEVENT19: u16 = 0x333;
pub const MHPMEVENT20: u16 = 0x334;
pub const MHPMEVENT21: u16 = 0x335;
pub const MHPMEVENT22: u16 = 0x336;
pub const MHPMEVENT23: u16 = 0x337;
pub const MHPMEVENT24: u16 = 0x338;
pub const MHPMEVENT25: u16 = 0x339;
pub const MHPMEVENT26: u16 = 0x33A;
pub const MHPMEVENT27: u16 = 0x33B;
pub const MHPMEVENT28: u16 = 0x33C;
pub const MHPMEVENT29: u16 = 0x33D;
pub const MHPMEVENT30: u16 = 0x33E;
pub const MHPMEVENT31: u16 = 0x33F;
#[cfg(target_arch = "riscv32")]
pub const MHPMEVENT3H: u16 = 0x723;
#[cfg(target_arch = "riscv32")]
pub const MHPMEVENT4H: u16 = 0x724;
#[cfg(target_arch = "riscv32")]
pub const MHPMEVENT5H: u16 = 0x725;
#[cfg(target_arch = "riscv32")]
pub const MHPMEVENT6H: u16 = 0x726;
#[cfg(target_arch = "riscv32")]
pub const MHPMEVENT7H: u16 = 0x727;
#[cfg(target_arch = "riscv32")]
pub const MHPMEVENT8H: u16 = 0x728;
#[cfg(target_arch = "riscv32")]
pub const MHPMEVENT9H: u16 = 0x729;
#[cfg(target_arch = "riscv32")]
pub const MHPMEVENT10H: u16 = 0x72A;
#[cfg(target_arch = "riscv32")]
pub const MHPMEVENT11H: u16 = 0x72B;
#[cfg(target_arch = "riscv32")]
pub const MHPMEVENT12H: u16 = 0x72C;
#[cfg(target_arch = "riscv32")]
pub const MHPMEVENT13H: u16 = 0x72D;
#[cfg(target_arch = "riscv32")]
pub const MHPMEVENT14H: u16 = 0x72E;
#[cfg(target_arch = "riscv32")]
pub const MHPMEVENT15H: u16 = 0x72F;
#[cfg(target_arch = "riscv32")]
pub const MHPMEVENT16H: u16 = 0x730;
#[cfg(target_arch = "riscv32")]
pub const MHPMEVENT17H: u16 = 0x731;
#[cfg(target_arch = "riscv32")]
pub const MHPMEVENT18H: u16 = 0x732;
#[cfg(target_arch = "riscv32")]
pub const MHPMEVENT19H: u16 = 0x733;
#[cfg(target_arch = "riscv32")]
pub const MHPMEVENT20H: u16 = 0x734;
#[cfg(target_arch = "riscv32")]
pub const MHPMEVENT21H: u16 = 0x735;
#[cfg(target_arch = "riscv32")]
pub const MHPMEVENT22H: u16 = 0x736;
#[cfg(target_arch = "riscv32")]
pub const MHPMEVENT23H: u16 = 0x737;
#[cfg(target_arch = "riscv32")]
pub const MHPMEVENT24H: u16 = 0x738;
#[cfg(target_arch = "riscv32")]
pub const MHPMEVENT25H: u16 = 0x739;
#[cfg(target_arch = "riscv32")]
pub const MHPMEVENT26H: u16 = 0x73A;
#[cfg(target_arch = "riscv32")]
pub const MHPMEVENT27H: u16 = 0x73B;
#[cfg(target_arch = "riscv32")]
pub const MHPMEVENT28H: u16 = 0x73C;
#[cfg(target_arch = "riscv32")]
pub const MHPMEVENT29H: u16 = 0x73D;
#[cfg(target_arch = "riscv32")]
pub const MHPMEVENT30H: u16 = 0x73E;
#[cfg(target_arch = "riscv32")]
pub const MHPMEVENT31H: u16 = 0x73F;
