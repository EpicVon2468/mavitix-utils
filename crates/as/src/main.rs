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
	cli::{Arch, ArgIter, CLI, DwarfVersion, Endian, OptLevel, Verbosity, arg::Arg as _},
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
					&[] => &*try_next_as_operand!({
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
		macro_rules! short_arg_no_val {
			($arg_name:expr $(,)?) => {
				let "" = rest else {
					err!(CLIError::UnexpectedOperand {
						arg_name: $arg_name,
						actual: rest.to_owned(),
					});
				};
			};
		}
		match second {
			'V' => {
				short_arg_no_val!("-V");
				const_println!(env!("CARGO_PKG_VERSION"));
				return Ok(());
			},
			'g' => {
				short_arg_no_val!("-g");
				let _ = cli.debug_info();
			},
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
				macro_rules! long_arg_no_val {
					($arg_name:expr $(,)?) => {
						if let Some(actual) = operand {
							err!(CLIError::UnexpectedOperand {
								arg_name: $arg_name,
								actual: actual.to_owned(),
							});
						};
					};
				}
				macro_rules! long_arg_required_val {
					($arg_ty:ty $(,)?) => {
						match operand {
							Some(inner) => inner,
							None => &*try_next_as_operand!({
								err!(CLIError::ExpectedOperand {
									arg_name: const { <$arg_ty>::LONG_NAME.unwrap() },
									possible_operands: <$arg_ty>::POSSIBLE_OPERANDS,
								});
							}),
						}
					};
				}
				macro_rules! long_arg_optional_val {
					($(,)?) => {
						match operand {
							Some(inner) => Some(inner),
							None => 'try_next: {
								Some(&*try_next_as_operand!(break 'try_next None))
							},
						}
					};
				}
				match prefix {
					"version" => {
						long_arg_no_val!("--version");
						const_println!(concat!(
							env!("CARGO_PKG_NAME"),
							' ',
							env!("CARGO_PKG_VERSION"),
						));
						return Ok(());
					},
					"verbose" => {
						long_arg_no_val!("--verbose");
						cli.verbosity = Verbosity::Verbose;
					},
					"silent" => {
						long_arg_no_val!("--silent");
						cli.verbosity = Verbosity::Silent;
					},
					"debug" => {
						let input: Option<&str> = long_arg_optional_val!();
						if cli::arg::parse_boolish(input, "--debug")? {
							let _ = cli.debug_info();
						} else {
							cli.debug_info = None;
						};
					},
					"debug-embed-source" => {
						let input: Option<&str> = long_arg_optional_val!();
						cli.debug_info().embed_source =
							cli::arg::parse_boolish(input, "--debug-embed-source")?;
					},
					"debug-record-args" => {
						let input: Option<&str> = long_arg_optional_val!();
						cli.debug_info().record_args =
							cli::arg::parse_boolish(input, "--debug-record-args")?;
					},
					"debug-dwarf-version" => {
						let input: &str = long_arg_required_val!(DwarfVersion);
						cli.debug_info().dwarf_version = cli::arg::try_parse_long(input)?;
					},
					"debug-dwarf64" => {
						long_arg_no_val!("--debug-dwarf64");
						cli.debug_info().dwarf64 = true;
					},
					"debug-dwarf32" => {
						long_arg_no_val!("--debug-dwarf32");
						cli.debug_info().dwarf64 = false;
					},
					"opt-level" => {
						let input: &str = long_arg_required_val!(OptLevel);
						cli.opt_level = cli::arg::try_parse_long(input)?;
					},
					"arch" => {
						let input: &str = long_arg_required_val!(Arch);
						cli.arch = cli::arg::try_parse_long(input)?;
					},
					"endian" => {
						let input: &str = long_arg_required_val!(Endian);
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
							Some(inner) => Some(inner.to_owned()),
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
	backend::write(&cli);
	Ok(())
}
