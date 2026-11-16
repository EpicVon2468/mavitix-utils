use core::ffi::c_char;

pub const LC_CTYPE: i32 = 0;
pub const LC_NUMERIC: i32 = 1;
pub const LC_TIME: i32 = 2;
pub const LC_COLLATE: i32 = 3;
pub const LC_MONETARY: i32 = 4;
pub const LC_MESSAGES: i32 = 5;
pub const LC_ALL: i32 = 6;
pub const LC_PAPER: i32 = 7;
pub const LC_NAME: i32 = 8;
pub const LC_ADDRESS: i32 = 9;
pub const LC_TELEPHONE: i32 = 10;
pub const LC_MEASUREMENT: i32 = 11;
pub const LC_IDENTIFICATION: i32 = 12;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Locale {
	pub language: Language,
	pub territory: Territory,
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
				Territory::None => c"jp.UTF-8",
				Territory::AU => c"jp_AU.UTF-8",
				Territory::GB => c"jp_GB.UTF-8",
				Territory::JP => c"jp_JP.UTF-8",
				Territory::NZ => c"jp_NZ.UTF-8",
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
