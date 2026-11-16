#![no_std]
#![crate_name = "strerror"]
#![crate_type = "cdylib"]
#![feature(ffi_const)]

use core::ffi::c_char;

#[path = "../rsinclude/errno.rs"]
pub mod errno;

#[unsafe(no_mangle)]
pub const extern "C" fn strerror(errnum: i32) -> *const c_char {
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
		errno::ERANGE => c"Maths result not representable",
		_ => c"Unknown error",
	}
	.as_ptr();
}
