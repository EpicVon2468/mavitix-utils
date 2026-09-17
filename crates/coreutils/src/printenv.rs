use std::{
	env::{args, var_os as get_var, vars_os},
	ffi::OsString,
	hint::cold_path,
	io::{BufWriter, StdoutLock, Write as _, stdout},
	process::exit,
};

use mavitix_utils::{bold, const_println, italic, unbuffer};

macro_rules! try_io {
	($action:expr $(,)?) => {{
		let Ok(_) = $action else {
			cold_path();
			exit(2);
		};
	}};
}

// See: https://www.gnu.org/software/coreutils/printenv
pub fn main() {
	#[cfg(any(target_env = "gnu", feature = "libc-is-buffered"))]
	unbuffer!();
	let mut named_env_vars: Vec<String> = Vec::with_capacity(8);
	let mut use_null: bool = false;
	let mut seen_double_dash: bool = false;
	// TODO: args_os() ?
	for arg in args().skip(1) {
		if seen_double_dash || arg.as_bytes()[0] != b'-' {
			named_env_vars.push(arg);
			continue;
		};
		match &*arg {
			"-h" | "--help" => {
				const_println!(concat!(
					"Usage:\n\t",
					bold!("printenv"),
					" [",
					bold!("-0"),
					'|',
					bold!("--null"),
					"] [",
					bold!("-h"),
					'|',
					bold!("--help"),
					"] [",
					bold!("-V"),
					'|',
					bold!("--version"),
					"] [",
					italic!("VARIABLE"),
					"...]\n\nWritten by Mavity The Madity.",
				));
				return;
			},
			"--version" => {
				const_println!(concat!(
					"printenv (Mavitix coreutils) ",
					env!("CARGO_PKG_VERSION"),
				));
				return;
			},
			"-V" => {
				const_println!(env!("CARGO_PKG_VERSION"));
				return;
			},
			"-0" | "--null" => use_null = true,
			"--" => seen_double_dash = true,
			unexpected => {
				eprintln!("printenv: unexpected option {unexpected:?}");
				exit(1);
			},
		};
	}
	let separator: &[u8] = &[if use_null { b'\0' } else { b'\n' }];
	if named_env_vars.is_empty() {
		let mut stdout: BufWriter<StdoutLock> = BufWriter::new(stdout().lock());
		for (key, value) in vars_os() {
			try_io!(stdout.write(key.as_encoded_bytes()));
			try_io!(stdout.write(const { &[b'='] }));
			try_io!(stdout.write(value.as_encoded_bytes()));
			try_io!(stdout.write(separator));
			// SANITY(inconsistent):
			// Can't discard `flush()` here, as this is when the standard output is written to.
			// GNU printenv says that failures to print must result in error code 2.
			try_io!(stdout.flush());
		}
		drop(stdout);
		return;
	} else {
		let mut exit_err: bool = false;
		let mut stdout: BufWriter<StdoutLock> = BufWriter::new(stdout().lock());
		for var_name in named_env_vars {
			// FIXME:
			// Isn't this a standard violation on Rust's part?
			// There's nothing which says an environment variable has to be valid string data,
			// in fact, there are more explicit confirmations it can be non-valid string data than denials.
			let Some(value): Option<OsString> = get_var(&var_name) else {
				exit_err = true;
				// SANITY(unusual): GNU printenv seems to print all variables it can, and just return `1` if any were not present.
				continue;
			};
			try_io!(stdout.write(value.as_encoded_bytes()));
			try_io!(stdout.write(separator));
			try_io!(stdout.flush());
		}
		drop(stdout);
		exit(exit_err as i32);
	};
}
