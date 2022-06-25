use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

// display the printed data to the terminal instead of the virtual machine

// lazy static is initialized at the runtime so that it can be used in the macros
lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

// function is not usable in main directly
#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
}

// basic print and println macros for the terminal
#[macro_export]
macro_rules! serial_print {
    ($($args:tt)*) => {
        $crate::serial::_print(format_args!($($args)*));
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}