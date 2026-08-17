use thiserror::Error;

#[must_use]
#[derive(Debug, Error)]
pub enum CLIError {
	#[error("Invalid value {value:?} passed to '{arg_name}'!{}", if let Some(expected) = expected {
		format!(" Expected {expected}.")
	} else { String::new() })]
	InvalidValue {
		arg_name: &'static str,
		value: String,
		expected: Option<&'static str>,
	},
	#[error("Unexpected or unknown argument {arg_name:?} was found!")]
	UnknownArgument { arg_name: String },
}
