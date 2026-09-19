#![no_std]
#![crate_name = "utsname"]
#![crate_type = "cdylib"]
#![feature(extern_types, ffi_const)]

use core::arch::asm;

// SAFETY: The function declarations given below are in line with the header files of `libc`.
#[link(name = "c")]
unsafe extern "C" {

	pub type utsname;

	#[unsafe(ffi_const)]
	pub safe fn __errno_location() -> *mut i32;
}

// #[unsafe(no_mangle)]
// pub extern "C" fn uname(name: *mut utsname) -> i32 {
// asm!("mov rax, 0x3F", "syscall");
// todo!();
// }
