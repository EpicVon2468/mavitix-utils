#![feature(const_default, const_trait_impl)]

use std::ffi::{CStr, CString};

pub mod cli;
pub mod login;
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

#[macro_export]
macro_rules! bold {
	($value:expr $(,)?) => {
		concat!("\x1B[1m", $value, "\x1B[22m")
	};
}

#[macro_export]
macro_rules! italic {
	($value:expr $(,)?) => {
		concat!("\x1B[4m", $value, "\x1B[24m")
	};
}

// SANITY(const-hack + unusual): Me?  Abuse potentially triple-buffered I/O streams?  Noooo...
#[macro_export]
macro_rules! const_println {
	($($value:expr),* $(,)?) => {{
		use std::io::Write as _;

		let mut stdout: std::io::BufWriter<_> = std::io::BufWriter::new(std::io::stdout().lock());
		$({
			let Ok(_) = stdout.write(const { $value.as_bytes() }) else {
				std::hint::cold_path();
				std::process::exit(1);
			};
		})*;
		let Ok(_) = stdout.write(const { &[b'\n'] }) else {
			std::hint::cold_path();
			std::process::exit(1);
		};
		let Ok(_) = stdout.flush() else {
			std::hint::cold_path();
			std::process::exit(1);
		};
		drop(stdout);
	}};
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
