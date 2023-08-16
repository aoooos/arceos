use super::context::TrapFrame;
use core::arch::asm;
//use libax::println;
use log::info;
use loongarch64::register::csr::Register;
use loongarch64::register::eentry::Eentry;
use loongarch64::register::estat::{self, Estat, Exception, Interrupt, Trap};
use loongarch64::register::ticlr::Ticlr;

core::arch::global_asm!(
    include_str!("trap.S"),
    trapframe_size = const core::mem::size_of::<TrapFrame>(),
);

fn handle_breakpoint(era: &mut usize) {
    debug!("Exception(Breakpoint) @ {:#x} ", era);
}

#[no_mangle]
fn loongarch64_trap_handler(tf: &mut TrapFrame) {
    let cause = Estat::read().cause();
    let mut estat: usize;
    unsafe {
        asm!("csrrd {}, 0x5", out(reg) estat);
    }
    debug!(
        "loongarch64_trap_handler, estat = {:#x?}, irq = {}, pie = {}",
        estat,
        loongarch64::register::crmd::Crmd::read().get_ie(),
        loongarch64::register::prmd::Prmd::read().get_pie(),
    );
    match cause {
        Trap::Exception(Exception::Breakpoint) => handle_breakpoint(&mut tf.era),
        Trap::Interrupt(Interrupt::Timer) => {
            Ticlr::read().clear_timer().write();
            debug!(
                "Trap::Interrupt(Interrupt::Timer), irq = {}, pie = {}",
                loongarch64::register::crmd::Crmd::read().get_ie(),
                loongarch64::register::prmd::Prmd::read().get_pie()
            );
            let irq_num: usize = tf.estat.trailing_zeros() as usize;
            crate::trap::handle_irq_extern(irq_num)
        }
        Trap::Interrupt(_) => {
            let irq_num: usize = tf.estat.trailing_zeros() as usize;
            crate::trap::handle_irq_extern(irq_num)
        }
        _ => {
            panic!("Unhandled trap {:?} @ {:#x}:\n{:#x?}", cause, tf.era, tf);
        }
    }
}
