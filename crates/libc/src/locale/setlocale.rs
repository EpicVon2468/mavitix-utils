#![no_std]
#![crate_name = "setlocale"]
#![crate_type = "cdylib"]
#![feature(thread_local, ffi_const)]

use core::{
	ffi::c_char,
	hint::{assert_unchecked, cold_path},
};

#[path = "../rsinclude/errno.rs"]
pub mod errno;
#[path = "../rsinclude/locale.rs"]
pub mod locale;

use locale::{Language, Locale, Territory};

unsafe extern "C" {

	pub fn strlen(r#str: *const c_char) -> usize;

	pub safe fn __mavitix_immediate_abort(code: i32) -> !;
}

macro_rules! einval {
	($(,)?) => {{
		unsafe {
			use errno::__errno_location;

			*__errno_location() = errno::EINVAL;
		};
		return core::ptr::null();
	}};
}

// TODO: Fix this to properly support `LC_ALL`.
#[thread_local]
static mut MAVITIX_LOCALE: [Locale; 12] = [Locale {
	language: Language::English,
	territory: Territory::None,
}; 12];

#[unsafe(no_mangle)]
pub extern "C" fn __mavitix_getlocale() -> *mut [Locale; 12] {
	&raw mut MAVITIX_LOCALE
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn setlocale(category: i32, locale: *const c_char) -> *const c_char {
	let category: usize = category as usize;
	if category > 12 {
		cold_path();
		einval!();
	};
	// SAFETY: Already done bounds checks.
	let locale_info: &mut Locale = unsafe { (*__mavitix_getlocale()).get_unchecked_mut(category) };
	if locale.is_null() {
		return locale_info.to_c_str();
	};
	// SAFETY: Callers ensure `locale` is well-formed.
	let len: usize = unsafe { strlen(locale) };
	if len == 0 {
		// todo!("update current locale based on environment");
		__mavitix_immediate_abort(75);
	};
	// SANITY(unusual):
	// Using `cast_slice` here only works because `locale` is an array of bytes.
	// If it was any other type, the length would be incorrect.
	let bytes: &[u8] = unsafe { (locale as *const u8).cast_slice(len).as_ref_unchecked() };
	// TODO:
	// It may actually generate more efficient assembly to replace this with verbosely
	// matching each possibility instead of slicing.
	match bytes {
		&[b'C', ref codeset @ ..] | &[b'P', b'O', b'S', b'I', b'X', ref codeset @ ..]
			if let b"" | b".utf8" | b".UTF-8" = codeset =>
		{
			locale_info.language = Language::English;
			locale_info.territory = Territory::None;
		},
		&[b'e', b'n', ref rest @ ..] => match rest {
			b"" | b".utf8" | b".UTF-8" => {
				locale_info.language = Language::English;
				locale_info.territory = Territory::None;
			},
			&[b'_', first, second, ref codeset @ ..]
				if let b"" | b".utf8" | b".UTF-8" = codeset =>
			{
				// SAFETY: The constructed territory passed has a length of 2.
				if !unsafe { parse_territory(&mut locale_info.territory, &[first, second]) } {
					einval!();
				};
				locale_info.language = Language::English;
			},
			_ => einval!(),
		},
		&[b'j', b'p', ref rest @ ..] => match rest {
			b"" | b".utf8" | b".UTF-8" => {
				locale_info.language = Language::Japanese;
				locale_info.territory = Territory::None;
			},
			&[b'_', first, second, ref codeset @ ..]
				if let b"" | b".utf8" | b".UTF-8" = codeset =>
			{
				// SAFETY: The constructed territory passed has a length of 2.
				if !unsafe { parse_territory(&mut locale_info.territory, &[first, second]) } {
					einval!();
				};
				locale_info.language = Language::Japanese;
			},
			_ => einval!(),
		},
		_ => einval!(),
	};
	return locale_info.to_c_str();
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
			cold_path();
			// EX_CONFIG
			__mavitix_immediate_abort(78);
		},
		_ => return false,
	};
	return true;
}
