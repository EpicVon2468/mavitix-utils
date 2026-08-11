use std::{
	env::args_os,
	ffi::{CStr, c_char},
	hint::cold_path,
	io::{BufWriter, Error, StdoutLock, Write as _, stdout},
	os::unix::ffi::OsStrExt as _,
	process::exit,
};

use mavitix_utils::{bold, const_println, login::getlogin};

pub fn main() {
	let mut seen_double_dash: bool = false;
	for os_arg in args_os().skip(1) {
		if seen_double_dash {
			cold_path();
			eprintln!("logname: unexpected argument {os_arg:?}!");
			exit(1);
		};
		let arg: &[u8] = os_arg.as_bytes();
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
				cold_path();
				eprintln!(
					"logname: unexpected {} {os_arg:?}!",
					if arg[0] == b'-' { "option" } else { "argument" },
				);
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
	let mut stdout: BufWriter<StdoutLock> = BufWriter::new(stdout().lock());
	// SAFETY:
	let login: &CStr = unsafe { CStr::from_ptr(ptr) };
	let Ok(_): Result<usize, Error> = stdout.write(login.to_bytes()) else {
		cold_path();
		exit(1);
	};
	let Ok(_): Result<usize, Error> = stdout.write(const { &[b'\n'] }) else {
		cold_path();
		exit(1);
	};
	let Ok(_): Result<(), Error> = stdout.flush() else {
		cold_path();
		exit(1);
	};
	drop(stdout);
}
