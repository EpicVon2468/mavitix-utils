use std::{
	error::Error,
	fmt::{Display, Formatter, Result as FmtResult},
	hint::cold_path,
};

use crate::cli::arg::Arg;

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
		possible_values: &'static [&'static str],
	},
	UnexpectedOperand {
		arg_name: &'static str,
		actual: String,
	},
	InvalidOperand {
		arg_name: &'static str,
		possible_values: &'static [&'static str],
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
	pub fn invalid_operand<T: const Arg>(actual: String) -> Self {
		Self::InvalidOperand {
			arg_name: const { T::LONG_NAME.or(T::SHORT_NAME).unwrap() },
			possible_values: T::POSSIBLE_VALUES,
			suggestion: T::suggest_from(&actual),
			actual,
		}
	}
}

impl Error for CLIError {}

#[inline]
fn format_possible(fmt: &mut Formatter<'_>, possible_values: &'static [&'static str]) -> FmtResult {
	write!(fmt, "expected one of: ")?;
	for index in 0..possible_values.len() {
		let value: &'static str = possible_values[index];
		if index + 1 == possible_values.len() {
			cold_path();
			write!(fmt, "or '{value}'")?;
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
				possible_values,
			} => {
				writeln!(fmt, "expected operand for option '{arg_name}'")?;
				format_possible(fmt, possible_values)?;
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
				possible_values,
				suggestion,
				ref actual,
			} => {
				writeln!(fmt, "invalid operand for option '{arg_name}'")?;
				writeln!(fmt, "invalid operand was '{}'", actual.escape_debug())?;
				format_possible(fmt, possible_values)?;
				if let Some(suggestion) = suggestion {
					write!(fmt, "\ntry '{suggestion}' instead")?;
				};
				Ok(())
			},
			Self::UnrecognisedOption { ref arg, ref value } => {
				writeln!(fmt, "unrecognised option '{}'", arg.escape_debug())?;
				if let Some(ref value) = *value {
					writeln!(
						fmt,
						"the following operand was also found: '{}'",
						value.escape_debug(),
					)?;
				};
				write!(fmt, "use '--help' to view recognised options")?;
				Ok(())
			},
		}
	}
}
