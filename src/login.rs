use std::ffi::c_char;

// SAFETY: The function declarations given below are in line with the header files of `libc`.
#[link(name = "c")]
unsafe extern "C" {

	pub fn getlogin() -> *mut c_char;
}
