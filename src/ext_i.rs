/// 整数寄存器
pub struct IntegerRegister {
    /// x0 zero：硬件零寄存器
    /// x1 链接寄存器：记录过程调用返回地址 caller-save
    pub ra: usize,
    /// x2 栈指针寄存器 callee-save
    pub sp: usize,
    /// x3 全局指针寄存器
    pub gp: usize,
    /// x4 线程指针寄存器
    pub tp: usize,
    /// x5 - x7 临时寄存器 x5可作为备用链接寄存器 caller-save
    pub t0: usize,
    pub t1: usize,
    pub t2: usize,
    /// x8 - x9 跨越调用过程保存的值 callee-save
    pub s0: usize,
    pub s1: usize,
    /// x10 - x17 参数寄存器 过程调用传递参数 caller-save
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
    pub a6: usize,
    pub a7: usize,
    /// x18 - x27 跨越调用过程保存的值 callee-save
    pub s2: usize,
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,
    /// x28 - x31 临时寄存器 caller-save
    pub t3: usize,
    pub t4: usize,
    pub t5: usize,
    pub t6: usize,
}
