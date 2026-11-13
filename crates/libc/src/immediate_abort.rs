#![no_std]
#![crate_name = "mavitix_immediate_abort"]
#![crate_type = "cdylib"]

use core::arch::asm;

#[unsafe(no_mangle)]
pub extern "C" fn __mavitix_immediate_abort(code: i32) -> ! {
	// SAFETY: UB-free.
	unsafe {
		cfg_select! {
			target_arch = "x86_64" => asm!(
				// https://syscall.sh/x64/exit/
				"mov rax, 0x3C",
				"syscall",
				in("rdi") code,
				options(noreturn, raw),
			),
			target_arch = "aarch64" => asm!(
				// https://syscall.sh/arm64/exit/
				"mov x8, #0x5D",
				"svc #0",
				in("x0") code,
				options(noreturn, raw),
			),
			target_arch = "riscv64" => asm!(
				// https://syscall.sh/riscv64/exit/
				"mov a7, 0x5D",
				"ecall",
				in("a0") code,
				options(noreturn, raw),
			),
			_ => compile_error!(),
		};
	};
}
