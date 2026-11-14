#![no_std]
#![crate_name = "errno"]
#![crate_type = "cdylib"]
#![feature(thread_local)]

#[thread_local]
#[unsafe(no_mangle)]
static mut __mavitix_errno: i32 = 0;

#[unsafe(no_mangle)]
pub extern "C" fn __errno_location() -> *mut i32 {
	return &raw mut __mavitix_errno;
}
