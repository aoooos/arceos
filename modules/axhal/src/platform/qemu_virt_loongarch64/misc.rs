/// Shutdown the whole system, including all CPUs.
pub fn terminate() -> ! {
    while true {}
    unreachable!("It should shutdown!")
}
