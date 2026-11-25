#![no_std]
#![crate_name = "rmdir"]
#![crate_type = "cdylib"]
#![feature(likely_unlikely, ffi_const)]

use core::ffi::c_char;

include!("../rsinclude/syscall.rs");

unsafe extern "C" {

	pub fn unlinkat(fd: i32, path: *const c_char, flag: i32) -> i32;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rmdir(path: *const c_char) -> i32 {
	cfg_select! {
		// https://syscall.sh/x64/rmdir/
		target_arch = "x86_64" => syscall!(0x54, path) as i32,
		_ => unsafe {
			const AT_FDCWD: i32 = -100;
			const AT_REMOVEDIR: i32 = 0x200;

			unlinkat(AT_FDCWD, path, AT_REMOVEDIR)
		},
	}
}
