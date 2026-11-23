#![no_std]
#![crate_name = "fsync"]
#![crate_type = "cdylib"]
#![feature(likely_unlikely, ffi_const)]

include!("../rsinclude/syscall.rs");

#[unsafe(no_mangle)]
pub extern "C" fn fsync(fd: i32) -> i32 {
	syscall!(
		cfg_select! {
			// https://syscall.sh/x64/fsync/
			target_arch = "x86_64" => 0x4A,
			// https://syscall.sh/arm64/fsync/
			target_arch = "aarch64" => 0x52,
			// https://syscall.sh/riscv64/fsync/
			target_arch = "riscv64" => 0x52,
			_ => compile_error!(),
		},
		fd,
	) as i32
}

#[unsafe(no_mangle)]
pub extern "C" fn fdatasync(fd: i32) -> i32 {
	syscall!(
		cfg_select! {
			// https://syscall.sh/x64/fdatasync/
			target_arch = "x86_64" => 0x4B,
			// https://syscall.sh/arm64/fdatasync/
			target_arch = "aarch64" => 0x53,
			// https://syscall.sh/riscv64/fdatasync/
			target_arch = "riscv64" => 0x53,
			_ => compile_error!(),
		},
		fd,
	) as i32
}
