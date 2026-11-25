#![no_std]
#![crate_name = "link"]
#![crate_type = "cdylib"]
#![feature(likely_unlikely, ffi_const)]

use core::ffi::c_char;

include!("../rsinclude/syscall.rs");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn link(oldpath: *const c_char, newpath: *const c_char) -> i32 {
	cfg_select! {
		// https://syscall.sh/x64/link/
		target_arch = "x86_64" => syscall!(0x56, oldpath, newpath) as i32,
		_ => unsafe {
			const AT_FDCWD: i32 = -100;

			linkat(AT_FDCWD, oldpath, AT_FDCWD, newpath, 0)
		},
	}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn linkat(
	oldfd: i32,
	oldpath: *const c_char,
	newfd: i32,
	newpath: *const c_char,
	flags: i32,
) -> i32 {
	syscall!(
		cfg_select! {
			// https://syscall.sh/x64/linkat/
			target_arch = "x86_64" => 0x109,
			// https://syscall.sh/arm64/linkat/
			target_arch = "aarch64" => 0x25,
			// https://syscall.sh/riscv64/linkat/
			target_arch = "riscv64" => 0x25,
			_ => compile_error!(),
		},
		oldfd,
		oldpath,
		newfd,
		newpath,
		flags
	) as i32
}
