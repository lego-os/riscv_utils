/// Supervisor State Enable Registers

/// sstateen0(SRW)
#[cfg(feature = "superv")]
pub const SSTATEEN0: u16 = 0x10C;

/// sstateen1(SRW)
#[cfg(feature = "superv")]
pub const SSTATEEN1: u16 = 0x10D;

/// sstateen2(SRW)
#[cfg(feature = "superv")]
pub const SSTATEEN2: u16 = 0x10E;

/// sstateen3(SRW)
#[cfg(feature = "superv")]
pub const SSTATEEN3: u16 = 0x10F;

/// Hypervisor State Enable Registers
/// hstateen0(HRW)
#[cfg(feature = "hyperv")]
pub const HSTATEEN0: u16 = 0x60C;

/// hstateen1(HRW)
#[cfg(feature = "hyperv")]
pub const HSTATEEN1: u16 = 0x60D;

/// hstateen2(HRW)
#[cfg(feature = "hyperv")]
pub const HSTATEEN2: u16 = 0x60E;

/// hstateen3(HRW)
#[cfg(feature = "hyperv")]
pub const HSTATEEN3: u16 = 0x60F;

/// hstateen0h(HRW)
#[cfg(all(target_arch = "riscv32",feature="hyperv"))]
pub const HSTATEEN0H: u16 = 0x61C;

/// hstateen1h(HRW)
#[cfg(all(target_arch = "riscv32",feature="hyperv"))]
pub const HSTATEEN1H: u16 = 0x61D;

/// hstateen2h(HRW)
#[cfg(all(target_arch = "riscv32",feature="hyperv"))]
pub const HSTATEEN2H: u16 = 0x61E;

/// hstateen3h(HRW)
#[cfg(all(target_arch = "riscv32",feature="hyperv"))]
pub const HSTATEEN3H: u16 = 0x61F;

/// Machine State Enable Registers

/// Machine State Enable 0 Register.
pub const MSTATEEN0: u16 = 0x30C;
/// Machine State Enable 1 Register.
pub const MSTATEEN1: u16 = 0x30D;
/// Machine State Enable 2 Register.
pub const MSTATEEN2: u16 = 0x30E;
/// Machine State Enable 3 Register.
pub const MSTATEEN3: u16 = 0x30F;
#[cfg(target_arch = "riscv32")]
pub const MSTATEEN0H: u16 = 0x31C;
#[cfg(target_arch = "riscv32")]
pub const MSTATEEN1H: u16 = 0x31D;
#[cfg(target_arch = "riscv32")]
pub const MSTATEEN2H: u16 = 0x31E;
#[cfg(target_arch = "riscv32")]
pub const MSTATEEN3H: u16 = 0x31F;
