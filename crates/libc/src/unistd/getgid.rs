#![no_std]
#![crate_name = "getgid"]
#![crate_type = "cdylib"]

include!("../rsinclude/global_asm.rs");

asm_prelude_syscall! {
	"getgid"; cfg_select! {
		// https://syscall.sh/x64/getgid/
		target_arch = "x86_64" => "0x68",
		// https://syscall.sh/arm64/getgid/
		target_arch = "aarch64" => "#0xB0",
		// https://syscall.sh/riscv64/getgid/
		target_arch = "riscv64" => "0xB0",
		_ => compile_error!(),
	},
}

asm_prelude_syscall! {
	"getegid"; cfg_select! {
		// https://syscall.sh/x64/getegid/
		target_arch = "x86_64" => "0x6C",
		// https://syscall.sh/arm64/getegid/
		target_arch = "aarch64" => "#0xB1",
		// https://syscall.sh/riscv64/getegid/
		target_arch = "riscv64" => "0xB1",
		_ => compile_error!(),
	},
}
