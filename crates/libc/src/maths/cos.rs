#![no_std]
#![crate_name = "cos"]
#![crate_type = "cdylib"]

extern crate libm;

#[unsafe(no_mangle)]
pub fn cos(value: f64) -> f64 {
	libm::cos(value)
}
