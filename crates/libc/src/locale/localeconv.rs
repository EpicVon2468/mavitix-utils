#![no_std]
#![crate_name = "localeconv"]
#![crate_type = "cdylib"]

use core::ffi::c_char;

include!("../rsinclude/global_asm.rs");

#[repr(C)]
pub struct lconv {
	pub decimal_point: *mut c_char,
	pub thousands_sep: *mut c_char,
	pub grouping: *mut c_char,
	pub int_curr_symbol: *mut c_char,
	pub currency_symbol: *mut c_char,
	pub mon_decimal_point: *mut c_char,
	pub mon_thousands_sep: *mut c_char,
	pub mon_grouping: *mut c_char,
	pub positive_sign: *mut c_char,
	pub negative_sign: *mut c_char,
	pub int_frac_digits: c_char,
	pub frac_digits: c_char,
	pub p_cs_precedes: c_char,
	pub p_sep_by_space: c_char,
	pub n_cs_precedes: c_char,
	pub n_sep_by_space: c_char,
	pub p_sign_posn: c_char,
	pub n_sign_posn: c_char,
	pub int_p_cs_precedes: c_char,
	pub int_p_sep_by_space: c_char,
	pub int_n_cs_precedes: c_char,
	pub int_n_sep_by_space: c_char,
	pub int_p_sign_posn: c_char,
	pub int_n_sign_posn: c_char,
}

#[unsafe(no_mangle)]
pub extern "C" fn localeconv() -> *mut lconv {
	todo!();
}

symbol_alias!(localeconv, "__localeconv");
