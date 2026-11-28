use std::{
	byte_str::ByteStr,
	error::Error,
	fmt::{Display, Formatter, Result as FmtResult},
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
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match *self {
			Self::CLI(ref inner) => Some(inner),
		}
	}
}

impl Display for MainError {
	fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
		match *self {
			Self::CLI(_) => write!(fmt, "Failed to parse CLI arguments"),
		}
	}
}

#[must_use]
#[derive(Debug)]
pub enum CLIError {
	ExpectedOperand {
		arg_name: &'static str,
	},
	UnexpectedOperand {
		arg_name: &'static str,
		actual: &'static [u8],
	},
	InvalidOperand {
		arg_name: &'static str,
		actual: &'static [u8],
	},
	UnrecognisedOption {
		arg: &'static [u8],
	},
}

impl Error for CLIError {}

impl Display for CLIError {
	fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
		match *self {
			Self::ExpectedOperand { arg_name } =>
				write!(fmt, "Expected operand for option '{arg_name}'"),
			Self::UnexpectedOperand { arg_name, actual } => write!(
				fmt,
				"Unexpected operand '{}' for option '{arg_name}'",
				ByteStr::new(actual),
			),
			Self::InvalidOperand { arg_name, actual } => write!(
				fmt,
				"Invalid operand '{}' for option '{arg_name}'",
				ByteStr::new(actual),
			),
			Self::UnrecognisedOption { arg } =>
				write!(fmt, "Unrecognised option '{}'", ByteStr::new(arg)),
		}
	}
}
