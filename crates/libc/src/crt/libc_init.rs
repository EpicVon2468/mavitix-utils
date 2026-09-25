#![no_std]
#![crate_name = "libc_init"]
#![crate_type = "cdylib"]
#![feature(likely_unlikely, linkage)]

use core::{
	ffi::{c_char, c_void},
	hint::likely as __likely,
};

include!("../rsinclude/elf.rs");
include!("../rsinclude/ld_syms.rs");
include!("../rsinclude/rpmalloc.rs");

unsafe extern "C" {

	pub static mut environ: *mut *mut c_char;

	pub safe fn __mavitix_libc__stdio_init() -> i32;
	pub safe fn __mavitix_libc__stdio_fini() -> i32;

	pub fn __libc_fini(status: i32, fini: *mut c_void);

	pub safe fn on_exit(
		r#fn: *mut unsafe extern "C" fn(status: i32, usr_ptr: *mut c_void),
		usr_ptr: *mut c_void,
	) -> i32;
	pub safe fn atexit(r#fn: *mut unsafe extern "C" fn()) -> i32;
}

static mut MAVITIX_EXECFN: *const c_char = core::ptr::null_mut();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __mavitix_getexecfn() -> *const c_char {
	return unsafe { MAVITIX_EXECFN };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __mavitix_libc_init(
	_argc: i32,
	_argv: *mut *mut c_char,
	envp: *mut *mut c_char,
	fini: *mut ld_syms::fini,
	rtld_fini: *mut ld_syms::fini,
) -> i32 {
	unsafe {
		environ = envp;
	};
	let mut size: usize = 0;
	while !unsafe { envp.add(size) }.is_null() {
		size += 1;
	}
	let raw_auxv: *const elf::auxv = unsafe { envp.add(size + 1) } as *const elf::auxv;
	size = 0;
	let mut page_size: usize = 0;
	loop {
		let entry: elf::auxv = unsafe { *raw_auxv.add(size) };
		match entry.a_type as usize {
			elf::AT_EXECFN => {
				unsafe {
					MAVITIX_EXECFN = entry.a_un.a_ptr as *const c_char;
				};
			},
			elf::AT_PAGESZ => page_size = unsafe { entry.a_un.a_val } as usize,
			elf::AT_SECURE => {},
			elf::AT_NULL => break,
			_ => (),
		};
		size += 1;
	}
	let mut config: rpmalloc::Config = rpmalloc::Config {
		page_size,
		enable_huge_pages: 0,
		enable_thp: 0,
		disable_decommit: 0,
		page_name: const { "mavitix-libc-rpmalloc-page\0".as_ptr().cast() },
		huge_page_name: const { "mavitix-libc-rpmalloc-huge-page\0".as_ptr().cast() },
		unmap_on_finalise: 0,
		disable_thp: 0,
	};
	rpmalloc::rpmalloc_initialise_config(core::ptr::null_mut(), &raw mut config);
	if __mavitix_libc__stdio_init() == 0 {
		return false as i32;
	};

	if __likely(!rtld_fini.is_null()) {
		// "a function pointer that the application should register with atexit"
		atexit(rtld_fini);
	};
	if __likely(!fini.is_null()) {
		on_exit(__libc_fini as _, fini as *mut c_void);
	};

	return true as i32;
}

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
