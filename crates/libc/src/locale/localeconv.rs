#![no_std]
#![crate_name = "localeconv"]
#![crate_type = "cdylib"]
#![feature(const_trait_impl, const_default, thread_local)]

use core::ffi::c_char;

include!("../rsinclude/global_asm.rs");

#[path = "../rsinclude/locale.rs"]
pub mod locale;

use locale::{__mavitix_get_lc_monetary, __mavitix_get_lc_numeric, Territory};

// https://lh.2xlibre.net/locale/en_AU/
// https://lh.2xlibre.net/locale/en_GB/
// https://lh.2xlibre.net/locale/ja_JP/
// https://lh.2xlibre.net/locale/en_NZ/
// https://lh.2xlibre.net/locale/de_DE/
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lconv {
	pub decimal_point: *const c_char,
	pub thousands_sep: *const c_char,
	pub grouping: *const c_char,
	pub int_curr_symbol: *const c_char,
	pub currency_symbol: *const c_char,
	pub mon_decimal_point: *const c_char,
	pub mon_thousands_sep: *const c_char,
	pub mon_grouping: *const c_char,
	pub positive_sign: *const c_char,
	pub negative_sign: *const c_char,
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
pub extern "C" fn localeconv() -> *const lconv {
	#[thread_local]
	static mut LCONV: lconv = lconv {
		decimal_point: c".".as_ptr(),
		thousands_sep: c"".as_ptr(),
		grouping: c"".as_ptr(),
		int_curr_symbol: c"".as_ptr(),
		currency_symbol: c"".as_ptr(),
		mon_decimal_point: c"".as_ptr(),
		mon_thousands_sep: c"".as_ptr(),
		mon_grouping: c"".as_ptr(),
		positive_sign: c"".as_ptr(),
		negative_sign: c"".as_ptr(),
		int_frac_digits: c_char::MAX,
		frac_digits: c_char::MAX,
		p_cs_precedes: c_char::MAX,
		p_sep_by_space: c_char::MAX,
		n_cs_precedes: c_char::MAX,
		n_sep_by_space: c_char::MAX,
		p_sign_posn: c_char::MAX,
		n_sign_posn: c_char::MAX,
		int_p_cs_precedes: c_char::MAX,
		int_p_sep_by_space: c_char::MAX,
		int_n_cs_precedes: c_char::MAX,
		int_n_sep_by_space: c_char::MAX,
		int_p_sign_posn: c_char::MAX,
		int_n_sign_posn: c_char::MAX,
	};
	unsafe {
		let territory: Territory = __mavitix_get_lc_numeric().territory;
		LCONV.decimal_point = match territory {
			Territory::AU | Territory::GB | Territory::JP | Territory::NZ => c".",
			Territory::DE => c",",
			Territory::AQ | Territory::None => c"",
		}
		.as_ptr();
		LCONV.thousands_sep = match territory {
			Territory::AU | Territory::GB | Territory::JP | Territory::NZ => c",",
			Territory::DE => c".",
			Territory::AQ | Territory::None => c"",
		}
		.as_ptr();
		LCONV.grouping = match territory {
			Territory::AU | Territory::DE | Territory::GB | Territory::NZ => c"3;3",
			Territory::JP => c"3",
			Territory::AQ | Territory::None => c"",
		}
		.as_ptr();
	};
	unsafe {
		let territory: Territory = __mavitix_get_lc_monetary().territory;
		LCONV.int_curr_symbol = match territory {
			Territory::AU => c"AUD ",
			Territory::DE => c"EUR ",
			Territory::GB => c"GBP ",
			Territory::JP => c"JPY ",
			Territory::NZ => c"NZD ",
			Territory::AQ | Territory::None => c"",
		}
		.as_ptr();
		LCONV.currency_symbol = match territory {
			Territory::AU | Territory::NZ => c"$",
			Territory::DE => c"€",
			Territory::GB => c"£",
			Territory::JP => c"￥",
			Territory::AQ | Territory::None => c"",
		}
		.as_ptr();
		LCONV.mon_decimal_point = match territory {
			Territory::AU | Territory::GB | Territory::JP | Territory::NZ => c".",
			Territory::DE => c",",
			Territory::AQ | Territory::None => c"",
		}
		.as_ptr();
		LCONV.mon_thousands_sep = match territory {
			Territory::AU | Territory::GB | Territory::JP | Territory::NZ => c",",
			Territory::DE => c".",
			Territory::AQ | Territory::None => c"",
		}
		.as_ptr();
		LCONV.mon_grouping = match territory {
			Territory::AU | Territory::DE | Territory::GB | Territory::NZ => c"3;3",
			Territory::JP => c"3",
			Territory::AQ | Territory::None => c"",
		}
		.as_ptr();
		LCONV.positive_sign = match territory {
			Territory::AQ
			| Territory::AU
			| Territory::DE
			| Territory::GB
			| Territory::JP
			| Territory::NZ
			| Territory::None => c"",
		}
		.as_ptr();
		LCONV.negative_sign = match territory {
			Territory::AU | Territory::DE | Territory::GB | Territory::JP | Territory::NZ => c"-",
			Territory::AQ | Territory::None => c"",
		}
		.as_ptr();
		LCONV.int_frac_digits = match territory {
			Territory::AU | Territory::DE | Territory::GB | Territory::NZ => b'2' as c_char,
			Territory::JP => b'0' as c_char,
			Territory::AQ | Territory::None => c_char::MAX,
		};
		LCONV.frac_digits = match territory {
			Territory::AU | Territory::DE | Territory::GB | Territory::NZ => b'2' as c_char,
			Territory::JP => b'0' as c_char,
			Territory::AQ | Territory::None => c_char::MAX,
		};
		LCONV.p_cs_precedes = match territory {
			Territory::AU | Territory::GB | Territory::JP | Territory::NZ => b'1' as c_char,
			Territory::DE => b'0' as c_char,
			Territory::AQ | Territory::None => c_char::MAX,
		};
		LCONV.p_sep_by_space = match territory {
			Territory::AU | Territory::GB | Territory::JP | Territory::NZ => b'0' as c_char,
			Territory::DE => b'1' as c_char,
			Territory::AQ | Territory::None => c_char::MAX,
		};
		LCONV.n_cs_precedes = match territory {
			Territory::AU | Territory::GB | Territory::JP | Territory::NZ => b'1' as c_char,
			Territory::DE => b'0' as c_char,
			Territory::AQ | Territory::None => c_char::MAX,
		};
		LCONV.n_sep_by_space = match territory {
			Territory::AU | Territory::GB | Territory::JP | Territory::NZ => b'0' as c_char,
			Territory::DE => b'1' as c_char,
			Territory::AQ | Territory::None => c_char::MAX,
		};
		LCONV.p_sign_posn = match territory {
			Territory::AU | Territory::DE | Territory::GB | Territory::NZ => b'1' as c_char,
			Territory::JP => b'4' as c_char,
			Territory::AQ | Territory::None => c_char::MAX,
		};
		LCONV.n_sign_posn = match territory {
			Territory::AU | Territory::DE | Territory::GB | Territory::NZ => b'1' as c_char,
			Territory::JP => b'4' as c_char,
			Territory::AQ | Territory::None => c_char::MAX,
		};
	};
	&raw const LCONV
}

symbol_alias!(localeconv, "__localeconv");
