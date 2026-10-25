use std::{
	env::{Args, args},
	fmt::{Debug, Error as FmtError, Formatter},
	path::Path,
	str::FromStr,
};

#[derive(Debug)]
pub struct CLI {
	pub debug_info: bool,
	pub debug_embed_source: bool,
	pub relax_relocations: bool,
	pub warn: bool,
	pub verbose: bool,
	pub files: Vec<&'static Path>,
	pub read_from_stdin: bool,
	pub target: Target,
	pub opt_level: OptLevel,
}

impl Default for CLI {
	fn default() -> Self {
		CLI {
			debug_info: false,
			debug_embed_source: false,
			relax_relocations: false,
			warn: true,
			verbose: false,
			files: Vec::with_capacity(4),
			read_from_stdin: false,
			target: const { Target::default() },
			opt_level: const { OptLevel::default() },
		}
	}
}

pub mod arg {
	use std::{fmt::Debug, str::FromStr};

	use crate::errors::CLIError;

	pub const trait Arg: Debug + const FromStr<Err = ()> + const Default {
		const LONG_NAME: Option<&'static str>;
		const SHORT_NAME: Option<&'static str>;

		const POSSIBLE_VALUES: &'static [&'static str];

		fn suggest_from(input: &str) -> Option<&'static str>;
	}

	#[inline]
	pub fn try_parse<T: const Arg>(input: &str) -> Result<T, CLIError> {
		match T::from_str(input) {
			Ok(value) => Ok(value),
			Err(()) => Err(CLIError::invalid_operand::<T>(input.to_owned())),
		}
	}
}

use arg::Arg;

#[allow(non_camel_case_types)]
pub enum Target {
	x86_64,
	AArch64,
}

const impl Arg for Target {
	const LONG_NAME: Option<&'static str> = Some("--target");
	const SHORT_NAME: Option<&'static str> = None;

	const POSSIBLE_VALUES: &'static [&'static str] = &["x86-64", "aarch64"];

	fn suggest_from(input: &str) -> Option<&'static str> {
		match input {
			"x86_64" | "amd64" | "x64" | "x86" => Some("x86-64"),
			"aarch64le" | "arm64" | "arm64le" | "arm" => Some("aarch64"),
			_ => None,
		}
	}
}

const impl Default for Target {
	fn default() -> Self {
		const {
			match Target::from_str(option_env!("MAVITIX_AS_DEFAULT_TARGET").unwrap_or("x86-64")) {
				Ok(value) => value,
				Err(()) => panic!(),
			}
		}
	}
}

impl Debug for Target {
	fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), FmtError> {
		write!(
			fmt,
			"{}",
			match self {
				Self::x86_64 => "x86-64",
				Self::AArch64 => "AArch64",
			},
		)
	}
}

const impl FromStr for Target {
	type Err = ();

	fn from_str(r#str: &str) -> Result<Self, Self::Err> {
		match r#str {
			"x86-64" => Ok(Self::x86_64),
			"aarch64" => Ok(Self::AArch64),
			_ => Err(()),
		}
	}
}

#[derive_const(Default)]
pub enum OptLevel {
	#[default]
	None,
	Min,
	Med,
	Max,
}

const impl Arg for OptLevel {
	const LONG_NAME: Option<&'static str> = None;
	const SHORT_NAME: Option<&'static str> = Some("-O");

	const POSSIBLE_VALUES: &'static [&'static str] = &["0", "1", "2", "3"];

	fn suggest_from(_: &str) -> Option<&'static str> {
		None
	}
}

impl Debug for OptLevel {
	fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), FmtError> {
		write!(
			fmt,
			"{}",
			match self {
				Self::None => "None [O0]",
				Self::Min => "Min [O1]",
				Self::Med => "Med [02]",
				Self::Max => "Max [O3]",
			},
		)
	}
}

const impl FromStr for OptLevel {
	type Err = ();

	fn from_str(r#str: &str) -> Result<Self, Self::Err> {
		match r#str {
			"0" => Ok(Self::None),
			"1" => Ok(Self::Min),
			"2" => Ok(Self::Med),
			"3" => Ok(Self::Max),
			_ => Err(()),
		}
	}
}

pub struct ArgIter {
	args: Args,
	peeked: Option<String>,
	pub seen_double_dash: bool,
}

impl ArgIter {
	pub fn new() -> Self {
		let mut instance: Self = Self {
			args: args(),
			peeked: None,
			seen_double_dash: false,
		};
		let _ = instance.args.next();
		instance
	}

	#[inline]
	pub fn peek(&mut self) -> Option<&<Self as Iterator>::Item> {
		if let None = self.peeked {
			self.peeked = self.args.next();
		};
		self.peeked.as_ref()
	}
}

impl Iterator for ArgIter {
	type Item = String;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		match self.peeked {
			None => self.args.next(),
			_ => self.peeked.take(),
		}
	}
}
