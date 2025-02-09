/// Supervisor Protection and Translation

/// satp(SRW)
pub const SATP: u16 = 0x180;
pub mod satp {
    use bitflags::bitflags;

    // #[cfg(target_arch = "riscv64")]
    bitflags! {
        #[derive(Debug,Clone, Copy,PartialEq, Eq)]
        pub struct Satp:usize{
            const ppn = 0xFFFFFFFFFFF;
            const asid = 0xFFFF << 44;
            const mode = 0xF << 60;
        }
    }
    #[cfg(target_arch = "riscv32")]
    bitflags! {
        #[derive(Debug,Clone, Copy,PartialEq, Eq)]
        pub struct Satp:usize{
            const ppn = 0x3FFFFF;
            const asid = 0x1FF << 22;
            const mode = 1 << 31;
        }
    }
    #[cfg(target_arch = "riscv32")]
    pub const MODE_SHIFT: u8 = 31;
    #[cfg(target_arch = "riscv64")]
    pub const MODE_SHIFT: u8 = 60;
    pub const MODE_BARE: usize = 0;
    #[cfg(target_arch = "riscv32")]
    pub const MODE_SV32: usize = 1;
    #[cfg(target_arch = "riscv64")]
    pub const MODE_SV39: usize = 8;
    #[cfg(target_arch = "riscv64")]
    pub const MODE_SV48: usize = 9;
    #[cfg(target_arch = "riscv64")]
    pub const MODE_SV57: usize = 10;
    #[cfg(target_arch = "riscv64")]
    pub const MODE_SV64: usize = 11;

    /// VPN mask
    #[cfg(target_arch = "riscv32")]
    pub const VPN_MASK: usize = 0x3FF;
    #[cfg(target_arch = "riscv64")]
    pub const VPN_MASK: usize = 0x1FF;

    /// VPN shift
    pub const VPN0_SHIFT: u8 = 12;
    #[cfg(target_arch = "riscv32")]
    pub const VPN1_SHIFT: u8 = 22;
    #[cfg(target_arch = "riscv64")]
    pub const VPN1_SHIFT: u8 = 21;
    #[cfg(target_arch = "riscv64")]
    pub const VPN2_SHIFT: u8 = 30;
    #[cfg(target_arch = "riscv64")]
    pub const VPN3_SHIFT: u8 = 39;
    #[cfg(target_arch = "riscv64")]
    pub const VPN4_SHIFT: u8 = 48;

    /// ppn mask
    #[cfg(target_arch = "riscv32")]
    pub const PPN_MASK: usize = 0x3FF;
    #[cfg(target_arch = "riscv64")]
    pub const PPN_MASK: usize = 0x1FF;
    #[cfg(target_arch = "riscv32")]
    pub const SV32_PPN1_MASK: usize = 0xFFF;
    #[cfg(target_arch = "riscv64")]
    pub const SV39_PPN2_MASK: usize = 0x3FFFFFF;
    #[cfg(target_arch = "riscv64")]
    // pub const SV48_PPN3_MASK: usize = 0x1FFFF;
    // #[cfg(target_arch = "riscv64")]
    // pub const SV57_PPN4_MASK: usize = 0xFF;

    /// ppn shift
    pub const PPN0_SHIFT: u8 = 12;
    #[cfg(target_arch = "riscv32")]
    pub const PPN1_SHIFT: u8 = 22;
    #[cfg(target_arch = "riscv64")]
    pub const PPN1_SHIFT: u8 = 21;
    #[cfg(target_arch = "riscv64")]
    pub const PPN2_SHIFT: u8 = 30;
    #[cfg(target_arch = "riscv64")]
    pub const PPN3_SHIFT: u8 = 39;
    #[cfg(target_arch = "riscv64")]
    pub const PPN4_SHIFT: u8 = 48;

    /// pte ppn mask
    #[cfg(target_arch = "riscv32")]
    pub const PTE_ALL_PPN_MASK: usize = 0x3FFFFF;
    #[cfg(target_arch = "riscv64")]
    pub const PTE_ALL_PPN_MASK: usize = 0xFFFFFFFFFFF;

    /// pte ppn shift
    pub const PTE_PPN0_SHIFT: u8 = 10;
    #[cfg(target_arch = "riscv32")]
    pub const PTE_PPN1_SHIFT: u8 = 20;
    #[cfg(target_arch = "riscv64")]
    pub const PTE_PPN1_SHIFT: u8 = 19;
    #[cfg(target_arch = "riscv64")]
    pub const PTE_PPN2_SHIFT: u8 = 28;
    #[cfg(target_arch = "riscv64")]
    pub const PTE_PPN3_SHIFT: u8 = 37;
    #[cfg(target_arch = "riscv64")]
    pub const PTE_PPN4_SHIFT: u8 = 48;

    #[cfg(target_arch = "riscv32")]
    pub const PTE_SIZE: usize = 4;
    #[cfg(target_arch = "riscv64")]
    pub const PTE_SIZE: usize = 8;

    bitflags! {
        #[derive(Debug,Clone, Copy,PartialEq, Eq)]
        pub struct PteFlags:u16{
            const v = 1;
            const r = 1 << 1;
            const w = 1 << 2;
            const x = 1 << 3;
            const u = 1 << 4;
            const g = 1 << 5;
            const a = 1 << 6;
            const d = 1 << 7;
            const rsw = 0b11 << 8;
        }
    }
}
/// Hypervisor guest address translation and protection
pub const HGATP: u16 = 0x680;

/// Virtual supervisor address translation and protection
pub const VSATP: u16 = 0x280;
