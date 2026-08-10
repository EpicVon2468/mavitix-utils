#[macro_export]
macro_rules! bool_program {
	($name:expr, $exit_code:expr $(,)?) => {
		pub fn main() {
			use std::{
				env::{ArgsOs, args_os},
				os::unix::ffi::OsStrExt as _,
				process::exit,
			};

			use mavitix_utils::{bold, const_println, italic};

			let mut args: ArgsOs = args_os();
			let _ = args.next();
			if let Some(arg) = args.next()
				&& let None = args.next()
			{
				match arg.as_bytes() {
					b"-h" | b"--help" => const_println!(concat!(
						"Usage:\n\t",
						bold!($name),
						" [",
						bold!("-h"),
						'|',
						bold!("--help"),
						'|',
						bold!("--version"),
						'|',
						italic!("IGNORED"),
						"...]\n\nWritten by Mavity The Madity.",
					)),
					b"--version" => const_println!(concat!(
						$name,
						" (Mavitix coreutils) ",
						env!("CARGO_PKG_VERSION"),
					)),
					_ => (),
				};
			};
			exit($exit_code);
		}
	};
}
