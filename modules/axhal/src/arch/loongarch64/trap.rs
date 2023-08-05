use super::context::TrapFrame;
use core::arch::asm;
use loongarch64::register::csr::Register;
use loongarch64::register::eentry::Eentry;
use loongarch64::register::estat::{self, Estat, Exception, Trap};
use loongarch64::register::prcfg3::Prcfg3;
use loongarch64::tlb::TLBELO;

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
    let eentry = Eentry::read();
    match estat.cause() {
        Trap::Exception(Exception::Breakpoint) => handle_breakpoint(&mut tf.era),
        /* Trap::Exception(Exception::StorePageInvalid)
        | Trap::Exception(Exception::LoadPageInvalid)
        | Trap::Exception(Exception::FetchPageInvalid) => {
            unsafe { asm!("tlbsrch", "tlbrd",) }
            let tlbelo0 = TLBELO::read(0);
            let tlbelo1 = TLBELO::read(1);
        }*/
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
}
