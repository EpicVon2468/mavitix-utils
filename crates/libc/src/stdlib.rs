#![no_std]
#![crate_name = "stdlib"]
#![crate_type = "cdylib"]

use core::arch::naked_asm;

#[unsafe(naked)]
#[unsafe(no_mangle)]
pub extern "C" fn _Exit() -> ! {
	#[rustfmt::skip]
	cfg_select! {
		target_arch = "x86_64" => naked_asm!("
			endbr64
			mov rax, 0x3C
			syscall
		"),
		target_arch = "aarch64" => naked_asm!("
			mov w8, 0x3C
			svc #0
		"),
	};
}
