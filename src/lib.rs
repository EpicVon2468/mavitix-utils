#![feature(const_default, const_trait_impl)]

use std::ffi::{CStr, CString};

pub mod passwd;
pub mod uname;

#[inline(always)]
pub fn cstr_clone(value: &CStr) -> CString {
	let mut dest: CString = {
		let tmp: Vec<u8> = Vec::with_capacity(value.count_bytes() + 1);
		// SAFETY:
		unsafe { CString::from_vec_unchecked(tmp) }
	};
	value.clone_into(&mut dest);
	dest
}
