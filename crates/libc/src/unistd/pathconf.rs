#![no_std]
#![crate_name = "pathconf"]
#![crate_type = "cdylib"]
#![feature(ffi_const)]

include!("../rsinclude/global_asm.rs");

#[path = "../rsinclude/errno.rs"]
pub mod errno;

use core::ffi::c_char;

pub const PC_LINK_MAX: i32 = 0;
pub const PC_MAX_CANON: i32 = 1;
pub const PC_MAX_INPUT: i32 = 2;
pub const PC_NAME_MAX: i32 = 3;
pub const PC_PATH_MAX: i32 = 4;
pub const PC_PIPE_BUF: i32 = 5;
pub const PC_CHOWN_RESTRICTED: i32 = 6;
pub const PC_NO_TRUNC: i32 = 7;
pub const PC_VDISABLE: i32 = 8;
pub const PC_SYNC_IO: i32 = 9;
pub const PC_ASYNC_IO: i32 = 10;
pub const PC_PRIO_IO: i32 = 11;
pub const PC_SOCK_MAXBUF: i32 = 12;
pub const PC_FILESIZEBITS: i32 = 13;
pub const PC_REC_INCR_XFER_SIZE: i32 = 14;
pub const PC_REC_MAX_XFER_SIZE: i32 = 15;
pub const PC_REC_MIN_XFER_SIZE: i32 = 16;
pub const PC_REC_XFER_ALIGN: i32 = 17;
pub const PC_ALLOC_SIZE_MIN: i32 = 18;
pub const PC_SYMLINK_MAX: i32 = 19;
pub const PC_2_SYMLINKS: i32 = 20;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pathconf(path: *const c_char, name: i32) -> i64 {
	if path.is_null() {
		unsafe {
			use errno::__errno_location;

			*__errno_location() = errno::EINVAL;
		};
		return -1;
	};
	// SAFETY: Callers ensure `path` is a well-formed C string.
	if unsafe { *path } == 0 {
		unsafe {
			use errno::__errno_location;

			*__errno_location() = errno::ENOENT;
		};
		return -1;
	};
	return match name {
		/* See: `_POSIX_CHOWN_RESTRICTED` */
		PC_CHOWN_RESTRICTED => 0,
		/* See: `_POSIX_NO_TRUNC` */
		PC_NO_TRUNC => 1,
		/* No restrictions */
		PC_SYMLINK_MAX => -1,
		/* See: `_POSIX2_SYMLINKS` */
		PC_2_SYMLINKS => 1,
		_ => {
			unsafe {
				use errno::__errno_location;

				*__errno_location() = errno::EINVAL;
			};
			-1
		},
	};
}

symbol_alias!(pathconf, "__pathconf");
