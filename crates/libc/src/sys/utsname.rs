#![no_std]
#![crate_name = "utsname"]
#![crate_type = "cdylib"]
#![feature(likely_unlikely, extern_types, ffi_const)]

include!("../rsinclude/syscall.rs");

// SAFETY: The function declarations given below are in line with the header files of `libc`.
#[link(name = "c")]
unsafe extern "C" {

	pub type utsname;
}

#[unsafe(no_mangle)]
pub extern "C" fn uname(name: *mut utsname) -> i32 {
	syscall!(
		cfg_select! {
			// https://syscall.sh/x64/uname/
			target_arch = "x86_64" => 0x3F,
			// https://syscall.sh/arm64/uname/
			target_arch = "aarch64" => 0xA0,
			// https://syscall.sh/riscv64/uname/
			target_arch = "riscv64" => 0xA0,
			_ => compile_error!(),
		},
		name,
	) as i32
}
