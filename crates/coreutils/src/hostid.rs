use std::{env::args_os, os::unix::ffi::OsStrExt as _, process::exit};

use mavitix_utils::{bold, const_println};

pub fn main() {
	let mut seen_double_dash: bool = false;
	for os_arg in args_os().skip(1) {
		let arg: &[u8] = os_arg.as_bytes();
		if seen_double_dash || arg[0] != b'-' {
			eprintln!("hostid: unexpected operand {os_arg:?}");
			exit(1);
		};
		match arg {
			b"-h" | b"--help" => {
				const_println!(concat!(
					"Usage:\n\t",
					bold!("hostid"),
					" [",
					bold!("-h"),
					'|',
					bold!("--help"),
					"] [",
					bold!("-V"),
					'|',
					bold!("--version"),
					"]\n\nWritten by Mavity The Madity",
				));
				return;
			},
			b"--version" => {
				const_println!(concat!(
					"hostid (Mavitix coreutils) ",
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
				eprintln!("hostid: unexpected option {os_arg:?}");
				exit(1);
			},
		};
	}
	println!("{:0>8x}", gethostid());
}

// SAFETY: The function declarations given below are in line with the header files of `libc`.
#[link(name = "c")]
unsafe extern "C" {
	pub safe fn gethostid() -> i64;
}
