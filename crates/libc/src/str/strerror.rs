#![no_std]
#![crate_name = "strerror"]
#![crate_type = "cdylib"]
#![feature(const_trait_impl, const_default, ffi_const)]

use core::ffi::c_char;

include!("../rsinclude/global_asm.rs");

#[path = "../rsinclude/errno.rs"]
pub mod errno;
#[path = "../rsinclude/locale.rs"]
pub mod locale;

use locale::{__mavitix_get_lc_messages, Language, Locale};

#[unsafe(no_mangle)]
pub const unsafe extern "C" fn strerror_l(errnum: i32, locale: *const Locale) -> *const c_char {
	match unsafe { *locale }.language {
		Language::English => strerror_en(errnum),
		Language::Japanese => strerror_ja(errnum),
	}
}

symbol_alias!(strerror_l, "__strerror_l");

#[unsafe(no_mangle)]
pub extern "C" fn strerror(errnum: i32) -> *const c_char {
	let tmp: Locale = __mavitix_get_lc_messages();
	unsafe { strerror_l(errnum, &raw const tmp) }
}

#[inline]
pub const fn strerror_en(errnum: i32) -> *const c_char {
	return match errnum {
		errno::EPERM => c"Operation not permitted",
		errno::ENOENT => c"No such file or directory",
		errno::ESRCH => c"No such process",
		errno::EINTR => c"Interrupted system call",
		errno::EIO => c"I/O error",
		errno::ENXIO => c"No such device or address",
		errno::E2BIG => c"Argument list too long",
		errno::ENOEXEC => c"Exec format error",
		errno::EBADF => c"Bad file number",
		errno::ECHILD => c"No child processes",
		errno::EAGAIN => c"Try again",
		errno::ENOMEM => c"Out of memory",
		errno::EACCES => c"Permission denied",
		errno::EFAULT => c"Bad address",
		errno::ENOTBLK => c"Block device required",
		errno::EBUSY => c"Device or resource busy",
		errno::EEXIST => c"File exists",
		errno::EXDEV => c"Cross-device link",
		errno::ENODEV => c"No such device",
		errno::ENOTDIR => c"Not a directory",
		errno::EISDIR => c"Is a directory",
		errno::EINVAL => c"Invalid argument",
		errno::ENFILE => c"File table overflow",
		errno::EMFILE => c"Too many open files",
		errno::ENOTTY => c"Not a typewriter",
		errno::ETXTBSY => c"Text file busy",
		errno::EFBIG => c"File too large",
		errno::ENOSPC => c"No space left on device",
		errno::ESPIPE => c"Illegal seek",
		errno::EROFS => c"Read-only file system",
		errno::EMLINK => c"Too many links",
		errno::EPIPE => c"Broken pipe",
		errno::EDOM => c"Maths argument out of domain of function",
		errno::ERANGE => c"Exceeded range",
		errno::EDEADLK => c"Resource deadlock would occur",
		errno::ENAMETOOLONG => c"File name too long",
		errno::ENOLCK => c"No record locks available",
		errno::ENOSYS => c"Invalid system call number",
		errno::ENOTEMPTY => c"Directory not empty",
		errno::ELOOP => c"Too many symbolic links encountered",
		errno::EREMOTEIO => c"Remote I/O error",
		_ => c"Unknown error code",
	}
	.as_ptr();
}

#[inline]
pub const fn strerror_ja(errnum: i32) -> *const c_char {
	return match errnum {
		errno::EIO => c"入出力エラー",
		// Not 100% certain about the end of this one.
		errno::EEXIST => c"ファイルあるです",
		errno::ENOTTY => c"タイプライターではないです",
		errno::EREMOTEIO => c"リモート入出力エラー",
		_ => c"未知エラーコードです",
	}
	.as_ptr();
}
