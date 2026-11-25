#![no_std]
#![crate_name = "unlink"]
#![crate_type = "cdylib"]
#![feature(likely_unlikely, ffi_const)]

use core::ffi::c_char;

include!("../rsinclude/syscall.rs");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn unlink(path: *const c_char) -> i32 {
	cfg_select! {
		// https://syscall.sh/x64/unlink/
		target_arch = "x86_64" => syscall!(0x57, path) as i32,
		_ => unsafe {
			const AT_FDCWD: i32 = -100;

			unlinkat(AT_FDCWD, path, 0)
		},
	}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn unlinkat(fd: i32, path: *const c_char, flags: i32) -> i32 {
	syscall!(
		cfg_select! {
			// https://syscall.sh/x64/unlinkat/
			target_arch = "x86_64" => 0x107,
			// https://syscall.sh/arm64/unlinkat/
			target_arch = "aarch64" => 0x23,
			// https://syscall.sh/riscv64/unlinkat/
			target_arch = "riscv64" => 0x23,
			_ => compile_error!(),
		},
		fd,
		path,
		flags,
	) as i32
}
