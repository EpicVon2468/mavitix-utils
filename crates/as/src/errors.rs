use std::{
	error::Error,
	fmt::{Display, Formatter, Result as FmtResult},
	hint::cold_path,
};

use crate::{
	cli::{ALL_OPTIONS, arg::Arg},
	fuzzy,
};

#[must_use]
#[derive(Debug)]
pub enum MainError {
	CLI(CLIError),
}

const impl From<CLIError> for MainError {
	#[inline]
	fn from(value: CLIError) -> Self {
		Self::CLI(value)
	}
}

impl Error for MainError {
	fn cause(&self) -> Option<&dyn Error> {
		match *self {
			Self::CLI(ref inner) => Some(inner),
		}
	}
}

impl Display for MainError {
	fn fmt(&self, _: &mut Formatter<'_>) -> FmtResult {
		unimplemented!("MainError should not be shown via std::fmt::Display")
	}
}

#[must_use]
#[derive(Debug)]
pub enum CLIError {
	ExpectedOperand {
		arg_name: &'static str,
		possible_operands: &'static [&'static str],
	},
	UnexpectedOperand {
		arg_name: &'static str,
		actual: String,
	},
	InvalidOperand {
		arg_name: &'static str,
		possible_operands: &'static [&'static str],
		suggestion: Option<&'static str>,
		actual: String,
	},
	UnrecognisedOption {
		arg: String,
		value: Option<String>,
	},
}

impl CLIError {
	#[inline]
	pub fn invalid_operand<T: const Arg>(actual: String, is_long: bool) -> Self {
		Self::InvalidOperand {
			arg_name: match if is_long { T::LONG_NAME } else { T::SHORT_NAME } {
				Some(arg_name) => arg_name,
				None => unreachable!(),
			},
			possible_operands: T::POSSIBLE_OPERANDS,
			suggestion: T::suggest_from(&actual),
			actual,
		}
	}
}

impl Error for CLIError {}

#[inline]
fn format_possible(fmt: &mut Formatter<'_>, possible_values: &[&str], join: &str) -> FmtResult {
	let len: usize = possible_values.len();
	for index in 0..len {
		let value: &str = possible_values[index];
		if index + 1 == len {
			cold_path();
			write!(fmt, "{join} '{value}'")?;
		} else {
			write!(fmt, "'{value}', ")?;
		};
	}
	Ok(())
}

impl Display for CLIError {
	fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
		match *self {
			Self::ExpectedOperand {
				arg_name,
				possible_operands,
			} => {
				writeln!(fmt, "expected operand for option '{arg_name}'")?;
				write!(fmt, "\u{0002}help\u{0003} expected one of: ")?;
				format_possible(fmt, possible_operands, "or")?;
				Ok(())
			},
			Self::UnexpectedOperand {
				arg_name,
				ref actual,
			} => write!(
				fmt,
				"unexpected operand '{}' for option '{arg_name}'",
				actual.escape_debug(),
			),
			Self::InvalidOperand {
				arg_name,
				possible_operands,
				suggestion,
				ref actual,
			} => {
				writeln!(fmt, "invalid operand for option '{arg_name}'")?;
				writeln!(
					fmt,
					"\u{0002}help\u{0003} invalid operand was '{}'",
					actual.escape_debug(),
				)?;
				write!(fmt, "\u{0002}help\u{0003} expected one of: ")?;
				format_possible(fmt, possible_operands, "or")?;
				writeln!(fmt)?;
				if let Some(suggestion) = suggestion {
					write!(fmt, "\u{0002}help\u{0003} try '{suggestion}' instead")?;
				};
				Ok(())
			},
			Self::UnrecognisedOption { ref arg, ref value } => {
				let closest: &'static [&'static str] = fuzzy::filter_cli_range(arg, ALL_OPTIONS, 1);
				writeln!(fmt, "unrecognised option '{}'", arg.escape_debug())?;
				if let Some(ref value) = *value {
					writeln!(
						fmt,
						"\u{0002}help\u{0003} the following operand was also found: '{}'",
						value.escape_debug(),
					)?;
				};
				match closest {
					&[] => (),
					&[singular] => {
						writeln!(
							fmt,
							"\u{0002}help\u{0003} the closest recognised option is: '{singular}'",
						)?;
					},
					_ => {
						write!(
							fmt,
							"\u{0002}help\u{0003} the closest recognised options are: ",
						)?;
						format_possible(fmt, closest, "and")?;
						writeln!(fmt)?;
					},
				};
				write!(
					fmt,
					"\u{0002}help\u{0003} use '--help' to view all recognised options",
				)?;
				Ok(())
			},
		}
	}
}
