use std::{
	hint::{assert_unchecked, cold_path},
	str::Chars,
};

use anyhow::Result;

use mavitix_utils::{const_println, main};

use crate::cli::{ArgIter, CLI};

pub mod cli;

main!(main_impl());

pub fn main_impl() -> Result<()> {
	let mut cli: CLI = CLI::default();
	let mut args: ArgIter = ArgIter::new();
	while let Some(arg) = args.next() {
		if args.seen_double_dash || arg.as_bytes()[0] != b'-' {
			cli.files.push(arg);
			continue;
		};
		let (first, second, rest): (char, char, &str) = {
			let mut chars: Chars = arg.chars();
			(chars.next().unwrap(), chars.next().unwrap(), chars.as_str())
		};
		cfg_select! {
			debug_assertions => {
				assert!(first == '-');
			},
			not(debug_assertions) => {
				// SAFETY:
				// The operand catch at the start of the loop handles args which
				// do not start with '-', so `first` is always '-' here.
				unsafe {
					assert_unchecked(first == '-');
				};
			},
		};
		match second {
			'V' => {
				const_println!(env!("CARGO_PKG_VERSION"));
				return Ok(());
			},
			'g' => cli.debug_info = true,
			'v' => cli.verbose = true,
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
							None => match args.peek() {
								Some(next) if next.as_bytes()[0] != b'-' => unsafe { args.next().unwrap_unchecked() }.as_str(),
								_ => panic!(/* missing operand for option */),
							},
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
	Ok(())
}
