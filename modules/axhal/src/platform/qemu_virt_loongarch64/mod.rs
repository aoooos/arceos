mod boot;
pub mod console;
pub mod mem;
pub mod misc;
pub mod time;

#[cfg(feature = "irq")]
pub mod irq;

#[cfg(feature = "smp")]
pub mod mp;

extern "C" {
    fn trap_vector_base();
    fn rust_main(cpu_id: usize, dtb: usize);
    #[cfg(feature = "smp")]
    fn rust_main_secondary(cpu_id: usize);
}

unsafe extern "C" fn rust_entry(cpu_id: usize, _dtb: usize) {
    crate::mem::clear_bss();
    crate::cpu::init_primary(cpu_id);
    crate::arch::set_trap_vector_base(trap_vector_base as usize);
    rust_main(cpu_id, 0);
}

#[cfg(feature = "smp")]
unsafe extern "C" fn rust_entry_secondary(cpu_id: usize) {
    crate::arch::set_trap_vector_base(trap_vector_base as usize);
    crate::cpu::init_secondary(cpu_id);
    rust_main_secondary(cpu_id);
}

/// Initializes the platform devices for the primary CPU.
///
/// For example, the interrupt controller and external interrupts.
pub fn platform_init() {
    use loongarch64::register::csr::Register;
    #[cfg(feature = "irq")]
    {
        debug!(
            "mod.rs -> platform_init, before irq::init_primary(), irq = {}, pie = {}",
            loongarch64::register::crmd::Crmd::read().get_ie(),
            loongarch64::register::prmd::Prmd::read().get_pie()
        );
        self::irq::init_primary();
        debug!(
            "mod.rs -> platform_init, after irq::init_primary(), before time::init_primary(), irq = {}, pie = {}",
            loongarch64::register::crmd::Crmd::read().get_ie(),
            loongarch64::register::prmd::Prmd::read().get_pie()
        );
        self::time::init_primary();
        debug!(
            "mod.rs -> platform_init, after time::init_primary(), irq = {}, pie = {}",
            loongarch64::register::crmd::Crmd::read().get_ie(),
            loongarch64::register::prmd::Prmd::read().get_pie()
        );
    }
}

/// Initializes the platform devices for secondary CPUs.
#[cfg(feature = "smp")]
pub fn platform_init_secondary() {}
