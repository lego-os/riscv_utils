/// Zicsr 拓展

/// 读取csr
#[macro_export]
macro_rules! csrr {
    ($csr:ident) => {{
        use core::arch::asm;
        let mut ret:usize;
        unsafe{
            asm!("csrr {0},{1}", out(reg) ret,const $csr,options(nomem, nostack));
        }
        ret
    }};
    ($csr:ident,$xlen:ty) => {{
        use core::arch::asm;
        let mut ret:$xlen;
        unsafe{
            asm!("csrr {0},{1}", out(reg) ret,const $csr,options(nomem, nostack));
        }
        ret
    }};
}

/// 写入csr
#[macro_export]
macro_rules! csrw {
    ($csr:ident,$val:ident) => {{
        use core::arch::asm;
        unsafe {
            asm!("csrw {0}, {1}",const $csr,in(reg) $val,options(nomem, nostack));
        }
    }};
    ($csr:ident,$val:expr) => {{
        use core::arch::asm;
        unsafe {
            asm!("csrw {0}, {1}",const $csr,in(reg) $val,options(nomem, nostack));
        }
    }};
}

/// 置位csr
#[macro_export]
macro_rules! csrs {
    ($csr:ident,$val:ident) => {{
        use core::arch::asm;
        unsafe {
            asm!("csrs {0}, {1}", const $csr,in(reg) $val,options(nomem, nostack));
        }
    }};
    ($csr:ident,$val:expr) => {{
        use core::arch::asm;
        unsafe {
            asm!("csrs {0}, {1}", const $csr,in(reg) $val,options(nomem, nostack));
        }
    }};
}

/// 清除位csr
#[macro_export]
macro_rules! csrc {
    ($csr:ident,$val:ident) => {{
        use core::arch::asm;
        unsafe {
            asm!("csrc {0}, {1}", const $csr,in(reg) $val,options(nomem, nostack));
        }
    }};
    ($csr:ident,$val:expr) => {{
        use core::arch::asm;
        unsafe {
            asm!("csrc {0}, {1}", const $csr,in(reg) $val,options(nomem, nostack));
        }
    }};
}
