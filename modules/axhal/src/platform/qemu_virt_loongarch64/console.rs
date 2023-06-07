//! Uart 16550.

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

const UART_ADDR:usize = 0x1FE001E0;

// static COM1: SpinNoIrq<Uart16550> = SpinNoIrq::new(Uart16550::new(0x0508));
static COM1: SpinNoIrq<Uart> = SpinNoIrq::new(Uart::new(UART_ADDR));

bitflags::bitflags! {
    /// Line status flags
    struct LineStsFlags: u8 {
        const INPUT_FULL = 1;
        // 1 to 4 unknown
        const OUTPUT_EMPTY = 1 << 5;
        // 6 and 7 unknown
    }
}

struct Uart16550 {
    data: Port,
    int_en: Port,
    fifo_ctrl: Port,
    line_ctrl: Port,
    modem_ctrl: Port,
    line_sts: Port,
}

// impl Uart16550 {
//     const fn new(port: u16) -> Self {
//         Self {
//             data: Port::new(port),
//             int_en: Port::new(port + 1),
//             fifo_ctrl: Port::new(port + 2),
//             line_ctrl: Port::new(port + 3),
//             modem_ctrl: Port::new(port + 4),
//             line_sts: Port::new(port + 5),
//         }
//     }
//
//     fn init(&mut self, baud_rate: usize) {
//         unsafe {
//             // Disable interrupts
//             self.int_en.write(0x00);
//
//             // Enable DLAB
//             self.line_ctrl.write(0x80);
//
//             // Set maximum speed according the input baud rate by configuring DLL and DLM
//             let divisor = OSC_FREQ / (baud_rate * UART_CLOCK_FACTOR);
//             self.data.write((divisor & 0xff) as u8);
//             self.int_en.write((divisor >> 8) as u8);
//
//             // Disable DLAB and set data word length to 8 bits
//             self.line_ctrl.write(0x03);
//
//             // Enable FIFO, clear TX/RX queues and
//             // set interrupt watermark at 14 bytes
//             self.fifo_ctrl.write(0xC7);
//
//             // Mark data terminal ready, signal request to send
//             // and enable auxilliary output #2 (used as interrupt line for CPU)
//             self.modem_ctrl.write(0x0B);
//         }
//     }
//
//     fn line_sts(&mut self) -> LineStsFlags {
//         unsafe { LineStsFlags::from_bits_truncate(self.line_sts.read()) }
//     }
//
//     fn putchar(&mut self, c: u8) {
//         while !self.line_sts().contains(LineStsFlags::OUTPUT_EMPTY) {}
//         unsafe { self.data.write(c) };
//     }
//
//     fn getchar(&mut self) -> Option<u8> {
//         if self.line_sts().contains(LineStsFlags::INPUT_FULL) {
//             unsafe { Some(self.data.read()) }
//         } else {
//             None
//         }
//     }
// }
pub struct Uart {
    base_address: usize,
}

//    fn init(&mut self, baud_rate: usize) {
//        unsafe {
            // Disable interrupts
//            self.int_en.write(0x00);

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

/// Reads a byte from the console, or returns [`None`] if no input is available.
pub fn getchar() -> Option<u8> {
    COM1.lock().getchar()
}

pub(super) fn init() {
    // COM1.lock().init(115200);
}
