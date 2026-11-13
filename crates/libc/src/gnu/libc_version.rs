#![no_std]
#![crate_name = "libc_version"]
#![crate_type = "cdylib"]

use core::ffi::c_char;

#[inline(never)]
#[unsafe(no_mangle)]
pub const extern "C" fn gnu_get_libc_release() -> *const c_char {
	return const { c"stable".as_ptr() };
}

#[inline(never)]
#[unsafe(no_mangle)]
pub const extern "C" fn gnu_get_libc_version() -> *const c_char {
	return const { c"2.44.0".as_ptr() };
}
