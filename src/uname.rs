use std::{ffi::c_char, slice};

use crate::strlen;

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

macro_rules! utsname_conv {
	($fn_name:ident, $field:ident $(,)?) => {
		pub fn $fn_name(&self) -> String {
			let ptr: *const c_char = self.$field.as_ptr();
			assert!(
				!ptr.is_null(),
				concat!(
					"uname(2) returned null for the '",
					stringify!($field),
					"' field.",
				),
			);
			// SAFETY: uname(2) returns well-formed C strings.
			let len: usize = unsafe { strlen(ptr) };
			// SAFETY: The length is directly calculated from relevant variables.
			let dest: &[u8] = unsafe { slice::from_raw_parts(ptr as *const u8, len) };
			String::from_utf8(dest.to_vec()).unwrap()
		}
	};
}

impl utsname {
	utsname_conv!(get_sysname, sysname);
	utsname_conv!(get_nodename, nodename);
	utsname_conv!(get_release, release);
	utsname_conv!(get_version, version);
	utsname_conv!(get_machine, machine);
	utsname_conv!(get_domainname, domainname);
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

	pub fn uname(buf: *mut utsname) -> i32;
}

pub fn get_uname() -> utsname {
	let mut dest: utsname = Default::default();
	// SAFETY:
	unsafe {
		uname(&raw mut dest);
	};
	dest
}
