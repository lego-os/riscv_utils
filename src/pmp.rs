#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
use crate::{csrr, csrs, csrw};

/// Machine Memory Protection
pub const PMPCFG0: u16 = 0x3A0;
#[cfg(target_arch = "riscv32")]
pub const PMPCFG1: u16 = 0x3A1;
pub const PMPCFG2: u16 = 0x3A2;
#[cfg(target_arch = "riscv32")]
pub const PMPCFG3: u16 = 0x3A3;
pub const PMPCFG4: u16 = 0x3A4;
#[cfg(target_arch = "riscv32")]
pub const PMPCFG5: u16 = 0x3A5;
pub const PMPCFG6: u16 = 0x3A6;
#[cfg(target_arch = "riscv32")]
pub const PMPCFG7: u16 = 0x3A7;
pub const PMPCFG8: u16 = 0x3A8;
#[cfg(target_arch = "riscv32")]
pub const PMPCFG9: u16 = 0x3A9;
pub const PMPCFG10: u16 = 0x3AA;
#[cfg(target_arch = "riscv32")]
pub const PMPCFG11: u16 = 0x3AB;
pub const PMPCFG12: u16 = 0x3AC;
#[cfg(target_arch = "riscv32")]
pub const PMPCFG13: u16 = 0x3AD;
pub const PMPCFG14: u16 = 0x3AE;
#[cfg(target_arch = "riscv32")]
pub const PMPCFG15: u16 = 0x3AF;
pub const PMPADDR0: u16 = 0x3B0;
pub const PMPADDR1: u16 = 0x3B1;
pub const PMPADDR2: u16 = 0x3B2;
pub const PMPADDR3: u16 = 0x3B3;
pub const PMPADDR4: u16 = 0x3B4;
pub const PMPADDR5: u16 = 0x3B5;
pub const PMPADDR6: u16 = 0x3B6;
pub const PMPADDR7: u16 = 0x3B7;
pub const PMPADDR8: u16 = 0x3B8;
pub const PMPADDR9: u16 = 0x3B9;
pub const PMPADDR10: u16 = 0x3BA;
pub const PMPADDR11: u16 = 0x3BB;
pub const PMPADDR12: u16 = 0x3BC;
pub const PMPADDR13: u16 = 0x3BD;
pub const PMPADDR14: u16 = 0x3BE;
pub const PMPADDR15: u16 = 0x3BF;
pub const PMPADDR16: u16 = 0x3C0;
pub const PMPADDR17: u16 = 0x3C1;
pub const PMPADDR18: u16 = 0x3C2;
pub const PMPADDR19: u16 = 0x3C3;
pub const PMPADDR20: u16 = 0x3C4;
pub const PMPADDR21: u16 = 0x3C5;
pub const PMPADDR22: u16 = 0x3C6;
pub const PMPADDR23: u16 = 0x3C7;
pub const PMPADDR24: u16 = 0x3C8;
pub const PMPADDR25: u16 = 0x3C9;
pub const PMPADDR26: u16 = 0x3CA;
pub const PMPADDR27: u16 = 0x3CB;
pub const PMPADDR28: u16 = 0x3CC;
pub const PMPADDR29: u16 = 0x3CD;
pub const PMPADDR30: u16 = 0x3CE;
pub const PMPADDR31: u16 = 0x3CF;
pub const PMPADDR32: u16 = 0x3D0;
pub const PMPADDR33: u16 = 0x3D1;
pub const PMPADDR34: u16 = 0x3D2;
pub const PMPADDR35: u16 = 0x3D3;
pub const PMPADDR36: u16 = 0x3D4;
pub const PMPADDR37: u16 = 0x3D5;
pub const PMPADDR38: u16 = 0x3D6;
pub const PMPADDR39: u16 = 0x3D7;
pub const PMPADDR40: u16 = 0x3D8;
pub const PMPADDR41: u16 = 0x3D9;
pub const PMPADDR42: u16 = 0x3DA;
pub const PMPADDR43: u16 = 0x3DB;
pub const PMPADDR44: u16 = 0x3DC;
pub const PMPADDR45: u16 = 0x3DD;
pub const PMPADDR46: u16 = 0x3DE;
pub const PMPADDR47: u16 = 0x3DF;
pub const PMPADDR48: u16 = 0x3E0;
pub const PMPADDR49: u16 = 0x3E1;
pub const PMPADDR50: u16 = 0x3E2;
pub const PMPADDR51: u16 = 0x3E3;
pub const PMPADDR52: u16 = 0x3E4;
pub const PMPADDR53: u16 = 0x3E5;
pub const PMPADDR54: u16 = 0x3E6;
pub const PMPADDR55: u16 = 0x3E7;
pub const PMPADDR56: u16 = 0x3E8;
pub const PMPADDR57: u16 = 0x3E9;
pub const PMPADDR58: u16 = 0x3EA;
pub const PMPADDR59: u16 = 0x3EB;
pub const PMPADDR60: u16 = 0x3EC;
pub const PMPADDR61: u16 = 0x3ED;
pub const PMPADDR62: u16 = 0x3EE;
pub const PMPADDR63: u16 = 0x3EF;

pub const PMP_FLAG_A_SHIFT: u8 = 3;
pub const PMP_FLAG_A_MASK: u8 = 0b11 << 3;
pub const PMP_FLAG_A_OFF: u8 = 0 << PMP_FLAG_A_SHIFT;
pub const PMP_FLAG_A_TOR: u8 = 1 << PMP_FLAG_A_SHIFT;
pub const PMP_FLAG_A_NA4: u8 = 2 << PMP_FLAG_A_SHIFT;
pub const PMP_FLAG_A_NA_POT: u8 = 3 << PMP_FLAG_A_SHIFT;
pub const PMP_FLAG_R: u8 = 1;
pub const PMP_FLAG_W: u8 = 1 << 1;
pub const PMP_FLAG_X: u8 = 1 << 2;
pub const PMP_FLAG_L: u8 = 1 << 7;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pmp {
    Cfg0 = 0,
    Cfg1 = 1,
    Cfg2 = 2,
    Cfg3 = 3,
    Cfg4 = 4,
    Cfg5 = 5,
    Cfg6 = 6,
    Cfg7 = 7,
    Cfg8 = 8,
    Cfg9 = 9,
    Cfg10 = 10,
    Cfg11 = 11,
    Cfg12 = 12,
    Cfg13 = 13,
    Cfg14 = 14,
    Cfg15 = 15,
    Cfg16 = 16,
    Cfg17 = 17,
    Cfg18 = 18,
    Cfg19 = 19,
    Cfg20 = 20,
    Cfg21 = 21,
    Cfg22 = 22,
    Cfg23 = 23,
    Cfg24 = 24,
    Cfg25 = 25,
    Cfg26 = 26,
    Cfg27 = 27,
    Cfg28 = 28,
    Cfg29 = 29,
    Cfg30 = 30,
    Cfg31 = 31,
    Cfg32 = 32,
    Cfg33 = 33,
    Cfg34 = 34,
    Cfg35 = 35,
    Cfg36 = 36,
    Cfg37 = 37,
    Cfg38 = 38,
    Cfg39 = 39,
    Cfg40 = 40,
    Cfg41 = 41,
    Cfg42 = 42,
    Cfg43 = 43,
    Cfg44 = 44,
    Cfg45 = 45,
    Cfg46 = 46,
    Cfg47 = 47,
    Cfg48 = 48,
    Cfg49 = 49,
    Cfg50 = 50,
    Cfg51 = 51,
    Cfg52 = 52,
    Cfg53 = 53,
    Cfg54 = 54,
    Cfg55 = 55,
    Cfg56 = 56,
    Cfg57 = 57,
    Cfg58 = 58,
    Cfg59 = 59,
    Cfg60 = 60,
    Cfg61 = 61,
    Cfg62 = 62,
    Cfg63 = 63,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PmpAddr {
    pub addr: usize,
    pub power: Option<u8>,
}

#[cfg(target_arch = "riscv64")]
pub fn config_pmp(flag: u8, pmp: Pmp, pmp_addr: PmpAddr) {
    let addr = addr_to_pmpaddr(flag & PMP_FLAG_A_MASK, pmp_addr);
    let mask = (flag as usize) << ((pmp as usize) % 8 * 8);
    match pmp {
        Pmp::Cfg0 => {
            csrw!(PMPADDR0, addr);
            csrs!(PMPCFG0, mask);
        }
        Pmp::Cfg1 => {
            csrw!(PMPADDR1, addr);
            csrs!(PMPCFG0, mask);
        }
        Pmp::Cfg2 => {
            csrw!(PMPADDR2, addr);
            csrs!(PMPCFG0, mask);
        }
        Pmp::Cfg3 => {
            csrw!(PMPADDR3, addr);
            csrs!(PMPCFG0, mask);
        }
        Pmp::Cfg4 => {
            csrw!(PMPADDR4, addr);
            csrs!(PMPCFG0, mask);
        }
        Pmp::Cfg5 => {
            csrw!(PMPADDR5, addr);
            csrs!(PMPCFG0, mask);
        }
        Pmp::Cfg6 => {
            csrw!(PMPADDR6, addr);
            csrs!(PMPCFG0, mask);
        }
        Pmp::Cfg7 => {
            csrw!(PMPADDR7, addr);
            csrs!(PMPCFG0, mask);
        }
        Pmp::Cfg8 => {
            csrw!(PMPADDR8, addr);
            csrs!(PMPCFG2, mask);
        }
        Pmp::Cfg9 => {
            csrw!(PMPADDR9, addr);
            csrs!(PMPCFG2, mask);
        }
        Pmp::Cfg10 => {
            csrw!(PMPADDR10, addr);
            csrs!(PMPCFG2, mask);
        }
        Pmp::Cfg11 => {
            csrw!(PMPADDR11, addr);
            csrs!(PMPCFG2, mask);
        }
        Pmp::Cfg12 => {
            csrw!(PMPADDR12, addr);
            csrs!(PMPCFG2, mask);
        }
        Pmp::Cfg13 => {
            csrw!(PMPADDR13, addr);
            csrs!(PMPCFG2, mask);
        }
        Pmp::Cfg14 => {
            csrw!(PMPADDR14, addr);
            csrs!(PMPCFG2, mask);
        }
        Pmp::Cfg15 => {
            csrw!(PMPADDR15, addr);
            csrs!(PMPCFG2, mask);
        }
        Pmp::Cfg16 => {
            csrw!(PMPADDR16, addr);
            csrs!(PMPCFG4, mask);
        }
        Pmp::Cfg17 => {
            csrw!(PMPADDR17, addr);
            csrs!(PMPCFG4, mask);
        }
        Pmp::Cfg18 => {
            csrw!(PMPADDR18, addr);
            csrs!(PMPCFG4, mask);
        }
        Pmp::Cfg19 => {
            csrw!(PMPADDR19, addr);
            csrs!(PMPCFG4, mask);
        }
        Pmp::Cfg20 => {
            csrw!(PMPADDR20, addr);
            csrs!(PMPCFG4, mask);
        }
        Pmp::Cfg21 => {
            csrw!(PMPADDR21, addr);
            csrs!(PMPCFG4, mask);
        }
        Pmp::Cfg22 => {
            csrw!(PMPADDR22, addr);
            csrs!(PMPCFG4, mask);
        }
        Pmp::Cfg23 => {
            csrw!(PMPADDR23, addr);
            csrs!(PMPCFG4, mask);
        }
        Pmp::Cfg24 => {
            csrw!(PMPADDR24, addr);
            csrs!(PMPCFG6, mask);
        }
        Pmp::Cfg25 => {
            csrw!(PMPADDR25, addr);
            csrs!(PMPCFG6, mask);
        }
        Pmp::Cfg26 => {
            csrw!(PMPADDR26, addr);
            csrs!(PMPCFG6, mask);
        }
        Pmp::Cfg27 => {
            csrw!(PMPADDR27, addr);
            csrs!(PMPCFG6, mask);
        }
        Pmp::Cfg28 => {
            csrw!(PMPADDR28, addr);
            csrs!(PMPCFG6, mask);
        }
        Pmp::Cfg29 => {
            csrw!(PMPADDR29, addr);
            csrs!(PMPCFG6, mask);
        }
        Pmp::Cfg30 => {
            csrw!(PMPADDR30, addr);
            csrs!(PMPCFG6, mask);
        }
        Pmp::Cfg31 => {
            csrw!(PMPADDR31, addr);
            csrs!(PMPCFG6, mask);
        }
        Pmp::Cfg32 => {
            csrw!(PMPADDR32, addr);
            csrs!(PMPCFG8, mask);
        }
        Pmp::Cfg33 => {
            csrw!(PMPADDR33, addr);
            csrs!(PMPCFG8, mask);
        }
        Pmp::Cfg34 => {
            csrw!(PMPADDR34, addr);
            csrs!(PMPCFG8, mask);
        }
        Pmp::Cfg35 => {
            csrw!(PMPADDR35, addr);
            csrs!(PMPCFG8, mask);
        }
        Pmp::Cfg36 => {
            csrw!(PMPADDR36, addr);
            csrs!(PMPCFG8, mask);
        }
        Pmp::Cfg37 => {
            csrw!(PMPADDR37, addr);
            csrs!(PMPCFG8, mask);
        }
        Pmp::Cfg38 => {
            csrw!(PMPADDR38, addr);
            csrs!(PMPCFG8, mask);
        }
        Pmp::Cfg39 => {
            csrw!(PMPADDR39, addr);
            csrs!(PMPCFG8, mask);
        }
        Pmp::Cfg40 => {
            csrw!(PMPADDR40, addr);
            csrs!(PMPCFG10, mask);
        }
        Pmp::Cfg41 => {
            csrw!(PMPADDR41, addr);
            csrs!(PMPCFG10, mask);
        }
        Pmp::Cfg42 => {
            csrw!(PMPADDR42, addr);
            csrs!(PMPCFG10, mask);
        }
        Pmp::Cfg43 => {
            csrw!(PMPADDR43, addr);
            csrs!(PMPCFG10, mask);
        }
        Pmp::Cfg44 => {
            csrw!(PMPADDR44, addr);
            csrs!(PMPCFG10, mask);
        }
        Pmp::Cfg45 => {
            csrw!(PMPADDR45, addr);
            csrs!(PMPCFG10, mask);
        }
        Pmp::Cfg46 => {
            csrw!(PMPADDR46, addr);
            csrs!(PMPCFG10, mask);
        }
        Pmp::Cfg47 => {
            csrw!(PMPADDR47, addr);
            csrs!(PMPCFG10, mask);
        }
        Pmp::Cfg48 => {
            csrw!(PMPADDR48, addr);
            csrs!(PMPCFG12, mask);
        }
        Pmp::Cfg49 => {
            csrw!(PMPADDR49, addr);
            csrs!(PMPCFG12, mask);
        }
        Pmp::Cfg50 => {
            csrw!(PMPADDR50, addr);
            csrs!(PMPCFG12, mask);
        }
        Pmp::Cfg51 => {
            csrw!(PMPADDR51, addr);
            csrs!(PMPCFG12, mask);
        }
        Pmp::Cfg52 => {
            csrw!(PMPADDR52, addr);
            csrs!(PMPCFG12, mask);
        }
        Pmp::Cfg53 => {
            csrw!(PMPADDR53, addr);
            csrs!(PMPCFG12, mask);
        }
        Pmp::Cfg54 => {
            csrw!(PMPADDR54, addr);
            csrs!(PMPCFG12, mask);
        }
        Pmp::Cfg55 => {
            csrw!(PMPADDR55, addr);
            csrs!(PMPCFG12, mask);
        }
        Pmp::Cfg56 => {
            csrw!(PMPADDR56, addr);
            csrs!(PMPCFG14, mask);
        }
        Pmp::Cfg57 => {
            csrw!(PMPADDR57, addr);
            csrs!(PMPCFG14, mask);
        }
        Pmp::Cfg58 => {
            csrw!(PMPADDR58, addr);
            csrs!(PMPCFG14, mask);
        }
        Pmp::Cfg59 => {
            csrw!(PMPADDR59, addr);
            csrs!(PMPCFG14, mask);
        }
        Pmp::Cfg60 => {
            csrw!(PMPADDR60, addr);
            csrs!(PMPCFG14, mask);
        }
        Pmp::Cfg61 => {
            csrw!(PMPADDR61, addr);
            csrs!(PMPCFG14, mask);
        }
        Pmp::Cfg62 => {
            csrw!(PMPADDR62, addr);
            csrs!(PMPCFG14, mask);
        }
        Pmp::Cfg63 => {
            csrw!(PMPADDR63, addr);
            csrs!(PMPCFG14, mask);
        }
    }
}

#[cfg(target_arch = "riscv32")]
pub fn config_pmp(flag: u8, pmp: Pmp, pmp_addr: PmpAddr) {
    let addr = addr_to_pmpaddr(flag & PMP_FLAG_A_MASK, pmp_addr);
    let mask = (flag as usize) << ((pmp as usize) % 4 * 4);
    match pmp {
        Pmp::Cfg0 => {
            csrw!(PMPADDR0, addr);
            csrs!(PMPCFG0, mask);
        }
        Pmp::Cfg1 => {
            csrw!(PMPADDR1, addr);
            csrs!(PMPCFG0, mask);
        }
        Pmp::Cfg2 => {
            csrw!(PMPADDR2, addr);
            csrs!(PMPCFG0, mask);
        }
        Pmp::Cfg3 => {
            csrw!(PMPADDR3, addr);
            csrs!(PMPCFG0, mask);
        }
        Pmp::Cfg4 => {
            csrw!(PMPADDR4, addr);
            csrs!(PMPCFG1, mask);
        }
        Pmp::Cfg5 => {
            csrw!(PMPADDR5, addr);
            csrs!(PMPCFG1, mask);
        }
        Pmp::Cfg6 => {
            csrw!(PMPADDR6, addr);
            csrs!(PMPCFG1, mask);
        }
        Pmp::Cfg7 => {
            csrw!(PMPADDR7, addr);
            csrs!(PMPCFG1, mask);
        }
        Pmp::Cfg8 => {
            csrw!(PMPADDR8, addr);
            csrs!(PMPCFG2, mask);
        }
        Pmp::Cfg9 => {
            csrw!(PMPADDR9, addr);
            csrs!(PMPCFG2, mask);
        }
        Pmp::Cfg10 => {
            csrw!(PMPADDR10, addr);
            csrs!(PMPCFG2, mask);
        }
        Pmp::Cfg11 => {
            csrw!(PMPADDR11, addr);
            csrs!(PMPCFG2, mask);
        }
        Pmp::Cfg12 => {
            csrw!(PMPADDR12, addr);
            csrs!(PMPCFG3, mask);
        }
        Pmp::Cfg13 => {
            csrw!(PMPADDR13, addr);
            csrs!(PMPCFG3, mask);
        }
        Pmp::Cfg14 => {
            csrw!(PMPADDR14, addr);
            csrs!(PMPCFG3, mask);
        }
        Pmp::Cfg15 => {
            csrw!(PMPADDR15, addr);
            csrs!(PMPCFG3, mask);
        }
        Pmp::Cfg16 => {
            csrw!(PMPADDR16, addr);
            csrs!(PMPCFG4, mask);
        }
        Pmp::Cfg17 => {
            csrw!(PMPADDR17, addr);
            csrs!(PMPCFG4, mask);
        }
        Pmp::Cfg18 => {
            csrw!(PMPADDR18, addr);
            csrs!(PMPCFG4, mask);
        }
        Pmp::Cfg19 => {
            csrw!(PMPADDR19, addr);
            csrs!(PMPCFG4, mask);
        }
        Pmp::Cfg20 => {
            csrw!(PMPADDR20, addr);
            csrs!(PMPCFG5, mask);
        }
        Pmp::Cfg21 => {
            csrw!(PMPADDR21, addr);
            csrs!(PMPCFG5, mask);
        }
        Pmp::Cfg22 => {
            csrw!(PMPADDR22, addr);
            csrs!(PMPCFG5, mask);
        }
        Pmp::Cfg23 => {
            csrw!(PMPADDR23, addr);
            csrs!(PMPCFG5, mask);
        }
        Pmp::Cfg24 => {
            csrw!(PMPADDR24, addr);
            csrs!(PMPCFG6, mask);
        }
        Pmp::Cfg25 => {
            csrw!(PMPADDR25, addr);
            csrs!(PMPCFG6, mask);
        }
        Pmp::Cfg26 => {
            csrw!(PMPADDR26, addr);
            csrs!(PMPCFG6, mask);
        }
        Pmp::Cfg27 => {
            csrw!(PMPADDR27, addr);
            csrs!(PMPCFG6, mask);
        }
        Pmp::Cfg28 => {
            csrw!(PMPADDR28, addr);
            csrs!(PMPCFG7, mask);
        }
        Pmp::Cfg29 => {
            csrw!(PMPADDR29, addr);
            csrs!(PMPCFG7, mask);
        }
        Pmp::Cfg30 => {
            csrw!(PMPADDR30, addr);
            csrs!(PMPCFG7, mask);
        }
        Pmp::Cfg31 => {
            csrw!(PMPADDR31, addr);
            csrs!(PMPCFG7, mask);
        }
        Pmp::Cfg32 => {
            csrw!(PMPADDR32, addr);
            csrs!(PMPCFG8, mask);
        }
        Pmp::Cfg33 => {
            csrw!(PMPADDR33, addr);
            csrs!(PMPCFG8, mask);
        }
        Pmp::Cfg34 => {
            csrw!(PMPADDR34, addr);
            csrs!(PMPCFG8, mask);
        }
        Pmp::Cfg35 => {
            csrw!(PMPADDR35, addr);
            csrs!(PMPCFG8, mask);
        }
        Pmp::Cfg36 => {
            csrw!(PMPADDR36, addr);
            csrs!(PMPCFG9, mask);
        }
        Pmp::Cfg37 => {
            csrw!(PMPADDR37, addr);
            csrs!(PMPCFG9, mask);
        }
        Pmp::Cfg38 => {
            csrw!(PMPADDR38, addr);
            csrs!(PMPCFG9, mask);
        }
        Pmp::Cfg39 => {
            csrw!(PMPADDR39, addr);
            csrs!(PMPCFG9, mask);
        }
        Pmp::Cfg40 => {
            csrw!(PMPADDR40, addr);
            csrs!(PMPCFG10, mask);
        }
        Pmp::Cfg41 => {
            csrw!(PMPADDR41, addr);
            csrs!(PMPCFG10, mask);
        }
        Pmp::Cfg42 => {
            csrw!(PMPADDR42, addr);
            csrs!(PMPCFG10, mask);
        }
        Pmp::Cfg43 => {
            csrw!(PMPADDR43, addr);
            csrs!(PMPCFG10, mask);
        }
        Pmp::Cfg44 => {
            csrw!(PMPADDR44, addr);
            csrs!(PMPCFG11, mask);
        }
        Pmp::Cfg45 => {
            csrw!(PMPADDR45, addr);
            csrs!(PMPCFG11, mask);
        }
        Pmp::Cfg46 => {
            csrw!(PMPADDR46, addr);
            csrs!(PMPCFG11, mask);
        }
        Pmp::Cfg47 => {
            csrw!(PMPADDR47, addr);
            csrs!(PMPCFG11, mask);
        }
        Pmp::Cfg48 => {
            csrw!(PMPADDR48, addr);
            csrs!(PMPCFG12, mask);
        }
        Pmp::Cfg49 => {
            csrw!(PMPADDR49, addr);
            csrs!(PMPCFG12, mask);
        }
        Pmp::Cfg50 => {
            csrw!(PMPADDR50, addr);
            csrs!(PMPCFG12, mask);
        }
        Pmp::Cfg51 => {
            csrw!(PMPADDR51, addr);
            csrs!(PMPCFG12, mask);
        }
        Pmp::Cfg52 => {
            csrw!(PMPADDR52, addr);
            csrs!(PMPCFG13, mask);
        }
        Pmp::Cfg53 => {
            csrw!(PMPADDR53, addr);
            csrs!(PMPCFG13, mask);
        }
        Pmp::Cfg54 => {
            csrw!(PMPADDR54, addr);
            csrs!(PMPCFG13, mask);
        }
        Pmp::Cfg55 => {
            csrw!(PMPADDR55, addr);
            csrs!(PMPCFG13, mask);
        }
        Pmp::Cfg56 => {
            csrw!(PMPADDR56, addr);
            csrs!(PMPCFG14, mask);
        }
        Pmp::Cfg57 => {
            csrw!(PMPADDR57, addr);
            csrs!(PMPCFG14, mask);
        }
        Pmp::Cfg58 => {
            csrw!(PMPADDR58, addr);
            csrs!(PMPCFG14, mask);
        }
        Pmp::Cfg59 => {
            csrw!(PMPADDR59, addr);
            csrs!(PMPCFG14, mask);
        }
        Pmp::Cfg60 => {
            csrw!(PMPADDR60, addr);
            csrs!(PMPCFG15, mask);
        }
        Pmp::Cfg61 => {
            csrw!(PMPADDR61, addr);
            csrs!(PMPCFG15, mask);
        }
        Pmp::Cfg62 => {
            csrw!(PMPADDR62, addr);
            csrs!(PMPCFG15, mask);
        }
        Pmp::Cfg63 => {
            csrw!(PMPADDR63, addr);
            csrs!(PMPCFG15, mask);
        }
    }
}

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
fn addr_to_pmpaddr(flag_a: u8, pmp_addr: PmpAddr) -> usize {
    if flag_a == PMP_FLAG_A_OFF {
        0
    } else if flag_a == PMP_FLAG_A_NA_POT {
        let g = pmp_addr.power.unwrap() - 2;
        let addr = pmp_addr.addr >> (g + 2);
        (addr << g) | (1 << (g - 1) - 1)
    } else {
        pmp_addr.addr >> 2
    }
}

#[cfg(target_arch = "riscv64")]
pub fn read_cfg(pmp: Pmp) -> Option<PmpAddr> {
    let offset = (pmp as usize) % 8 * 8;
    match pmp {
        Pmp::Cfg0 => pmpaddr_to_addr(
            csrr!(PMPADDR0),
            (csrr!(PMPCFG0) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg1 => pmpaddr_to_addr(
            csrr!(PMPADDR1),
            (csrr!(PMPCFG0) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg2 => pmpaddr_to_addr(
            csrr!(PMPADDR2),
            (csrr!(PMPCFG0) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg3 => pmpaddr_to_addr(
            csrr!(PMPADDR3),
            (csrr!(PMPCFG0) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg4 => pmpaddr_to_addr(
            csrr!(PMPADDR4),
            (csrr!(PMPCFG0) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg5 => pmpaddr_to_addr(
            csrr!(PMPADDR5),
            (csrr!(PMPCFG0) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg6 => pmpaddr_to_addr(
            csrr!(PMPADDR6),
            (csrr!(PMPCFG0) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg7 => pmpaddr_to_addr(
            csrr!(PMPADDR7),
            (csrr!(PMPCFG0) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg8 => pmpaddr_to_addr(
            csrr!(PMPADDR8),
            (csrr!(PMPCFG2) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg9 => pmpaddr_to_addr(
            csrr!(PMPADDR9),
            (csrr!(PMPCFG2) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg10 => pmpaddr_to_addr(
            csrr!(PMPADDR10),
            (csrr!(PMPCFG2) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg11 => pmpaddr_to_addr(
            csrr!(PMPADDR11),
            (csrr!(PMPCFG2) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg12 => pmpaddr_to_addr(
            csrr!(PMPADDR12),
            (csrr!(PMPCFG2) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg13 => pmpaddr_to_addr(
            csrr!(PMPADDR13),
            (csrr!(PMPCFG2) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg14 => pmpaddr_to_addr(
            csrr!(PMPADDR14),
            (csrr!(PMPCFG2) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg15 => pmpaddr_to_addr(
            csrr!(PMPADDR15),
            (csrr!(PMPCFG2) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg16 => pmpaddr_to_addr(
            csrr!(PMPADDR16),
            (csrr!(PMPCFG4) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg17 => pmpaddr_to_addr(
            csrr!(PMPADDR17),
            (csrr!(PMPCFG4) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg18 => pmpaddr_to_addr(
            csrr!(PMPADDR18),
            (csrr!(PMPCFG4) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg19 => pmpaddr_to_addr(
            csrr!(PMPADDR19),
            (csrr!(PMPCFG4) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg20 => pmpaddr_to_addr(
            csrr!(PMPADDR20),
            (csrr!(PMPCFG4) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg21 => pmpaddr_to_addr(
            csrr!(PMPADDR21),
            (csrr!(PMPCFG4) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg22 => pmpaddr_to_addr(
            csrr!(PMPADDR22),
            (csrr!(PMPCFG4) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg23 => pmpaddr_to_addr(
            csrr!(PMPADDR23),
            (csrr!(PMPCFG4) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg24 => pmpaddr_to_addr(
            csrr!(PMPADDR24),
            (csrr!(PMPCFG6) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg25 => pmpaddr_to_addr(
            csrr!(PMPADDR25),
            (csrr!(PMPCFG6) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg26 => pmpaddr_to_addr(
            csrr!(PMPADDR26),
            (csrr!(PMPCFG6) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg27 => pmpaddr_to_addr(
            csrr!(PMPADDR27),
            (csrr!(PMPCFG6) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg28 => pmpaddr_to_addr(
            csrr!(PMPADDR28),
            (csrr!(PMPCFG6) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg29 => pmpaddr_to_addr(
            csrr!(PMPADDR29),
            (csrr!(PMPCFG6) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg30 => pmpaddr_to_addr(
            csrr!(PMPADDR30),
            (csrr!(PMPCFG6) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg31 => pmpaddr_to_addr(
            csrr!(PMPADDR31),
            (csrr!(PMPCFG6) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg32 => pmpaddr_to_addr(
            csrr!(PMPADDR32),
            (csrr!(PMPCFG8) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg33 => pmpaddr_to_addr(
            csrr!(PMPADDR33),
            (csrr!(PMPCFG8) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg34 => pmpaddr_to_addr(
            csrr!(PMPADDR34),
            (csrr!(PMPCFG8) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg35 => pmpaddr_to_addr(
            csrr!(PMPADDR35),
            (csrr!(PMPCFG8) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg36 => pmpaddr_to_addr(
            csrr!(PMPADDR36),
            (csrr!(PMPCFG8) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg37 => pmpaddr_to_addr(
            csrr!(PMPADDR37),
            (csrr!(PMPCFG8) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg38 => pmpaddr_to_addr(
            csrr!(PMPADDR38),
            (csrr!(PMPCFG8) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg39 => pmpaddr_to_addr(
            csrr!(PMPADDR39),
            (csrr!(PMPCFG8) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg40 => pmpaddr_to_addr(
            csrr!(PMPADDR40),
            (csrr!(PMPCFG10) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg41 => pmpaddr_to_addr(
            csrr!(PMPADDR41),
            (csrr!(PMPCFG10) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg42 => pmpaddr_to_addr(
            csrr!(PMPADDR42),
            (csrr!(PMPCFG10) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg43 => pmpaddr_to_addr(
            csrr!(PMPADDR43),
            (csrr!(PMPCFG10) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg44 => pmpaddr_to_addr(
            csrr!(PMPADDR44),
            (csrr!(PMPCFG10) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg45 => pmpaddr_to_addr(
            csrr!(PMPADDR45),
            (csrr!(PMPCFG10) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg46 => pmpaddr_to_addr(
            csrr!(PMPADDR46),
            (csrr!(PMPCFG10) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg47 => pmpaddr_to_addr(
            csrr!(PMPADDR47),
            (csrr!(PMPCFG10) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg48 => pmpaddr_to_addr(
            csrr!(PMPADDR48),
            (csrr!(PMPCFG12) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg49 => pmpaddr_to_addr(
            csrr!(PMPADDR49),
            (csrr!(PMPCFG12) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg50 => pmpaddr_to_addr(
            csrr!(PMPADDR50),
            (csrr!(PMPCFG12) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg51 => pmpaddr_to_addr(
            csrr!(PMPADDR51),
            (csrr!(PMPCFG12) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg52 => pmpaddr_to_addr(
            csrr!(PMPADDR52),
            (csrr!(PMPCFG12) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg53 => pmpaddr_to_addr(
            csrr!(PMPADDR53),
            (csrr!(PMPCFG12) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg54 => pmpaddr_to_addr(
            csrr!(PMPADDR54),
            (csrr!(PMPCFG12) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg55 => pmpaddr_to_addr(
            csrr!(PMPADDR55),
            (csrr!(PMPCFG12) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg56 => pmpaddr_to_addr(
            csrr!(PMPADDR56),
            (csrr!(PMPCFG14) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg57 => pmpaddr_to_addr(
            csrr!(PMPADDR57),
            (csrr!(PMPCFG14) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg58 => pmpaddr_to_addr(
            csrr!(PMPADDR58),
            (csrr!(PMPCFG14) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg59 => pmpaddr_to_addr(
            csrr!(PMPADDR59),
            (csrr!(PMPCFG14) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg60 => pmpaddr_to_addr(
            csrr!(PMPADDR60),
            (csrr!(PMPCFG14) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg61 => pmpaddr_to_addr(
            csrr!(PMPADDR61),
            (csrr!(PMPCFG14) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg62 => pmpaddr_to_addr(
            csrr!(PMPADDR62),
            (csrr!(PMPCFG14) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg63 => pmpaddr_to_addr(
            csrr!(PMPADDR63),
            (csrr!(PMPCFG14) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
    }
}

#[cfg(target_arch = "riscv32")]
pub fn read_cfg(pmp: Pmp) -> Option<PmpAddr> {
    let offset = (pmp as usize) % 4 * 4;
    match pmp {
        Pmp::Cfg0 => pmpaddr_to_addr(
            csrr!(PMPADDR0),
            (csrr!(PMPCFG0) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg1 => pmpaddr_to_addr(
            csrr!(PMPADDR1),
            (csrr!(PMPCFG0) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg2 => pmpaddr_to_addr(
            csrr!(PMPADDR2),
            (csrr!(PMPCFG0) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg3 => pmpaddr_to_addr(
            csrr!(PMPADDR3),
            (csrr!(PMPCFG0) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg4 => pmpaddr_to_addr(
            csrr!(PMPADDR4),
            (csrr!(PMPCFG1) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg5 => pmpaddr_to_addr(
            csrr!(PMPADDR5),
            (csrr!(PMPCFG1) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg6 => pmpaddr_to_addr(
            csrr!(PMPADDR6),
            (csrr!(PMPCFG1) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg7 => pmpaddr_to_addr(
            csrr!(PMPADDR7),
            (csrr!(PMPCFG1) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg8 => pmpaddr_to_addr(
            csrr!(PMPADDR8),
            (csrr!(PMPCFG2) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg9 => pmpaddr_to_addr(
            csrr!(PMPADDR9),
            (csrr!(PMPCFG2) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg10 => pmpaddr_to_addr(
            csrr!(PMPADDR10),
            (csrr!(PMPCFG2) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg11 => pmpaddr_to_addr(
            csrr!(PMPADDR11),
            (csrr!(PMPCFG2) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg12 => pmpaddr_to_addr(
            csrr!(PMPADDR12),
            (csrr!(PMPCFG3) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg13 => pmpaddr_to_addr(
            csrr!(PMPADDR13),
            (csrr!(PMPCFG3) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg14 => pmpaddr_to_addr(
            csrr!(PMPADDR14),
            (csrr!(PMPCFG3) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg15 => pmpaddr_to_addr(
            csrr!(PMPADDR15),
            (csrr!(PMPCFG3) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg16 => pmpaddr_to_addr(
            csrr!(PMPADDR16),
            (csrr!(PMPCFG4) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg17 => pmpaddr_to_addr(
            csrr!(PMPADDR17),
            (csrr!(PMPCFG4) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg18 => pmpaddr_to_addr(
            csrr!(PMPADDR18),
            (csrr!(PMPCFG4) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg19 => pmpaddr_to_addr(
            csrr!(PMPADDR19),
            (csrr!(PMPCFG4) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg20 => pmpaddr_to_addr(
            csrr!(PMPADDR20),
            (csrr!(PMPCFG5) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg21 => pmpaddr_to_addr(
            csrr!(PMPADDR21),
            (csrr!(PMPCFG5) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg22 => pmpaddr_to_addr(
            csrr!(PMPADDR22),
            (csrr!(PMPCFG5) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg23 => pmpaddr_to_addr(
            csrr!(PMPADDR23),
            (csrr!(PMPCFG5) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg24 => pmpaddr_to_addr(
            csrr!(PMPADDR24),
            (csrr!(PMPCFG6) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg25 => pmpaddr_to_addr(
            csrr!(PMPADDR25),
            (csrr!(PMPCFG6) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg26 => pmpaddr_to_addr(
            csrr!(PMPADDR26),
            (csrr!(PMPCFG6) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg27 => pmpaddr_to_addr(
            csrr!(PMPADDR27),
            (csrr!(PMPCFG6) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg28 => pmpaddr_to_addr(
            csrr!(PMPADDR28),
            (csrr!(PMPCFG7) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg29 => pmpaddr_to_addr(
            csrr!(PMPADDR29),
            (csrr!(PMPCFG7) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg30 => pmpaddr_to_addr(
            csrr!(PMPADDR30),
            (csrr!(PMPCFG7) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg31 => pmpaddr_to_addr(
            csrr!(PMPADDR31),
            (csrr!(PMPCFG7) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg32 => pmpaddr_to_addr(
            csrr!(PMPADDR32),
            (csrr!(PMPCFG8) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg33 => pmpaddr_to_addr(
            csrr!(PMPADDR33),
            (csrr!(PMPCFG8) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg34 => pmpaddr_to_addr(
            csrr!(PMPADDR34),
            (csrr!(PMPCFG8) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg35 => pmpaddr_to_addr(
            csrr!(PMPADDR35),
            (csrr!(PMPCFG8) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg36 => pmpaddr_to_addr(
            csrr!(PMPADDR36),
            (csrr!(PMPCFG9) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg37 => pmpaddr_to_addr(
            csrr!(PMPADDR37),
            (csrr!(PMPCFG9) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg38 => pmpaddr_to_addr(
            csrr!(PMPADDR38),
            (csrr!(PMPCFG9) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg39 => pmpaddr_to_addr(
            csrr!(PMPADDR39),
            (csrr!(PMPCFG9) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg40 => pmpaddr_to_addr(
            csrr!(PMPADDR40),
            (csrr!(PMPCFG10) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg41 => pmpaddr_to_addr(
            csrr!(PMPADDR41),
            (csrr!(PMPCFG10) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg42 => pmpaddr_to_addr(
            csrr!(PMPADDR42),
            (csrr!(PMPCFG10) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg43 => pmpaddr_to_addr(
            csrr!(PMPADDR43),
            (csrr!(PMPCFG10) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg44 => pmpaddr_to_addr(
            csrr!(PMPADDR44),
            (csrr!(PMPCFG11) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg45 => pmpaddr_to_addr(
            csrr!(PMPADDR45),
            (csrr!(PMPCFG11) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg46 => pmpaddr_to_addr(
            csrr!(PMPADDR46),
            (csrr!(PMPCFG11) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg47 => pmpaddr_to_addr(
            csrr!(PMPADDR47),
            (csrr!(PMPCFG11) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg48 => pmpaddr_to_addr(
            csrr!(PMPADDR48),
            (csrr!(PMPCFG12) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg49 => pmpaddr_to_addr(
            csrr!(PMPADDR49),
            (csrr!(PMPCFG12) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg50 => pmpaddr_to_addr(
            csrr!(PMPADDR50),
            (csrr!(PMPCFG12) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg51 => pmpaddr_to_addr(
            csrr!(PMPADDR51),
            (csrr!(PMPCFG12) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg52 => pmpaddr_to_addr(
            csrr!(PMPADDR52),
            (csrr!(PMPCFG13) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg53 => pmpaddr_to_addr(
            csrr!(PMPADDR53),
            (csrr!(PMPCFG13) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg54 => pmpaddr_to_addr(
            csrr!(PMPADDR54),
            (csrr!(PMPCFG13) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg55 => pmpaddr_to_addr(
            csrr!(PMPADDR55),
            (csrr!(PMPCFG13) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg56 => pmpaddr_to_addr(
            csrr!(PMPADDR56),
            (csrr!(PMPCFG14) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg57 => pmpaddr_to_addr(
            csrr!(PMPADDR57),
            (csrr!(PMPCFG14) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg58 => pmpaddr_to_addr(
            csrr!(PMPADDR58),
            (csrr!(PMPCFG14) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg59 => pmpaddr_to_addr(
            csrr!(PMPADDR59),
            (csrr!(PMPCFG14) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg60 => pmpaddr_to_addr(
            csrr!(PMPADDR60),
            (csrr!(PMPCFG15) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg61 => pmpaddr_to_addr(
            csrr!(PMPADDR61),
            (csrr!(PMPCFG15) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg62 => pmpaddr_to_addr(
            csrr!(PMPADDR62),
            (csrr!(PMPCFG15) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
        Pmp::Cfg63 => pmpaddr_to_addr(
            csrr!(PMPADDR63),
            (csrr!(PMPCFG15) >> offset) as u8 & PMP_FLAG_A_MASK,
        ),
    }
}

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
fn pmpaddr_to_addr(csr_addr_value: usize, flag_a: u8) -> Option<PmpAddr> {
    if flag_a == PMP_FLAG_A_OFF {
        None
    } else if flag_a == PMP_FLAG_A_NA_POT {
        let trailing_ones = csr_addr_value.trailing_ones();
        let addr = csr_addr_value & !((1 << trailing_ones) - 1);
        let g = (trailing_ones + 1) as u8;
        Some(PmpAddr {
            addr,
            power: Some(g + 2),
        })
    } else {
        Some(PmpAddr {
            addr: csr_addr_value << 2,
            power: None,
        })
    }
}
