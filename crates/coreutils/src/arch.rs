use std::{
	env::{args_os, consts::ARCH},
	hint::cold_path,
	os::unix::ffi::OsStrExt as _,
	process::exit,
};

use mavitix_utils::{bold, const_println};

pub fn main() {
	for os_arg in args_os().skip(1) {
		let arg: &[u8] = os_arg.as_bytes();
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
			_ => {
				cold_path();
				eprintln!(
					"arch: unexpected or invalid {} {os_arg:?}!",
					if arg[0] == b'-' { "option" } else { "argument" },
				);
				exit(1);
			},
		};
	}
	const_println!(ARCH);
}
