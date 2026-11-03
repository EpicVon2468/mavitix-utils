use std::fs::File;

use object::{
	BinaryFormat,
	SectionFlags,
	SectionKind,
	elf::SHT_PROGBITS,
	write::{Object, Section, SectionId},
};

use gimli::{
	DW_AT_producer,
	Encoding,
	Format,
	write::{AttributeValue, DebuggingInformationEntry, DwarfUnit},
};

use crate::cli::{self, CLI};

pub mod aarch64;
pub mod x86_64;

const ASSEMBLER_IDENTIFIER: &'static [u8] =
	concat!(env!("CARGO_BIN_NAME"), " v", env!("CARGO_PKG_VERSION")).as_bytes();

pub fn write(cli: &CLI) {
	let mut dest: Object<'_> = Object::new(BinaryFormat::Elf, cli.arch.into(), cli.endian.into());
	{
		let id: SectionId = dest.add_section(
			Vec::with_capacity(ASSEMBLER_IDENTIFIER.len() + 1),
			b".comment".to_vec(),
			SectionKind::OtherString,
		);
		let section: &mut Section<'_> = dest.section_mut(id);
		section.flags = SectionFlags::Elf {
			sh_type: SHT_PROGBITS,
			sh_flags: Default::default(),
		};
		section.append_data(ASSEMBLER_IDENTIFIER, 0);
		section.append_data(b"\0", 0);
	};
	dest.add_file_symbol(b"file_name_here".to_vec());
	let mut file: File = File::options()
		.create(true)
		.write(true)
		.open("foo.o")
		.unwrap();
	dest.write_stream(&mut file).unwrap();
	if let Some(ref debug_info) = cli.debug_info {
		let encoding: Encoding = Encoding {
			address_size: 8,
			format: if debug_info.dwarf64 {
				Format::Dwarf64
			} else {
				Format::Dwarf32
			},
			version: Into::<usize>::into(debug_info.dwarf_version) as u16,
		};
		write_debug_info(debug_info, encoding);
	};
}

pub fn write_debug_info(debug_info: &cli::DebugInfo, encoding: Encoding) {
	let mut dwarf: DwarfUnit = DwarfUnit::new(encoding);
	let entry: &mut DebuggingInformationEntry = dwarf.unit.get_mut(dwarf.unit.root());
	if debug_info.record_args {
		let mut producer: Vec<u8> = Vec::with_capacity(ASSEMBLER_IDENTIFIER.len() * 4);
		producer.extend_from_slice(ASSEMBLER_IDENTIFIER);
		let mut args: _ = std::env::args();
		let _ = args.next();
		for arg in args {
			let arg: Vec<u8> = arg.into_bytes();
			producer.reserve(arg.len() + 1);
			producer.push(b' ');
			producer.extend(arg);
		}
		let _ = dbg!(str::from_utf8(&producer));
		entry.set(DW_AT_producer, AttributeValue::String(producer));
	};
	if debug_info.embed_source {
		// No support for https://dwarfstd.org/issues/180201.1.html yet in `gimli`.
		// Closest for now is `DW_LNCT_LLVM_source`
	};
}
