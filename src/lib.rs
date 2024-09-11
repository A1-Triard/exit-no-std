#![deny(warnings)]

#![no_std]

/// Terminates the current process with the specified exit code.
#[cfg(target_os="dos")]
pub fn exit(code: u8) -> ! {
    pc_ints::int_21h_ah_4Ch_exit(code);
    #[allow(clippy::empty_loop)]
    loop { }
}

/// Terminates the current process with the specified exit code.
#[cfg(all(not(target_os="dos"), windows))]
pub fn exit(code: u8) -> ! {
    unsafe { winapi::um::processthreadsapi::ExitProcess(
        code as winapi::shared::minwindef::UINT
    ); }
    #[allow(clippy::empty_loop)]
    loop { }
}

/// Terminates the current process with the specified exit code.
#[cfg(all(not(target_os="dos"), not(windows)))]
pub fn exit(code: u8) -> ! {
    unsafe { libc::exit(code as u16 as i16 as libc::c_int) }
}
