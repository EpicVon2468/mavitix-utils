#![no_std]

use core::arch::naked_asm;

#[unsafe(naked)]
#[unsafe(no_mangle)]
pub extern "C" fn _Exit() -> ! {
	naked_asm!(
		"
			endbr64
			mov rax, 0x3C
			syscall
		",
	);
}
