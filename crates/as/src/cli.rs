use std::{
	env::{Args, args},
	fmt::{Debug, Error as FmtError, Formatter},
	iter::Peekable,
	path::Path,
	str::FromStr,
};

#[derive(Debug)]
pub struct CLI {
	pub debug_info: bool,
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

#[allow(non_camel_case_types)]
pub enum Target {
	x86_64,
	AArch64,
}

const impl Default for Target {
	fn default() -> Self {
		const {
			match Target::from_str(option_env!("AS_DEFAULT_TARGET").unwrap_or("x86-64")) {
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
			"x86-64" | "x86_64" | "amd64" | "x64" => Ok(Self::x86_64),
			"aarch64" | "aarch64le" | "arm64" | "arm64le" => Ok(Self::AArch64),
			_ => Err(()),
		}
	}
}

#[derive_const(Default)]
pub enum OptLevel {
	#[default]
	None,
	Low,
	High,
	Small,
}

impl Debug for OptLevel {
	fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), FmtError> {
		write!(
			fmt,
			"{}",
			match self {
				Self::None => "None [O0]",
				Self::Low => "Low [O1]",
				Self::High => "High [O2]",
				Self::Small => "Small [Os]",
			},
		)
	}
}

const impl FromStr for OptLevel {
	type Err = ();

	fn from_str(r#str: &str) -> Result<Self, Self::Err> {
		match r#str {
			"0" => Ok(Self::None),
			"1" => Ok(Self::Low),
			"2" | "3" => Ok(Self::High),
			"s" => Ok(Self::Small),
			_ => Err(()),
		}
	}
}

pub struct ArgIter {
	args: Peekable<Args>,
	pub seen_double_dash: bool,
}

impl ArgIter {
	pub fn new() -> Self {
		let mut instance: Self = Self {
			args: args().peekable(),
			seen_double_dash: false,
		};
		let _ = instance.args.next();
		instance
	}

	#[inline]
	pub fn peek(&mut self) -> Option<&<Self as Iterator>::Item> {
		self.args.peek()
	}
}

impl Iterator for ArgIter {
	type Item = String;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.args.next()
	}
}
