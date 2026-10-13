#![feature(const_trait_impl, const_convert, derive_const, const_cmp)]

use std::{
	env::{args, Args},
	fs::File,
	hint::cold_path,
	io::Read as _,
};

use anyhow::{bail, Result};

use mavitix_utils::main;

pub mod elf;
pub mod errors;

use crate::{
	elf::{
		ELFType,
		ExecutableHeader,
		Machine,
		ProgramHeader,
		ProgramType,
		SectionHeader,
		SectionType,
		ABI,
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
	{
		let abi: &str = match header.abi {
			Some(value) => match value {
				ABI::SysV => "System V",
				ABI::HPUX => "HP-UX",
				ABI::NetBSD => "NetBSD",
				ABI::Linux => "Linux",
				ABI::Solaris => "Solaris",
				ABI::AIX => "AIX",
				ABI::IRIX => "IRIS",
				ABI::FreeBSD => "FreeBSD",
				ABI::TRU64 => "TRU64",
				ABI::Modesto => "Modesto",
				ABI::OpenBSD => "OpenBSD",
				ABI::ARMEABI => "ARM EABI",
				ABI::ARM => "ARM",
				ABI::Standalone => "Standalone [Embedded]",
			},
			None => "Unknown",
		};
		println!("ABI: {abi}");
	};
	println!("ABI Version: {:#b}", header.abi_version);
	println!("Type: {:?}", header.r#type);
	println!("Machine: {:?}", machine);
	println!("Entry Address: {:#X}", header.entry_addr);
	println!("Flags: {:#b}", header.flags);
	for program_header in program_headers {
		println!("=== Program Header ===");
		let type_description: &str = match program_header.r#type {
			ProgramType::Null => "Unused Entry",
			ProgramType::Load => "Loadable",
			ProgramType::Dynamic => "Dynamic Linking Information",
			ProgramType::Interpreter => "Interpreter",
			ProgramType::Note => "Note[s]",
			ProgramType::_Reserved => "Reserved",
			ProgramType::ProgramHeaderTable => "Header Table Entry",
			ProgramType::ThreadLocalStorage => "Thread Local Storage",
			ProgramType::GNUEhFrame => "GNU `.eh_frame_hdr`",
			ProgramType::GNUStack => "GNU Stack Note",
			ProgramType::GNUReadOnlyAfterRelocation => "Read-Only After Relocation",
			ProgramType::GNUProperty => "GNU Property",
			ProgramType::GNUSFrame => "GNU SFrame",
			ProgramType::SUNBss => "SUN `.bss`",
			ProgramType::SUNStack => "SUN Stack",
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
					Machine::MIPS | Machine::MIPSX | Machine::MIPSR3000LittleEndian =>
						match value {
							0x70000000 => "MIPS Register Information",
							0x70000001 => "MIPS Runtime Procedure Table",
							0x70000002 => "MIPS Options",
							0x70000003 => "MIPS ABI Flags",
							_ => unspecified!(),
						},
					_ => unspecified!(),
				}
			},
		};
		println!("Type: {type_description}");
	}
	for section_header in section_headers {
		println!("=== Section Header ===");
		let type_description: &str = match section_header.r#type {
			SectionType::Null => "Unused Entry",
			SectionType::Progbits => "Program Data (`@progbits`)",
			SectionType::SymbolTable => "Symbol Table",
			SectionType::StringTable => "String Table",
			SectionType::RelocationAddends => "Relocation Entries With Addends",
			SectionType::SymbolHashTable => "Symbol Hash Table",
			SectionType::Dynamic => "Dynamic Linking Information",
			SectionType::Note => "Note[s] (`@note`)",
			SectionType::Bss => "Null Program Data",
			SectionType::RelocationNoAddends => "Relocation Entries Without Addends",
			SectionType::_Reserved => "Reserved",
			SectionType::DynamicSymbolTable => "Dynamic Symbol Table",
			SectionType::InitArray => "Initialisation Array (`@init_array`)",
			SectionType::FiniArray => "Finalisation Array (`@fini_array`)",
			SectionType::PreinitArray => "Pre-Initialisation Array (`@preinit_array`)",
			SectionType::SectionGroup => "Section Group",
			SectionType::SymbolTableIndices => "Extended Symbol Table Indices",
			SectionType::RelativeRelocations => "Relative Relocation[s]",
			SectionType::GNUObjectAttributes => "GNU Object Attributes",
			SectionType::GNUHashTable => "GNU Hash Table",
			SectionType::GNULibraryList => "GNU [Pre-Link] Library List",
			SectionType::Checksum => "Checksum",
			SectionType::SUNMove => "SUN Move",
			SectionType::SUNCOMDAT => "SUN COMDAT",
			SectionType::SUNSyminfo => "SUN Syminfo",
			SectionType::GNUVersionDefinition => "GNU Version Definition",
			SectionType::GNUVersionNeeds => "GNU Version Needs",
			SectionType::GNUVersionSymbolTable => "GNU Version Symbol Table",
			SectionType::OSSpecific(value) =>
				&format!("Unspecified OS-Specific Value; Raw: {value:#X}"),
			SectionType::ProcessorSpecific(value) => {
				macro_rules! unspecified {
					($(,)?) => {
						&format!("Unspecified Processor-Specific Value; Raw: {value:#X}")
					};
				}
				match machine {
					Machine::MIPS | Machine::MIPSX | Machine::MIPSR3000LittleEndian =>
						match value {
							0x70000000 => "MIPS Shared Library List",
							0x70000002 => "MIPS Conflicting Symbols List",
							0x70000003 => "MIPS Global Pointer Table",
							0x70000004 => "MIPS Reserved",
							0x70000005 => "MIPS Debug Information",
							0x70000006 => "MIPS Register Information",
							_ => unspecified!(),
						},
					Machine::PrecisionArchitectureRISC => match value {
						0x70000000 => "PA-RISC Extension Bits",
						0x70000001 => "PA-RISC Unwind Table",
						0x70000002 => "PA-RISC Debug Information For Optimised Code",
						0x70000003 => "PA-RISC Code Annotations",
						_ => unspecified!(),
					},
					_ => unspecified!(),
				}
			},
			SectionType::ApplicationSpecific(value) =>
				&format!("Unspecified Application-Specific Value; Raw: {value:#X}"),
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
