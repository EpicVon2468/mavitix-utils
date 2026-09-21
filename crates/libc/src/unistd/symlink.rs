#![no_std]
#![crate_name = "symlink"]
#![crate_type = "cdylib"]

use core::{arch::asm, ffi::c_char};

#[unsafe(no_mangle)]
pub extern "C" fn symlink(target: *const c_char, linkpath: *const c_char) -> i32 {
	let result: i32;
	// SAFETY:
	unsafe {
		cfg_select! {
			target_arch = "x86_64" => asm!(
				"mov rax, 0x58",
				"syscall",
				in("rdi") target,
				in("rsi") linkpath,
				lateout("rax") result,
				options(raw),
			),
			target_arch = "aarch64" => todo!(),
			_ => compile_error!(),
		};
	};
	return result;
}
