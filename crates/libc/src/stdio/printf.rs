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

	pub fn atoi(r#str: *const c_char) -> i32;

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

	fn push(&mut self, item: u8) -> (bool, usize) {
		if self.index < 512 {
			self.inner[self.index] = item;
			self.index += 1;
			(true, 0)
		} else {
			let flushed: usize = self.index;
			(self.flush(), flushed)
		}
	}

	fn flush(&mut self) -> bool {
		// SAFETY: `self.inner.as_ptr()` is a non-null pointer of at least 512 (the maximum value of `self.index`) bytes in length.
		if unsafe { write_all(self.fd, self.inner.as_ptr(), self.index) } {
			self.index = 0;
			self.inner = [0; _];
			true
		} else {
			false
		}
	}
}

unsafe fn write_all(fd: i32, ptr: *const u8, len: usize) -> bool {
	let mut flushed: usize = 0;
	loop {
		let result: isize = unsafe { write(fd, ptr.add(flushed) as *const c_void, len - flushed) };
		if result == -1 {
			return false;
		};
		flushed += result as usize;
		if flushed >= len {
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
	let mut total: usize = 0;
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
	macro_rules! push {
		($ch:expr $(,)?) => {{
			let (ok, flushed): (bool, usize) = buf.push($ch);
			total += flushed;
			if !ok {
				return -1;
			};
		}};
	}
	loop {
		match next(&mut fmt) {
			Some(b'%') => {
				let Some(mut ch): Option<u8> = next(&mut fmt) else {
					einval!();
				};
				let mut _precision: i32 = 0;
				match ch {
					b'.' => {
						let mut precision_bytes: [u8; 24] = [0; _];
						let mut index: usize = 0;
						loop {
							let Some(next): Option<u8> = next(&mut fmt) else {
								einval!();
							};
							match next {
								b'0'..b'9' => {
									if index < 23 {
										precision_bytes[index] = next;
										index += 1;
									} else {
										// Too many characters in precision format.
										einval!();
									};
								},
								b'$' => {
									// UNIX `num$` syntax is too complex to support.
									einval!();
								},
								_ => {
									ch = next;
									break;
								},
							};
						}
						if index != 0 {
							_precision = unsafe { atoi(precision_bytes.as_ptr() as *const c_char) };
						};
					},
					_ => (),
				};
				match ch {
					b'%' => {
						push!(ch);
						continue;
					},
					b'n' => {
						let arg: *mut i32 = unsafe { args.next_arg() };
						if arg.is_null() {
							einval!();
						};
						unsafe { *arg = total as i32 };
						continue;
					},
					b's' | b'p' => {
						total += buf.index;
						if !buf.flush() {
							return -1;
						};
						let mut arg: *const c_char = cfg_select! {
							// Satiate miri
							any(feature = "verbose-ub-checks", miri) => unsafe {
								if ch == b's' {
									args.next_arg::<*const c_char>()
								} else {
									args.next_arg::<*const c_void>() as *const c_char
								}
							},
							_ => unsafe { args.next_arg::<*const c_char>() },
						};
						let len: usize = if arg.is_null() {
							arg = c"(null)".as_ptr();
							6
						} else {
							unsafe { strlen(arg) }
						};
						total += len;
						if !unsafe { write_all(fd, arg as *const u8, len) } {
							return -1;
						};
						continue;
					},
					_ => einval!(),
				};
			},
			Some(ch) => {
				push!(ch);
				continue;
			},
			None => break,
		};
	}
	total += buf.index;
	if !buf.flush() {
		return -1;
	};
	total as i32
}
