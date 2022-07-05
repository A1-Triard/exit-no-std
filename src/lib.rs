#![deny(warnings)]

#![no_std]

#[cfg(dos)]
pub fn exit(code: u8) -> ! {
    pc_ints::int_21h_ah_4Ch_exit(code);
    loop { }
}

#[cfg(all(not(dos), windows))]
pub fn exit(code: u8) -> ! {
    unsafe { winapi::um::processthreadsapi::ExitProcess(
        code as winapi::shared::minwindef::UINT
    ); }
    #[allow(clippy::empty_loop)]
    loop { }
}

#[cfg(all(not(dos), not(windows)))]
pub fn exit(code: u8) -> ! {
    unsafe { libc::exit(code as u16 as i16 as libc::c_int) }
}
