use std::ffi::{CStr, CString, c_char};

// This seems to be accurate for all my targets.
// It's more favourable than taking a dependency on the `libc` crate.
#[allow(nonstandard_style)]
pub type uid_t = u32;
#[allow(nonstandard_style)]
pub type gid_t = u32;

#[repr(C)]
pub struct passwd {
	/// username
	pub pw_name: *mut c_char,
	/// user password
	pub pw_passwd: *mut c_char,
	/// user ID
	pub pw_uid: uid_t,
	/// group ID
	pub pw_gid: gid_t,
	/// user information
	pub pw_gecos: *mut c_char,
	/// home dir
	pub pw_dir: *mut c_char,
	/// shell program
	pub pw_shell: *mut c_char,
}

// SAFETY: The function declarations given below are in line with the header files of `libc`.
#[link(name = "c")]
unsafe extern "C" {

	/// `geteuid()` – get user identity.
	///
	/// Returns the effective user ID of the calling process.
	///
	/// # Library
	///
	/// Source(s):
	///
	/// - C Standard Library (`libc`).
	///
	/// Standard(s):
	///
	/// - [POSIX.1-2024].
	///
	/// Declaration:
	///
	/// ```
	/// #include <unistd.h>
	///
	/// uid_t geteuid(void);
	/// ```
	///
	/// # Safety
	///
	/// This function is guaranteed to be unconditionally safe.<br>
	/// It is unreasonable to expect that undefined, unsafe, or erroneous behaviour may occur inside this function.
	///
	/// # Errors
	///
	/// The `geteuid()` function shall not modify <u>`errno`</u>.
	///
	/// # Returns
	///
	/// The `geteuid()` function shall return the effective user ID of the calling process.
	///
	/// The `geteuid()` function shall always be successful and no return value is reserved to indicate an error.
	///
	/// # See Also
	///
	/// **getuid**(2), **getresuid**(2)], **setreuid**(2), **setuid**(2), **credentials**(7), [POSIX.1], [POSIX.1-2024]
	///
	/// [POSIX.1-2024]: https://pubs.opengroup.org/onlinepubs/9799919799.2024edition/functions/geteuid.html
	/// [POSIX.1]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/geteuid.html
	pub safe fn geteuid() -> u32;

	/// `getpwuid()` – get password file entry.
	///
	/// # Library
	///
	/// Source(s):
	///
	/// - C Standard Library (`libc`).
	///
	/// Standard(s):
	///
	/// - [POSIX.1-2024]
	///
	/// Declaration:
	///
	/// ```
	/// #include <sys/types.h>
	/// #include <pwd.h>
	///
	/// struct passwd *getpwuid(uid_t uid);
	/// ```
	///
	/// # Safety
	///
	/// # Errors
	///
	/// # Returns
	///
	/// # See Also
	///
	/// **getpwnam**(3), **getpwuid**(3), **endpwent**(3), **fgetpwent**(3), **getgrnam**(3), **getpw**(3), **getpwent**(3), **getspnam**(3), **putpwent**(3), **setpwent**(3), **nsswitch.conf**(5), **passwd**(5), [POSIX.1], [POSIX.1-2024]
	///
	/// [POSIX.1-2024]: https://pubs.opengroup.org/onlinepubs/9799919799.2024edition/functions/getpwuid.html
	/// [POSIX.1]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/getpwuid.html
	pub fn getpwuid(uid: uid_t) -> *mut passwd;
}

pub fn get_passwd() -> Option<*mut passwd> {
	let uid: uid_t = geteuid();
	// SAFETY:
	let passwd: *mut passwd = unsafe { getpwuid(uid) };
	if passwd.is_null() { None } else { Some(passwd) }
}

pub fn get_username() -> Option<String> {
	let passwd: *mut passwd = match get_passwd() {
		Some(value) => value,
		None => return None,
	};
	// SAFETY: `get_passwd()` returns `None` if `passwd` is `NULL`, therefore reading is not UB.
	let passwd: passwd = unsafe { passwd.read() };
	let pw_name: *mut c_char = passwd.pw_name;
	if pw_name.is_null() {
		return None;
	};
	// SAFETY: `passwd->pw_name` is generally well-formed.
	let user: CString = crate::cstr_clone(unsafe { CStr::from_ptr(passwd.pw_name) });
	match user.into_string() {
		Ok(value) => Some(value),
		Err(_) => None,
	}
}
