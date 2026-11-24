#![no_std]
#![crate_name = "read"]
#![crate_type = "cdylib"]
#![feature(likely_unlikely, ffi_const)]

use core::ffi::c_void;

include!("../rsinclude/syscall.rs");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn read(fd: i32, buf: *const c_void, len: usize) -> isize {
	syscall!(
		cfg_select! {
			// https://syscall.sh/x64/read/
			target_arch = "x86_64" => 0x0,
			// https://syscall.sh/arm64/read/
			target_arch = "aarch64" => 0x3F,
			// https://syscall.sh/riscv64/read/
			target_arch = "riscv64" => 0x3F,
			_ => compile_error!(),
		},
		fd,
		buf,
		len,
	) as isize
}
