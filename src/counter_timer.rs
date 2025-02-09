/// 计数器、定时器

/// Unprivileged Counter/Timers

/// Cycle counter for RDCYCLE instruction. cycle(URO)
pub const CYCLE: u16 = 0xC00;
/// Timer for RDTIME instruction. time(URO)
pub const TIME: u16 = 0xC01;
/// Instructions-retired counter for RDINSTRET instruction. instret(URO)
pub const INSTRET: u16 = 0xC02;
/// Performance-monitoring counter. hpmcounterX(URO)
pub const HPMPCOUNTER3: u16 = 0xC03;
pub const HPMPCOUNTER4: u16 = 0xC04;
pub const HPMPCOUNTER5: u16 = 0xC05;
pub const HPMPCOUNTER6: u16 = 0xC06;
pub const HPMPCOUNTER7: u16 = 0xC07;
pub const HPMPCOUNTER8: u16 = 0xC08;
pub const HPMPCOUNTER9: u16 = 0xC09;
pub const HPMPCOUNTER10: u16 = 0xC0A;
pub const HPMPCOUNTER11: u16 = 0xC0B;
pub const HPMPCOUNTER12: u16 = 0xC0C;
pub const HPMPCOUNTER13: u16 = 0xC0D;
pub const HPMPCOUNTER14: u16 = 0xC0E;
pub const HPMPCOUNTER15: u16 = 0xC0F;
pub const HPMPCOUNTER16: u16 = 0xC10;
pub const HPMPCOUNTER17: u16 = 0xC11;
pub const HPMPCOUNTER18: u16 = 0xC12;
pub const HPMPCOUNTER19: u16 = 0xC13;
pub const HPMPCOUNTER20: u16 = 0xC14;
pub const HPMPCOUNTER21: u16 = 0xC15;
pub const HPMPCOUNTER22: u16 = 0xC16;
pub const HPMPCOUNTER23: u16 = 0xC17;
pub const HPMPCOUNTER24: u16 = 0xC18;
pub const HPMPCOUNTER25: u16 = 0xC19;
pub const HPMPCOUNTER26: u16 = 0xC1A;
pub const HPMPCOUNTER27: u16 = 0xC1B;
pub const HPMPCOUNTER28: u16 = 0xC1C;
pub const HPMPCOUNTER29: u16 = 0xC1D;
pub const HPMPCOUNTER30: u16 = 0xC1E;
pub const HPMPCOUNTER31: u16 = 0xC1F;
/// Upper 32 bits of cycle, RV32 only. cycleh(URO)
#[cfg(target_arch = "riscv32")]
pub const CYCLEH: u16 = 0xC80;
/// Upper 32 bits of time, RV32 only. timeh(URO)
#[cfg(target_arch = "riscv32")]
pub const TIMEH: u16 = 0xC81;
/// Upper 32 bits of instret, RV32 only. instreth(URO)
#[cfg(target_arch = "riscv32")]
pub const INSTRETH: u16 = 0xC82;
/// Upper 32 bits of hpmcounter, RV32 only. hpmcounterXh(URO)
#[cfg(target_arch = "riscv32")]
pub const HPMPCOUNTER3H: u16 = 0xC83;
#[cfg(target_arch = "riscv32")]
pub const HPMPCOUNTER4H: u16 = 0xC84;
#[cfg(target_arch = "riscv32")]
pub const HPMPCOUNTER5H: u16 = 0xC85;
#[cfg(target_arch = "riscv32")]
pub const HPMPCOUNTER6H: u16 = 0xC86;
#[cfg(target_arch = "riscv32")]
pub const HPMPCOUNTER7H: u16 = 0xC87;
#[cfg(target_arch = "riscv32")]
pub const HPMPCOUNTER8H: u16 = 0xC88;
#[cfg(target_arch = "riscv32")]
pub const HPMPCOUNTER9H: u16 = 0xC89;
#[cfg(target_arch = "riscv32")]
pub const HPMPCOUNTER10H: u16 = 0xC8A;
#[cfg(target_arch = "riscv32")]
pub const HPMPCOUNTER11H: u16 = 0xC8B;
#[cfg(target_arch = "riscv32")]
pub const HPMPCOUNTER12H: u16 = 0xC8C;
#[cfg(target_arch = "riscv32")]
pub const HPMPCOUNTER13H: u16 = 0xC8D;
#[cfg(target_arch = "riscv32")]
pub const HPMPCOUNTER14H: u16 = 0xC8E;
#[cfg(target_arch = "riscv32")]
pub const HPMPCOUNTER15H: u16 = 0xC8F;
#[cfg(target_arch = "riscv32")]
pub const HPMPCOUNTER16H: u16 = 0xC90;
#[cfg(target_arch = "riscv32")]
pub const HPMPCOUNTER17H: u16 = 0xC91;
#[cfg(target_arch = "riscv32")]
pub const HPMPCOUNTER18H: u16 = 0xC92;
#[cfg(target_arch = "riscv32")]
pub const HPMPCOUNTER19H: u16 = 0xC93;
#[cfg(target_arch = "riscv32")]
pub const HPMPCOUNTER20H: u16 = 0xC94;
#[cfg(target_arch = "riscv32")]
pub const HPMPCOUNTER21H: u16 = 0xC95;
#[cfg(target_arch = "riscv32")]
pub const HPMPCOUNTER22H: u16 = 0xC96;
#[cfg(target_arch = "riscv32")]
pub const HPMPCOUNTER23H: u16 = 0xC97;
#[cfg(target_arch = "riscv32")]
pub const HPMPCOUNTER24H: u16 = 0xC98;
#[cfg(target_arch = "riscv32")]
pub const HPMPCOUNTER25H: u16 = 0xC99;
#[cfg(target_arch = "riscv32")]
pub const HPMPCOUNTER26H: u16 = 0xC9A;
#[cfg(target_arch = "riscv32")]
pub const HPMPCOUNTER27H: u16 = 0xC9B;
#[cfg(target_arch = "riscv32")]
pub const HPMPCOUNTER28H: u16 = 0xC9C;
#[cfg(target_arch = "riscv32")]
pub const HPMPCOUNTER29H: u16 = 0xC9D;
#[cfg(target_arch = "riscv32")]
pub const HPMPCOUNTER30H: u16 = 0xC9E;
#[cfg(target_arch = "riscv32")]
pub const HPMPCOUNTER31H: u16 = 0xC9F;

/// Hypervisor Counter/Timer Virtualization Registers

/// Delta for VS/VU-mode timer
#[cfg(feature = "hyperv")]
pub const HTIMEDELTA: u16 = 0x605;
#[cfg(all(target_arch = "riscv32",feature = "hyperv"))]
pub const HTIMEDELTAH: u16 = 0x615;

/// Machine Counter/Timers

/// Machine cycle counter
pub const MCYCLE: u16 = 0xB00;
/// Machine instructions-retired counter
pub const MINSTRET: u16 = 0xB02;
/// Machine performance-monitoring counter
pub const MHPMPCOUNTER3: u16 = 0xB03;
pub const MHPMPCOUNTER4: u16 = 0xB04;
pub const MHPMPCOUNTER5: u16 = 0xB05;
pub const MHPMPCOUNTER6: u16 = 0xB06;
pub const MHPMPCOUNTER7: u16 = 0xB07;
pub const MHPMPCOUNTER8: u16 = 0xB08;
pub const MHPMPCOUNTER9: u16 = 0xB09;
pub const MHPMPCOUNTER10: u16 = 0xB0A;
pub const MHPMPCOUNTER11: u16 = 0xB0B;
pub const MHPMPCOUNTER12: u16 = 0xB0C;
pub const MHPMPCOUNTER13: u16 = 0xB0D;
pub const MHPMPCOUNTER14: u16 = 0xB0E;
pub const MHPMPCOUNTER15: u16 = 0xB0F;
pub const MHPMPCOUNTER16: u16 = 0xB10;
pub const MHPMPCOUNTER17: u16 = 0xB11;
pub const MHPMPCOUNTER18: u16 = 0xB12;
pub const MHPMPCOUNTER19: u16 = 0xB13;
pub const MHPMPCOUNTER20: u16 = 0xB14;
pub const MHPMPCOUNTER21: u16 = 0xB15;
pub const MHPMPCOUNTER22: u16 = 0xB16;
pub const MHPMPCOUNTER23: u16 = 0xB17;
pub const MHPMPCOUNTER24: u16 = 0xB18;
pub const MHPMPCOUNTER25: u16 = 0xB19;
pub const MHPMPCOUNTER26: u16 = 0xB1A;
pub const MHPMPCOUNTER27: u16 = 0xB1B;
pub const MHPMPCOUNTER28: u16 = 0xB1C;
pub const MHPMPCOUNTER29: u16 = 0xB1D;
pub const MHPMPCOUNTER30: u16 = 0xB1E;
pub const MHPMPCOUNTER31: u16 = 0xB1F;
/// Upper 32 bits of mcycle, RV32 only
#[cfg(target_arch = "riscv32")]
pub const MCYCLEH: u16 = 0xB80;
/// Upper 32 bits of minstret, RV32 only
#[cfg(target_arch = "riscv32")]
pub const MINSTRETH: u16 = 0xB82;
/// Upper 32 bits of mhpmcounter, RV32 only
#[cfg(target_arch = "riscv32")]
pub const MHPMPCOUNTER3H: u16 = 0xB83;
#[cfg(target_arch = "riscv32")]
pub const MHPMPCOUNTER4H: u16 = 0xB84;
#[cfg(target_arch = "riscv32")]
pub const MHPMPCOUNTER5H: u16 = 0xB85;
#[cfg(target_arch = "riscv32")]
pub const MHPMPCOUNTER6H: u16 = 0xB86;
#[cfg(target_arch = "riscv32")]
pub const MHPMPCOUNTER7H: u16 = 0xB87;
#[cfg(target_arch = "riscv32")]
pub const MHPMPCOUNTER8H: u16 = 0xB88;
#[cfg(target_arch = "riscv32")]
pub const MHPMPCOUNTER9H: u16 = 0xB89;
#[cfg(target_arch = "riscv32")]
pub const MHPMPCOUNTER10H: u16 = 0xB8A;
#[cfg(target_arch = "riscv32")]
pub const MHPMPCOUNTER11H: u16 = 0xB8B;
#[cfg(target_arch = "riscv32")]
pub const MHPMPCOUNTER12H: u16 = 0xB8C;
#[cfg(target_arch = "riscv32")]
pub const MHPMPCOUNTER13H: u16 = 0xB8D;
#[cfg(target_arch = "riscv32")]
pub const MHPMPCOUNTER14H: u16 = 0xB8E;
#[cfg(target_arch = "riscv32")]
pub const MHPMPCOUNTER15H: u16 = 0xB8F;
#[cfg(target_arch = "riscv32")]
pub const MHPMPCOUNTER16H: u16 = 0xB90;
#[cfg(target_arch = "riscv32")]
pub const MHPMPCOUNTER17H: u16 = 0xB91;
#[cfg(target_arch = "riscv32")]
pub const MHPMPCOUNTER18H: u16 = 0xB92;
#[cfg(target_arch = "riscv32")]
pub const MHPMPCOUNTER19H: u16 = 0xB93;
#[cfg(target_arch = "riscv32")]
pub const MHPMPCOUNTER20H: u16 = 0xB94;
#[cfg(target_arch = "riscv32")]
pub const MHPMPCOUNTER21H: u16 = 0xB95;
#[cfg(target_arch = "riscv32")]
pub const MHPMPCOUNTER22H: u16 = 0xB96;
#[cfg(target_arch = "riscv32")]
pub const MHPMPCOUNTER23H: u16 = 0xB97;
#[cfg(target_arch = "riscv32")]
pub const MHPMPCOUNTER24H: u16 = 0xB98;
#[cfg(target_arch = "riscv32")]
pub const MHPMPCOUNTER25H: u16 = 0xB99;
#[cfg(target_arch = "riscv32")]
pub const MHPMPCOUNTER26H: u16 = 0xB9A;
#[cfg(target_arch = "riscv32")]
pub const MHPMPCOUNTER27H: u16 = 0xB9B;
#[cfg(target_arch = "riscv32")]
pub const MHPMPCOUNTER28H: u16 = 0xB9C;
#[cfg(target_arch = "riscv32")]
pub const MHPMPCOUNTER29H: u16 = 0xB9D;
#[cfg(target_arch = "riscv32")]
pub const MHPMPCOUNTER30H: u16 = 0xB9E;
#[cfg(target_arch = "riscv32")]
pub const MHPMPCOUNTER31H: u16 = 0xB9F;