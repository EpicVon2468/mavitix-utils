#![feature(const_default, const_trait_impl)]

use std::ffi::{CStr, CString};

pub mod cli;
pub mod passwd;
pub mod uname;

#[macro_export]
macro_rules! main {
	($main_impl:expr $(,)?) => {
		pub fn main() -> std::process::ExitCode {
			match $main_impl {
				Ok(()) => std::process::ExitCode::SUCCESS,
				Err(error) => {
					eprintln!("{error}");
					let chain: Vec<&(dyn std::error::Error + 'static)> =
						error.chain().skip(1).collect();
					if !chain.is_empty() {
						eprintln!("Caused by:");
						for error in chain {
							eprintln!("\t{error}");
						}
					};
					std::process::ExitCode::FAILURE
				},
			}
		}
	};
}

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
