#![no_std]
#![crate_name = "write"]
#![crate_type = "cdylib"]
#![feature(likely_unlikely, ffi_const)]

use core::ffi::c_void;

include!("../rsinclude/syscall.rs");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn write(fd: i32, buf: *const c_void, len: usize) -> isize {
	syscall!(
		cfg_select! {
			// https://syscall.sh/x64/write/
			target_arch = "x86_64" => 0x1,
			// https://syscall.sh/arm64/write/
			target_arch = "aarch64" => 0x40,
			// https://syscall.sh/riscv64/write/
			target_arch = "riscv64" => 0x40,
			_ => compile_error!(),
		},
		fd,
		buf,
		len,
	) as isize
}
