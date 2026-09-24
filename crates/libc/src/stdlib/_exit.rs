#![no_std]
#![crate_name = "_exit"]
#![crate_type = "cdylib"]

use core::arch::asm;

// Rust doesn't have symbol aliasing for some fucking reason.
// See also:
// - https://internals.rust-lang.org/t/pre-rfc-defining-function-aliases/11424
// - https://internals.rust-lang.org/t/symbol-aliases/13297
// - https://github.com/rust-lang/compiler-team/issues/526
// - https://github.com/rust-lang/compiler-builtins/issues/70
#[unsafe(no_mangle)]
pub extern "C" fn _Exit() -> ! {
	_exit();
}

#[unsafe(no_mangle)]
pub extern "C" fn _exit() -> ! {
	// SAFETY:
	unsafe {
		cfg_select! {
			target_arch = "x86_64" => asm!("mov rax, 0x3C", "syscall", options(noreturn, raw)),
			target_arch = "aarch64" => asm!("mov w8, #0x5D", "svc #0", options(noreturn, raw)),
			_ => compile_error!(),
		};
	};
}
