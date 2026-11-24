#![no_std]
#![crate_name = "strlen"]
#![crate_type = "cdylib"]

use core::ffi::c_char;

#[unsafe(no_mangle)]
pub const unsafe extern "C" fn strlen(mut r#str: *const c_char) -> usize {
	let start: *const c_char = r#str;
	loop {
		// SAFETY: Callers ensure input is well-formed.
		if unsafe { *r#str } == 0 {
			// SAFETY:
			return unsafe { r#str.offset_from_unsigned(start) };
		};
		// SAFETY: Callers ensure input is well-formed.
		r#str = unsafe { r#str.add(1) };
	}
}
