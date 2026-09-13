use std::{
	env::args_os,
	ffi::{c_char, c_int, OsStr},
	io::Error,
	mem::transmute,
	os::unix::ffi::OsStrExt as _,
	process::exit,
};

use mavitix_utils::{bold, const_println, italic};

pub fn main() {
	let mut source: Option<&'static OsStr> = None;
	let mut dest: Option<&'static OsStr> = None;
	let mut seen_double_dash: bool = false;
	for os_arg in args_os().skip(1) {
		let arg: &[u8] = os_arg.as_bytes();
		if seen_double_dash || arg[0] != b'-' {
			if let None = source {
				source = Some(os_arg.leak());
				continue;
			};
			if let None = dest {
				dest = Some(os_arg.leak());
				continue;
			};
			eprintln!("link: too many operands");
			exit(1);
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
					italic!("SOURCE"),
					' ',
					italic!("DEST"),
					"\n\nWritten by Mavity The Madity.",
				));
				return;
			},
			b"--version" => {
				const_println!(concat!(
					"link (Mavitix coreutils) ",
					env!("CARGO_PKG_VERSION"),
				));
				return;
			},
			b"--" => seen_double_dash = true,
			_ => {
				eprintln!("link: unexpected option {os_arg:?}");
				exit(1);
			},
		};
	}
	let Some(source): Option<&'static OsStr> = source else {
		eprintln!("link: missing operand(s)");
		exit(1);
	};
	let Some(dest): Option<&'static OsStr> = dest else {
		eprintln!("link: missing operand(s)");
		exit(1);
	};
	// SAFETY: This is "intentional" for some reason...
	let source_ptr: *const c_char = unsafe { transmute(source.as_bytes().as_ptr()) };
	// SAFETY: This is "intentional" for some reason...
	let dest_ptr: *const c_char = unsafe { transmute(dest.as_bytes().as_ptr()) };
	// SAFETY:
	if unsafe { link(source_ptr, dest_ptr) } == -1 {
		let err: Error = Error::last_os_error();
		eprintln!("link: couldn't link {source:?} and {dest:?}; {err}");
		exit(1);
	};
}

// SAFETY: The function declarations given below are in line with the header files of `libc`.
#[link(name = "c")]
unsafe extern "C" {

	pub fn link(oldpath: *const c_char, newpath: *const c_char) -> c_int;
}
