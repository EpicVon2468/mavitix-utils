#![no_std]
#![crate_name = "syscall"]
#![crate_type = "cdylib"]
#![feature(likely_unlikely, ffi_const)]

use core::ffi::VaList;

include!("../rsinclude/global_asm.rs");
include!("../rsinclude/syscall.rs");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall(number: i64, args: ...) -> i64 {
	__mavitix_syscall(number, args)
}

symbol_alias!(syscall, "__syscall");

#[inline(always)]
fn __mavitix_syscall(number: i64, mut args: VaList<'_>) -> i64 {
	// SAFETY: This can invoke undefined behaviour; no mitigations are present nor possible.
	let arg0: i64 = unsafe { args.next_arg() };
	// SAFETY: This can invoke undefined behaviour; no mitigations are present nor possible.
	let arg1: i64 = unsafe { args.next_arg() };
	// SAFETY: This can invoke undefined behaviour; no mitigations are present nor possible.
	let arg2: i64 = unsafe { args.next_arg() };
	// SAFETY: This can invoke undefined behaviour; no mitigations are present nor possible.
	let arg3: i64 = unsafe { args.next_arg() };
	// SAFETY: This can invoke undefined behaviour; no mitigations are present nor possible.
	let arg4: i64 = unsafe { args.next_arg() };
	// SAFETY: This can invoke undefined behaviour; no mitigations are present nor possible.
	let arg5: i64 = unsafe { args.next_arg() };
	drop(args);
	return syscall!(number, arg0, arg1, arg2, arg3, arg4, arg5);
}
