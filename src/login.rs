use std::ffi::c_char;

// SAFETY: The function declarations given below are in line with the header files of `libc`.
#[link(name = "c")]
unsafe extern "C" {

	pub safe fn getlogin() -> *mut c_char;
}
