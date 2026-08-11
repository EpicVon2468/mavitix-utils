use std::{
	env::args_os,
	ffi::c_void,
	hint::cold_path,
	io::{BufWriter, Error, StdoutLock, Write as _, stdout},
	os::unix::ffi::OsStrExt as _,
	process::exit,
	slice,
};

use mavitix_utils::{bold, const_println, italic, malloc, memcpy};

pub fn main() {
	let mut named_operands: Vec<&'static [u8]> = Vec::with_capacity(8);
	let mut seen_double_dash: bool = false;
	for os_arg in args_os().skip(1) {
		let arg: &[u8] = os_arg.as_bytes();
		if seen_double_dash || arg[0] != b'-' {
			let bytes: &'static [u8] = os_arg.leak().as_bytes();
			if named_operands.is_empty() {
				// SANITY(unusual): This only occurs a maximum of once.
				cold_path();
				named_operands.push(bytes);
			} else {
				let len: usize = bytes.len() + 1;
				assert!(len > 1);
				// SAFETY:
				// The memory allocated here will never need to be freed (manually or otherwise),
				// as `yes(1)` runs in an endless loop until a signal is sent to kill the process.
				//
				// Can't use `alloca` because it's a builtin :(
				let dest: *mut u8 = unsafe { malloc(len * size_of::<u8>()) } as *mut u8;
				if dest.is_null() {
					// SANITY(unusual):
					// If `malloc` isn't working, you have bigger problems than `yes` not working.
					cold_path();
					unreachable!("yes: `malloc` failed to allocate memory!");
				};
				// SAFETY: `malloc` returns well-formed pointers.
				unsafe {
					// Prepend the space character.
					*dest = b' ';
				};
				{
					// SAFETY:
					// `malloc` returns a pointer with `len` length.
					// `len` is guaranteed to always be at least 1.
					let dest: *mut u8 = unsafe { dest.add(1) };
					// SAFETY:
					unsafe {
						memcpy(
							dest as *mut c_void,
							bytes.as_ptr() as *const c_void,
							len - 1,
						);
					};
				};
				// SAFETY: `dest` is a well-formed pointer to a sequence of `u8`s with `len` length.
				named_operands.push(unsafe { slice::from_raw_parts::<'static, u8>(dest, len) });
			};
			continue;
		};
		match arg {
			b"-h" | b"--help" => {
				const_println!(concat!(
					"Usage:\n\t",
					bold!("yes"),
					" [",
					bold!("-h"),
					'|',
					bold!("--help"),
					"] [",
					bold!("--version"),
					"] [",
					italic!("STRING"),
					"...]\n\nWritten by Mavity The Madity.",
				));
				return;
			},
			b"--version" => {
				const_println!(concat!(
					"yes (Mavitix coreutils) ",
					env!("CARGO_PKG_VERSION"),
				));
				return;
			},
			b"--" => seen_double_dash = true,
			unexpected => {
				cold_path();
				eprintln!("yes: unexpected or invalid option {unexpected:?}!");
				exit(1);
			},
		};
	}
	if named_operands.is_empty() {
		named_operands.push(b"y");
	};
	let mut stdout: BufWriter<StdoutLock> = BufWriter::new(stdout().lock());
	loop {
		for operand in &named_operands {
			let Ok(_): Result<_, Error> = stdout.write(operand) else {
				cold_path();
				exit(1);
			};
		}
		let Ok(_): Result<_, Error> = stdout.write(const { &[b'\n'] }) else {
			cold_path();
			exit(1);
		};
		// GNU yes seems to flush after every '$pat ' sequence.  Not sure if complying with that is required.
		// Can't inspect their source code to see how they do it, so will remain like this unless further research uncovers something better.
		let Ok(_): Result<_, Error> = stdout.flush() else {
			cold_path();
			exit(1);
		};
	}
}
