#![no_std]
#![crate_name = "setlocale"]
#![crate_type = "cdylib"]

use core::{
	ffi::c_char,
	hint::{assert_unchecked, cold_path},
};

#[path = "../rsinclude/locale.rs"]
pub mod locale;

use locale::{Language, Locale, Territory};

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
	let mut locale_info: Locale = Locale {
		language: Language::English,
		territory: Territory::None,
	};
	// SAFETY: Callers ensure `locale` is well-formed.
	let len: usize = unsafe { strlen(locale) };
	if len == 0 {
		// Read environment.
	};
	// SANITY(unusual):
	// Using `cast_slice` here only works because `locale` is an array of bytes.
	// If it was any other type, the length would be incorrect.
	let bytes: &[u8] = unsafe { (locale as *const u8).cast_slice(len).as_ref_unchecked() };
	match bytes {
		&[b'C', ref codeset @ ..] | &[b'P', b'O', b'S', b'I', b'X', ref codeset @ ..]
			if let b"" | b".utf8" | b".UTF-8" = codeset =>
		{
			// SANITY(unusual):
			// This branch does absolutely nothing, as `locale_info` is
			// already initialised to the correct values.
		},
		&[b'e', b'n', ref rest @ ..] => match rest {
			b"" | b".utf8" | b".UTF-8" => {
				// SANITY(unusual):
				// This branch does absolutely nothing, as `locale_info` is
				// already initialised to the correct values.
			},
			&[b'_', first, second, ref codeset @ ..] => {
				// SAFETY: The constructed territory passed has a length of 2.
				if !unsafe { parse_territory(&mut locale_info.territory, &[first, second]) } {
					// TODO: EINVAL
					return core::ptr::null();
				};
				let (b"" | b".utf8" | b".UTF-8") = codeset else {
					// TODO: EINVAL
					return core::ptr::null();
				};
				// SANITY(unusual):
				// The language isn't explicitly changed here,
				// as it is already set to English.
			},
			_ => {
				// TODO: EINVAL
				return core::ptr::null();
			},
		},
		&[b'j', b'p', ref rest @ ..] => match rest {
			b"" | b".utf8" | b".UTF-8" => locale_info.language = Language::Japanese,
			&[b'_', first, second, ref codeset @ ..] => {
				// SAFETY: The constructed territory passed has a length of 2.
				if !unsafe { parse_territory(&mut locale_info.territory, &[first, second]) } {
					// TODO: EINVAL
					return core::ptr::null();
				};
				let (b"" | b".utf8" | b".UTF-8") = codeset else {
					// TODO: EINVAL
					return core::ptr::null();
				};
				locale_info.language = Language::Japanese;
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

unsafe fn parse_territory(territory: &mut Territory, territory_bytes: &[u8]) -> bool {
	// SAFETY: Callers must ensure this condition.
	unsafe {
		assert_unchecked(territory_bytes.len() == 2);
	};
	match territory_bytes {
		b"AU" => *territory = Territory::AU,
		b"GB" => *territory = Territory::GB,
		b"JP" => *territory = Territory::JP,
		b"NZ" => *territory = Territory::NZ,
		b"US" | b"IL" => {
			unsafe extern "C" {

				pub safe fn __mavitix_immediate_abort(code: i32) -> !;
			}

			cold_path();
			// EX_CONFIG
			__mavitix_immediate_abort(78);
		},
		_ => return false,
	};
	return true;
}
