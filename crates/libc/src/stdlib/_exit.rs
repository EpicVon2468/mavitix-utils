#![no_std]
#![crate_name = "_exit"]
#![crate_type = "cdylib"]

include!("../rsinclude/global_asm.rs");

unsafe extern "C" {

	pub safe fn __mavitix_immediate_abort(code: i32) -> !;
}

#[unsafe(no_mangle)]
pub extern "C" fn _exit() -> ! {
	// TODO: shutdown stuff here first
	__mavitix_immediate_abort(0);
}

symbol_alias!(_exit, "_Exit");
