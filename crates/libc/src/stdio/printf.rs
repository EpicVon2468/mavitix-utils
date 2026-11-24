#![no_std]
#![crate_name = "printf"]
#![crate_type = "cdylib"]
#![feature(extern_types, ffi_const, ffi_pure)]

use core::ffi::{VaList, c_char, c_void};

#[path = "../rsinclude/errno.rs"]
pub mod errno;

unsafe extern "C" {

	pub type FILE;

	#[unsafe(ffi_pure)]
	pub fn strlen(r#str: *const c_char) -> usize;

	pub safe fn fileno(stream: *mut FILE) -> i32;

	#[must_use]
	pub fn write(fd: i32, buf: *const c_void, count: usize) -> isize;
}

struct Buffer {
	inner: [u8; 512],
	index: usize,
	fd: i32,
}

impl Buffer {
	const fn new(fd: i32) -> Self {
		Self {
			inner: [0; _],
			index: 0,
			fd,
		}
	}

	fn push(&mut self, item: u8) -> bool {
		if self.index < 512 {
			self.inner[self.index] = item;
			self.index += 1;
			true
		} else {
			self.flush()
		}
	}

	fn flush(&mut self) -> bool {
		// SAFETY: `self.inner.as_ptr()` is a non-null pointer of at least 512 bytes in length.
		if unsafe { write_all(self.fd, self.inner.as_ptr(), 512) } {
			self.index = 0;
			self.inner = [0; _];
			true
		} else {
			false
		}
	}
}

unsafe fn write_all(fd: i32, ptr: *const u8, len: usize) -> bool {
	let mut completion: usize = 0;
	loop {
		let result: isize = unsafe { write(fd, ptr.add(completion).cast(), len - completion) };
		if result == -1 {
			return false;
		};
		completion += result as usize;
		if completion >= len {
			return true;
		};
	}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn printf(fmt: *const c_char, args: ...) -> i32 {
	const STDOUT_FILENO: i32 = 1;

	unsafe { dprintf(STDOUT_FILENO, fmt, args) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fprintf(stream: *mut FILE, fmt: *const c_char, args: ...) -> i32 {
	unsafe { dprintf(fileno(stream), fmt, args) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dprintf(fd: i32, fmt: *const c_char, args: ...) -> i32 {
	unsafe { __mavitix_printf(fd, fmt as *const u8, args) }
}

macro_rules! einval {
	($(,)?) => {{
		unsafe {
			use errno::__errno_location;

			*__errno_location() = errno::EINVAL;
		};
		return -1;
	}};
}

#[inline(always)]
unsafe fn __mavitix_printf(fd: i32, mut fmt: *const u8, mut args: VaList<'_>) -> i32 {
	if fmt.is_null() {
		einval!();
	};
	let mut buf: Buffer = Buffer::new(fd);
	fn next(ptr: &mut *const u8) -> Option<u8> {
		let ch: u8 = unsafe { **ptr };
		if ch == 0 {
			None
		} else {
			*ptr = unsafe { ptr.add(1) };
			Some(ch)
		}
	}
	loop {
		match next(&mut fmt) {
			Some(b'%') => {
				let Some(ch) = next(&mut fmt) else {
					einval!();
				};
				match ch {
					b'%' => {
						if !buf.push(ch) {
							return -1;
						};
						continue;
					},
					b's' => {
						if !buf.flush() {
							return -1;
						};
						let arg: *const c_char = unsafe { args.next_arg() };
						let len: usize = unsafe { strlen(arg) };
						if !unsafe { write_all(fd, arg as *const u8, len) } {
							return -1;
						};
						continue;
					},
					_ => einval!(),
				};
			},
			Some(ch) => {
				if !buf.push(ch) {
					return -1;
				};
			},
			None => {
				break;
			},
		};
	}
	if !buf.flush() {
		return -1;
	};
	return 0;
}
