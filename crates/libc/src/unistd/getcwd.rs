#![no_std]
#![crate_name = "getcwd"]
#![crate_type = "cdylib"]
#![feature(likely_unlikely, ffi_const)]

use core::ffi::c_char;

#[path = "../rsinclude/errno.rs"]
pub mod errno;

include!("../rsinclude/syscall.rs");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn getcwd(buf: *mut c_char, size: usize) -> *mut c_char {
	if size == 0 {
		unsafe {
			use errno::__errno_location;

			*__errno_location() = errno::EINVAL;
		};
		return core::ptr::null_mut();
	};
	if buf.is_null() {
		unsafe {
			use errno::__errno_location;

			*__errno_location() = errno::ENOMEM;
		};
		return core::ptr::null_mut();
	};
	let result: i64 = syscall!(
		cfg_select! {
			// https://syscall.sh/x64/getcwd/
			target_arch = "x86_64" => 0x4F,
			// https://syscall.sh/arm64/getcwd/
			target_arch = "aarch64" => 0x11,
			// https://syscall.sh/riscv64/getcwd/
			target_arch = "riscv64" => 0x11,
			_ => compile_error!(),
		},
		buf,
		size,
	);
	if result == -1 {
		return core::ptr::null_mut();
	};
	// SANITY(unusual):
	// The `getcwd(3)` man page mentions the syscall returns "(unreachable)" under certain circumstances, recommending to check for '/' as a countermeasure.
	if unsafe { *buf } as u8 != b'/' {
		unsafe {
			use errno::__errno_location;

			*__errno_location() = errno::ENOENT;
		};
		return core::ptr::null_mut();
	};
	return buf;
}
