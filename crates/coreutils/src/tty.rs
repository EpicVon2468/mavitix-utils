use std::{
	env::args_os,
	ffi::c_char,
	hint::{cold_path, unreachable_unchecked},
	io::Error,
	os::unix::ffi::OsStrExt as _,
	process::exit,
};

use mavitix_utils::{bold, const_println, errno, puts, unbuffer};

// 1 if standard input is a non-terminal file (i.e. `tty < /dev/full`)
// 2 if given incorrect operands (i.e. `tty foo`)
// 3 if a write error occurs (i.e. `tty > /dev/full`)
// 4 if the terminal’s name cannot be determined
pub fn main() {
	#[cfg(any(target_env = "gnu", feature = "libc-is-buffered"))]
	unbuffer!();
	let mut silent: bool = false;
	let mut seen_double_dash: bool = false;
	for os_arg in args_os().skip(1) {
		let arg: &[u8] = os_arg.as_bytes();
		if seen_double_dash || arg[0] != b'-' {
			eprintln!("tty: unexpected operand {os_arg:?}");
			exit(2);
		};
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
					bold!("-V"),
					'|',
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
			b"-V" => {
				const_println!(env!("CARGO_PKG_VERSION"));
				return;
			},
			b"--" => seen_double_dash = true,
			_ => {
				eprintln!("tty: unexpected option {os_arg:?}");
				exit(2);
			},
		}
	}
	// SAFETY: Trusted compile-time fileno.
	let name: *const c_char = unsafe { ttyname(0) }.cast_const();
	if name.is_null() {
		match errno() {
			// SAFETY: From `ttyname(3)`: "The function `ttyname()` returns a pointer to a pathname on success.  On error, NULL is returned, and errno is set to indicate the error."
			0 => unsafe { unreachable_unchecked() },
			// EBADF
			9 => {
				// SANITY(unreachable):
				// The value passed to `ttyname` is a known safe file descriptor constant (`STDIN_FILENO`).
				cold_path();
				unreachable!("tty: unreachable; bad file descriptor");
			},
			// ENODEV
			19 => {
				eprintln!("tty: no such device");
				exit(4);
			},
			25 => {
				eprintln!("tty: not a tty");
				exit(1);
			},
			unexpected => {
				// SANITY(unreachable): `ttyname(3)` does not specify any other errors.
				cold_path();
				let error: Error = Error::from_raw_os_error(unexpected as _);
				unreachable!("tty: unexpected error; {error}");
			},
		};
	};
	if silent {
		return;
	};
	// SAFETY: From `ttyname(3)`: "... ttyname() returns a pointer to the null-terminated pathname ..."
	if unsafe { puts(name) } == -1 {
		cold_path();
		// SANITY(unusual): Can't print an error message here, because if `eprintln!()` fails, the program won't exit with status '3'.
		exit(3);
	};
}

// SAFETY: The function declarations given below are in line with the header files of `libc`.
#[link(name = "c")]
unsafe extern "C" {

	pub fn ttyname(fd: i32) -> *mut c_char;
}
