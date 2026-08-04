use std::ffi::{CStr, c_char, c_int};

pub const UTSNAME_LENGTH: usize = 65;

#[repr(C)]
pub struct utsname {
	pub sysname: [c_char; UTSNAME_LENGTH],
	pub nodename: [c_char; UTSNAME_LENGTH],
	pub release: [c_char; UTSNAME_LENGTH],
	pub version: [c_char; UTSNAME_LENGTH],
	pub machine: [c_char; UTSNAME_LENGTH],
	pub domainname: [c_char; UTSNAME_LENGTH],
}

const impl Default for utsname {
	fn default() -> Self {
		Self {
			sysname: [0; _],
			nodename: [0; _],
			release: [0; _],
			version: [0; _],
			machine: [0; _],
			domainname: [0; _],
		}
	}
}

// SAFETY: The function declarations given below are in line with the header files of `libc`.
#[link(name = "c")]
unsafe extern "C" {

	pub fn uname(buf: *mut utsname) -> c_int;
}

pub fn get_uname_raw() -> utsname {
	let mut dest: utsname = Default::default();
	// SAFETY:
	unsafe {
		uname(&raw mut dest);
	};
	dest
}

#[inline]
pub fn get_uname() -> UnixTimesharingSystemName {
	UnixTimesharingSystemName::from(get_uname_raw())
}

pub struct UnixTimesharingSystemName {
	pub sysname: String,
	pub nodename: String,
	pub release: String,
	pub version: String,
	pub machine: String,
	pub domainname: String,
}

impl From<utsname> for UnixTimesharingSystemName {
	fn from(value: utsname) -> Self {
		macro_rules! map_field {
			($field:ident $(,)?) => {
				match crate::cstr_clone(CStr::from_ptr(value.$field.as_ptr())).into_string() {
					Ok(value) => value,
					Err(_) => {
						std::hint::cold_path();
						unreachable!("uname(2) returns well-formed C strings.");
					},
				}
			};
		}
		Self {
			// SAFETY:
			sysname: unsafe { map_field!(sysname) },
			// SAFETY:
			nodename: unsafe { map_field!(nodename) },
			// SAFETY:
			release: unsafe { map_field!(release) },
			// SAFETY:
			version: unsafe { map_field!(version) },
			// SAFETY:
			machine: unsafe { map_field!(machine) },
			// SAFETY:
			domainname: unsafe { map_field!(domainname) },
		}
	}
}
