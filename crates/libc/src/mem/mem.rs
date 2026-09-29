#![no_std]
#![crate_name = "mem"]
#![crate_type = "cdylib"]
#![allow(nonstandard_style)]

use core::ffi::c_void;

include!("../rsinclude/rpmalloc.rs");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn malloc(size: usize) -> *mut c_void {
	// SAFETY: Passthrough; Callers ensure no UB.
	return unsafe { rpmalloc::malloc(size) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn calloc(num: usize, size: usize) -> *mut c_void {
	// SAFETY: Passthrough; Callers ensure no UB.
	return unsafe { rpmalloc::calloc(num, size) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void {
	// SAFETY: Passthrough; Callers ensure no UB.
	return unsafe { rpmalloc::realloc(ptr, size) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn aligned_alloc(align: usize, size: usize) -> *mut c_void {
	// SAFETY: Passthrough; Callers ensure no UB.
	return unsafe { rpmalloc::aligned_alloc(align, size) };
}

#[unsafe(no_mangle)]
#[deprecated(
	since = "0.0.0",
	note = "Obsolete function from glibc provided just-in-case; Use `aligned_alloc` or `posix_memalign` instead."
)]
pub unsafe extern "C" fn memalign(align: usize, size: usize) -> *mut c_void {
	// SAFETY: Passthrough; Callers ensure no UB.
	return unsafe { rpmalloc::memalign(align, size) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn posix_memalign(
	memptr: *mut *mut c_void,
	align: usize,
	size: usize,
) -> i32 {
	// SAFETY: Passthrough; Callers ensure no UB.
	return unsafe { rpmalloc::posix_memalign(memptr, align, size) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free(ptr: *mut c_void) {
	// SAFETY: Passthrough; Callers ensure no UB.
	unsafe { rpmalloc::free(ptr) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_sized(ptr: *mut c_void, _size: usize) {
	// SANITY(unusual): "A conforming implementation may ignore size and call free."
	// SAFETY: Passthrough; Callers ensure no UB.
	unsafe { rpmalloc::free(ptr) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_aligned_size(ptr: *mut c_void, _align: usize, _size: usize) {
	// SANITY(unusual): "A conforming implementation may ignore alignment and size and call free."
	// SAFETY: Passthrough; Callers ensure no UB.
	unsafe { rpmalloc::free(ptr) };
}

// Stub (glibc), see also: https://github.com/mjansson/rpmalloc/issues/116
#[unsafe(no_mangle)]
pub unsafe extern "C" fn malloc_trim(_: usize) -> i32 {
	return 0;
}
