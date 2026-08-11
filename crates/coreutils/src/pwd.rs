use std::{env::args_os, hint::cold_path, os::unix::ffi::OsStrExt as _, process::exit};

use mavitix_utils::{bold, const_println};

pub fn main() {
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
			b"--" => seen_double_dash = true,
			_ => {
				cold_path();
				eprintln!("pwd: unexpected or invalid option {os_arg:?}!");
				exit(1);
			},
		};
	}
	// TODO: `getcwd(3)`
	// https://www.gnu.org/savannah-checkouts/gnu/coreutils/manual/html_node/pwd-invocation.html
}
