#![deny(warnings)]

#![no_std]
#![no_main]

use core::ffi::{c_char, c_int};
use core::panic::PanicInfo;
use exit_no_std::exit;

#[cfg(windows)]
#[link(name="msvcrt")]
extern "C" { }

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    exit(99)
}

#[unsafe(no_mangle)]
extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    exit(0);
}
