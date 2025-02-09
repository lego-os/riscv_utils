/// Trap 处理相关
use bitflags::bitflags;
/** Supervisor Trap Handling */

/// Supervisor scratch register. sscratch(SRW)
#[cfg(feature = "superv")]
pub const SSCRATCH: u16 = 0x140;

/// Supervisor exception program counter
#[cfg(feature = "superv")]
pub const SEPC: u16 = 0x141;

/**
 * Supervisor trap cause. scause(SRW)
 *
 * SXLEN-1 SXLEN-2                  0
 * | interrupt |       cause        |
 */
#[cfg(feature = "superv")]
pub const SCAUSE: u16 = 0x142;
#[cfg(feature = "superv")]
pub mod scause {
    use bitflags::bitflags;
    bitflags! {
        #[derive(Debug,Clone, Copy,PartialEq, Eq)]
        pub struct Scause:usize{
            #[cfg(target_arch = "riscv64")]
            const interrupt = 1 << 63;
            #[cfg(target_arch = "riscv64")]
            const cause = !(1<< 63);
            #[cfg(target_arch = "riscv32")]
            const interrupt = 1 << 31;
            #[cfg(target_arch = "riscv32")]
            const cause = !(1<<31);
        }
    }
    pub use super::mcause::{
        EXC_BREAKPOINT, EXC_ECALL_S, EXC_ECALL_U, EXC_HARDWARE_ERR, EXC_ILLEGAL_INST,
        EXC_INST_ACCESS_FAULT, EXC_INST_ADDR_MISALIGNED, EXC_INST_PAGE_FAULT,
        EXC_LOAD_ACCESS_FAULT, EXC_LOAD_ADDR_MISALIGNED, EXC_LOAD_PAGE_FAULT, EXC_SOFTWARE_CHECK,
        EXC_STORE_OR_AMO_ACCESS_FAULT, EXC_STORE_OR_AMO_ADDR_MISALIGNED,
        EXC_STORE_OR_AMO_PAGE_FAULT, INTR_COUNTER_OVERFLOW, INTR_S_EXTERNAL, INTR_S_SOFT,
        INTR_S_TIMER,
    };
}

/// Supervisor trap value. stval(SRW)
#[cfg(feature = "superv")]
pub const STVAL: u16 = 0x143;

/**
 * Supervisor interrupt pending. sip(SRW)
 *
 *  15 14    13    12 10   9     8 6    5     4 2    1     0
 * |  0  | LCOFIP |  0  | SEIP |  0  | STIP |  0  | SSIP | 0 |
 */
#[cfg(feature = "superv")]
pub const SIP: u16 = 0x144;
#[cfg(feature = "superv")]
bitflags! {
    #[derive(Debug,Clone, Copy,PartialEq, Eq)]
    pub struct Sip:usize{
        const ssip = 1 << 1;
        const stip = 1 << 5;
        const seip = 1 << 9;
        const lcofip = 1 << 13;
    }
}

/// Supervisor count overflow. scountovf(SRO)
#[cfg(feature = "superv")]
pub const SCOUNTOVF: u16 = 0xDA0;

/** Hypervisor Trap Handling */

/// Hypervisor trap value
#[cfg(feature = "hyperv")]
pub const HTVAL: u16 = 0x643;

/**
 * Hypervisor interrupt pending. hie
 *
 *  15 13   12      11    10    9   7    6    5   3    2    1   0
 * |  0  | SGEIE |  0  | VSEIE |  0  | VSTIE |  0  | VSSIE |  0  |
 */
#[cfg(feature = "hyperv")]
pub const HIP: u16 = 0x644;
#[cfg(feature = "hyperv")]
bitflags! {
    #[derive(Debug,Clone, Copy,PartialEq, Eq)]
    pub struct Hip:usize{
        const vssip = 1 <<2;
        const vstip = 1 << 6;
        const vseip = 1 << 10;
        const sgeip = 1 << 12;
    }
}

/**
 * Hypervisor virtual interrupt pending. hvip
 *
 *  15 11   10    9   7    6    5   3    2    1   0
 * |  0  | VSEIP |  0  | VSTIP |  0  | VSSIP |  0  |
 */
#[cfg(feature = "hyperv")]
pub const HVIP: u16 = 0x645;
#[cfg(feature = "hyperv")]
bitflags! {
    #[derive(Debug,Clone, Copy,PartialEq, Eq)]
    pub struct Hvip:usize{
        const vssip = 1 <<2;
        const vstip = 1 << 6;
        const vseip = 1 << 10;
    }
}

/// Hypervisor trap instruction (transformed)
#[cfg(feature = "hyperv")]
pub const HTINST: u16 = 0x64A;

/**
 * Hypervisor guest external interrupt pending. hgeip
 *
 *  HSXLEN-1                  1  0
 * | Guest External Interrupts | 0 |
 */
#[cfg(feature = "hyperv")]
pub const HGEIP: u16 = 0xE12;

/** Virtual supervisor Trap Handling */

/// Virtual supervisor scratch register
#[cfg(feature = "hyperv")]
pub const VSSCRATCH: u16 = 0x240;

/// Virtual supervisor exception program counter
#[cfg(feature = "hyperv")]
pub const VSEPC: u16 = 0x241;

/**
 * Virtual supervisor trap cause. vscause
 *
 * SXLEN-1 SXLEN-2                  0
 * | interrupt |       cause        |
 */
#[cfg(feature = "hyperv")]
pub const VSCAUSE: u16 = 0x242;
#[cfg(feature = "hyperv")]
pub mod vscause {
    use bitflags::bitflags;
    bitflags! {
        #[derive(Debug,Clone, Copy,PartialEq, Eq)]
        pub struct VScause:usize{
            #[cfg(target_arch = "riscv64")]
            const interrupt = 1 << 63;
            #[cfg(target_arch = "riscv64")]
            const cause = !(1<< 63);
            #[cfg(target_arch = "riscv32")]
            const interrupt = 1 << 31;
            #[cfg(target_arch = "riscv32")]
            const cause = !(1<<31);
        }
    }
    pub use super::mcause::{
        EXC_BREAKPOINT, EXC_ECALL_S, EXC_ECALL_U, EXC_HARDWARE_ERR, EXC_ILLEGAL_INST,
        EXC_INST_ACCESS_FAULT, EXC_INST_ADDR_MISALIGNED, EXC_INST_PAGE_FAULT,
        EXC_LOAD_ACCESS_FAULT, EXC_LOAD_ADDR_MISALIGNED, EXC_LOAD_PAGE_FAULT, EXC_SOFTWARE_CHECK,
        EXC_STORE_OR_AMO_ACCESS_FAULT, EXC_STORE_OR_AMO_ADDR_MISALIGNED,
        EXC_STORE_OR_AMO_PAGE_FAULT, INTR_COUNTER_OVERFLOW, INTR_S_EXTERNAL, INTR_S_SOFT,
        INTR_S_TIMER,
    };
}

/// Virtual supervisor trap value
#[cfg(feature = "hyperv")]
pub const VSTVAL: u16 = 0x243;

/**
 * Virtual supervisor interrupt pending. vsip
 *
 *  15 14    13    12 10   9     8 6    5     4 2    1     0
 * |  0  | LCOFIP |  0  | SEIP |  0  | STIP |  0  | SSIP | 0 |
 */
#[cfg(feature = "hyperv")]
pub const VSIP: u16 = 0x244;
bitflags! {
    #[derive(Debug,Clone, Copy,PartialEq, Eq)]
    pub struct VSip:usize{
        const ssip = 1 << 1;
        const stip = 1 << 5;
        const seip = 1 << 9;
        const lcofip = 1 << 13;
    }
}

/** Machine Trap Handling */

/// Machine scratch register
pub const MSCRATCH: u16 = 0x340;
/// Machine exception program counter
pub const MEPC: u16 = 0x341;

/**
 * Machine trap cause. mcause
 *
 * MXLEN-1 MXLEN-2                  0
 * | interrupt |       cause        |
 */
pub const MCAUSE: u16 = 0x342;
pub mod mcause {
    #[cfg(target_arch = "riscv64")]
    pub const INTERRUPT: usize = 1 << 63;
    #[cfg(target_arch = "riscv32")]
    pub const INTERRUPT: usize = 1 << 31;
    #[cfg(target_arch = "riscv64")]
    pub const CAUSE_MASK: usize = !(1 << 63);
    #[cfg(target_arch = "riscv32")]
    pub const CAUSE_MASK: usize = !(1 << 31);
    pub const INTR_S_SOFT: usize = 1;
    pub const INTR_M_SOFT: usize = 3;
    pub const INTR_S_TIMER: usize = 5;
    pub const INTR_M_TIMER: usize = 7;
    pub const INTR_S_EXTERNAL: usize = 9;
    pub const INTR_M_EXTERNAL: usize = 11;
    pub const INTR_COUNTER_OVERFLOW: usize = 13;

    pub const EXC_INST_ADDR_MISALIGNED: usize = 0;
    pub const EXC_INST_ACCESS_FAULT: usize = 1;
    pub const EXC_ILLEGAL_INST: usize = 2;
    pub const EXC_BREAKPOINT: usize = 3;
    pub const EXC_LOAD_ADDR_MISALIGNED: usize = 4;
    pub const EXC_LOAD_ACCESS_FAULT: usize = 5;
    pub const EXC_STORE_OR_AMO_ADDR_MISALIGNED: usize = 6;
    pub const EXC_STORE_OR_AMO_ACCESS_FAULT: usize = 7;
    pub const EXC_ECALL_U: usize = 8;
    pub const EXC_ECALL_S: usize = 9;
    pub const EXC_ECALL_M: usize = 11;
    pub const EXC_INST_PAGE_FAULT: usize = 12;
    pub const EXC_LOAD_PAGE_FAULT: usize = 13;
    pub const EXC_STORE_OR_AMO_PAGE_FAULT: usize = 15;
    pub const EXC_SOFTWARE_CHECK: usize = 18;
    pub const EXC_HARDWARE_ERR: usize = 19;
}

/// Machine trap value
pub const MTVAL: u16 = 0x343;

/**
 * Machine interrupt pending. mip
 *
 *  15 14    13    12    11   10    9     8    7     6    5     4    3     2    1     0
 * |  0  | LCOFIP | 0 | MEIP | 0 | SEIP | 0 | MTIP | 0 | STIP | 0 | MSIP | 0 | SSIP | 0 |
 */
pub const MIP: u16 = 0x344;
bitflags! {
    #[derive(Debug,Clone, Copy,PartialEq, Eq)]
    pub struct Mip:usize{
        const ssip = 1 << 1;
        #[cfg(feature = "hyperv")]
        const vssip = 1 << 2;
        const msip = 1 << 3;
        const stip = 1 << 5;
        #[cfg(feature = "hyperv")]
        const vstip = 1 << 6;
        const mtip = 1 << 7;
        const seip = 1 << 9;
        #[cfg(feature = "hyperv")]
        const vseip = 1 << 10;
        const meip = 1 << 11;
        #[cfg(feature = "hyperv")]
        const sgeip = 1 << 12;
        const lcofip = 1 << 13;
    }
}
/// Machine trap instruction (transformed)
#[cfg(feature = "hyperv")]
pub const MTINST: u16 = 0x34A;
/// Machine second trap value
#[cfg(feature = "hyperv")]
pub const MTVAL2: u16 = 0x34B;
