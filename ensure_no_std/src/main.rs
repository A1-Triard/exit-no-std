#![feature(start)]

#![deny(warnings)]

#![no_std]

use core::panic::PanicInfo;
use exit_no_std::exit;

#[cfg(windows)]
#[link(name="msvcrt")]
extern { }

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    exit(99)
}

#[start]
pub fn main(_argc: isize, _argv: *const *const u8) -> isize {
    exit(0);
}
