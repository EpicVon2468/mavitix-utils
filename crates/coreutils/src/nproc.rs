#![feature(slice_split_once)]

use std::{
	env::{args_os, ArgsOs},
	ffi::OsString,
	io::Error,
	os::unix::ffi::OsStrExt as _,
	process::exit,
};

use mavitix_utils::{bold, const_println, italic};

macro_rules! invalid_operand {
	($operand:expr $(,)?) => {{
		eprintln!("nproc: invalid operand {:?}", $operand);
		exit(1);
	}};
}

macro_rules! unexpected_option {
	($option:expr $(,)?) => {{
		eprintln!("nproc: unexpected option {:?}", $option);
		exit(1);
	}};
}

// https://stackoverflow.com/questions/4586405/how-to-get-the-number-of-cpus-in-linux-using-c
// Important notes (values are samples, tested on my machine using GNU `nproc(1)`):
// `nproc` -> `24`
// `nproc --ignore=0` -> `24`
// `nproc --ignore=1` -> `23`
// `nproc --ignore=20` -> `4`
// `nproc --ignore=24` -> `1`
// `nproc --ignore=25` -> `1`
// `nproc --ignore=26` -> `1`
pub fn main() {
	let mut all: bool = false;
	let mut ignore: usize = 0;
	let mut seen_double_dash: bool = false;
	let args: &mut ArgsOs = &mut args_os();
	// Skip arg0
	args.next();
	while let Some(os_arg) = args.next() {
		let arg: &[u8] = os_arg.as_bytes();
		if seen_double_dash || arg[0] != b'-' {
			eprintln!("nproc: unexpected operand {os_arg:?}");
			exit(1);
		};
		match arg {
			b"-h" | b"--help" => {
				const_println!(concat!(
					"Usage:\n\t",
					bold!("nproc"),
					" [",
					bold!("-h"),
					'|',
					bold!("--help"),
					"] [",
					bold!("-V"),
					'|',
					bold!("--version"),
					"] [",
					bold!("--all"),
					"] [",
					bold!("--ignore"),
					'=',
					italic!("N"),
					"]\n\nWritten by Mavity The Madity",
				));
				return;
			},
			b"--version" => {
				const_println!(concat!(
					"nproc (Mavitix coreutils) ",
					env!("CARGO_PKG_VERSION"),
				));
				return;
			},
			b"-V" => {
				const_println!(env!("CARGO_PKG_VERSION"));
				return;
			},
			b"--" => seen_double_dash = true,
			b"--all" => all = true,
			b"--ignore" => {
				let Some(operand): Option<OsString> = args.next() else {
					eprintln!("nproc: missing operand");
					exit(1);
				};
				let Some(operand): Option<&str> = operand.to_str() else {
					invalid_operand!(operand);
				};
				ignore = match operand.parse() {
					Ok(value) => value,
					Err(_) => invalid_operand!(operand),
				};
			},
			_ if let Some((option, operand)) = arg.split_once(|ch: &u8| *ch == b'=') => {
				match option {
					b"--ignore" => {
						let operand: &str = match str::from_utf8(operand) {
							Ok(value) => value,
							Err(_) => invalid_operand!(operand),
						};
						ignore = match operand.parse() {
							Ok(value) => value,
							Err(_) => invalid_operand!(operand),
						};
					},
					_ => unexpected_option!(os_arg),
				};
			},
			_ => unexpected_option!(os_arg),
		};
	}
	let mut count: usize = if all {
		// SAFETY: Ignored.
		let value: i64 = unsafe { sysconf(_SC_NPROCESSORS_ONLN) };
		if value < 1 {
			// SANITY(unusual): Ignore errors & coerce to 1.
			1
		} else {
			value as usize
		}
	} else {
		let mut mask: cpu_set_t = const { cpu_set_t { bits: [0; _] } };
		// SAFETY:
		if unsafe { sched_getaffinity(0, cpu_set_t::SIZE as u32, &raw mut mask) } == -1 {
			let error: Error = Error::last_os_error();
			eprintln!("nproc: failed to get affinity mask; {error}");
			exit(1);
		};
		// Based roughly on https://docs.rs/libc/0.2.189/src/libc/unix/linux_like/linux_l4re_shared.rs.html#1505
		let mut count: usize = 0;
		for bit in &mask.bits[..(cpu_set_t::SIZE / size_of_val(&mask.bits[0]))] {
			count += bit.count_ones() as usize;
		}
		if count < 1 {
			1
		} else {
			count
		}
	};
	if ignore != 0 {
		count = if ignore >= count { 1 } else { count - ignore };
	};
	println!("{count}");
}

#[repr(C)]
pub struct cpu_set_t {
	pub bits: [u64; 16],
}

impl cpu_set_t {
	pub const SIZE: usize = size_of::<cpu_set_t>();
}

pub const _SC_NPROCESSORS_ONLN: i32 = 84;

unsafe extern "C" {

	pub fn sysconf(name: i32) -> i64;

	pub fn sched_getaffinity(pid: i32, cpusetsize: u32, mask: *mut cpu_set_t) -> i32;
}
