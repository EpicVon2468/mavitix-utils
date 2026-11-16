#![no_std]
#![crate_name = "setlocale"]
#![crate_type = "cdylib"]
#![feature(derive_const, const_default, const_trait_impl, thread_local, ffi_const)]

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
	__mavitix_get_lc_nomut,
	numeric,
);
lc_func!(
	lc_telephone,
	lc_telephone_nomut,
	__mavitix_get_lc_telephone,
	telephone,
);
lc_func!(lc_time, lc_time_nomut, __mavitix_get_lc_time, time);

// TODO:
// This shouldn't be thread-local, as user programs expect a change on
// the main thread to be reflected in all threads spawned afterwards.
#[thread_local]
static mut MAVITIX_LOCALE: GlobalLocale = GlobalLocale::default();

#[inline]
fn __mavitix_getlocale() -> *mut GlobalLocale {
	&raw mut MAVITIX_LOCALE
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn setlocale(category: i32, locale: *const c_char) -> *const c_char {
	if locale.is_null() {
		return unsafe {
			match category {
				locale::LC_CTYPE => (*__mavitix_getlocale()).lc_ctype_nomut(),
				locale::LC_NUMERIC => (*__mavitix_getlocale()).lc_numeric_nomut(),
				locale::LC_TIME => (*__mavitix_getlocale()).lc_time_nomut(),
				locale::LC_COLLATE => (*__mavitix_getlocale()).lc_collate_nomut(),
				locale::LC_MONETARY => (*__mavitix_getlocale()).lc_monetary_nomut(),
				locale::LC_MESSAGES => (*__mavitix_getlocale()).lc_messages_nomut(),
				locale::LC_ALL => (*__mavitix_getlocale()).lc_all_nomut(),
				locale::LC_NAME => (*__mavitix_getlocale()).lc_name_nomut(),
				locale::LC_ADDRESS => (*__mavitix_getlocale()).lc_address_nomut(),
				locale::LC_TELEPHONE => (*__mavitix_getlocale()).lc_telephone_nomut(),
				_ => {
					cold_path();
					einval!();
				},
			}
			.to_c_str()
		};
	};
	let locale_info: &mut Locale = unsafe {
		match category {
			locale::LC_CTYPE => (*__mavitix_getlocale()).lc_ctype(),
			locale::LC_NUMERIC => (*__mavitix_getlocale()).lc_numeric(),
			locale::LC_TIME => (*__mavitix_getlocale()).lc_time(),
			locale::LC_COLLATE => (*__mavitix_getlocale()).lc_collate(),
			locale::LC_MONETARY => (*__mavitix_getlocale()).lc_monetary(),
			locale::LC_MESSAGES => (*__mavitix_getlocale()).lc_messages(),
			locale::LC_ALL => (*__mavitix_getlocale()).lc_all(),
			locale::LC_NAME => (*__mavitix_getlocale()).lc_name(),
			locale::LC_ADDRESS => (*__mavitix_getlocale()).lc_address(),
			locale::LC_TELEPHONE => (*__mavitix_getlocale()).lc_telephone(),
			_ => {
				cold_path();
				einval!();
			},
		}
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
