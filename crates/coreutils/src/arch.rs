use std::{
	env::{ArgsOs, args_os, consts::ARCH},
	os::unix::ffi::OsStrExt as _,
	process::exit,
};

use mavitix_utils::{bold, const_println};

pub fn main() {
	let mut seen_double_dash: bool = false;
	let mut args: ArgsOs = args_os();
	// Skip argv[0]
	let _ = args.next();
	for os_arg in args {
		let arg: &[u8] = os_arg.as_bytes();
		if seen_double_dash || arg[0] != b'-' {
			eprintln!("arch: unexpected operand {os_arg:?}");
			exit(1);
		};
		match arg {
			b"-h" | b"--help" => {
				const_println!(concat!(
					"Usage:\n\t",
					bold!("arch"),
					" [",
					bold!("-h"),
					'|',
					bold!("--help"),
					"] [",
					bold!("-V"),
					'|',
					bold!("--version"),
					"]\n\nWritten by Mavity The Madity.",
				));
				return;
			},
			b"--version" => {
				const_println!(concat!(
					"arch (Mavitix coreutils) ",
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
				eprintln!("arch: unexpected option {os_arg:?}");
				exit(1);
			},
		};
	}
	const_println!(ARCH);
}
