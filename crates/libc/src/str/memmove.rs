#![no_std]
#![crate_name = "memmove"]
#![crate_type = "cdylib"]
#![expect(suspicious_runtime_symbol_definitions)]

unsafe extern "C" {

	pub fn memcpy(dst: *mut u8, src: *const u8, len: usize) -> *mut u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memmove(dst: *mut u8, src: *const u8, len: usize) -> *mut u8 {
	if src.addr() <= dst.addr() {
		return unsafe { memcpy(dst, src, len) };
	};
	let mut offset: usize = 0;
	while offset < len {
		unsafe { *(dst.add(len)) = *(src.add(len)) };
		offset += 1;
	}
	dst
}
