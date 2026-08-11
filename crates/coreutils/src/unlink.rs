use std::{
	env::args_os,
	ffi::{OsStr, c_char, c_int},
	hint::cold_path,
	io::Error,
	mem::transmute,
	os::unix::ffi::OsStrExt as _,
	process::exit,
};

use mavitix_utils::{bold, const_println, italic};

pub fn main() {
	// Mavitix extension; GNU unlink(1) only supports one file.
	let mut files: Vec<&'static OsStr> = Vec::with_capacity(4);
	let mut seen_double_dash: bool = false;
	for os_arg in args_os().skip(1) {
		let arg: &[u8] = os_arg.as_bytes();
		if seen_double_dash || arg[0] != b'-' {
			files.push(os_arg.leak());
			continue;
		};
		match arg {
			b"-h" | b"--help" => {
				const_println!(concat!(
					"Usage:\n\t",
					bold!("unlink"),
					" [",
					bold!("-h"),
					'|',
					bold!("--help"),
					"] [",
					bold!("--version"),
					"] ",
					italic!("FILE"),
					"...\n\nWritten by Mavity The Madity.",
				));
				return;
			},
			b"--version" => {
				const_println!(concat!(
					"unlink (Mavitix coreutils) ",
					env!("CARGO_PKG_VERSION"),
				));
				return;
			},
			b"--" => seen_double_dash = true,
			_ => {
				cold_path();
				eprintln!("unlink: unexpected or invalid option {os_arg:?}!");
				exit(1);
			},
		};
	}
	if files.is_empty() {
		eprintln!("unlink: missing operand(s).");
		exit(1);
	} else {
		let mut exit_err: bool = false;
		for file in files {
			// SAFETY: This is "intentional" for some reason...
			let ptr: *const c_char = unsafe { transmute(file.as_bytes().as_ptr()) };
			// SAFETY: Soundness is guaranteed, errors are handled below.
			if unsafe { unlink(ptr) } == -1 {
				let err: Error = Error::last_os_error();
				eprintln!("unlink: couldn't unlink file {file:?}; {err}");
				exit_err = true;
			};
		}
		exit(exit_err as i32);
	};
}

// SAFETY: The function declarations given below are in line with the header files of `libc`.
#[link(name = "c")]
unsafe extern "C" {

	pub fn unlink(path: *const c_char) -> c_int;
}
