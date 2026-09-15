#![no_std]
#![crate_name = "utsname"]
#![crate_type = "cdylib"]
#![feature(extern_types)]

use core::arch::asm;

// SAFETY: The function declarations given below are in line with the header files of `libc`.
#[link(name = "c")]
unsafe extern "C" {
	type utsname;
}

// #[unsafe(no_mangle)]
// pub extern "C" fn uname(name: *mut utsname) -> i32 {
// 	#[rustfmt::skip]
// 	asm!("
// 		mov rax, 0x3F
// 		syscall
// 	");
// 	todo!();
// }
