#![no_std]
#![crate_name = "memset"]
#![crate_type = "cdylib"]

#[unsafe(no_mangle)]
#[expect(suspicious_runtime_symbol_definitions)]
pub const unsafe extern "C" fn memset(dst: *mut u8, value: i32, mut len: usize) -> *mut u8 {
	loop {
		if len == 0 {
			return dst;
		};
		len -= 1;
		unsafe { *(dst.add(len)) = value as u8 };
	}
}
