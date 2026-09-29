pub mod rpmalloc {
	use core::ffi::{c_char, c_void};

	#[repr(C)]
	#[derive(Copy, Clone)]
	pub struct Config {
		pub page_size: usize,
		pub enable_huge_pages: i32,
		pub enable_thp: i32,
		pub disable_decommit: i32,
		pub page_name: *const c_char,
		pub huge_page_name: *const c_char,
		pub unmap_on_finalise: i32,
		pub disable_thp: i32,
	}

	#[repr(C)]
	#[derive(Copy, Clone)]
	pub struct Interface {
		pub memory_map: *mut unsafe extern "C" fn(
			size: usize,
			alignment: usize,
			offset: *mut usize,
			mapped_size: *mut usize,
		) -> *mut c_void,
		pub memory_commit: *mut unsafe extern "C" fn(address: *mut c_void, size: usize) -> i32,
		pub memory_decommit: *mut unsafe extern "C" fn(address: *mut c_void, size: usize) -> i32,
		pub memory_unmap:
			*mut unsafe extern "C" fn(address: *mut c_void, offset: usize, mapped_size: usize),
		pub map_fail_callback: *mut unsafe extern "C" fn(size: usize) -> i32,
		pub error_callback: *mut unsafe extern "C" fn(message: *const c_char),
	}

	unsafe extern "C" {

		#[link_name = "rpmalloc_initialize"]
		pub safe fn rpmalloc_initialise(memory_interface: *mut Interface) -> i32;

		#[link_name = "rpmalloc_initialize_config"]
		pub safe fn rpmalloc_initialise_config(
			memory_interface: *mut Interface,
			config: *mut Config,
		) -> i32;

		pub safe fn rpmalloc_config() -> *const Config;

		#[link_name = "rpmalloc_finalize"]
		pub safe fn rpmalloc_finalise();

		#[link_name = "rpmalloc_thread_initialize"]
		pub safe fn rpmalloc_thread_initialise();

		#[link_name = "rpmalloc_thread_finalize"]
		pub safe fn rpmalloc_thread_finalise();

		pub safe fn rpmalloc_thread_collect() -> i32;

		#[link_name = "rpmalloc_is_thread_initialized"]
		pub safe fn rpmalloc_is_thread_initialised() -> i32;
	}

	unsafe extern "C" {

		#[link_name = "rpmalloc"]
		pub fn malloc(size: usize) -> *mut c_void;

		#[link_name = "rpzalloc"]
		pub fn zalloc(size: usize) -> *mut c_void;

		#[link_name = "rpcalloc"]
		pub fn calloc(num: usize, size: usize) -> *mut c_void;

		#[link_name = "rprealloc"]
		pub fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;

		#[link_name = "rpaligned_alloc"]
		pub fn aligned_alloc(align: usize, size: usize) -> *mut c_void;

		#[link_name = "rpaligned_zalloc"]
		pub fn aligned_zalloc(align: usize, size: usize) -> *mut c_void;

		#[link_name = "rpaligned_calloc"]
		pub fn aligned_calloc(align: usize, num: usize, size: usize) -> *mut c_void;

		#[link_name = "rpaligned_realloc"]
		pub fn aligned_realloc(
			ptr: *mut c_void,
			align: usize,
			size: usize,
			oldsize: usize,
			flags: u32,
		) -> *mut c_void;

		#[link_name = "rpmemalign"]
		pub fn memalign(align: usize, size: usize) -> *mut c_void;

		#[link_name = "rpposix_memalign"]
		pub fn posix_memalign(memptr: *mut *mut c_void, align: usize, size: usize) -> i32;

		#[link_name = "rpfree"]
		pub fn free(ptr: *mut c_void);

		#[link_name = "rpmalloc_usable_size"]
		pub fn malloc_usable_size(ptr: *mut c_void) -> usize;
	}
}
