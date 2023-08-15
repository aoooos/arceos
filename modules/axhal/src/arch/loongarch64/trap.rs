use super::context::TrapFrame;
use core::arch::asm;
//use libax::println;
use log::info;
use loongarch64::register::csr::Register;
use loongarch64::register::eentry::Eentry;
use loongarch64::register::estat::{self, Estat, Exception, Interrupt, Trap};

core::arch::global_asm!(
    include_str!("trap.S"),
    trapframe_size = const core::mem::size_of::<TrapFrame>(),
);

fn handle_breakpoint(era: &mut usize) {
    debug!("Exception(Breakpoint) @ {:#x} ", era);
}

#[no_mangle]
fn loongarch64_trap_handler(tf: &mut TrapFrame) {
    let estat = Estat::read();
    match estat.cause() {
        Trap::Exception(Exception::Breakpoint) => handle_breakpoint(&mut tf.era),
        // Trap::Interrupt(Interrupt::Timer) => crate::trap::handle_irq_extern(11),
        Trap::Interrupt(_) => {
            let irq_num: usize = tf.estat.trailing_zeros() as usize;
            crate::trap::handle_irq_extern(irq_num)
        }
        _ => {
            panic!(
                "Unhandled trap {:?} @ {:#x}:\n{:#x?}",
                estat.cause(),
                tf.era,
                tf
            );
        }
    }
}
