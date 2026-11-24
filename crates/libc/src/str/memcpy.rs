#![no_std]
#![crate_name = "memcpy"]
#![crate_type = "cdylib"]

#[unsafe(no_mangle)]
#[expect(suspicious_runtime_symbol_definitions)]
pub const unsafe extern "C" fn memcpy(dst: *mut u8, src: *const u8, mut len: usize) -> *mut u8 {
	loop {
		if len == 0 {
			return dst;
		};
		len -= 1;
		unsafe { *(dst.add(len)) = *(src.add(len)) };
	}
}
