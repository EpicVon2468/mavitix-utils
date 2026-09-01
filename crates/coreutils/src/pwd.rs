use std::{
	env::args_os,
	ffi::{c_char, c_int, c_void},
	hint::{cold_path, unreachable_unchecked},
	io::Error,
	os::unix::ffi::OsStrExt as _,
	process::exit,
};

use mavitix_utils::{bold, const_println, errno, free, malloc, realloc};

const PATH_MAX: usize = 4096;

// GNU pwd ignores 'non option arguments' ???
// https://www.gnu.org/savannah-checkouts/gnu/coreutils/manual/html_node/pwd-invocation.html
pub fn main() {
	let mut use_physical: bool = false;
	let mut seen_double_dash: bool = false;
	for os_arg in args_os().skip(1) {
		let arg: &[u8] = os_arg.as_bytes();
		if seen_double_dash {
			continue;
		};
		match arg {
			b"-h" | b"--help" => {
				const_println!(concat!(
					"Usage:\n\t",
					bold!("pwd"),
					" [",
					bold!("-h"),
					'|',
					bold!("--help"),
					"] [",
					bold!("-L"),
					'|',
					bold!("--logical"),
					"] [",
					bold!("-P"),
					'|',
					bold!("--physical"),
					"] [",
					bold!("--version"),
					"]\n\nWritten by Mavity The Madity.",
				));
				return;
			},
			b"--version" => {
				const_println!(concat!(
					"pwd (Mavitix coreutils) ",
					env!("CARGO_PKG_VERSION"),
				));
				return;
			},
			b"-L" | b"--logical" => use_physical = false,
			b"-P" | b"--physical" => use_physical = true,
			b"--" => seen_double_dash = true,
			_ => {
				cold_path();
				eprintln!("pwd: unexpected or invalid option {os_arg:?}");
				exit(1);
			},
		};
	}
	// TODO:
	// Add a compile-time configuration to use `pathconf(3)` instead?
	// See also: the example code in https://pubs.opengroup.org/onlinepubs/9799919799.2024edition/functions/getcwd.html
	let mut size: usize = 256;
	macro_rules! malloc {
		($(,)?) => {
			match unsafe { malloc::<c_char>(size) } {
				Some(buf) => buf,
				None => {
					// SANITY(unusual):
					// If `malloc` isn't working, you have bigger problems.
					cold_path();
					let error: Error = Error::last_os_error();
					unreachable!("pwd: failed to allocate memory; {error}");
				},
			}
		};
	}
	// SAFETY: The allocated memory is well-managed.
	let mut buf: *mut c_char = malloc!();
	macro_rules! free {
		($(,)?) => {{
			// SAFETY: About to terminate; End of lifetime.
			unsafe { free(buf as *mut c_void) };
		}};
	}
	loop {
		if unsafe { getcwd(buf, size) }.is_null() {
			match errno() {
				// SAFETY: From `getcwd(3)`: "On failure, these functions return NULL, and errno is set to indicate the error."
				0 => unsafe { unreachable_unchecked() },
				// ENOENT
				2 => {
					free!();
					eprintln!("pwd: working directory no longer exists");
					exit(1);
				},
				// EACCES
				13 => {
					free!();
					eprintln!("pwd: insufficient permissions to access working directory");
					exit(1);
				},
				// EFAULT
				14 => {
					cold_path();
					free!();
					unreachable!("pwd: unreachable; buffer was considered invalid");
				},
				// EINVAL
				// SAFETY:
				// From `getcwd(3)`: "EINVAL  The size argument is zero and buf is not a null pointer."
				// `size` is a known non-zero value in this program, therefore this cannot occur.
				// Any system violating this contract is already UB unto itself.
				22 => unsafe { unreachable_unchecked() },
				// ERANGE
				34 => {
					size *= 2;
					if size > PATH_MAX {
						// SANITY(unusual):
						// Looping this many times and still not having enough space is incredibly abnormal.
						cold_path();
						eprintln!("pwd: working directory exceeded PATH_MAX");
						exit(1);
					};
					if unsafe { !realloc(buf, size) } {
						// SANITY(unusual):
						// If `realloc` isn't working, you have bigger problems.
						cold_path();
						let error: Error = Error::last_os_error();
						// SANITY(dangling + ptr):
						// Leave `buf` dangling as its contents are UB on failure.
						// Freeing `buf` could potentially segfault or worse, thus skipping
						// this error message and giving a confusing crash without context.
						unreachable!("pwd: failed to (re)allocate memory; {error}");
					};
					continue;
				},
				unexpected => {
					cold_path();
					free!();
					let error: Error = Error::from_raw_os_error(unexpected as _);
					unreachable!("pwd: unexpected error; {error}");
				},
			};
		};
		break;
	}
	if use_physical {
		// TODO: `realpath(3)`.
	} else {
		if unsafe { puts(buf.cast_const()) } == -1 {
			cold_path();
			exit(1);
		};
	};
}

// SAFETY: The function declarations given below are in line with the header files of `libc`.
#[link(name = "c")]
unsafe extern "C" {

	pub fn getcwd(buf: *mut c_char, size: usize) -> *mut c_char;
	pub fn realpath(path: *const c_char, resolved_path: *mut c_char) -> *mut c_char;
	pub fn puts(s: *const c_char) -> c_int;
}
