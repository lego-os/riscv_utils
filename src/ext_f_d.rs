/// 浮点数扩展 F,D

/// Unprivileged Floating-Point CSRs
pub mod u_float {
    /// Floating-Point Accrued Exceptions. fflags(URW)
    pub const FFLAGS: u16 = 0x001;

    /// Floating-Point Dynamic Rounding Mode. frm(URW)
    pub const FRM: u16 = 0x002;

    /// Floating-Point Control and Status Register (frm +fflags). fcsr(URW)
    pub const FCSR: u16 = 0x003;
}

/// 浮点数寄存器
#[cfg(target_arch = "riscv32")]
pub struct FloatRegister {
    /// f0 - f7 临时寄存器 caller-save
    pub ft0: f32,
    pub ft1: f32,
    pub ft2: f32,
    pub ft3: f32,
    pub ft4: f32,
    pub ft5: f32,
    pub ft6: f32,
    pub ft7: f32,
    /// f8 - f9 跨越调用过程保存的值 callee-save
    pub fs0: f32,
    pub fs1: f32,
    /// f10 - f17 参数寄存器
    pub fa0: f32,
    pub fa1: f32,
    pub fa2: f32,
    pub fa3: f32,
    pub fa4: f32,
    pub fa5: f32,
    pub fa6: f32,
    pub fa7: f32,
    /// f18 - f27 跨越调用过程保存的值 callee-save
    pub fs2: f32,
    pub fs3: f32,
    pub fs4: f32,
    pub fs5: f32,
    pub fs6: f32,
    pub fs7: f32,
    pub fs8: f32,
    pub fs9: f32,
    pub fs10: f32,
    pub fs11: f32,
    /// f28 - f31 临时寄存器 caller-save
    pub ft8: f32,
    pub ft9: f32,
    pub ft10: f32,
    pub ft11: f32,
}

/// 浮点数寄存器
#[cfg(target_arch = "riscv64")]
pub struct FloatRegister {
    /// f0 - f7 临时寄存器 caller-save
    pub ft0: f64,
    pub ft1: f64,
    pub ft2: f64,
    pub ft3: f64,
    pub ft4: f64,
    pub ft5: f64,
    pub ft6: f64,
    pub ft7: f64,
    /// f18 - f27 跨越调用过程保存的值 callee-save
    pub fs0: f64,
    pub fs1: f64,
    /// f10 - f17 参数寄存器
    pub fa0: f64,
    pub fa1: f64,
    pub fa2: f64,
    pub fa3: f64,
    pub fa4: f64,
    pub fa5: f64,
    pub fa6: f64,
    pub fa7: f64,
    /// f18 - f27 跨越调用过程保存的值 callee-save
    pub fs2: f64,
    pub fs3: f64,
    pub fs4: f64,
    pub fs5: f64,
    pub fs6: f64,
    pub fs7: f64,
    pub fs8: f64,
    pub fs9: f64,
    pub fs10: f64,
    pub fs11: f64,
    /// f28 - f31 临时寄存器 caller-save
    pub ft8: f64,
    pub ft9: f64,
    pub ft10: f64,
    pub ft11: f64,
}
