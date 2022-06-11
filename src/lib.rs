#![deny(warnings)]

#![no_std]

#[cfg(windows)]
pub fn exit(code: u8) -> ! {
    unsafe { winapi::um::processthreadsapi::ExitProcess(
        code as winapi::shared::minwindef::UINT
    ); }
    #[allow(clippy::empty_loop)]
    loop { }
}

#[cfg(not(windows))]
pub fn exit(code: u8) -> ! {
    unsafe { libc::exit(code as u16 as i16 as libc::c_int) }
}
