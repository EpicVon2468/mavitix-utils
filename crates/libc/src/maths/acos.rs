#![no_std]
#![crate_name = "acos"]
#![crate_type = "cdylib"]

extern crate libm;

#[unsafe(no_mangle)]
pub fn acos(value: f64) -> f64 {
	libm::acos(value)
}
