#![no_std]
#![crate_name = "sync"]
#![crate_type = "cdylib"]
#![feature(likely_unlikely, ffi_const)]

include!("../rsinclude/syscall.rs");

#[unsafe(no_mangle)]
pub extern "C" fn sync() {
	syscall_nofail!(cfg_select! {
		// https://syscall.sh/x64/sync/
		target_arch = "x86_64" => 0xA2,
		// https://syscall.sh/arm64/sync/
		target_arch = "aarch64" => 0x51,
		// https://syscall.sh/riscv64/sync/
		target_arch = "riscv64" => 0x51,
		_ => compile_error!(),
	});
}

#[unsafe(no_mangle)]
pub extern "C" fn syncfs(fd: i32) -> i32 {
	syscall!(
		cfg_select! {
			// https://syscall.sh/x64/syncfs/
			target_arch = "x86_64" => 0x132,
			// https://syscall.sh/arm64/syncfs/
			target_arch = "aarch64" => 0x10B,
			// https://syscall.sh/riscv64/syncfs/
			target_arch = "riscv64" => 0x10B,
			_ => compile_error!(),
		},
		fd,
	) as i32
}
