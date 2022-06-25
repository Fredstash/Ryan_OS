#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use ryan_os::{add, println, sub};


#[no_mangle]
pub extern "C" fn _start() -> ! {
    // testing the println which uses print as part of the macro
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    // trying my macros
    println!("{}", add!(4,5));
    println!("{}", sub!(10,15));
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ryan_os::test_panic_handler(info)
}