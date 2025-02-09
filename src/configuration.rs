use bitflags::bitflags;

/// Supervisor environment configuration register. senvcfg(SRW)
#[cfg(feature = "superv")]
pub const SENVCFG: u16 = 0x10A;
#[cfg(feature = "superv")]
pub mod senvcfg {
    use bitflags::bitflags;
    bitflags! {
        /// Machine environment configuration register field.
        /// ```text
        ///    0      1      2     3    4  5     6      7     8 15
        /// | FIOM | WPRI | LPE | SSE | CBIE | CBCFE | CBZE | WPRI |
        ///  16                                                  31
        /// |                         WPRI                         |
        ///  32 33 34                                            47
        /// | PWW |                  WPRI                          |
        ///  48                58  59    60     61     62      63
        /// |         WPRI       | DTE | CDE | ADUE | PBMTE | STCE |
        /// ```
        #[derive(Debug,Clone, Copy,PartialEq, Eq)]
        pub struct Senvcfg:usize{
            const fiom = 1;
            const lpe = 1 << 2;
            const sse = 1 << 3;
            const cbie = 0b11 << 4;
            const cbcfe = 1 << 6;
            const cbze = 1 << 7;
            #[cfg(target_arch = "riscv64")]
            const pmm = 0b11 << 32;
        }
    }
    pub use super::menvcfg::CBIE_SHIFT;
    #[cfg(target_arch = "riscv64")]
    pub use super::menvcfg::PMM_SHIFT;
}
/// Hypervisor environment configuration register
#[cfg(feature = "hyperv")]
pub const HENVCFG: u16 = 0x60A;
#[cfg(all(target_arch = "riscv32",feature = "hyperv"))]
pub const HENVCFGH: u16 = 0x61A;
#[cfg(feature = "hyperv")]
pub mod henvcfg {
    use bitflags::bitflags;
    bitflags! {
        /// Hypervisor environment configuration register field.
        /// ```text
        ///    0      1      2     3    4  5     6      7     8 15
        /// | FIOM | WPRI | LPE | SSE | CBIE | CBCFE | CBZE | WPRI |
        ///  16                                                  31
        /// |                         WPRI                         |
        ///  32 33 34                                            47
        /// | PWW |                  WPRI                          |
        ///  48                58  59    60     61     62      63
        /// |         WPRI       | DTE | CDE | ADUE | PBMTE | STCE |
        /// ```
        #[derive(Debug,Clone, Copy,PartialEq, Eq)]
        pub struct Henvcfg:usize{
            const fiom = 1;
            const lpe = 1 << 2;
            const sse = 1 << 3;
            const cbie = 0b11 << 4;
            const cbcfe = 1 << 6;
            const cbze = 1 << 7;
            #[cfg(target_arch = "riscv64")]
            const pmm = 0b11 << 32;
            #[cfg(target_arch = "riscv64")]
            const dte = 1 << 59;
            #[cfg(target_arch = "riscv64")]
            const cde = 1 << 60;
            #[cfg(target_arch = "riscv64")]
            const adue = 1 << 61;
            #[cfg(target_arch = "riscv64")]
            const pbmte = 1 << 62;
            #[cfg(target_arch = "riscv64")]
            const stce = 1 << 63;
        }
    }

    #[cfg(target_arch = "riscv32")]
    bitflags! {
        /// Hypervisor environment configuration register field.
        /// ```text
        ///  0   1 3                                             15
        /// | PMM |                  WPRI                          |
        ///  16                26  27    28     29     30      31
        /// |         WPRI       | DTE | CDE | ADUE | PBMTE | STCE |
        /// ```
        #[derive(Debug,Clone, Copy,PartialEq, Eq)]
        pub struct Henvcfgh:usize{
            const pmm = 0b11;
            const dte = 1 << 27;
            const cde = 1 << 28;
            const adue = 1 << 29;
            const pbmte = 1 << 30;
            const stce = 1 << 31;
        }
    }
    pub use super::menvcfg::CBIE_SHIFT;
    #[cfg(target_arch = "riscv64")]
    pub use super::menvcfg::PMM_SHIFT;
}

/// Machine Configuration

/// Machine environment configuration register
pub const MENVCFG: u16 = 0x30A;
#[cfg(target_arch = "riscv32")]
pub const MENVCFGH: u16 = 0x31A;
pub mod menvcfg {
    use bitflags::bitflags;
    bitflags! {
        /// Machine environment configuration register field.
        /// ```text
        ///    0      1      2     3    4  5     6      7     8 15
        /// | FIOM | WPRI | LPE | SSE | CBIE | CBCFE | CBZE | WPRI |
        ///  16                                                  31
        /// |                         WPRI                         |
        ///  32 33 34                                            47
        /// | PWW |                  WPRI                          |
        ///  48                58  59    60     61     62      63
        /// |         WPRI       | DTE | CDE | ADUE | PBMTE | STCE |
        /// ```
        #[derive(Debug,Clone, Copy,PartialEq, Eq)]
        pub struct Menvcfg:usize{
            const fiom = 1;
            const lpe = 1 << 2;
            const sse = 1 << 3;
            const cbie = 0b11 << 4;
            const cbcfe = 1 << 6;
            const cbze = 1 << 7;
            #[cfg(target_arch = "riscv64")]
            const pmm = 0b11 << 32;
            #[cfg(target_arch = "riscv64")]
            const dte = 1 << 59;
            #[cfg(target_arch = "riscv64")]
            const cde = 1 << 60;
            #[cfg(target_arch = "riscv64")]
            const adue = 1 << 61;
            #[cfg(target_arch = "riscv64")]
            const pbmte = 1 << 62;
            #[cfg(target_arch = "riscv64")]
            const stce = 1 << 63;
        }
    }

    #[cfg(target_arch = "riscv32")]
    bitflags! {
        /// Machine environment configuration register field.
        /// ```text
        ///  0   1 3                                             15
        /// | PMM |                  WPRI                          |
        ///  16                26  27    28     29     30      31
        /// |         WPRI       | DTE | CDE | ADUE | PBMTE | STCE |
        /// ```
        #[derive(Debug,Clone, Copy,PartialEq, Eq)]
        pub struct Menvcfgh:usize{
            const pmm = 0b11;
            const dte = 1 << 27;
            const cde = 1 << 28;
            const adue = 1 << 29;
            const pbmte = 1 << 30;
            const stce = 1 << 31;
        }
    }
    pub const CBIE_SHIFT: u8 = 4;
    #[cfg(target_arch = "riscv64")]
    pub const PMM_SHIFT: u8 = 32;
}

/// Machine security configuration register
pub const MSECCFG: u16 = 0x747;
#[cfg(target_arch = "riscv32")]
pub const MSECCFGH: u16 = 0x757;
bitflags! {
    /// Machine security configuration register field.
    /// ```text
    ///    0     1      2    3  7     8       9      10   11  15
    /// | MML | MMWP | RLB | WPRI | USEED | SSEED | MLPE | WPRI |
    ///  16                                                  31
    /// |                         WPRI                         |
    ///  32 33 34                                            47
    /// | PMM |                  WPRI                          |
    ///  48                                                  63
    /// |                         WPRI                         |
    /// ```
    #[derive(Debug,Clone, Copy,PartialEq, Eq)]
    pub struct Mseccfg:usize{
        const mml = 1;
        const mmwp = 1 << 1;
        const rlb = 1 << 2;
        const useed = 1 << 8;
        const sseed = 1 << 9;
        const mlpe = 1 << 10;
        #[cfg(target_arch = "riscv64")]
        const pmm = 0b11 << 32;
    }
}
#[cfg(target_arch = "riscv32")]
bitflags! {
    /// Machine security configuration register field.
    /// ```text
    ///   0 1  2      15
    /// | PMM |   WPRI  |
    ///  48           63
    /// |      WPRI     |
    /// ```
    #[derive(Debug,Clone, Copy,PartialEq, Eq)]
    pub struct Mseccfgh:usize{
        const pmm = 0b11;
    }
}
