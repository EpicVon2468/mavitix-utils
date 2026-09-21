#![no_std]
#![crate_name = "_Exit"]
#![crate_type = "cdylib"]

use core::arch::asm;

#[unsafe(no_mangle)]
pub extern "C" fn _Exit() -> ! {
	// SAFETY:
	unsafe {
		cfg_select! {
			target_arch = "x86_64" => asm!("mov rax, 0x3C", "syscall", options(noreturn, raw)),
			target_arch = "aarch64" => asm!("mov w8, #0x5D", "svc #0", options(noreturn, raw)),
			_ => compile_error!(),
		};
	};
}
