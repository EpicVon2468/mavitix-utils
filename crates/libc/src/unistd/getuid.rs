#![no_std]
#![crate_name = "getuid"]
#![crate_type = "cdylib"]

include!("../rsinclude/syscall.rs");

#[unsafe(no_mangle)]
pub extern "C" fn getuid() -> u32 {
	syscall_nofail!(cfg_select! {
		// https://syscall.sh/x64/getuid/
		target_arch = "x86_64" => 0x66,
		// https://syscall.sh/arm64/getuid/
		target_arch = "aarch64" => 0xAE,
		// https://syscall.sh/riscv64/getuid/
		target_arch = "riscv64" => 0xAE,
		_ => compile_error!(),
	}) as u32
}

#[unsafe(no_mangle)]
pub extern "C" fn geteuid() -> u32 {
	syscall_nofail!(cfg_select! {
		// https://syscall.sh/x64/geteuid/
		target_arch = "x86_64" => 0x6B,
		// https://syscall.sh/arm64/geteuid/
		target_arch = "aarch64" => 0xAF,
		// https://syscall.sh/riscv64/geteuid/
		target_arch = "riscv64" => 0xAF,
		_ => compile_error!(),
	}) as u32
}
