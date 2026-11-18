use core::ffi::c_char;

pub const LC_CTYPE: i32 = 0;
pub const LC_NUMERIC: i32 = 1;
pub const LC_TIME: i32 = 2;
pub const LC_COLLATE: i32 = 3;
pub const LC_MONETARY: i32 = 4;
pub const LC_MESSAGES: i32 = 5;
pub const LC_ALL: i32 = 6;
pub const LC_NAME: i32 = 8;
pub const LC_ADDRESS: i32 = 9;
pub const LC_TELEPHONE: i32 = 10;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Locale {
	pub language: Language,
	pub territory: Territory,
}

const impl Default for Locale {
	fn default() -> Self {
		Self {
			language: Language::English,
			territory: Territory::None,
		}
	}
}

impl Locale {
	pub const fn to_c_str(&self) -> *const c_char {
		match self.language {
			Language::English => match self.territory {
				Territory::None => c"C.UTF-8",
				Territory::AU => c"en_AU.UTF-8",
				Territory::GB => c"en_GB.UTF-8",
				Territory::JP => c"en_JP.UTF-8",
				Territory::NZ => c"en_NZ.UTF-8",
			}
			.as_ptr(),
			Language::Japanese => match self.territory {
				Territory::None => c"ja.UTF-8",
				Territory::AU => c"ja_AU.UTF-8",
				Territory::GB => c"ja_GB.UTF-8",
				Territory::JP => c"ja_JP.UTF-8",
				Territory::NZ => c"ja_NZ.UTF-8",
			}
			.as_ptr(),
		}
	}
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum Language {
	English,
	Japanese,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum Territory {
	None,
	AU,
	GB,
	JP,
	NZ,
}

unsafe extern "C" {

	pub safe fn __mavitix_get_lc_address() -> Locale;

	pub safe fn __mavitix_get_lc_collate() -> Locale;

	pub safe fn __mavitix_get_lc_ctype() -> Locale;

	pub safe fn __mavitix_get_lc_messages() -> Locale;

	pub safe fn __mavitix_get_lc_monetary() -> Locale;

	pub safe fn __mavitix_get_lc_name() -> Locale;

	pub safe fn __mavitix_get_lc_numeric() -> Locale;

	pub safe fn __mavitix_get_lc_telephone() -> Locale;

	pub safe fn __mavitix_get_lc_time() -> Locale;
}
