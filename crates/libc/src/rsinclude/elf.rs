pub mod elf {
	use core::ffi::c_void;

	pub const AT_NULL: usize = 0;
	pub const AT_IGNORE: usize = 1;
	pub const AT_EXECFD: usize = 2;
	pub const AT_PHDR: usize = 3;
	pub const AT_PHENT: usize = 4;
	pub const AT_PHNUM: usize = 5;
	pub const AT_PAGESZ: usize = 6;
	pub const AT_BASE: usize = 7;
	pub const AT_FLAGS: usize = 8;
	pub const AT_ENTRY: usize = 9;
	pub const AT_NOTELF: usize = 10;
	pub const AT_UID: usize = 11;
	pub const AT_EUID: usize = 12;
	pub const AT_GID: usize = 13;
	pub const AT_EGID: usize = 14;
	pub const AT_PLATFORM: usize = 15;
	pub const AT_HWCAP: usize = 16;
	pub const AT_CLKTCK: usize = 17;
	pub const AT_SECURE: usize = 23;
	pub const AT_BASE_PLATFORM: usize = 24;
	pub const AT_RANDOM: usize = 25;
	pub const AT_HWCAP2: usize = 27;
	pub const AT_EXECFN: usize = 31;

	#[repr(C)]
	#[derive(Copy, Clone)]
	pub struct auxv {
		pub a_type: i32,
		pub a_un: a_un,
	}

	#[repr(C)]
	#[derive(Copy, Clone)]
	pub union a_un {
		pub a_val: i64,
		pub a_ptr: *mut c_void,
		pub a_fnc: *mut unsafe extern "C" fn(),
	}
}
