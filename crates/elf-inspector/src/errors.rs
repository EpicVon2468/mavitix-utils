use std::{
	error::Error,
	fmt::{Display, Formatter, Result as FmtResult},
};

#[must_use]
#[derive(Debug)]
pub enum ELFError {
	NotELF { actual: [u8; 0x4] },
	ELF32,
	InvalidEndianness { actual: u8 },
	InvalidELFClass { actual: u8 },
	InvalidELFType { actual: u16 },
	InvalidProgramType { actual: u32 },
}

impl Error for ELFError {}

impl Display for ELFError {
	fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
		match *self {
			Self::NotELF { actual } => write!(fmt, "not an elf file; actual magic was {actual:?}"),
			Self::ELF32 => write!(fmt, "file was 32-bit elf, which is unsupported"),
			Self::InvalidEndianness { actual } => write!(
				fmt,
				"invalid endianness value; actual value was {actual:#X}",
			),
			Self::InvalidELFClass { actual } =>
				write!(fmt, "invalid elf class; actual value was {actual:#X}"),
			Self::InvalidELFType { actual } => write!(
				fmt,
				"invalid elf type in header; actual value was {actual:#X}",
			),
			Self::InvalidProgramType { actual } => write!(
				fmt,
				"invalid program type in program header; actual value was {actual:#X}",
			),
		}
	}
}
