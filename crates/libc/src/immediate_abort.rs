#![no_std]
#![crate_name = "mavitix_immediate_abort"]
#![crate_type = "cdylib"]
#![feature(likely_unlikely, ffi_const)]

use core::hint::unreachable_unchecked;

include!("rsinclude/syscall.rs");

#[unsafe(no_mangle)]
pub extern "C" fn __mavitix_immediate_abort(code: i32) -> ! {
	syscall!(
		cfg_select! {
			// https://syscall.sh/x64/exit/
			target_arch = "x86_64" => 0x3C,
			// https://syscall.sh/arm64/exit/
			target_arch = "aarch64" => 0x5D,
			// https://syscall.sh/riscv64/exit/
			target_arch = "riscv64" => 0x5D,
		},
		code,
	);
	// SAFETY: The above syscall does not return.
	unsafe {
		unreachable_unchecked();
	};
}
