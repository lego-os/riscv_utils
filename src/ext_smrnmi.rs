/// Machine Non-Maskable Interrupt Handling

/// Resumable NMI scratch register
pub const MNSCRATCH: u16 = 0x740;
/// Resumable NMI program counter
pub const MNEPC: u16 = 0x741;
/// Resumable NMI cause
pub const MNCAUSE: u16 = 0x742;
/// Resumable NMI status
pub const MNSTATUS: u16 = 0x744;
