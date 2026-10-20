#![feature(
	const_trait_impl,
	derive_const,
	const_convert,
	const_option_ops,
	const_cmp,
	const_default
)]

use std::{
	ffi::OsStr,
	hint::cold_path,
	os::unix::ffi::OsStrExt as _,
	path::Path,
	str::{Chars, FromStr as _},
};

use anyhow::Result;

use mavitix_utils::{const_println, main};

use crate::cli::{ArgIter, CLI, OptLevel, Target};

pub mod backend;
pub mod cli;

main!(main_impl());

pub fn main_impl() -> Result<()> {
	let mut cli: CLI = CLI::default();
	let mut args: ArgIter = ArgIter::new();
	while let Some(arg) = args.next() {
		if args.seen_double_dash || arg.as_bytes()[0] != b'-' {
			cli.files.push({
				// SANITY(unusual):
				// Bypass indirection in conversion.
				// Less work for the optimiser, and less potential overhead.
				Path::new(OsStr::from_bytes(arg.into_bytes().leak()))
			});
			continue;
		};
		if arg.len() == 1 {
			cold_path();
			cli.read_from_stdin = true;
			continue;
		};
		let (second, rest): (char, &str) = {
			// SAFETY:
			// The first check of the loop will catch any case where the first byte is not '-'.
			// Thus, the length of the first character is known to be one byte at this point, and we can slice past.
			let mut chars: Chars = unsafe { arg.get_unchecked(1..) }.chars();
			(
				// SAFETY: The above checks make it impossible for this to fail.
				unsafe { chars.next().unwrap_unchecked() },
				chars.as_str(),
			)
		};
		macro_rules! try_next_as_operand {
			($(,)?) => {
				match args.peek() {
					// SANITY(unusual): Have to use a borrow instead of the `to_str()` method due to `Drop` issues.
					// SAFETY: If `peek()` was `Some(_)`, then there will be a value when `next()` is called.
					Some(next) if next.as_bytes()[0] != b'-' => &unsafe { args.next().unwrap_unchecked() },
					_ => panic!(/* missing operand for option */),
				}
			};
		}
		macro_rules! short_arg_required_val {
			($(,)?) => {
				if rest.is_empty() {
					try_next_as_operand!()
				} else if let b'=' = rest.as_bytes()[0] {
					let value: &str = unsafe { rest.get_unchecked(1..) };
					if value.is_empty() {
						panic!(/* missing operand for option */);
					};
					value
				} else {
					rest
				}
			};
		}
		match second {
			'V' => {
				const_println!(env!("CARGO_PKG_VERSION"));
				return Ok(());
			},
			'g' => cli.debug_info = true,
			'v' => cli.verbose = true,
			'O' => {
				let input: &str = short_arg_required_val!();
				let Ok(value): Result<OptLevel, _> = OptLevel::from_str(input) else {
					panic!(/* invalid operand */);
				};
				cli.opt_level = value;
			},
			'-' => {
				if rest.is_empty() {
					// SANITY(unusual): This is true at most once.
					cold_path();
					args.seen_double_dash = true;
					continue;
				};
				let (prefix, value): (&str, Option<&str>) = match rest.split_once('=') {
					None => (rest, None),
					Some((prefix, value)) => (prefix, Some(value)),
				};
				macro_rules! no_value_allowed {
					($(,)?) => {
						if let Some(_) = value {
							panic!(/* unexpected operand for flag-type option */);
						};
					};
				}
				macro_rules! get_raw_value {
					($(,)?) => {
						match value {
							Some(inner) => inner,
							None => try_next_as_operand!(),
						}
					};
				}
				match prefix {
					"version" => {
						const_println!(concat!(
							env!("CARGO_PKG_NAME"),
							' ',
							env!("CARGO_PKG_VERSION"),
						));
						return Ok(());
					},
					"gen-debug" => {
						no_value_allowed!();
						cli.debug_info = true;
					},
					"verbose" => {
						no_value_allowed!();
						cli.verbose = true;
					},
					"warn" => {
						no_value_allowed!();
						cli.warn = true;
					},
					"no-warn" => {
						no_value_allowed!();
						cli.warn = false;
					},
					"divide" | "64" => (),
					"target" => {
						let input: &str = get_raw_value!();
						let Ok(value): Result<Target, _> = Target::from_str(input) else {
							panic!(/* unknown or invalid target */);
						};
						cli.target = value;
					},
					_ => panic!(/* unknown arg */),
				};
			},
			'm' if !rest.is_empty() => {
				match rest {
					"naked-reg" | "syntax=intel" | "mnemonic=intel" | "intel64" | "abi=lp64" => (),
					_ => panic!(/* unknown arg */),
				};
			},
			_ => panic!(/* unknown arg */),
		};
		// match arg {
		// 	"--version" => {
		// 		const_println!(concat!(
		// 			env!("CARGO_BIN_NAME"),
		// 			' ',
		// 			env!("CARGO_PKG_VERSION"),
		// 		));
		// 		return Ok(());
		// 	},
		// 	"-V" => {
		// 		const_println!(env!("CARGO_PKG_VERSION"));
		// 		return Ok(());
		// 	},
		// 	"-g" | "--gen-debug" => cli.debug_info = true,
		// 	"-v" | "--verbose" => cli.verbose = true,
		// 	"--warn" => cli.warn = true,
		// 	"--no-warn" => cli.warn = false,
		// 	"--" => args.seen_double_dash = true,
		// 	"-mnaked-reg" | "-msyntax=intel" | "-mmnemonic=intel" | "-mintel64" | "--divide"
		// 	| "--64" | "-mabi=lp64" => (),
		// 	_ => panic!(),
		// };
	}
	dbg!(&cli);
	Ok(())
}
