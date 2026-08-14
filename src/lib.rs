#![feature(const_default, const_trait_impl, ffi_const)]

use std::ffi::{CStr, CString, c_int, c_void};

pub mod cli;
pub mod login;
pub mod passwd;
pub mod uname;

const _: () = cfg_select! {
	target_os = "linux" => (),
	_ => compile_error!("Unsupported OS!"),
};

#[macro_export]
macro_rules! main {
	($main_impl:expr $(,)?) => {
		pub fn main() -> std::process::ExitCode {
			match $main_impl {
				Ok(()) => std::process::ExitCode::SUCCESS,
				Err(error) => {
					eprintln!("{error}");
					let chain: Vec<&(dyn std::error::Error + 'static)> =
						error.chain().skip(1).collect();
					if !chain.is_empty() {
						eprintln!("Caused by:");
						for error in chain {
							eprintln!("\t{error}");
						}
					};
					std::process::ExitCode::FAILURE
				},
			}
		}
	};
}

#[macro_export]
macro_rules! bold {
	($value:expr $(,)?) => {
		concat!("\x1B[1m", $value, "\x1B[22m")
	};
}

#[macro_export]
macro_rules! italic {
	($value:expr $(,)?) => {
		concat!("\x1B[4m", $value, "\x1B[24m")
	};
}

// SANITY(const-hack + unusual): Me?  Abuse potentially triple-buffered I/O streams?  Noooo...
#[macro_export]
macro_rules! const_println {
	($($value:expr),* $(,)?) => {{
		use std::io::Write as _;

		let mut stdout: std::io::BufWriter<_> = std::io::BufWriter::new(std::io::stdout().lock());
		$({
			let Ok(_) = stdout.write(const { $value.as_bytes() }) else {
				std::hint::cold_path();
				std::process::exit(1);
			};
		})*;
		let Ok(_) = stdout.write(const { &[b'\n'] }) else {
			std::hint::cold_path();
			std::process::exit(1);
		};
		let Ok(_) = stdout.flush() else {
			std::hint::cold_path();
			std::process::exit(1);
		};
		drop(stdout);
	}};
}

#[inline(always)]
pub fn cstr_clone(value: &CStr) -> CString {
	let mut dest: CString = {
		let tmp: Vec<u8> = Vec::with_capacity(value.count_bytes() + 1);
		// SAFETY:
		unsafe { CString::from_vec_unchecked(tmp) }
	};
	value.clone_into(&mut dest);
	dest
}

pub unsafe fn malloc<T>(size: usize) -> Option<*mut T> {
	// SAFETY: Callers manage returned memory.
	let mem: *mut T = unsafe { raw_malloc(size * size_of::<T>()) } as *mut T;
	if mem.is_null() { None } else { Some(mem) }
}

pub unsafe fn realloc<T>(buf: *mut T, size: usize) -> bool {
	// SAFETY: Callers manage everything, not my problem.
	unsafe { raw_realloc(buf as *mut c_void, size * size_of::<T>()) }.is_null()
}

pub fn errno() -> c_int {
	// SAFETY: `__errno_location()` is always set.
	unsafe { *__errno_location() }
}

// SAFETY: The function declarations given below are in line with the header files of `libc`.
#[link(name = "c")]
unsafe extern "C" {

	#[link_name = "malloc"]
	pub fn raw_malloc(size: usize) -> *mut c_void;

	#[link_name = "realloc"]
	pub fn raw_realloc(ptr: *mut c_void, size: usize) -> *mut c_void;

	pub fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;

	pub fn free(ptr: *mut c_void);

	#[unsafe(ffi_const)]
	pub safe fn __errno_location() -> *mut c_int;
}
