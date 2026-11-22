// Rust doesn't have symbol aliasing (yet).
// See also:
// - https://internals.rust-lang.org/t/pre-rfc-defining-function-aliases/11424
// - https://internals.rust-lang.org/t/symbol-aliases/13297
// - https://github.com/rust-lang/compiler-team/issues/526
// - https://github.com/rust-lang/compiler-builtins/issues/70
#[allow(unused_macros)]
macro_rules! symbol_alias {
	($name:ident, $alias:expr $(,)?) => {
		core::arch::global_asm! {
			".pushsection .text",
			concat!(".global ", $alias),
			concat!(".type ", $alias, ", @function"),
			concat!(".equiv ", $alias, ", ", stringify!($name)),
			".popsection",
			options(raw),
		}
	};
}
