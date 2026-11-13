#![no_std]
#![crate_name = "setlocale"]
#![crate_type = "cdylib"]

use core::{ffi::c_char, hint::cold_path};

unsafe extern "C" {
	pub fn strlen(r#str: *const c_char) -> usize;
}

// TODO: Create shared rust module to house repr(C) enum to represent locale data internally.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn setlocale(category: i32, locale: *const c_char) -> *const c_char {
	if locale.is_null() {
		// TODO: EINVAL
		return core::ptr::null();
	};
	// SAFETY: Callers ensure `locale` is well-formed.
	let len: usize = unsafe { strlen(locale) };
	// SANITY(unusual):
	// Using `cast_slice` here only works because `locale` is an array of bytes.
	// If it was any other type, the length would be incorrect.
	let bytes: &[u8] = unsafe { (locale as *const u8).cast_slice(len).as_ref_unchecked() };
	match bytes {
		&[b'C', ref codeset @ ..] | &[b'P', b'O', b'S', b'I', b'X', ref codeset @ ..]
			if let b"" | b".utf8" | b".UTF-8" = codeset => {},
		&[b'e', b'n', ref rest @ ..] => match rest {
			b"" | b".utf8" | b".UTF-8" => {},
			&[b'_', first, second, ref codeset @ ..] => {
				let territory: &[u8] = &[first, second];
				match territory {
					b"GB" => {},
					b"AU" => {},
					b"US" => {
						unsafe extern "C" {
							pub safe fn __mavitix_immediate_abort(code: i32) -> !;
						}

						cold_path();
						// EX_CONFIG
						__mavitix_immediate_abort(78);
					},
					_ => {
						// TODO: EINVAL
						return core::ptr::null();
					},
				};
			},
			_ => {
				// TODO: EINVAL
				return core::ptr::null();
			},
		},
		_ => {
			// TODO: EINVAL
			return core::ptr::null();
		},
	};
	c"C.UTF-8".as_ptr()
}
