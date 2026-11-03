use std::{
	env::{ArgsOs, args_os},
	io::Error,
	os::unix::ffi::OsStrExt as _,
	process::exit,
	str::FromStr as _,
};

use mavitix_utils::{bold, const_println, italic};

macro_rules! invalid_operand {
	($operand:expr $(,)?) => {{
		eprintln!("nproc: invalid operand {:?}", $operand);
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
	let mut args: ArgsOs = args_os();
	// Skip args[0]
	let _ = args.next();
	while let Some(os_arg) = args.next() {
		let arg: &[u8] = os_arg.as_bytes();
		if seen_double_dash || arg[0] != b'-' {
			eprintln!("nproc: unexpected operand {os_arg:?}");
			exit(1);
		};
		let (option, operand): (&[u8], Option<&[u8]>) = match arg.split_once(|ch: &u8| *ch == b'=')
		{
			Some((option, operand)) => (
				option,
				match operand {
					&[] => None,
					other => Some(other),
				},
			),
			None => (arg, None),
		};
		match option {
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
				let operand: &[u8] = match operand {
					Some(operand) => operand,
					None => match args.next() {
						Some(inner) => inner.leak().as_bytes(),
						None => {
							eprintln!("nproc: missing operand for option '--ignore'");
							exit(1);
						},
					},
				};
				let Ok(operand): Result<&str, _> = str::from_utf8(operand) else {
					invalid_operand!(operand);
				};
				ignore = match usize::from_str(operand) {
					Ok(value) => value,
					Err(_) => invalid_operand!(operand),
				};
			},
			_ => {
				eprintln!("nproc: unexpected option {os_arg:?}");
				exit(1);
			},
		};
	}
	let mut count: usize = if all {
		let value: i64 = sysconf(_SC_NPROCESSORS_ONLN);
		if value < 1 {
			// SANITY(unusual): Ignore errors & coerce to 1.
			1
		} else {
			value as usize
		}
	} else {
		let mut mask: CPUSet = const { CPUSet { bits: [0; _] } };
		// SAFETY: `&raw mut mask` is non-null.
		if unsafe { sched_getaffinity(0, CPUSet::SIZE as u32, &raw mut mask) } == -1 {
			let error: Error = Error::last_os_error();
			eprintln!("nproc: failed to get affinity mask; {error}");
			exit(1);
		};
		// Based roughly on https://docs.rs/libc/0.2.189/src/libc/unix/linux_like/linux_l4re_shared.rs.html#1505
		let mut count: usize = 0;
		for bit in &mask.bits[..(CPUSet::SIZE / size_of_val(&mask.bits[0]))] {
			count += bit.count_ones() as usize;
		}
		if count < 1 { 1 } else { count }
	};
	if ignore != 0 {
		count = if ignore >= count { 1 } else { count - ignore };
	};
	println!("{count}");
}

#[repr(C)]
#[derive(Debug)]
pub struct CPUSet {
	pub bits: [u64; 16],
}

impl CPUSet {
	pub const SIZE: usize = size_of::<CPUSet>();
}

pub const _SC_NPROCESSORS_ONLN: i32 = 84;

// SAFETY: The function declarations given below are in line with the header files of `libc`.
#[link(name = "c")]
unsafe extern "C" {

	pub safe fn sysconf(name: i32) -> i64;

	pub fn sched_getaffinity(pid: i32, cpusetsize: u32, mask: *mut CPUSet) -> i32;
}
