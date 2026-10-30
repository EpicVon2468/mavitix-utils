#![feature(
	const_trait_impl,
	derive_const,
	const_convert,
	const_option_ops,
	const_cmp,
	const_default,
	const_clone
)]

use std::{
	ffi::OsStr,
	hint::cold_path,
	os::unix::ffi::OsStrExt as _,
	path::Path,
	process::ExitCode,
	str::Chars,
};

use mavitix_utils::const_println;

use crate::{
	cli::{Arch, ArgIter, CLI, Endian, OptLevel, arg::Arg as _},
	errors::{CLIError, MainError},
};

pub mod backend;
pub mod cli;
pub mod errors;
pub mod frontend;
pub mod fuzzy;

pub fn main() -> ExitCode {
	match main_impl() {
		Ok(()) => ExitCode::SUCCESS,
		Err(error) => {
			match error {
				MainError::CLI(error) => {
					let display: String = format!("\u{001B}[1;91merror\u{001B}[22;39m: {error}")
						.replace(
							"\u{0002}help\u{0003}",
							"  \u{001B}[96m|\u{001B}[39m \u{001B}[1;35mhelp\u{001B}[22;39m:",
						);
					eprintln!("{display}");
				},
			};
			eprintln!(concat!(
				'\n',
				env!("CARGO_BIN_NAME"),
				": a fatal error occurred, aborting",
			));
			ExitCode::FAILURE
		},
	}
}

macro_rules! err {
	($error:expr $(,)?) => {
		return Err($error)?
	};
}

pub fn main_impl() -> Result<(), MainError> {
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
				// SAFETY: Always more than a length of 1 here.
				unsafe { chars.next().unwrap_unchecked() },
				chars.as_str(),
			)
		};
		macro_rules! try_next_as_operand {
			($else_case:expr $(,)?) => {
				match args.peek() {
					// SAFETY: If `peek()` was `Some(_)`, then there will be a value when `next()` is called.
					Some(next) if next.as_bytes()[0] != b'-' => unsafe { args.next().unwrap_unchecked() },
					_ => $else_case,
				}
			};
		}
		macro_rules! short_arg_required_val {
			($arg_ty:ty $(,)?) => {
				match rest.as_bytes() {
					&[] => &try_next_as_operand!({
						err!(CLIError::ExpectedOperand {
							arg_name: const { <$arg_ty>::SHORT_NAME.unwrap() },
							possible_operands: <$arg_ty>::POSSIBLE_OPERANDS,
						});
					}),
					&[b'=', ..] => {
						// SANITY(unusual):
						// Not using the `..` pattern matched above, as that would only yield `&[u8]`, not `&str`.
						// SAFETY:
						// The pattern match arm validates this slice.
						let value: &str = unsafe { rest.get_unchecked(1..) };
						if value.is_empty() {
							err!(CLIError::ExpectedOperand {
								arg_name: const { <$arg_ty>::SHORT_NAME.unwrap() },
								possible_operands: <$arg_ty>::POSSIBLE_OPERANDS,
							});
						};
						value
					},
					_ => rest,
				}
			};
		}
		match second {
			'V' => {
				const_println!(env!("CARGO_PKG_VERSION"));
				return Ok(());
			},
			'v' => cli.verbose = true,
			'g' => cli.debug_info = true,
			'O' => {
				let input: &str = short_arg_required_val!(OptLevel);
				cli.opt_level = cli::arg::try_parse_short(input)?;
			},
			'-' => {
				if rest.is_empty() {
					// SANITY(unusual): This is true at most once.
					cold_path();
					args.seen_double_dash = true;
					continue;
				};
				let (prefix, operand): (&str, Option<&str>) = match rest.split_once('=') {
					None => (rest, None),
					Some((prefix, operand)) => (prefix, Some(operand)),
				};
				macro_rules! no_value_allowed {
					($arg_name:literal $(,)?) => {
						if let Some(actual) = operand {
							err!(CLIError::UnexpectedOperand {
								arg_name: $arg_name,
								actual: actual.to_owned(),
							});
						};
					};
				}
				macro_rules! get_raw_value {
					($arg_ty:ty $(,)?) => {
						match operand {
							Some(inner) => inner,
							None => &try_next_as_operand!({
								err!(CLIError::ExpectedOperand {
									arg_name: const { <$arg_ty>::LONG_NAME.unwrap() },
									possible_operands: <$arg_ty>::POSSIBLE_OPERANDS,
								});
							}),
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
					"verbose" => {
						no_value_allowed!("--verbose");
						cli.verbose = true;
					},
					"debug" => {
						no_value_allowed!("--debug");
						cli.debug_info = true;
					},
					"debug-embed-source" => {
						no_value_allowed!("--debug-embed-source");
						cli.debug_embed_source = true;
					},
					"opt-level" => {
						let input: &str = get_raw_value!(OptLevel);
						cli.opt_level = cli::arg::try_parse_long(input)?;
					},
					// TODO: --warnings={all,none,etc.}
					"arch" => {
						let input: &str = get_raw_value!(Arch);
						cli.arch = cli::arg::try_parse_long(input)?;
					},
					"endian" => {
						let input: &str = get_raw_value!(Endian);
						cli.endian = cli::arg::try_parse_long(input)?;
					},
					_ => err!(CLIError::UnrecognisedOption {
						arg: {
							let mut dest: String = String::with_capacity(prefix.len() + 2);
							dest.push_str("--");
							dest.push_str(prefix);
							dest
						},
						value: match operand {
							Some(operand) => Some(operand.to_owned()),
							None => 'try_next: {
								Some(try_next_as_operand!(break 'try_next None))
							},
						},
					}),
				};
			},
			_ => err!(CLIError::UnrecognisedOption { arg, value: None }),
		};
	}
	dbg!(&cli);
	Ok(())
}
