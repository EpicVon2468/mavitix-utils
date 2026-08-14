use std::{
	env::args_os,
	ffi::{CStr, c_char, c_int},
	hint::cold_path,
	io::{BufWriter, Error, StdoutLock, Write as _, stdout},
	os::unix::ffi::OsStrExt as _,
	process::exit,
};

use mavitix_utils::{bold, const_println};

// 1 if standard input is a non-terminal file (i.e. `tty < /dev/full`)
// 2 if given incorrect arguments (i.e. `tty foo`)
// 3 if a write error occurs (i.e. `tty > /dev/full`)
// 4 if the terminal’s name cannot be determined
pub fn main() {
	let mut silent: bool = false;
	let mut seen_double_dash: bool = false;
	for os_arg in args_os().skip(1) {
		if seen_double_dash {
			cold_path();
			eprintln!("tty: unexpected argument {os_arg:?}!");
			exit(2);
		};
		let arg: &[u8] = os_arg.as_bytes();
		match arg {
			b"-s" | b"--silent" | b"--quiet" => silent = true,
			b"-h" | b"--help" => {
				const_println!(concat!(
					"Usage:\n\t",
					bold!("tty"),
					" [",
					bold!("-h"),
					'|',
					bold!("--help"),
					"] [",
					bold!("--version"),
					"] [",
					bold!("-s"),
					'|',
					bold!("--silent"),
					'|',
					bold!("--quiet"),
					"]\n\nWritten by Mavity The Madity.",
				));
				return;
			},
			b"--version" => {
				const_println!(concat!(
					"tty (Mavitix coreutils) ",
					env!("CARGO_PKG_VERSION"),
				));
				return;
			},
			b"--" => seen_double_dash = true,
			_ => {
				cold_path();
				eprintln!(
					"tty: unexpected {} {os_arg:?}!",
					if arg[0] == b'-' { "option" } else { "argument" },
				);
				exit(2);
			},
		}
	}
	// SAFETY: Trusted compile-time fileno.
	let name: *mut c_char = unsafe { ttyname(0) };
	if name.is_null() {
		let err: Error = Error::last_os_error();
		match err.raw_os_error() {
			// EBADF
			Some(9) => {
				// SANITY(unreachable):
				// The value passed to `ttyname` is a known safe file descriptor constant (`STDIN_FILENO`).
				cold_path();
				unreachable!("tty: {err}");
			},
			// ENODEV
			Some(19) => {
				eprintln!("tty: {err}");
				exit(4);
			},
			// ENOTTY
			Some(25) => {
				eprintln!("tty: not a tty");
				exit(1);
			},
			_ => {
				// SANITY(unreachable): `ttyname(3)` does not specify any other errors.
				cold_path();
				unreachable!("tty: {err}");
			},
		};
	};
	if silent {
		return;
	};
	// SAFETY: From `ttyname(3)`: "... ttyname() returns a pointer to the null-terminated pathname ..."
	let cstr: &CStr = unsafe { CStr::from_ptr(name) };
	let mut stdout: BufWriter<StdoutLock> = BufWriter::new(stdout().lock());
	let Ok(_): Result<_, Error> = stdout.write(cstr.to_bytes()) else {
		cold_path();
		exit(3);
	};
	let Ok(_): Result<_, Error> = stdout.write(const { &[b'\n'] }) else {
		cold_path();
		exit(3);
	};
	let Ok(_): Result<_, Error> = stdout.flush() else {
		cold_path();
		exit(3);
	};
}

// SAFETY: The function declarations given below are in line with the header files of `libc`.
#[link(name = "c")]
unsafe extern "C" {

	pub fn ttyname(fd: c_int) -> *mut c_char;
}
