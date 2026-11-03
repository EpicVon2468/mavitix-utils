use std::{
	env::{Args, args},
	fmt::{Debug, Error as FmtError, Formatter},
	path::Path,
	str::FromStr,
};

pub mod arg {
	use std::{fmt::Debug, str::FromStr};

	use crate::errors::CLIError;

	pub const trait Arg: Debug + const FromStr<Err = ()> + const Default {
		const LONG_NAME: Option<&'static str>;
		const SHORT_NAME: Option<&'static str>;

		const POSSIBLE_OPERANDS: &'static [&'static str];

		fn suggest_from(input: &str) -> Option<&'static str>;
	}

	#[inline]
	pub fn parse_boolish(input: Option<&str>, arg_name: &'static str) -> Result<bool, CLIError> {
		match input {
			Some("true" | "on") | None => Ok(true),
			Some("false" | "off") => Ok(false),
			Some(invalid) => Err(CLIError::InvalidOperand {
				arg_name,
				possible_operands: &["true", "on", "false", "off"],
				suggestion: None,
				actual: invalid.to_owned(),
			}),
		}
	}

	#[inline]
	#[must_use]
	pub fn try_parse_long<T: const Arg>(input: &str) -> Result<T, CLIError> {
		self::try_parse(input, true)
	}

	#[inline]
	#[must_use]
	pub fn try_parse_short<T: const Arg>(input: &str) -> Result<T, CLIError> {
		self::try_parse(input, false)
	}

	#[must_use]
	#[inline(always)]
	fn try_parse<T: const Arg>(input: &str, is_long: bool) -> Result<T, CLIError> {
		match T::from_str(input) {
			Ok(value) => Ok(value),
			Err(()) => Err(CLIError::invalid_operand::<T>(input.to_owned(), is_long)),
		}
	}
}

use arg::Arg;

pub const ALL_OPTIONS: &'static [&'static str] = &[
	"-V",
	"-g",
	"-O",
	"--version",
	"--verbose",
	"--silent",
	"--debug",
	"--debug-embed-source",
	"--debug-record-args",
	"--debug-dwarf-version",
	"--debug-dwarf64",
	"--debug-dwarf32",
	"--opt-level",
	"--arch",
	"--endian",
];

#[derive(Debug, PartialEq, Eq)]
pub struct CLI {
	pub debug_info: Option<DebugInfo>,
	pub verbosity: Verbosity,
	pub files: Vec<&'static Path>,
	pub read_from_stdin: bool,
	pub arch: Arch,
	pub endian: Endian,
	pub opt_level: OptLevel,
}

impl CLI {
	#[must_use]
	pub const fn debug_info(&mut self) -> &mut DebugInfo {
		if let None = self.debug_info {
			self.debug_info = Some(const { DebugInfo::default() });
		};
		// SAFETY: `self.debug_info` is always `Some(_)` at this point.
		unsafe { self.debug_info.as_mut().unwrap_unchecked() }
	}
}

impl Default for CLI {
	fn default() -> Self {
		Self {
			debug_info: None,
			verbosity: const { Verbosity::default() },
			files: Vec::with_capacity(4),
			read_from_stdin: false,
			arch: const { Arch::default() },
			endian: const { Endian::default() },
			opt_level: const { OptLevel::default() },
		}
	}
}

#[derive(Debug)]
#[derive_const(PartialEq, Eq)]
pub struct DebugInfo {
	pub embed_source: bool,
	pub record_args: bool,
	pub dwarf_version: DwarfVersion,
	pub dwarf64: bool,
}

const impl Default for DebugInfo {
	fn default() -> Self {
		Self {
			embed_source: false,
			record_args: true,
			dwarf_version: const { DwarfVersion::default() },
			dwarf64: false,
		}
	}
}

#[derive(Debug, Copy)]
#[derive_const(Default, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DwarfVersion {
	#[default]
	Five,
	Six,
}

const impl Arg for DwarfVersion {
	const LONG_NAME: Option<&'static str> = Some("--debug-dwarf-version");
	const SHORT_NAME: Option<&'static str> = None;

	const POSSIBLE_OPERANDS: &'static [&'static str] = &["5", "6", "latest", "next"];

	fn suggest_from(input: &str) -> Option<&'static str> {
		match usize::from_str(input) {
			Ok(7..) => Some("next"),
			_ => None,
		}
	}
}

const impl FromStr for DwarfVersion {
	type Err = ();

	fn from_str(r#str: &str) -> Result<Self, Self::Err> {
		match r#str {
			"5" => Ok(Self::Five),
			"6" | "next" => Ok(Self::Six),
			"latest" => Ok(Self::default()),
			_ => Err(()),
		}
	}
}

const impl Into<usize> for DwarfVersion {
	fn into(self) -> usize {
		match self {
			Self::Five => 5,
			Self::Six => 6,
		}
	}
}

#[derive(Debug, Copy)]
#[derive_const(Default, Clone, PartialEq, Eq)]
pub enum Verbosity {
	Verbose,
	#[default]
	Normal,
	Silent,
}

#[derive(Copy)]
#[derive_const(Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Arch {
	x86_64,
	AArch64,
	RiscV64,
}

const impl Arg for Arch {
	const LONG_NAME: Option<&'static str> = Some("--arch");
	const SHORT_NAME: Option<&'static str> = None;

	const POSSIBLE_OPERANDS: &'static [&'static str] = &["x86-64", "aarch64", "riscv64"];

	fn suggest_from(input: &str) -> Option<&'static str> {
		match input {
			"x86_64" | "amd64" | "intel64" | "x64" | "x86" => Some("x86-64"),
			"aarch64le" | "aarch64be" | "aarch64_lfi" | "arm64" | "arm64le" | "arm64be"
			| "arm64e" | "arm64e.x1" | "arm64ec" | "arm" => Some("aarch64"),
			"riscv64gc" | "riscv64a23" | "riscv64le" | "riscv64be" => Some("riscv64"),
			_ => None,
		}
	}
}

const impl Default for Arch {
	fn default() -> Self {
		const {
			match Arch::from_str(option_env!("MAVITIX_AS_DEFAULT_ARCH").unwrap_or("x86-64")) {
				Ok(value) => value,
				Err(()) => panic!(),
			}
		}
	}
}

impl Debug for Arch {
	fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), FmtError> {
		write!(
			fmt,
			"{}",
			match self {
				Self::x86_64 => "x86-64",
				Self::AArch64 => "aarch64",
				Self::RiscV64 => "riscv64",
			},
		)
	}
}

const impl FromStr for Arch {
	type Err = ();

	#[inline]
	fn from_str(r#str: &str) -> Result<Self, Self::Err> {
		match r#str {
			"x86-64" => Ok(Self::x86_64),
			"aarch64" => Ok(Self::AArch64),
			"riscv64" => Ok(Self::RiscV64),
			_ => Err(()),
		}
	}
}

const impl Into<object::Architecture> for Arch {
	fn into(self) -> object::Architecture {
		match self {
			Self::x86_64 => object::Architecture::X86_64,
			Self::AArch64 => object::Architecture::Aarch64,
			Self::RiscV64 => object::Architecture::Riscv64,
		}
	}
}

#[derive(Debug, Copy)]
#[derive_const(Default, Clone, PartialEq, Eq)]
pub enum Endian {
	Big,
	#[default]
	Little,
}

const impl Arg for Endian {
	const LONG_NAME: Option<&'static str> = Some("--endian");
	const SHORT_NAME: Option<&'static str> = None;

	const POSSIBLE_OPERANDS: &'static [&'static str] = &["big", "little"];

	fn suggest_from(_: &str) -> Option<&'static str> {
		None
	}
}

const impl FromStr for Endian {
	type Err = ();

	#[inline]
	fn from_str(r#str: &str) -> Result<Self, Self::Err> {
		match r#str {
			"big" => Ok(Self::Big),
			"little" => Ok(Self::Little),
			_ => Err(()),
		}
	}
}

const impl Into<object::Endianness> for Endian {
	#[inline]
	fn into(self) -> object::Endianness {
		match self {
			Self::Big => object::Endianness::Big,
			Self::Little => object::Endianness::Little,
		}
	}
}

#[derive(Debug, Copy)]
#[derive_const(Default, Clone, PartialEq, Eq)]
pub enum OptLevel {
	#[default]
	None,
	Min,
	Med,
	Max,
}

const impl Arg for OptLevel {
	const LONG_NAME: Option<&'static str> = Some("--opt-level");
	const SHORT_NAME: Option<&'static str> = Some("-O");

	const POSSIBLE_OPERANDS: &'static [&'static str] = &["0", "1", "2", "3"];

	fn suggest_from(_: &str) -> Option<&'static str> {
		None
	}
}

const impl FromStr for OptLevel {
	type Err = ();

	#[inline]
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

#[derive(Debug)]
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
