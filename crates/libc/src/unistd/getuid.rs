#![no_std]
#![crate_name = "getuid"]
#![crate_type = "cdylib"]

include!("../rsinclude/global_asm.rs");

asm_prelude_syscall! {
	"getuid"; cfg_select! {
		// https://syscall.sh/x64/getuid/
		target_arch = "x86_64" => "0x66",
		// https://syscall.sh/arm64/getuid/
		target_arch = "aarch64" => "#0xAE",
		// https://syscall.sh/riscv64/getuid/
		target_arch = "riscv64" => "0xAE",
		_ => compile_error!(),
	},
}

asm_prelude_syscall! {
	"geteuid"; cfg_select! {
		// https://syscall.sh/x64/geteuid/
		target_arch = "x86_64" => "0x6B",
		// https://syscall.sh/arm64/geteuid/
		target_arch = "aarch64" => "#0xAF",
		// https://syscall.sh/riscv64/geteuid/
		target_arch = "riscv64" => "0xAF",
		_ => compile_error!(),
	},
}
