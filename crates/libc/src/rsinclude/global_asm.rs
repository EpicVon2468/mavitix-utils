#[allow(unused_macros)]
macro_rules! asm_prelude {
	($name:expr; $($instruction:expr),* $(,)?) => {
		core::arch::global_asm! {
			".pushsection .text",
			concat!(".global ", $name),
			concat!(".type ", $name, ", @function"),
			concat!($name, ":"),
			#[cfg(target_arch = "x86_64")]
			"endbr64",
			$($instruction,)*
			"ret",
			".popsection",
			options(raw),
		}
	};
}

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

#[allow(unused_macros)]
macro_rules! asm_prelude_syscall {
	($name:expr; $value:expr $(,)?) => {
		asm_prelude! {
			$name;
			concat!(
				"mov ",
				cfg_select! {
					target_arch = "x86_64" => "rax",
					target_arch = "aarch64" => "x8",
					target_arch = "riscv64" => "a7",
				},
				", ",
				$value,
			),
			cfg_select! {
				target_arch = "x86_64" => "syscall",
				target_arch = "aarch64" => "svc #0",
				target_arch = "riscv64" => "ecall",
				_ => compile_error!(),
			},
		}
	};
}
