use std::{
	env::args_os,
	ffi::c_char,
	hint::cold_path,
	io::Error,
	os::unix::ffi::OsStrExt as _,
	process::exit,
};

use mavitix_utils::{bold, const_println, passwd::get_raw_username, puts, unbuffer};

pub fn main() {
	#[cfg(any(target_env = "gnu", feature = "libc-is-buffered"))]
	unbuffer!();
	let mut seen_double_dash: bool = false;
	for os_arg in args_os().skip(1) {
		let arg: &[u8] = os_arg.as_bytes();
		if seen_double_dash || arg[0] != b'-' {
			eprintln!("whoami: unexpected operand {os_arg:?}");
			exit(1);
		};
		match arg {
			b"-h" | b"--help" => {
				const_println!(concat!(
					"Usage:\n\t",
					bold!("whoami"),
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
					"whoami (Mavitix coreutils) ",
					env!("CARGO_PKG_VERSION"),
				));
				return;
			},
			b"--" => seen_double_dash = true,
			_ => {
				eprintln!("whoami: unexpected option {os_arg:?}");
				exit(1);
			},
		};
	}
	let Some(username): Option<*mut c_char> = get_raw_username() else {
		cold_path();
		eprintln!("whoami: could not get username");
		exit(1);
	};
	// SAFETY: The returned buffer is well-formed.
	if unsafe { puts(username.cast_const()) } == -1 {
		let error: Error = Error::last_os_error();
		cold_path();
		eprintln!("whoami: could not print username; {error}");
		exit(1);
	};
	// TODO: Do we need to free here?  Does the kernel handle dellocation?
}
