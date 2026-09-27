#![no_std]
#![crate_name = "environ"]
#![crate_type = "cdylib"]

use core::ffi::c_char;

#[unsafe(no_mangle)]
pub static mut enivron: *mut *mut c_char = core::ptr::null_mut();

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn getenv(name: *const c_char) -> *mut c_char {}

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn setenv(name: *const c_char, value: *const c_char, overwrite: i32) -> i32 {}

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn unsetenv(name: *const c_char) -> i32 {}
