#![feature(
	const_trait_impl,
	const_convert,
	const_default,
	error_reporter,
	ffi_pure,
	byte_str
)]
#![no_main]

use std::{error::Report, ffi::c_char, hint::cold_path, slice};

use cli::{ArgIter, CLI};
use errors::{CLIError, MainError};

pub mod cli;
pub mod detection;
pub mod errors;

macro_rules! err {
	($error:expr $(,)?) => {
		return Err($error)?
	};
}

macro_rules! try_next_as_operand {
	($args:expr, $else_case:expr $(,)?) => {
		match $args.peek() {
			// SAFETY: If `peek()` was `Some(_)`, then there will be a value when `next()` is called.
			Some(&[b'-', ..]) => unsafe { $args.next().unwrap_unchecked() },
			_ => $else_case,
		}
	};
}

macro_rules! short_arg_required_val {
	($args:expr, $rest:expr; $arg_name:expr $(,)?) => {
		match $rest {
			&[] => &*try_next_as_operand!($args, {
				err!(CLIError::ExpectedOperand {
					arg_name: $arg_name,
				});
			}),
			&[b'=', ..] => {
				// SANITY(unusual):
				// Not using the `..` pattern matched above, as that would only yield `&[u8]`, not `&str`.
				// SAFETY:
				// The pattern match arm validates this slice.
				let value: &str = unsafe { $rest.get_unchecked(1..) };
				if value.is_empty() {
					err!(CLIError::ExpectedOperand {
						arg_name: $arg_name,
					});
				};
				value
			},
			_ => $rest,
		}
	};
}

macro_rules! short_arg_no_val {
	($rest:expr; $arg_name:expr $(,)?) => {
		let &[]: &[u8] = $rest else {
			err!(CLIError::UnexpectedOperand {
				arg_name: $arg_name,
				actual: $rest,
			});
		};
	};
}

macro_rules! long_arg_no_val {
	($operand:expr; $arg_name:expr $(,)?) => {
		if let Some(actual) = $operand {
			err!(CLIError::UnexpectedOperand {
				arg_name: $arg_name,
				actual,
			});
		};
	};
}

macro_rules! long_arg_required_val {
	($args:expr, $operand:expr; $arg_name:expr $(,)?) => {
		match $operand {
			Some(inner) => inner,
			None => &*try_next_as_operand!($args, {
				err!(CLIError::ExpectedOperand {
					arg_name: $arg_name,
				});
			}),
		}
	};
}

macro_rules! long_arg_optional_val {
	($operand:expr; $(,)?) => {
		match $operand {
			Some(inner) => Some(inner),
			None => 'try_next: {
				Some(&*try_next_as_operand!(break 'try_next None))
			},
		}
	};
}

#[cfg(miri)]
#[unsafe(no_mangle)]
fn miri_start(argc: isize, argv: *const *const u8) -> isize {
	unsafe { main(argc as i32, argv as *const *const c_char) as isize }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: i32, argv: *const *const c_char) -> i32 {
	if let Err(error) = main_impl(unsafe { ArgIter::new(argc, argv) }) {
		let report: Report<MainError> = Report::from(error).pretty(true);
		println!("Error: {report}");
		return 1;
	};
	return 0;
}

pub fn main_impl(mut args: ArgIter) -> Result<(), MainError> {
	let mut cli: CLI = CLI::default();
	while let Some(arg) = args.next() {
		if args.seen_double_dash || arg[0] != b'-' {
			cli.file = Some(
				// SANITY(unusual):
				// I would just use `arg.as_ptr()` directly but Miri throws a fit even though I know better.
				// SAFETY:
				// `arg` and `*args.argv.sub(1)` are one and the same; `arg.len()` doesn't include the trailing nul byte, so it is safe to add one.
				unsafe { slice::from_raw_parts(*args.argv.sub(1), arg.len() + 1) }
					.to_owned()
					.leak(),
			);
			break;
		};
		let &[b'-', second, ref rest @ ..]: &[u8] = arg else {
			err!(CLIError::UnrecognisedOption { arg })
		};
		match second {
			b'V' => {
				short_arg_no_val!(rest; "-V");
				println!(env!("CARGO_PKG_VERSION"));
				return Ok(());
			},
			b'h' => {
				short_arg_no_val!(rest; "-h");
				show_help();
				return Ok(());
			},
			b'-' => {
				if rest.is_empty() {
					// SANITY(unusual): This is true at most once.
					cold_path();
					args.seen_double_dash = true;
					continue;
				};
				let (prefix, operand): (&[u8], Option<&[u8]>) = match rest.split_once(
					#[inline(always)]
					|ch: &u8| *ch == b'=',
				) {
					Some((prefix, operand)) => (prefix, Some(operand)),
					None => (rest, None),
				};
				match prefix {
					b"version" => {
						long_arg_no_val!(operand; "--version");
						println!(concat!(
							env!("CARGO_PKG_NAME"),
							' ',
							env!("CARGO_PKG_VERSION"),
						));
						return Ok(());
					},
					b"help" => {
						long_arg_no_val!(operand; "--help");
						show_help();
						return Ok(());
					},
					_ => err!(CLIError::UnrecognisedOption { arg }),
				};
			},
			_ => err!(CLIError::UnrecognisedOption { arg }),
		};
	}
	let (argc, argv): (i32, *const *const c_char) = (
		args.argc + 1,
		// SAFETY:
		unsafe { args.argv.sub(1) } as *const *const c_char,
	);
	dbg!(&cli);
	return Ok(());
}

fn show_help() {
	macro_rules! feature {
		($name:expr, $detect:path $(,)?) => {{
			use detection::*;

			println!(concat!("[{}] ", $name), if $detect { '*' } else { ' ' });
		}};
	}
	#[cfg(target_arch = "x86_64")]
	{
		feature!("x86-64-v4", x86_64::x86_64_v4);
		feature!("x86-64-v3", x86_64::x86_64_v3);
		feature!("x86-64-v2", x86_64::x86_64_v2);
		feature!("x86-64-v1", x86_64::x86_64_v1);
	};
	#[cfg(target_arch = "aarch64")]
	{
		feature!("ARMv9.5-A", aarch64::ARMv9_5_A);
		feature!("ARMv9.4-A", aarch64::ARMv9_4_A);
		feature!("ARMv9.3-A", aarch64::ARMv9_3_A);
		feature!("ARMv9.2-A", aarch64::ARMv9_2_A);
		feature!("ARMv9.1-A", aarch64::ARMv9_1_A);
		feature!("ARMv9.0-A", aarch64::ARMv9_0_A);
		feature!("ARMv8.9-A", aarch64::ARMv8_9_A);
		feature!("ARMv8.8-A", aarch64::ARMv8_8_A);
		feature!("ARMv8.7-A", aarch64::ARMv8_7_A);
		feature!("ARMv8.6-A", aarch64::ARMv8_6_A);
		feature!("ARMv8.5-A", aarch64::ARMv8_5_A);
		feature!("ARMv8.4-A", aarch64::ARMv8_4_A);
		feature!("ARMv8.3-A", aarch64::ARMv8_3_A);
		feature!("ARMv8.2-A", aarch64::ARMv8_2_A);
		feature!("ARMv8.1-A", aarch64::ARMv8_1_A);
		feature!("ARMv8.0-A", aarch64::ARMv8_0_A);
	};
}
