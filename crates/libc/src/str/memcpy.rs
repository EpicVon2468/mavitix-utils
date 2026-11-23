#![no_std]
#![crate_name = "memcpy"]
#![crate_type = "cdylib"]
#![feature(likely_unlikely)]

use core::hint::unlikely as __unlikely;

#[unsafe(no_mangle)]
#[expect(suspicious_runtime_symbol_definitions)]
pub unsafe extern "C" fn memcpy(dst: *mut u8, src: *const u8, mut len: usize) -> *mut u8 {
	loop {
		unsafe { *(dst.add(len)) = *(src.add(len)) };
		if __unlikely(len == 0) {
			return dst;
		};
		len -= 1;
	}
}
