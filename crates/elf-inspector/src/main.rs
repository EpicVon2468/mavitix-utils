#![feature(const_trait_impl, const_convert, derive_const, const_cmp)]

use std::{
	env::{Args, args},
	fs::File,
	hint::cold_path,
	io::Read as _,
};

use anyhow::{Result, bail};

use mavitix_utils::main;

pub mod elf;
pub mod errors;

use crate::{
	elf::{
		ABI,
		ELFType,
		ExecutableHeader,
		Machine,
		ProgramHeader,
		ProgramType,
		SectionHeader,
		SectionType,
	},
	errors::ELFError,
};

macro_rules! read_n {
	($file:expr, $le:expr, $num:literal, $int:ty $(,)?) => {{
		let mut buf: [u8; $num] = [0; _];
		$file.read_exact(&mut buf)?;
		if $le {
			<$int>::from_le_bytes(buf)
		} else {
			<$int>::from_be_bytes(buf)
		}
	}};
}

macro_rules! read_u16 {
	($file:expr, $le:expr $(,)?) => {
		read_n!($file, $le, 0x2, u16)
	};
}

macro_rules! read_u32 {
	($file:expr, $le:expr $(,)?) => {
		read_n!($file, $le, 0x4, u32)
	};
}

macro_rules! read_u64 {
	($file:expr, $le:expr $(,)?) => {
		read_n!($file, $le, 0x8, u64)
	};
}

macro_rules! elf_version_panic {
	($version:expr $(,)?) => {{
		match $version {
			1 => (),
			_ => {
				cold_path();
				panic!("New elf version?!");
			},
		};
	}};
}

main!(main_impl());

pub fn main_impl() -> Result<()> {
	let name: String = match {
		let mut args: Args = args();
		let _ = args.next();
		args.next()
	} {
		Some(value) => value,
		None => bail!("missing operand"),
	};
	let mut file: File = File::open(name)?;
	let header: ExecutableHeader = parse_exe_header(&mut file)?;
	let is_le: bool = header.is_le;
	dbg!(std::mem::size_of::<ExecutableHeader>());
	let mut program_headers: Vec<ProgramHeader> =
		Vec::with_capacity(header.program_header_entry_count as usize);
	for _ in 0..header.program_header_entry_count {
		program_headers.push(parse_program_header(&mut file, is_le)?);
	}
	print_information(header, program_headers, vec![]);
	Ok(())
}

pub fn print_information(
	header: ExecutableHeader,
	program_headers: Vec<ProgramHeader>,
	section_headers: Vec<SectionHeader>,
) {
	let machine: Machine = header.machine;
	println!("==== ELF File Information ====");
	println!("Endian: {}", if header.is_le { "Little" } else { "Big" });
	println!(
		"ABI: {}",
		header.abi.map_or_default(ABI::name).unwrap_or("Unknown"),
	);
	println!("ABI Version: {:#b}", header.abi_version);
	println!(
		"Type: {}",
		match header.r#type {
			ELFType::OSSpecific(value) => format!("Unspecified OS-Specific Value; Raw: {value:#X}"),
			ELFType::ProcessorSpecific(value) =>
				format!("Unspecified Processor-Specific Value; Raw: {value:#X}"),
			other => other.name().unwrap().to_owned(),
		},
	);
	println!(
		"Machine: {}",
		match machine {
			Machine::Unofficial(value) => format!("Unspecified Unofficial Value; Raw: {value:#X}"),
			_ => machine.name().unwrap().to_owned(),
		},
	);
	println!("Entry Address: {:#X}", header.entry_addr);
	println!("Flags: {:#b}", header.flags);
	for program_header in program_headers {
		println!("=== Program Header ===");
		let type_description: &str = {
			let r#type: ProgramType = program_header.r#type;
			if let Some(value) = r#type.name() {
				value
			} else {
				match r#type {
					ProgramType::SUNSpecific(value) =>
						&format!("Unspecified SUN-Specific Value; Raw: {value:#X}"),
					ProgramType::OSSpecific(value) =>
						&format!("Unspecified OS-Specific Value; Raw: {value:#X}"),
					ProgramType::ProcessorSpecific(value) => {
						macro_rules! unspecified {
							($(,)?) => {
								&format!("Unspecified Processor-Specific Value; Raw: {value:#X}")
							};
						}
						match machine {
							Machine::MIPSBE | Machine::MIPSLE | Machine::MIPSX => match value {
								0x70000000 => "MIPS Register Information",
								0x70000001 => "MIPS Runtime Procedure Table",
								0x70000002 => "MIPS Options",
								0x70000003 => "MIPS ABI Flags",
								_ => unspecified!(),
							},
							_ => unspecified!(),
						}
					},
					unexpected => unreachable!("{unexpected:?}"),
				}
			}
		};
		println!("Type: {type_description}");
	}
	for section_header in section_headers {
		println!("=== Section Header ===");
		let type_description: &str = {
			let r#type: SectionType = section_header.r#type;
			if let Some(value) = r#type.name() {
				value
			} else {
				match r#type {
					SectionType::OSSpecific(value) =>
						&format!("Unspecified OS-Specific Value; Raw: {value:#X}"),
					SectionType::ProcessorSpecific(value) => {
						macro_rules! unspecified {
							($(,)?) => {
								&format!("Unspecified Processor-Specific Value; Raw: {value:#X}")
							};
						}
						match machine {
							Machine::MIPSBE | Machine::MIPSLE | Machine::MIPSX => match value {
								0x70000000 => "MIPS Shared Library List",
								0x70000001 => "MIPS M Symbol",
								0x70000002 => "MIPS Conflicting Symbols List",
								0x70000003 => "MIPS Global Pointer Table",
								0x70000004 => "MIPS Reserved",
								0x70000005 => "MIPS Debug Information",
								0x70000006 => "MIPS Register Information",
								0x70000007 => "MIPS Package",
								0x70000008 => "MIPS Pack Symbol",
								0x70000009 => "MIPS RELD",
								0x7000000B => "MIPS IFACE",
								0x7000000C => "MIPS Content",
								0x7000000D => "MIPS Options",
								0x70000010 => "MIPS SHDR",
								0x70000011 => "MIPS FDESC",
								0x70000012 => "MIPS Ext Symbol",
								0x70000013 => "MIPS Dense",
								0x70000014 => "MIPS PDESC",
								0x70000015 => "MIPS Loc Symbol",
								0x70000016 => "MIPS Auxillary Symbol",
								0x70000017 => "MIPS Opt Symbol",
								0x70000018 => "MIPS Loc String",
								0x70000019 => "MIPS Line",
								0x7000001A => "MIPS RFDESC",
								0x7000001B => "MIPS Delta Symbol",
								0x7000001C => "MIPS Delta Inst",
								0x7000001D => "MIPS Delta Class",
								0x7000001E => "MIPS DWARF Debug Information",
								0x7000001F => "MIPS Delta Declaration",
								0x70000020 => "MIPS Symbol Library",
								0x70000021 => "MIPS Events",
								0x70000022 => "MIPS Translate",
								0x70000023 => "MIPS Pixie",
								0x70000024 => "MIPS XLate",
								0x70000025 => "MIPS XLate Debug",
								0x70000026 => "MIPS Whirl",
								0x70000027 => "MIPS EH Region",
								0x70000028 => "MIPS XLate [Old]",
								0x70000029 => "MIPS PDR Exception",
								0x7000002A => "MIPS ABI Flags",
								0x7000002B => "MIPS XHash",
								_ => unspecified!(),
							},
							Machine::PrecisionArchitectureRISC => match value {
								0x70000000 => "PA-RISC Extension Bits",
								0x70000001 => "PA-RISC Unwind Table",
								0x70000002 => "PA-RISC Debug Information For Optimised Code",
								0x70000003 => "PA-RISC Code Annotations",
								_ => unspecified!(),
							},
							Machine::Alpha => match value {
								0x70000001 => "Alpha Debug Information",
								0x70000002 => "Alpha Register Information",
								_ => unspecified!(),
							},
							Machine::ARM => match value {
								0x70000001 => "ARM [32-bit] Unwind Section",
								0x70000002 => "ARM [32-bit] Pre-Emption Details",
								0x70000003 => "ARM [32-bit] Attributes",
								_ => unspecified!(),
							},
							Machine::CSKY if let 0x70000001 = value => "C-SKY Attributes",
							Machine::IntelItanium64 => match value {
								0x70000000 => "IA-64 Extension Bits",
								0x70000001 => "IA-64 Unwind Bits",
								_ => unspecified!(),
							},
							Machine::X86_64 if let 0x70000001 = value =>
								"x86-64 Unwind Information",
							Machine::RISCV if let 0x70000003 = value => "RISC-V Attributes",
							Machine::ARC if let 0x70000001 = value => "ARC Attributes",
							_ => unspecified!(),
						}
					},
					SectionType::ApplicationSpecific(value) =>
						&format!("Unspecified Application-Specific Value; Raw: {value:#X}"),
					unexpected => unreachable!("{unexpected:?}"),
				}
			}
		};
		println!("Type: {type_description}");
	}
}

pub fn parse_program_header(file: &mut File, is_le: bool) -> Result<ProgramHeader> {
	Ok(ProgramHeader {
		r#type: match ProgramType::try_from(read_u32!(file, is_le)) {
			Ok(value) => value,
			Err(actual) => bail!(ELFError::InvalidProgramType { actual }),
		},
		flags: read_u32!(file, is_le),
		offset: read_u64!(file, is_le),
		virtual_addr: read_u64!(file, is_le),
		physical_addr: read_u64!(file, is_le),
		size_in_file: read_u64!(file, is_le),
		size_in_mem: read_u64!(file, is_le),
		align: read_u64!(file, is_le),
	})
}

pub fn parse_exe_header(file: &mut File) -> Result<ExecutableHeader> {
	let mut ident: [u8; 0x10] = [0; _];
	file.read_exact(&mut ident)?;
	// Check for the elf magic bytes.
	match &ident[elf::IDX_MAGIC_0..=elf::IDX_MAGIC_3] {
		[0x7F, b'E', b'L', b'F'] => (),
		_ => bail!(ELFError::NotELF {
			// SAFETY: Reinterpreting the value as an array of a smaller size.
			actual: unsafe { *(&raw const ident as *const [u8; 0x4]) },
		}),
	};
	match ident[elf::IDX_CLASS] {
		1 => bail!(ELFError::ELF32),
		2 => (),
		// Anything else (including 0) is invalid
		actual => bail!(ELFError::InvalidELFClass { actual }),
	};
	let is_le: bool = match ident[elf::IDX_DATA] {
		0 => unimplemented!("unknown data format"),
		1 => true,
		2 => false,
		actual => {
			cold_path();
			bail!(ELFError::InvalidEndianness { actual });
		},
	};
	elf_version_panic!(ident[elf::IDX_VERSION]);
	Ok(ExecutableHeader {
		is_le,
		abi: ABI::try_from(ident[elf::IDX_ABI]).ok(),
		abi_version: ident[elf::IDX_ABI_VERSION],
		r#type: match ELFType::try_from(read_u16!(file, is_le)) {
			Ok(value) => value,
			Err(actual) => bail!(ELFError::InvalidELFType { actual }),
		},
		machine: {
			// SAFETY: Failure is infallible.
			let machine: Machine =
				unsafe { Machine::try_from(read_u16!(file, is_le)).unwrap_unchecked() };
			elf_version_panic!(read_u32!(file, is_le));
			machine
		},
		entry_addr: read_u64!(file, is_le),
		program_header_offset: read_u64!(file, is_le),
		section_header_offset: read_u64!(file, is_le),
		flags: read_u32!(file, is_le),
		elf_header_size: read_u16!(file, is_le),
		program_header_entry_size: read_u16!(file, is_le),
		program_header_entry_count: read_u16!(file, is_le),
		section_header_entry_size: read_u16!(file, is_le),
		section_header_entry_count: read_u16!(file, is_le),
		section_header_string_index: read_u16!(file, is_le),
	})
}
