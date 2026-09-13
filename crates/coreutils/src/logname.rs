use std::{
	env::args_os,
	ffi::c_char,
	hint::cold_path,
	io::Error,
	os::unix::ffi::OsStrExt as _,
	process::exit,
};

use mavitix_utils::{bold, const_println, login::getlogin, puts, unbuffer};

pub fn main() {
	#[cfg(any(target_env = "gnu", feature = "libc-is-buffered"))]
	unbuffer!();
	let mut seen_double_dash: bool = false;
	for os_arg in args_os().skip(1) {
		let arg: &[u8] = os_arg.as_bytes();
		if seen_double_dash || arg[0] != b'-' {
			eprintln!("logname: unexpected operand {os_arg:?}");
			exit(1);
		};
		match arg {
			b"-h" | b"--help" => {
				const_println!(concat!(
					"Usage:\n\t",
					bold!("logname"),
					" [",
					bold!("-h"),
					'|',
					bold!("--help"),
					"] [",
					bold!("--version"),
					"]\n\nWritten by Mavity The Madity.",
				));
				return;
			},
			b"--version" => {
				const_println!(concat!(
					"logname (Mavitix coreutils) ",
					env!("CARGO_PKG_VERSION"),
				));
				return;
			},
			b"--" => seen_double_dash = true,
			_ => {
				eprintln!("logname: unexpected option {os_arg:?}");
				exit(1);
			},
		};
	}
	// SAFETY: Soundness is guaranteed, errors are handled below.
	let ptr: *mut c_char = unsafe { getlogin() };
	if ptr.is_null() {
		let err: Error = Error::last_os_error();
		eprintln!("logname: couldn't get user login name; {err}");
		exit(1);
	};
	// SAFETY: `ptr` has been validated as well-formed and non-null.
	if unsafe { puts(ptr.cast_const()) } == -1 {
		let error: Error = Error::last_os_error();
		cold_path();
		eprintln!("logname: failed to print user login name; {error}");
		exit(1);
	};
	// SAFETY + SANITY(dangling + ptr):
	// `ptr` is not freed because it is a statically allocated resource managed by `libc`.
}
