#![no_std]
#![crate_name = "libc_init"]
#![crate_type = "cdylib"]
#![feature(likely_unlikely)]

use core::{
	ffi::{c_char, c_void},
	hint::{cold_path, likely as __likely},
};

#[path = "../rsinclude/elf.rs"]
pub mod elf;
#[path = "../rsinclude/ld_syms.rs"]
pub mod ld_syms;
#[path = "../rsinclude/rpmalloc.rs"]
pub mod rpmalloc;

unsafe extern "C" {

	pub static mut environ: *mut *mut c_char;

	pub safe fn __mavitix_libc__stdio_init() -> i32;
	pub safe fn __mavitix_libc__stdio_fini() -> i32;

	pub fn __libc_fini(status: i32, fini: *mut c_void);

	pub fn on_exit(
		r#fn: *mut unsafe extern "C" fn(status: i32, usr_ptr: *mut c_void),
		usr_ptr: *mut c_void,
	) -> i32;
	pub fn atexit(r#fn: *mut unsafe extern "C" fn()) -> i32;
}

// FIXME:
// The use of literally anything from `core` (even just pointer stuff) is adding huge amounts of `core`'s source code into the output library.
// Entire files worth of redundant code, despite only using one or two functions.
// Found a related issue at https://github.com/rust-lang/rust/issues/56068.

static mut MAVITIX_EXECFN: *const c_char = core::ptr::null_mut();

#[unsafe(no_mangle)]
#[allow(nonstandard_style)]
pub unsafe extern "C" fn __mavitix_getexecfn() -> *const c_char {
	// SAFETY: This field is only written to once, when only one thread exists (see: `__mavitix_libc_init`).
	return unsafe { MAVITIX_EXECFN };
}

#[unsafe(no_mangle)]
#[allow(nonstandard_style)]
pub unsafe extern "C" fn __mavitix_libc_init(
	_argc: i32,
	_argv: *mut *mut c_char,
	envp: *mut *mut c_char,
	fini: *mut ld_syms::fini,
	rtld_fini: *mut ld_syms::fini,
) -> i32 {
	// SAFETY:
	// Problem(s):
	// - Writing to a mutable static may cause undefined behaviour across threads.
	// Excuse(s):
	// - Callers ensure that no other threads are created at the time of calling.
	unsafe {
		environ = envp;
	};
	let mut size: usize = 0;
	// SAFETY: Callers ensure `envp` is well-formed.
	while !unsafe { *envp.add(size) }.is_null() {
		size += 1;
	}
	// SAFETY: Callers ensure `envp` is well-formed.
	let auxv: *const elf::auxv = unsafe { envp.add(size + 1) } as *const elf::auxv;
	size = 0;
	let mut page_size: usize = 0;
	loop {
		let entry: elf::auxv = unsafe { *auxv.add(size) };
		match entry.a_type as usize {
			elf::AT_EXECFN => {
				// SAFETY:
				// Problem(s):
				// - Writing to a mutable static may cause undefined behaviour across threads.
				// - Accessing a field of a union may cause undefined behaviour.
				// Excuse(s):
				// - Callers ensure that no other threads are created at the time of calling.
				// - `AT_EXECFN` is guaranteed by spec to be initialised with the `a_ptr` field.
				unsafe {
					MAVITIX_EXECFN = entry.a_un.a_ptr as *const c_char;
				};
			},
			elf::AT_PAGESZ => {
				// SAFETY:
				// Problem(s):
				// - Accessing a field of a union may cause undefined behaviour.
				// Excuse(s):
				// - `AT_PAGESZ` is guaranteed by spec to be initalised with the `a_val` field.
				page_size = unsafe { entry.a_un.a_val } as usize;
			},
			elf::AT_SECURE => {},
			elf::AT_NULL => break,
			_ => (),
		};
		size += 1;
	}
	let mut config: rpmalloc::Config = rpmalloc::Config {
		page_size,
		// TODO: Is there a way to easily query huge pages & transparent huge pages?
		enable_huge_pages: 0,
		enable_thp: 0,
		disable_decommit: 0,
		page_name: const { c"mavitix-libc-rpmalloc-page".as_ptr() },
		huge_page_name: const { c"mavitix-libc-rpmalloc-huge-page".as_ptr() },
		unmap_on_finalise: 0,
		disable_thp: 0,
	};
	// SAFETY: `&raw mut config` is a well-formed pointer to a valid struct.
	if unsafe { rpmalloc::initialise_config(core::ptr::null_mut(), &raw mut config) } == 0 {
		return false as i32;
	};
	if __mavitix_libc__stdio_init() == 0 {
		return false as i32;
	};

	if __likely(!rtld_fini.is_null()) {
		// SAFETY: "a function pointer that the application should register with atexit"
		if unsafe { atexit(rtld_fini) } == 0 {
			cold_path();
			return false as i32;
		};
	};
	if __likely(!fini.is_null()) {
		// SAFETY:
		// `__libc_fini as _` is a valid function pointer matching the function prototype.
		// `fini` is a well-formed function pointer which lasts the length of the program.
		if unsafe { on_exit(__libc_fini as _, fini as *mut c_void) } == 0 {
			cold_path();
			return false as i32;
		};
	};

	return true as i32;
}

// #[unsafe(no_mangle)]
// #[allow(nonstandard_style)]
// pub unsafe extern "C" fn __libc_init(init: *const ld_syms::init) {
// 	let mut index: usize;

// 	{
// 		let size: usize = {
// 			ld_syms::__preinit_array_end.as_ptr().addr()
// 				- ld_syms::__preinit_array_start.as_ptr().addr()
// 		};
// 		index = 0;
// 		while index < size {
// 			#[cfg(feature = "verbose-ub-checks")]
// 			{
// 				let fn_ptr: *const ld_syms::init = ld_syms::__preinit_array_start[index];
// 				if __likely(!fn_ptr.is_null()) {
// 					// SAFETY:
// 					// Problem(s):
// 					// - Dereferencing a null ptr is Undefined Behaviour.
// 					// - Elements within the preinit array are ptrs to functions.
// 					// - Dereferencing such a ptr if it is null would thus be UB.
// 					// Excuse(s):
// 					// - We perform a null check before dereference.
// 					unsafe {
// 						(*fn_ptr)();
// 					};
// 				};
// 			};
// 			#[cfg(not(feature = "verbose-ub-checks"))]
// 			{
// 				// SAFETY:
// 				// Problem(s):
// 				// - Dereferencing a null ptr is Undefined Behaviour.
// 				// - Elements within the preinit array are ptrs to functions.
// 				// - Dereferencing such a ptr if it is null would thus be UB.
// 				// Excuse(s):
// 				// - The linker takes responsibility to guarantee the array is well-formed.
// 				unsafe {
// 					(*ld_syms::__preinit_array_start[index])();
// 				};
// 			};
// 			index += 1;
// 		}
// 	};

// 	if __likely(!init.is_null()) {
// 		// SAFETY:
// 		// Problem(s):
// 		// - Dereferencing a null ptr is Undefined Behaviour.
// 		// - The `init` parameter is not guaranteed to be non-null.
// 		// - Dereferencing `init` if it is null would thus be UB.
// 		// Excuse(s):
// 		// - We perform a null check before dereference.
// 		unsafe {
// 			(*init)();
// 		};
// 	};

// 	{
// 		let size: usize = {
// 			ld_syms::__init_array_end.as_ptr().addr() - ld_syms::__init_array_start.as_ptr().addr()
// 		};
// 		index = 0;
// 		while index < size {
// 			#[cfg(feature = "verbose-ub-checks")]
// 			{
// 				let fn_ptr: *const ld_syms::init = ld_syms::__init_array_start[index];
// 				if __likely(!fn_ptr.is_null()) {
// 					// SAFETY:
// 					// Problem(s):
// 					// - Dereferencing a null ptr is Undefined Behaviour.
// 					// - Elements within the init array are ptrs to functions.
// 					// - Dereferencing such a ptr if it is null would thus be UB.
// 					// Excuse(s):
// 					// - We perform a null check before dereference.
// 					unsafe {
// 						(*fn_ptr)();
// 					};
// 				};
// 			};
// 			#[cfg(not(feature = "verbose-ub-checks"))]
// 			{
// 				// SAFETY:
// 				// Problem(s):
// 				// - Dereferencing a null ptr is Undefined Behaviour.
// 				// - Elements within the init array are ptrs to functions.
// 				// - Dereferencing such a ptr if it is null would thus be UB.
// 				// Excuse(s):
// 				// - The linker takes responsibility to guarantee the array is well-formed.
// 				unsafe {
// 					(*ld_syms::__init_array_start[index])();
// 				};
// 			};
// 			index += 1;
// 		}
// 	};
// }

// #[unsafe(no_mangle)]
// #[allow(nonstandard_style)]
// unsafe extern "C" fn __libc_fini(_: i32, fini: *mut c_void) {}
