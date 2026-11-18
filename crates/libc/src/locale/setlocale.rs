#![no_std]
#![crate_name = "setlocale"]
#![crate_type = "cdylib"]
#![feature(derive_const, const_default, const_trait_impl, ffi_const)]

use core::{
	ffi::c_char,
	hint::{assert_unchecked, cold_path, unreachable_unchecked},
};

#[path = "../rsinclude/errno.rs"]
pub mod errno;
#[path = "../rsinclude/locale.rs"]
pub mod locale;

use locale::{
	LC_ADDRESS,
	LC_ALL,
	LC_COLLATE,
	LC_CTYPE,
	LC_MESSAGES,
	LC_MONETARY,
	LC_NAME,
	LC_NUMERIC,
	LC_TELEPHONE,
	LC_TIME,
	Language,
	Locale,
	Territory,
};

unsafe extern "C" {

	pub fn strlen(r#str: *const c_char) -> usize;

	pub fn getenv(name: *const c_char) -> *const c_char;

	pub safe fn __mavitix_immediate_abort(code: i32) -> !;
}

macro_rules! einval {
	($(,)?) => {{
		#[allow(unused_unsafe)]
		unsafe {
			use errno::__errno_location;

			*__errno_location() = errno::EINVAL;
		};
		return core::ptr::null();
	}};
}

#[derive_const(Default)]
#[derive(Debug, Clone)]
pub struct GlobalLocale {
	pub all: Locale,
	pub address: Option<Locale>,
	pub collate: Option<Locale>,
	pub ctype: Option<Locale>,
	pub messages: Option<Locale>,
	pub monetary: Option<Locale>,
	pub name: Option<Locale>,
	pub numeric: Option<Locale>,
	pub telephone: Option<Locale>,
	pub time: Option<Locale>,
}

macro_rules! lc_func {
	($func:ident, $immutable:ident, $raw:ident, $field:ident $(,)?) => {
		impl GlobalLocale {
			#[inline]
			pub const fn $func(&mut self) -> &mut Locale {
				if let None = self.$field {
					self.$field = Some(self.all);
				};
				unsafe { self.$field.as_mut().unwrap_unchecked() }
			}

			#[inline]
			pub const fn $immutable(&self) -> Locale {
				match self.$field {
					Some(value) => value,
					None => self.all,
				}
			}
		}

		#[unsafe(no_mangle)]
		pub extern "C" fn $raw() -> Locale {
			unsafe { (*__mavitix_getlocale()).$immutable() }
		}
	};
}

impl GlobalLocale {
	#[inline]
	pub const fn lc_all(&mut self) -> &mut Locale {
		self.address = None;
		self.collate = None;
		self.ctype = None;
		self.messages = None;
		self.monetary = None;
		self.name = None;
		self.numeric = None;
		self.telephone = None;
		self.time = None;
		return &mut self.all;
	}

	#[inline]
	pub const fn lc_all_nomut(&self) -> Locale {
		return self.all;
	}
}

lc_func!(
	lc_address,
	lc_address_nomut,
	__mavitix_get_lc_address,
	address,
);
lc_func!(
	lc_collate,
	lc_collate_nomut,
	__mavitix_get_lc_collate,
	collate,
);
lc_func!(lc_ctype, lc_ctype_nomut, __mavitix_get_lc_ctype, ctype);
lc_func!(
	lc_messages,
	lc_messages_nomut,
	__mavitix_get_lc_messages,
	messages,
);
lc_func!(
	lc_monetary,
	lc_monetary_nomut,
	__mavitix_get_lc_monetary,
	monetary,
);
lc_func!(lc_name, lc_name_nomut, __mavitix_get_lc_name, name);
lc_func!(
	lc_numeric,
	lc_numeric_nomut,
	__mavitix_get_lc_numeric,
	numeric,
);
lc_func!(
	lc_telephone,
	lc_telephone_nomut,
	__mavitix_get_lc_telephone,
	telephone,
);
lc_func!(lc_time, lc_time_nomut, __mavitix_get_lc_time, time);

// TODO: Add a compile-time configuration option to make this properly synchronised.
static mut MAVITIX_LOCALE: GlobalLocale = GlobalLocale::default();

#[inline]
fn __mavitix_getlocale() -> *mut GlobalLocale {
	&raw mut MAVITIX_LOCALE
}

// TODO: `uselocale` may require `#[thread_local]` again, even after trying to move `MAVITIX_LOCALE` to non thread-local.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn setlocale(category: i32, mut locale: *const c_char) -> *const c_char {
	if locale.is_null() {
		return match category {
			LC_CTYPE => __mavitix_get_lc_ctype(),
			LC_NUMERIC => __mavitix_get_lc_numeric(),
			LC_TIME => __mavitix_get_lc_time(),
			LC_COLLATE => __mavitix_get_lc_collate(),
			LC_MONETARY => __mavitix_get_lc_monetary(),
			LC_MESSAGES => __mavitix_get_lc_messages(),
			LC_ALL => unsafe { (*__mavitix_getlocale()).lc_all_nomut() },
			LC_NAME => __mavitix_get_lc_name(),
			LC_ADDRESS => __mavitix_get_lc_address(),
			LC_TELEPHONE => __mavitix_get_lc_telephone(),
			_ => {
				cold_path();
				einval!();
			},
		}
		.to_c_str();
	};
	let locale_info: &mut Locale = unsafe {
		match category {
			LC_CTYPE => (*__mavitix_getlocale()).lc_ctype(),
			LC_NUMERIC => (*__mavitix_getlocale()).lc_numeric(),
			LC_TIME => (*__mavitix_getlocale()).lc_time(),
			LC_COLLATE => (*__mavitix_getlocale()).lc_collate(),
			LC_MONETARY => (*__mavitix_getlocale()).lc_monetary(),
			LC_MESSAGES => (*__mavitix_getlocale()).lc_messages(),
			LC_ALL => (*__mavitix_getlocale()).lc_all(),
			LC_NAME => (*__mavitix_getlocale()).lc_name(),
			LC_ADDRESS => (*__mavitix_getlocale()).lc_address(),
			LC_TELEPHONE => (*__mavitix_getlocale()).lc_telephone(),
			_ => {
				cold_path();
				einval!();
			},
		}
	};
	// SAFETY: Callers ensure `locale` is well-formed.
	let mut len: usize = unsafe { strlen(locale) };
	if len == 0 {
		if category == locale::LC_ALL {
			for category in [
				LC_CTYPE,
				LC_NUMERIC,
				LC_TIME,
				LC_COLLATE,
				LC_MONETARY,
				LC_MESSAGES,
				LC_NAME,
				LC_ADDRESS,
				LC_TELEPHONE,
			] {
				if unsafe { setlocale(category, locale) }.is_null() {
					einval!();
				};
			}
		};
		locale = unsafe {
			getenv(
				match category {
					LC_CTYPE => c"LC_CTYPE",
					LC_NUMERIC => c"LC_NUMERIC",
					LC_TIME => c"LC_TIME",
					LC_COLLATE => c"LC_COLLATE",
					LC_MONETARY => c"LC_MONETARY",
					LC_MESSAGES => c"LC_MESSAGES",
					LC_ALL => c"LC_ALL",
					LC_NAME => c"LC_NAME",
					LC_ADDRESS => c"LC_ADDRESS",
					LC_TELEPHONE => c"LC_TELEPHONE",
					_ => unreachable_unchecked(),
				}
				.as_ptr(),
			)
		};
		if locale.is_null() {
			locale = unsafe { getenv(c"LC_ALL".as_ptr()) };
		};
		if locale.is_null() {
			locale = unsafe { getenv(c"LANG".as_ptr()) };
		};
		if locale.is_null() {
			locale = unsafe { getenv(c"LANGUAGE".as_ptr()) };
		};
		if locale.is_null() {
			locale = c"C.UTF-8".as_ptr();
		};
		len = unsafe { strlen(locale) };
	};
	// SANITY(unusual):
	// Using `cast_slice` here only works because `locale` is an array of bytes.
	// If it was any other type, the length would be incorrect.
	let bytes: &[u8] = unsafe { (locale as *const u8).cast_slice(len).as_ref_unchecked() };
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
		&[b'j', b'a', ref rest @ ..] => match rest {
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
