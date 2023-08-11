//! Uart 16550.

use core::fmt::Write;
use spinlock::SpinNoIrq;
pub struct Port {
    port: u16,
}
impl Port {
    /// Creates an I/O port with the given port number.
    #[inline]
    pub const fn new(port: u16) -> Self {
        Port { port }
    }
    pub unsafe fn write(&mut self, value: u8) {
        unsafe { core::arch::asm!("iocsrwr.w {},{}",in(reg)value,in(reg)self.port) }
    }
    pub unsafe fn read(&mut self) -> u8 {
        let mut value;
        unsafe { core::arch::asm!("iocsrrd.w {},{}",out(reg)value,in(reg)self.port) }
        value
    }
}

const UART_CLOCK_FACTOR: usize = 16;
const OSC_FREQ: usize = 1_843_200;

const UART_ADDR:usize = 0x900000001FE001E0;

// static COM1: SpinNoIrq<Uart16550> = SpinNoIrq::new(Uart16550::new(0x0508));
static COM1: SpinNoIrq<Uart> = SpinNoIrq::new(Uart::new(UART_ADDR));


pub struct Uart {
    base_address: usize,
}

impl Uart {
    pub const fn new(base_address: usize) -> Self {
        Uart { base_address }
    }

    pub fn putchar(&mut self, c: u8) {
        let mut ptr = self.base_address as *mut u8;
        loop {
            unsafe {
                let c = ptr.add(5).read_volatile();
                if c & (1<<5)!=0{
                    break;
                }
            }
        }
        ptr = self.base_address as *mut u8;
        unsafe {
            ptr.add(0).write_volatile(c);
        }
    }

    pub fn getchar(&mut self) -> Option<u8> {
        let ptr = self.base_address as *mut u8;
        unsafe {
            if ptr.add(5).read_volatile() & 1 == 0 {
                // The DR bit is 0, meaning no data
                None
            } else {
                // The DR bit is 1, meaning data!
                Some(ptr.add(0).read_volatile())
            }
        }
    }
}
impl Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.bytes() {
            self.putchar(c);
        }
        Ok(())
    }
}


/// Writes a byte to the console.
pub fn putchar(c: u8) {
    let mut uart = COM1.lock();
    match c {
        b'\n' => {
            uart.putchar(b'\r');
            uart.putchar(b'\n');
        }
        c => uart.putchar(c),
    }
}

// pub fn write_fmt(args: core::fmt::Arguments) {
//     use core::fmt::Write;
//     COM1.lock().write_fmt(args).unwrap();
// }


/// Reads a byte from the console, or returns [`None`] if no input is available.
pub fn getchar() -> Option<u8> {
    COM1.lock().getchar()
}

pub(super) fn init() {
    // COM1.lock().init(115200);
}
