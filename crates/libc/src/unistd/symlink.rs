#![no_std]
#![crate_name = "symlink"]
#![crate_type = "cdylib"]
#![feature(likely_unlikely, ffi_const)]

use core::ffi::c_char;

include!("../rsinclude/syscall.rs");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symlink(target: *const c_char, linkpath: *const c_char) -> i32 {
	cfg_select! {
		// https://syscall.sh/x64/symlink/
		target_arch = "x86_64" => syscall!(0x58, target, linkpath) as i32,
		_ => unsafe {
			const AT_FDCWD: i32 = -100;

			symlinkat(target, AT_FDCWD, linkpath)
		},
	}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symlinkat(
	target: *const c_char,
	newdirfd: i32,
	linkpath: *const c_char,
) -> i32 {
	syscall!(
		cfg_select! {
			// https://syscall.sh/x64/symlinkat/
			target_arch = "x86_64" => 0x10A,
			// https://syscall.sh/arm64/symlinkat/
			target_arch = "aarch64" => 0x24,
			// https://syscall.sh/riscv64/symlinkat/
			target_arch = "riscv64" => 0x24,
			_ => compile_error!(),
		},
		target,
		newdirfd,
		linkpath,
	) as i32
}
