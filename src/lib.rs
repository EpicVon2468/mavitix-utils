#![feature(const_default, const_trait_impl, ffi_const, extern_types)]

use std::ffi::{CStr, CString, c_char, c_void};

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

#[macro_export]
macro_rules! const_println {
	($value:expr $(,)?) => {
		mavitix_utils::const_println!(1; $value)
	};
	($code:expr; $value:expr $(,)?) => {{
		let ptr: *const std::ffi::c_char = const {
			const_str::concat_bytes!($value.as_bytes(), b'\0')
				.as_ptr()
				.cast()
		};
		if unsafe { mavitix_utils::puts(ptr) } == -1 {
			std::hint::cold_path();
			std::process::exit($code);
		};
		if unsafe { mavitix_utils::fflush(mavitix_utils::stdout) } == -1 {
			std::hint::cold_path();
			std::process::exit($code);
		};
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

pub fn errno() -> i32 {
	// SAFETY: `__errno_location()` is always set.
	unsafe { *__errno_location() }
}

#[macro_export]
macro_rules! unbuffer {
	($(,)?) => {
		unsafe { mavitix_utils::setbuf(mavitix_utils::stdout, std::ptr::null_mut()) }
	};
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
	pub safe fn __errno_location() -> *mut i32;

	pub fn puts(s: *const c_char) -> i32;

	pub type FILE;

	pub static stdout: *mut FILE;

	pub fn setbuf(stream: *mut FILE, buf: *mut c_char);
	pub fn fflush(stream: *mut FILE) -> i32;
}
