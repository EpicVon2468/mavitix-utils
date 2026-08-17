use std::{
	env::{Args, args},
	hint::cold_path,
	str::Chars,
};

use anyhow::{Result, bail};

use crate::errors::CLIError;

pub fn destructure(arg: &str) -> (char, char, String) {
	debug_assert!(!arg.is_empty());
	let mut chars: Chars = arg.chars();
	let Some(first): Option<char> = chars.next() else {
		cold_path();
		unreachable!();
	};
	let Some(second): Option<char> = chars.next() else {
		cold_path();
		unreachable!();
	};
	let rest: String = chars.as_str().to_owned();
	(first, second, rest)
}

pub struct ArgIter {
	args: Args,
	cached: Vec<String>,
}

impl ArgIter {
	pub fn new() -> Self {
		let mut instance: Self = Self {
			args: args(),
			cached: Vec::with_capacity(4),
		};
		// Skip executable name.
		let _ = instance.args.next();
		instance
	}

	pub fn restore(&mut self, arg: String) {
		self.cached.push(arg);
	}

	pub fn join_long(&mut self, mut arg: String) -> String {
		// Handle space separated long arguments.
		if arg.split_once('=') == None {
			if let Some(next) = self.next() {
				if next.chars().next() == Some('-') {
					// current arg is flag
					self.restore(next);
				} else {
					// pull in the value
					arg = format!("{arg}={next}");
				};
			};
		};
		arg
	}

	/// `-[A-Za-z]\{1,\} \{0,\}[^ ]\{1,\}`
	pub fn short_argument_required_value(
		&mut self,
		mut value: String,
		arg_name: &'static str,
		expected: Option<&'static str>,
	) -> Result<String> {
		let mut is_error: bool = false;
		if value.is_empty() {
			is_error = true;
			if let Some(next) = self.next() {
				if let Some('-') = next.chars().next() {
					self.restore(next);
				} else {
					value = next;
					is_error = false;
				};
			};
		};
		if is_error {
			bail!(CLIError::InvalidValue {
				arg_name,
				value,
				expected,
			});
		};
		Ok(value)
	}
}

impl Iterator for ArgIter {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		match self.cached.pop() {
			some @ Some(_) => some,
			None => self.args.next(),
		}
	}
}
