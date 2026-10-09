#![no_std]
#![crate_name = "classification"]
#![crate_type = "cdylib"]
#![allow(arithmetic_overflow)]

/// A more efficient range check (I think?).
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
		(($ch as u32 - $start as u32) < ($end as u32 - $start as u32 + 1))
	};
}

pub const ASCII_CAPITAL_OFFSET: u32 = 32;

#[unsafe(no_mangle)]
pub extern "C" fn isalnum(ch: i32) -> i32 {
	(isalpha(ch) != 0 || isdigit(ch) != 0) as i32
}

#[unsafe(no_mangle)]
pub extern "C" fn isalpha(ch: i32) -> i32 {
	// TODO: Can this be similarly optimised to `isxdigit` ?
	(islower(ch) != 0 || isupper(ch) != 0) as i32
}

#[unsafe(no_mangle)]
pub extern "C" fn isblank(ch: i32) -> i32 {
	(ch == (' ' as u32 as i32) || ch == ('\t' as u32 as i32)) as i32
}

/* iscntrl */

#[unsafe(no_mangle)]
pub extern "C" fn isdigit(ch: i32) -> i32 {
	char_range_check!(ch; '0', '9') as i32
}

/* isgraph */

#[unsafe(no_mangle)]
pub extern "C" fn islower(ch: i32) -> i32 {
	char_range_check!(ch; 'a', 'z') as i32
}

/* isprint */

/* ispunct */

/* isspace */

#[unsafe(no_mangle)]
pub extern "C" fn isupper(ch: i32) -> i32 {
	char_range_check!(ch; 'A', 'Z') as i32
}

#[unsafe(no_mangle)]
pub extern "C" fn isxdigit(ch: i32) -> i32 {
	// SANITY(unusual):
	// Manually cast first so that the overflow is still enforced even with the bitor.
	// Use a bitor of `32` as that is the offset between uppercase and lowercase ASCII characters.
	// I'm not entirely sure how the maths & binary stuff works out, but I did some testing in Rust playground and this seems to work.
	(isdigit(ch) != 0 || char_range_check!((ch as u32) | ASCII_CAPITAL_OFFSET; 'a', 'f')) as i32
}
