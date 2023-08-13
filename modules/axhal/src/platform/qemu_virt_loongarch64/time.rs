use loongarch64::register::time::Time;
const NANOS_PER_TICK: u64 = crate::time::NANOS_PER_SEC / axconfig::TIMER_FREQUENCY as u64;

/// Returns the current clock time in hardware ticks.
#[inline]
pub fn current_ticks() -> u64 {
    Time::read() as u64
}

/// Converts hardware ticks to nanoseconds.
#[inline]
pub const fn ticks_to_nanos(ticks: u64) -> u64 {
    ticks * NANOS_PER_TICK
}

/// Converts nanoseconds to hardware ticks.
#[inline]
pub const fn nanos_to_ticks(nanos: u64) -> u64 {
    nanos / NANOS_PER_TICK
}

/// Set a one-shot timer.
///
/// A timer interrupt will be triggered at the given deadline (in nanoseconds).
#[cfg(feature = "irq")]
pub fn set_oneshot_timer(deadline_ns: u64) {
    use loongarch64::register::csr::Register;
    use loongarch64::register::tcfg::Tcfg;
    use loongarch64::register::ticlr::Ticlr;
    Ticlr::read().clear_timer().write(); //清除时钟中断

    info!("set_oneshot_timer");
    Tcfg::read()
        .set_enable(true)
        .set_loop(true)
        .set_initval(nanos_to_ticks(deadline_ns) as usize)
        .write(); //设置计时器的配置
}

pub(super) fn init_primary() {
    #[cfg(feature = "irq")]
    {
        info!("time::init_primary");
        use loongarch64::register::csr::Register;
        use loongarch64::register::tcfg::Tcfg;
        use loongarch64::register::ticlr::Ticlr;
        Ticlr::read().clear_timer().write(); //清除时钟中断
        Tcfg::read()
            .set_enable(true)
            .set_loop(true)
            .set_initval(10000000 as usize)
            .write();
        //super::irq::set_enable(super::irq::TIMER_IRQ_NUM, true);
    }
}
