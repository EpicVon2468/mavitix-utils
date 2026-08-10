use std::{
	env::args_os,
	hint::cold_path,
	io::{BufWriter, StdoutLock, Write as _, stdout},
	os::unix::ffi::OsStrExt as _,
	process::exit,
};

use mavitix_utils::{bold, const_println, passwd::get_username};

pub fn main() {
	for os_arg in args_os().skip(1) {
		let arg: &[u8] = os_arg.as_bytes();
		match arg {
			b"-h" | b"--help" => {
				const_println!(concat!(
					"Usage:\n\t",
					bold!("whoami"),
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
					"whoami (Mavitix coreutils) ",
					env!("CARGO_PKG_VERSION"),
				));
				return;
			},
			_ => {
				cold_path();
				eprintln!(
					"whoami: unexpected or invalid {} {os_arg:?}!",
					if arg[0] == b'-' { "option" } else { "argument" },
				);
				exit(1);
			},
		};
	}
	let Some(username): Option<String> = get_username() else {
		cold_path();
		eprintln!("Could not determine username!");
		exit(1);
	};
	let mut stdout: BufWriter<StdoutLock> = BufWriter::new(stdout().lock());
	let Ok(_) = stdout.write(username.as_bytes()) else {
		cold_path();
		exit(1);
	};
	let Ok(_) = stdout.write(const { &[b'\n'] }) else {
		cold_path();
		exit(1);
	};
	let _ = stdout.flush();
	drop(stdout);
}
