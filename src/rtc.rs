use core::time::Duration;

/// 读取mtime时间
#[allow(unused)]
#[inline]
pub fn read_mtime(map_addr: usize) -> u64 {
    let addr = map_addr as *mut u64;
    unsafe { addr.read() }
}

/// 设置mtime时间
#[allow(unused)]
#[inline]
pub fn set_mtime(map_addr: usize, tick: u64) {
    let addr = map_addr as *mut u64;
    unsafe {
        addr.write(tick);
    }
}

/// 读取mtimecmp
#[allow(unused)]
#[inline]
pub fn read_mtimecmp(map_addr: usize) -> u64 {
    let addr = map_addr as *mut u64;
    unsafe { addr.read() }
}

/// 设置mtimecmp
#[allow(unused)]
#[inline]
pub fn set_mtimecmp(map_addr: usize, tick: u64) {
    let addr = map_addr as *mut u64;
    unsafe {
        addr.write(tick);
    }
}

/// mtime转为时间
#[inline(always)]
#[allow(unused)]
pub const fn tick_to_dur(tick: u64, time_base: u64) -> Duration {
    Duration::from_micros(tick.saturating_mul(1000000).saturating_div(time_base))
}

/// 时间转为mtime
#[inline(always)]
#[allow(unused)]
pub const fn micros_to_tick(micros: u64, time_base: u64) -> u64 {
    micros.saturating_mul(time_base).saturating_div(1000000)
}

#[inline(always)]
#[allow(unused)]
pub const fn millis_to_tick(millis: u64, time_base: u64) -> u64 {
    millis.saturating_mul(time_base).saturating_div(1000)
}

#[inline(always)]
#[allow(unused)]
pub const fn secs_to_tick(secs: u64, time_base: u64) -> u64 {
    secs.saturating_mul(time_base)
}
