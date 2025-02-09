/// trap 设置
use bitflags::bitflags;
/** Supervisor Trap Setup */

/**
* Supervisor status register. sstatus(SRW)
* ```text
*    0      1   2              4   5      6     7      8     9 10      11 12      13 14      15
* | WPRI | SIE |      WPRI      | SPIE | UBE | MPRI | SPP | VS[1:0] |   WPRI   | FS[1:0] | XS[1:0] |
*     16       17    18    19    20         22   23     24    25                  30       31
* | XS[1:0] | MPRI | SUM | MXR |     WPRI     | SPELP | SDT |           WPRI        | SD(riscv32) |
*    32 33    34                                                                                 47
* | UXL[1:0] |                                        WPRI                                         |
*  48                                                                                       62  63
* |                                             WPRI                                          | SD |
* ```
*/
#[cfg(feature = "superv")]
pub const SSTATUS: u16 = 0x100;
#[cfg(feature = "superv")]
pub mod sstatus {
    use bitflags::bitflags;
    bitflags! {
        #[derive(Debug,Clone, Copy,PartialEq, Eq)]
        pub struct Sstatus:usize{
            /// S模式中断使能
            const sie = 1 << 1;
            /// S模式中断前的中断状态
            const spie = 1 << 5;
            /// U模式大小端对其 0: little 1: big
            const ube = 1 << 6;
            /// S模式发生中断前的特权级别
            const spp = 1 << 8;
            /// 向量扩展状态
            const vs = 0b11 << 9;
            /// F/D/Q拓展状态
            const fs = 0b11 << 13;
            /// 用户模式扩展的状态
            const xs = 0b11 << 15;
            /// permit supervisor user memory access 1: 允许S模式访问用户内存 0: 反之
            const sum = 1 << 18;
            /// make executable readable 更改特权模式虚拟内存访存权限, 0: 可读, 1: 可读可执行
            const mxr = 1 << 19;
            const spelp = 1 << 23;
            const sdt = 1 << 24;
            #[cfg(target_arch = "riscv32")]
            const sd = 1 << 31;
            #[cfg(target_arch = "riscv64")]
            const uxl = 0b11 << 32;
            #[cfg(target_arch = "riscv64")]
            const sd = 1 << 63;
        }
    }
    #[cfg(target_arch = "riscv64")]
    pub use super::misa::{XLEN_128, XLEN_32, XLEN_64};
    #[cfg(target_arch = "riscv64")]
    pub use super::mstatus::UXL_SHIFT;
    pub use super::mstatus::{
        ENDIAN_BIG, ENDIAN_LITTLE, FS_CLEAN, FS_DIRTY, FS_INITIAL, FS_OFF, FS_SHIFT, MODE_MACHINE,
        MODE_SUPERVISOR, MODE_USER, SPP_SHIFT, UBE_SHIFT, VS_CLEAN, VS_DIRTY, VS_INITIAL, VS_OFF,
        VS_SHIFT, XS_ALL_OFF, XS_NON_DIRTY_OR_CLEAN_SOME_ON, XS_NON_DIRTY_SOME_CLEAN, XS_SHIFT,
        XS_SOME_DIRTY,
    };
}
/**
* Supervisor interrupt-enable register. sie(SRW)
* ```text
*  15 14    13    12 10    9    8 6    5     4 2     1    0
* |  0  | LCOFIE |  0  | SEIE |  0  | STIE |  0  | SSIE | 0 |
* ```
*/
#[cfg(feature = "superv")]
pub const SIE: u16 = 0x104;
#[cfg(feature = "superv")]
bitflags! {
    #[derive(Debug,Clone, Copy,PartialEq, Eq)]
    pub struct Sie:usize{
        const ssie = 1 << 1;
        const stie = 1 << 5;
        const seie = 1 << 9;
        const lcofie = 1 << 13;
    }
}

/**
 * Supervisor trap handler base address. stvec(SRW)
 *
 * SXLEN - 1                 2   1 0
 * |       BASE[SXLEN-1:2]    | MODE |
 * */
#[cfg(feature = "superv")]
pub const STVEC: u16 = 0x105;
#[cfg(feature = "superv")]
pub mod stvec {
    use bitflags::bitflags;
    bitflags! {
        #[derive(Debug,Clone, Copy,PartialEq, Eq)]
        pub struct Stvec:usize{
            const mode = 0b11;
            const base = !0b11;
        }
    }
    pub use super::mtvec::{BASE_SHIFT, MDOE_DIRECT, MDOE_VECTORED};
}

/**
 * Supervisor counter enable. scounteren(SRW)
 *
 *    31      30     29      5    4      3     2    1    0
 * | HPM31 | HPM30 |    ...    | HPM4 | HPM3 | IR | TM | CY |
 */
#[cfg(feature = "superv")]
pub const SCOUNTEREN: u16 = 0x106;
#[cfg(feature = "superv")]
bitflags! {
    #[derive(Debug,Clone, Copy,PartialEq, Eq)]
    pub struct Scounteren:u32{
        const cy = 1;
        const tm = 1 << 1;
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

/** Hypervisor Trap Setup */

/**
 * Hypervisor status register. hstatus, riscv32没有高32位
 *
 *  19 18   17       12 11 10   9     8      7     6     5    4    0
 * | WPRI | VGEIN[5:0] | WPRI | HU | SPVP | SPV | GVA | VSBE | WPRI |
 *  63  34  33      32 31  23   22    21     20
 * | WPRI | XSXL[1:0] | WPRI | VTSR | VTW | VTVM |
 */
#[cfg(feature = "hyperv")]
pub const HSTATUS: u16 = 0x600;
#[cfg(feature = "hyperv")]
pub mod hstatus {
    use bitflags::bitflags;
    bitflags! {
        #[derive(Debug,Clone, Copy,PartialEq, Eq)]
        pub struct Hstatus:usize{
            const vsbe = 1 << 5;
            const gva = 1 << 6;
            const spv = 1 << 7;
            const spvp = 1 << 8;
            const hu = 1 << 9;
            const vgein = 0x3F << 12;
            const vtvm = 1 << 20;
            const vtw = 1 << 21;
            const vtsr = 1 << 22;
            #[cfg(target_arch = "riscv64")]
            const xsxl = 0b11 << 32;
        }
    }

    pub const VSBE_SHIFT: u8 = 5;
    pub use super::mstatus::{ENDIAN_BIG, ENDIAN_LITTLE};
    #[cfg(target_arch = "riscv64")]
    pub const XSXL_SHIFT: u8 = 32;
    #[cfg(target_arch = "riscv64")]
    pub use super::misa::{XLEN_128, XLEN_32, XLEN_64};
}

/// Hypervisor exception delegation register
#[cfg(feature = "hyperv")]
pub const HEDELEG: u16 = 0x602;
/// Upper 32 bits of hedeleg, RV32 only
#[cfg(all(target_arch = "riscv32", feature = "hyperv"))]
pub const HEDELEGH: u16 = 0x612;
/// Hypervisor interrupt delegation register
#[cfg(feature = "hyperv")]
pub const HIDELEG: u16 = 0x603;

/**
 * Hypervisor interrupt-enable register. hie
 *
 *  15 13   12      11    10    9   7    6    5   3    2    1   0
 * |  0  | SGEIE |  0  | VSEIE |  0  | VSTIE |  0  | VSSIE |  0  |
 *  
 */
#[cfg(feature = "hyperv")]
pub const HIE: u16 = 0x604;
#[cfg(feature = "hyperv")]
bitflags! {
    #[derive(Debug,Clone, Copy,PartialEq, Eq)]
    pub struct Hie:usize{
        const vssie = 1 <<2;
        const vstie = 1 << 6;
        const vseie = 1 << 10;
        const sgeie = 1 << 12;
    }
}

/**
 * Hypervisor counter enable. hcounteren
 *
 *    31      30     29      5    4      3     2    1    0
 * | HPM31 | HPM30 |    ...    | HPM4 | HPM3 | IR | TM | CY |
 */
#[cfg(feature = "hyperv")]
pub const HCOUNTEREN: u16 = 0x606;
#[cfg(feature = "hyperv")]
bitflags! {
    #[derive(Debug,Clone, Copy,PartialEq, Eq)]
    pub struct Hcounteren:u32{
        const cy = 1;
        const tm = 1 << 1;
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

/**
 * Hypervisor guest external interrupt-enable register. hgeie
 *
 *  HSXLEN-1                  1  0
 * | Guest External Interrupts | 0 |
 */
#[cfg(feature = "hyperv")]
pub const HGEIE: u16 = 0x607;

/** Virtual supervisor Trap Setup */

/**
 *  Virtual supervisor status register. vsstatus
 *
 *    0      1   2              4   5      6     7      8     9 10      11 12      13 14      15
 * | WPRI | SIE |      WPRI      | SPIE | UBE | MPRI | SPP | VS[1:0] |   WPRI   | FS[1:0] | XS[1:0] |
 *     16       17    18    19    20         22   23     24    25                  30       31
 * | XS[1:0] | MPRI | SUM | MXR |     WPRI     | SPELP | SDT |           WPRI        | SD(riscv32) |
 *    32 33    34                                                                                 47
 * | UXL[1:0] |                                        WPRI                                         |
 *  48                                                                                       62  63
 * |                                             WPRI                                          | SD |
 */
#[cfg(feature = "hyperv")]
pub const VSSTATUS: u16 = 0x200;
#[cfg(feature = "hyperv")]
pub mod vsstatus {
    use bitflags::bitflags;
    bitflags! {
        #[derive(Debug,Clone, Copy,PartialEq, Eq)]
        pub struct VSstatus:usize{
            /// VS模式中断使能
            const sie = 1 << 1;
            /// VS模式中断前的中断状态
            const spie = 1 << 5;
            /// VU模式大小端对其 0: little 1: big
            const ube = 1 << 6;
            /// VS模式发生中断前的特权级别
            const spp = 1 << 8;
            /// 向量扩展状态
            const vs = 0b11 << 9;
            /// F/D/Q拓展状态
            const fs = 0b11 << 13;
            const xs = 0b11 << 15;
            const sum = 1 << 18;
            const mxr = 1 << 19;
            const spelp = 1 << 23;
            const sdt = 1 << 24;
            #[cfg(target_arch = "riscv32")]
            const sd = 1 << 31;
            #[cfg(target_arch = "riscv64")]
            const uxl = 0b11 << 32;
            #[cfg(target_arch = "riscv64")]
            const sd = 1 << 63;
        }
    }
    // #[cfg(target_arch = "riscv64")]
    pub use super::misa::{XLEN_128, XLEN_32, XLEN_64};
    #[cfg(target_arch = "riscv64")]
    pub use super::mstatus::UXL_SHIFT;
    pub use super::mstatus::{
        ENDIAN_BIG, ENDIAN_LITTLE, FS_CLEAN, FS_DIRTY, FS_INITIAL, FS_OFF, FS_SHIFT, MODE_MACHINE,
        MODE_SUPERVISOR, MODE_USER, SPP_SHIFT, UBE_SHIFT, VS_CLEAN, VS_DIRTY, VS_INITIAL, VS_OFF,
        VS_SHIFT, XS_ALL_OFF, XS_NON_DIRTY_OR_CLEAN_SOME_ON, XS_NON_DIRTY_SOME_CLEAN, XS_SHIFT,
        XS_SOME_DIRTY,
    };
}

/**
 * Virtual supervisor interrupt-enable register. vsie
 *
 *  15 14    13    12 10    9    8 6    5     4 2     1    0
 * |  0  | LCOFIE |  0  | SEIE |  0  | STIE |  0  | SSIE | 0 |
 */
#[cfg(feature = "hyperv")]
pub const VSIE: u16 = 0x204;
bitflags! {
    #[derive(Debug,Clone, Copy,PartialEq, Eq)]
    pub struct VSie:usize{
        const ssie = 1 << 1;
        const stie = 1 << 5;
        const seie = 1 << 9;
        const lcofie = 1 << 13;
    }
}

/**
 * Virtual supervisor trap handler base address. vstvec
 *
 * SXLEN - 1                 2   1 0
 * |       BASE[SXLEN-1:2]    | MODE |
 * */
#[cfg(feature = "hyperv")]
pub const VSTVEC: u16 = 0x205;
#[cfg(feature = "hyperv")]
pub mod vstvec {
    use bitflags::bitflags;
    bitflags! {
        #[derive(Debug,Clone, Copy,PartialEq, Eq)]
        pub struct VStvec:usize{
            const mode = 0b11;
            const base = !0b11;
        }
    }

    pub use super::mtvec::{BASE_SHIFT, MDOE_DIRECT, MDOE_VECTORED};
}

/** Machine Trap Setup */

/**
 * Machine status register. mstatus，riscv32 mstatush为高32位
 *
 *    0      1      2     3     4      5      6     7      8     9 10      11 12      13 14      15
 * | WPRI | SIE | WPRI | MIE | WPRI | SPIE | UBE | MPIE | SPP | VS[1:0] | MPP[1:0] | FS[1:0] | XS[1:0] |
 *     16       17    18    19    20    21   22     23     24    25                   30       31
 * | XS[1:0] | MPRV | SUM | MXR | TVM | TW | TSR | SPELP | SDT |           WPRI         | SD(mstatush) |
 *    32 33      34 35     36    37    38    39     40     41     42    43                           47
 * | UXL[1:0] | SXL[1:0] | SBE | MBE | GVA | MPV | WPRI | MPELP | MDT |              WPRI              |
 *  48                                                                                          62  63
 * |                                             WPRI                                             | SD |
 */
pub const MSTATUS: u16 = 0x300;
#[cfg(target_arch = "riscv32")]
pub const MSTATUSH: u16 = 0x310;
pub mod mstatus {
    use bitflags::bitflags;
    bitflags! {
        #[derive(Debug,Clone, Copy,PartialEq, Eq)]
        pub struct Mstatus:usize{
            /// S模式中断使能
            const sie = 1 << 1;
            /// M模式中断使能
            const mie = 1 << 3;
            /// S模式中断前的中断状态
            const spie = 1 << 5;
            /// U模式大小端对其 0: little 1: big
            const ube = 1 << 6;
            /// M模式中断前的中断状态
            const mpie = 1 << 7;
            /// S模式发生中断前的特权级别
            const spp = 1 << 8;
            /// 向量扩展状态
            const vs = 0b11 << 9;
            /// M模式发生中断前的特权级别
            const mpp = 0b11 << 11;
            /// F/D/Q拓展状态
            const fs = 0b11 << 13;
            /// 用户模式扩展的状态
            const xs = 0b11 << 15;
            /// modify privilege 更改特权模式访存权限, 0: 不受影响, 1: 使用当前特权模式的保护机制
            const mprv= 1 << 17;
            /// permit supervisor user memory access 1: 允许S模式访问用户内存 0: 反之
            const sum = 1 << 18;
            /// make executable readable 更改特权模式虚拟内存访存权限, 0: 可读, 1: 可读可执行
            const mxr = 1 << 19;
            const tvm = 1 << 20;
            const tw = 1 << 21;
            const tsr = 1 << 22;
            const spelp = 1 << 23;
            const sdt = 1 << 24;
            #[cfg(target_arch = "riscv32")]
            const sd = 1 << 31;
            #[cfg(target_arch = "riscv64")]
            const uxl = 0b11 << 32;
            #[cfg(target_arch = "riscv64")]
            const sxl = 0b11 << 34;
            /// U模式大小端对其 0: little 1: big
            #[cfg(target_arch = "riscv64")]
            const sbe = 1 << 36;
            /// U模式大小端对其 0: little 1: big
            #[cfg(target_arch = "riscv64")]
            const mbe = 1 << 37;
            #[cfg(all(target_arch = "riscv64", feature = "hyperv"))]
            const gva = 1 << 38;
            #[cfg(all(target_arch = "riscv64", feature = "hyperv"))]
            const mpv = 1 << 39;
            #[cfg(target_arch = "riscv64")]
            const mpelp = 1 << 41;
            #[cfg(target_arch = "riscv64")]
            const mdt = 1 << 42;
            #[cfg(target_arch = "riscv64")]
            const sd = 1 << 63;
        }
    }

    #[cfg(target_arch = "riscv32")]
    bitflags! {
        #[derive(Debug,Clone, Copy,PartialEq, Eq)]
        pub struct Mstatush:usize{
            const sbe = 1 << 4;
            const mbe = 1 << 5;
            #[cfg(feature = "hyperv")]
            const gva = 1 << 6;
            #[cfg(feature = "hyperv")]
            const mpv = 1 << 7;
            const mpelp = 1 << 9;
            const mdt = 1 << 10;
        }
    }

    pub const SPP_SHIFT: u8 = 8;
    pub const MPP_SHIFT: u8 = 11;

    pub const MODE_USER: usize = 0;
    pub const MODE_SUPERVISOR: usize = 1;
    pub const MODE_MACHINE: usize = 3;

    pub const UBE_SHIFT: u8 = 6;
    #[cfg(target_arch = "riscv64")]
    pub const SBE_SHIFT: u8 = 36;
    #[cfg(target_arch = "riscv32")]
    pub const SBE_SHIFT: u8 = 4;
    #[cfg(target_arch = "riscv64")]
    pub const MBE_SHIFT: u8 = 37;
    #[cfg(target_arch = "riscv32")]
    pub const MBE_SHIFT: u8 = 5;

    pub const ENDIAN_LITTLE: usize = 0;
    pub const ENDIAN_BIG: usize = 1;

    pub const VS_SHIFT: u8 = 9;
    pub const VS_OFF: usize = 0;
    pub const VS_INITIAL: usize = 1;
    pub const VS_CLEAN: usize = 2;
    pub const VS_DIRTY: usize = 3;

    pub const FS_SHIFT: u8 = 13;
    pub const FS_OFF: usize = 0;
    pub const FS_INITIAL: usize = 1;
    pub const FS_CLEAN: usize = 2;
    pub const FS_DIRTY: usize = 3;

    pub const XS_SHIFT: u8 = 15;
    pub const XS_ALL_OFF: usize = 0;
    pub const XS_NON_DIRTY_OR_CLEAN_SOME_ON: usize = 1;
    pub const XS_NON_DIRTY_SOME_CLEAN: usize = 2;
    pub const XS_SOME_DIRTY: usize = 3;

    #[cfg(target_arch = "riscv64")]
    pub const UXL_SHIFT: u8 = 32;
    #[cfg(target_arch = "riscv64")]
    pub const SXL_SHIFT: u8 = 34;
    #[cfg(target_arch = "riscv64")]
    pub use super::misa::{XLEN_128, XLEN_32, XLEN_64};
}

/// ISA and extension misa(RW)
pub const MISA: u16 = 0x301;
pub mod misa {
    use bitflags::bitflags;
    bitflags! {
        /// ```text
        /// MXLEN-1 MXLEN-2 MXLEN-3      26 25                   0
        /// |   MXL[1:0]   |    0(WARL)    |   Extensions[25:0]  |
        /// ```
        #[derive(Debug,Clone, Copy,PartialEq, Eq)]
        pub struct Misa:usize{
            #[cfg(target_arch = "riscv64")]
            const mxl = 0b11 << 62;
            #[cfg(target_arch = "riscv32")]
            const mxl = 0b11 << 30;
            const extensions = 0x3FFFFFF;
        }
    }

    #[cfg(target_arch = "riscv64")]
    pub const MXL_SHIFT: u8 = 62;
    #[cfg(target_arch = "riscv32")]
    pub const MXL_SHIFT: u8 = 30;
    pub const XLEN_32: usize = 1;
    pub const XLEN_64: usize = 2;
    pub const XLEN_128: usize = 3;

    bitflags! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct Extension:usize{
            /// Atomic extension
            const A = 1;
            /// B extension
            const B = 1 << 1;
            /// Compressed extension
            const C = 1 << 2;
            /// Double-precision floating-point extension
            const D = 1 << 3;
            /// RV32E/64E base ISA
            const E = 1 << 4;
            /// Single-precision floating-point extension
            const F = 1 << 5;
            /// Reserved
            const GReserved = 1 << 6;
            /// Hypervisor extension
            const H = 1 << 7;
            /// RV32I/64I/128I base ISA
            const I = 1 << 8;
            /// Reserved
            const J = 1 << 9;
            /// Reserved
            const K = 1 << 10;
            /// Reserved
            const L = 1 << 11;
            /// Integer Multiply/Divide extension
            const  M = 1 << 12;
            /// Tentatively reserved for User-Level Interrupts extension
            const N = 1 << 13;
            /// Reserved
            const O = 1 << 14;
            /// Tentatively reserved for Packed-SIMD extension
            const P = 1 << 15;
            /// Quad-precision floating-point extension
            const Q = 1 << 16;
            /// Reserved
            const R = 1 << 17;
            /// Supervisor mode implemented
            const S = 1 << 18;
            /// Reserved
            const T = 1 << 19;
            /// User mode implemented
            const U = 1 << 20;
            /// Vector extension
            const V = 1 << 21;
            /// Reserved
            const W = 1 << 22;
            /// Non-standard extensions present
            const X = 1 << 23;
            /// Reserved
            const Y = 1 << 24;
            /// Reserved
            const Z = 1 << 25;
        }
    }
}

/// Machine exception delegation register
pub const MEDELEG: u16 = 0x302;
#[cfg(target_arch = "riscv32")]
pub const MEDELEGH: u16 = 0x312;

/// Machine interrupt delegation register
pub const MIDELEG: u16 = 0x303;

/// Machine interrupt-enable register. mie
pub const MIE: u16 = 0x304;
bitflags! {
    /// ```text
    ///  15 14    13    12    11   10    9     8    7     6    5     4    3     2    1     0
    /// |  0  | LCOFIE | 0 | MEIE | 0 | SEIE | 0 | MTIE | 0 | STIE | 0 | MSIE | 0 | SSIE | 0 |
    /// ```
    #[derive(Debug,Clone, Copy,PartialEq, Eq)]
    pub struct Mie:usize{
        const ssie = 1 << 1;
        #[cfg(feature = "hyperv")]
        const vssie = 1 << 2;
        const msie = 1 << 3;
        const stie = 1 << 5;
        #[cfg(feature = "hyperv")]
        const vstie = 1 << 6;
        const mtie = 1 << 7;
        const seie = 1 << 9;
        #[cfg(feature = "hyperv")]
        const vseie = 1 << 10;
        const meie = 1 << 11;
        #[cfg(feature = "hyperv")]
        const sgeie = 1 << 12;
        const lcofie = 1 << 13;
    }
}

/// Machine trap-handler base address register. mtvec(MXLEN, MRW)
pub const MTVEC: u16 = 0x305;
pub mod mtvec {
    use bitflags::bitflags;
    bitflags! {
        /// Machine trap-handler base address register field.
        /// ```text
        /// MXLEN - 1                 2   1 0
        /// |       BASE[MXLEN-1:2]    | MODE |
        /// ```
        #[derive(Debug,Clone, Copy,PartialEq, Eq)]
        pub struct Mtvec:usize{
            const mode = 0b11;
            const base = !0b11;
        }
    }
    pub const BASE_SHIFT: u8 = 2;
    pub const MDOE_DIRECT: usize = 0;
    pub const MDOE_VECTORED: usize = 1;
}

/// Machine counter enable register. mcounteren(32bit, MRW)
pub const MCOUNTEREN: u16 = 0x306;
bitflags! {
    /// Machine counter enable register field.
    /// ```text
    ///    31      30     29      5    4      3     2    1    0
    /// | HPM31 | HPM30 |    ...    | HPM4 | HPM3 | IR | TM | CY |
    /// ```
    #[derive(Debug,Clone, Copy,PartialEq, Eq)]
    pub struct Mcounteren:u32{
        const cy = 1;
        const tm = 1 << 1;
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
