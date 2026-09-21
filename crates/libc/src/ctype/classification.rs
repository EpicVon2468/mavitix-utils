#![no_std]
#![crate_name = "classification"]
#![crate_type = "cdylib"]
#![allow(arithmetic_overflow)]

/// A more efficient range check (I think?).
///
/// This macro (ab)uses `i32 -> u32` overflow & `u32` subtraction underflow
/// to one subtraction & one comparison instead of two comparisions.
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

#[unsafe(no_mangle)]
pub extern "C" fn isalnum(ch: i32) -> i32 {
	(isalpha(ch) != 0 || isdigit(ch) != 0) as i32
}

#[unsafe(no_mangle)]
pub extern "C" fn isalpha(ch: i32) -> i32 {
	(islower(ch) != 0 || isupper(ch) != 0) as i32
}

#[unsafe(no_mangle)]
pub extern "C" fn isblank(ch: i32) -> i32 {
	(ch == ' ' as u32 as i32 || ch == '\t' as u32 as i32) as i32
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
	// TODO:
	// I vaguely remember some stuff about lowercase & uppercase characters being a fixed
	// offset apart from each other in ASCII; Can that be used to optimise this further?
	(isdigit(ch) != 0 || char_range_check!(ch; 'a', 'f') || char_range_check!(ch; 'A', 'F')) as i32
}
