use std::{ffi::c_char, slice};

#[derive(Debug, PartialEq, Eq)]
pub struct CLI {
	pub file: Option<&'static [u8]>,
	pub argv0: Option<&'static [u8]>,
}

const impl Default for CLI {
	fn default() -> Self {
		Self {
			file: None,
			argv0: None,
		}
	}
}

#[derive(Debug)]
pub struct ArgIter {
	pub argc: i32,
	pub argv: *const *const u8,
	peeked: Option<&'static [u8]>,
	pub seen_double_dash: bool,
}

impl ArgIter {
	pub const unsafe fn new(argc: i32, argv: *const *const c_char) -> Self {
		Self {
			argc: argc - 1,
			argv: unsafe { argv.add(1) } as *const *const u8,
			peeked: None,
			seen_double_dash: false,
		}
	}

	#[inline]
	#[must_use]
	pub fn peek(&mut self) -> Option<<Self as Iterator>::Item> {
		if let None = self.peeked {
			self.peeked = self.next();
		};
		self.peeked
	}
}

impl Iterator for ArgIter {
	type Item = &'static [u8];

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		if self.argc == 0 {
			return None;
		};
		match self.peeked {
			None => {
				unsafe extern "C" {

					#[unsafe(ffi_pure)]
					fn strlen(r#str: *const c_char) -> usize;
				}

				let value: *const u8 = unsafe { *self.argv };
				self.argc -= 1;
				self.argv = unsafe { self.argv.add(1) };
				let len: usize = unsafe { strlen(value as *const c_char) };
				Some(unsafe { slice::from_raw_parts(value, len) })
			},
			_ => self.peeked.take(),
		}
	}
}
