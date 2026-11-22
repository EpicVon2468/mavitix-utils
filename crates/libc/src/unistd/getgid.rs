#![no_std]
#![crate_name = "getgid"]
#![crate_type = "cdylib"]

include!("../rsinclude/syscall.rs");

#[unsafe(no_mangle)]
pub extern "C" fn getgid() -> u32 {
	syscall_nofail!(cfg_select! {
		// https://syscall.sh/x64/getgid/
		target_arch = "x86_64" => 0x68,
		// https://syscall.sh/arm64/getgid/
		target_arch = "aarch64" => 0xB0,
		// https://syscall.sh/riscv64/getgid/
		target_arch = "riscv64" => 0xB0,
		_ => compile_error!(),
	}) as u32
}

#[unsafe(no_mangle)]
pub extern "C" fn getegid() -> u32 {
	syscall_nofail!(cfg_select! {
		// https://syscall.sh/x64/getegid/
		target_arch = "x86_64" => 0x6C,
		// https://syscall.sh/arm64/getegid/
		target_arch = "aarch64" => 0xB1,
		// https://syscall.sh/riscv64/getegid/
		target_arch = "riscv64" => 0xB1,
		_ => compile_error!(),
	}) as u32
}
