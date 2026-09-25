pub mod ld_syms {

	#[allow(non_camel_case_types)]
	pub type init = unsafe extern "C" fn();
	#[allow(non_camel_case_types)]
	pub type fini = unsafe extern "C" fn();

	// FIXME:
	// error[E0791]: invalid type for variable with `#[linkage]` attribute
	// warning: the `link_section` attribute cannot be used on foreign statics
	unsafe extern "C" {

		// #[linkage = "weak"]
		// #[unsafe(link_section = ".preinit_array")]
		// pub(crate) safe static __preinit_array_start: [*const init; 0];
		// #[linkage = "weak"]
		// #[unsafe(link_section = ".preinit_array")]
		// pub(crate) safe static __preinit_array_end: [*const init; 0];

		// #[linkage = "weak"]
		// #[unsafe(link_section = ".init_array")]
		// pub(crate) safe static __init_array_start: [*const init; 0];
		// #[linkage = "weak"]
		// #[unsafe(link_section = ".init_array")]
		// pub(crate) safe static __init_array_end: [*const init; 0];

		// #[linkage = "weak"]
		// #[unsafe(link_section = ".fini_array")]
		// pub(crate) safe static __fini_array_start: [*const fini; 0];
		// #[linkage = "weak"]
		// #[unsafe(link_section = ".fini_array")]
		// pub(crate) safe static __fini_array_end: [*const fini; 0];
	}
}
