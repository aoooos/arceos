use super::context::TrapFrame;
use loongarch64::register::csr::Register;
use loongarch64::register::eentry::Eentry;
use loongarch64::register::estat::{self, Estat, Exception, Trap};

core::arch::global_asm!(
    include_str!("trap.S"),
    trapframe_size = const core::mem::size_of::<TrapFrame>(),
);

fn handle_breakpoint(era: &mut usize) {
    debug!("Exception(Breakpoint) @ {:#x} ", era);
    *era += 4;
}

#[no_mangle]
fn loongarch64_trap_handler(tf: &mut TrapFrame) {
    debug!("loongarch64_trap_handler()");
    let estat = Estat::read();
    debug!("estat:{:#x?}", estat.get_val());
    debug!("estat.cause():{:#x?})", estat.cause());
    let eentry = Eentry::read();
    debug!("eentry:{:#x}", eentry.get_eentry());
    match estat.cause() {
        Trap::Exception(Exception::Breakpoint) => handle_breakpoint(&mut tf.era),
        Trap::Interrupt(_) => crate::trap::handle_irq_extern(estat.bits),
        _ => {
            panic!(
                "Unhandled trap {:?} @ {:#x}:\n{:#x?}",
                estat.cause(),
                tf.era,
                tf
            );
        }
    }
    debug!("end of loongarch64_trap_handler");
}
