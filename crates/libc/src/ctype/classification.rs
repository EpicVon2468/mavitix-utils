#![no_std]
#![crate_name = "classification"]
#![crate_type = "cdylib"]
#![allow(arithmetic_overflow)]

// A majority of the checks in this file are based upon musl-libc's implementation(s).
// You can view their licence at https://git.musl-libc.org/cgit/musl/tree/COPYRIGHT.
// At the time of writing, the notice was:
// ----------------------------------------------------------------------
// Copyright © 2005-2020 Rich Felker, et al.
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
// TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
// SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
// ----------------------------------------------------------------------

/// A more efficient range check (apparently).
///
/// This macro (ab)uses `i32 -> u32` overflow & `u32` subtraction underflow
/// to emit one subtraction & one comparison instead of two comparisions.
///
/// Values less than 0x0 (0) ('\0') will underflow once cast to `u32`, meaning the `<` check will not return true for them.
///
/// Values less than `$start` will underflow once `$start` is subtracted, meaning the `<` check will not return true for them.
///
/// Thus, only elements within the range `$start..=$end` will pass the check (in theory).
macro_rules! char_range_check {
	($ch:expr; $start:expr, $end:expr $(,)?) => {
		(($ch as u32 - $start as u32) < const { $end as u32 - $start as u32 + 1 })
	};
}

include!("../rsinclude/global_asm.rs");

/// Exports symbol aliases `{__,}${name}_l` for `$name` using raw assembly.
///
/// As these are symbol aliases, the values in code should be identical, thus meaning no overhead from having to jump.
macro_rules! alias_locale {
	($name:ident $(,)?) => {
		alias_locale!($name, "");
		alias_locale!($name, "__");
	};
	($name:ident, $prefix:expr $(,)?) => {
		// SANITY(unusual):
		// No need to actually specify the `locale` parameter or the `locale_t` type.
		// When the original function is lowered to assembly, even if `locale` is passed, it will be safely ignored.
		symbol_alias!($name, concat!($prefix, stringify!($name), "_l"));
	};
}

pub const ASCII_CAPITAL_OFFSET: u32 = 32;

#[unsafe(no_mangle)]
pub extern "C" fn isalnum(ch: i32) -> i32 {
	(isalpha(ch) != 0 || isdigit(ch) != 0) as i32
}

alias_locale!(isalnum);

#[unsafe(no_mangle)]
pub extern "C" fn isalpha(ch: i32) -> i32 {
	// TODO: Can this be similarly optimised to `isxdigit` ?
	(islower(ch) != 0 || isupper(ch) != 0) as i32
}

alias_locale!(isalpha);

#[unsafe(no_mangle)]
pub extern "C" fn isblank(ch: i32) -> i32 {
	(ch == (' ' as u32 as i32) || ch == ('\t' as u32 as i32)) as i32
}

alias_locale!(isblank);

/* iscntrl */

#[unsafe(no_mangle)]
pub extern "C" fn isdigit(ch: i32) -> i32 {
	char_range_check!(ch; '0', '9') as i32
}

alias_locale!(isdigit);

/* isgraph */

#[unsafe(no_mangle)]
pub extern "C" fn islower(ch: i32) -> i32 {
	char_range_check!(ch; 'a', 'z') as i32
}

alias_locale!(islower);

/* isprint */

/* ispunct */

#[unsafe(no_mangle)]
pub extern "C" fn isspace(ch: i32) -> i32 {
	(ch == (' ' as u32 as i32) || char_range_check!(ch; '\t', '\r')) as i32
}

alias_locale!(isspace);

#[unsafe(no_mangle)]
pub extern "C" fn isupper(ch: i32) -> i32 {
	char_range_check!(ch; 'A', 'Z') as i32
}

alias_locale!(isupper);

#[unsafe(no_mangle)]
pub extern "C" fn isxdigit(ch: i32) -> i32 {
	// SANITY(unusual):
	// Manually cast first so that the overflow is still enforced even with the bitor.
	// Use a bitor of `32` as that is the offset between uppercase and lowercase ASCII characters.
	// I'm not entirely sure how the maths & binary stuff works out, but I did some testing in Rust playground and this seems to work.
	(isdigit(ch) != 0 || char_range_check!((ch as u32) | ASCII_CAPITAL_OFFSET; 'a', 'f')) as i32
}

alias_locale!(isxdigit);

#[unsafe(no_mangle)]
pub extern "C" fn isascii(ch: i32) -> i32 {
	(ch < 0x80) as i32
}

alias_locale!(isascii);
