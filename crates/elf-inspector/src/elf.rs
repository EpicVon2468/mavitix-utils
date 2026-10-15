#![allow(non_camel_case_types)]

pub const IDX_MAGIC_0: usize = 0x0;
pub const IDX_MAGIC_3: usize = 0x3;
pub const IDX_CLASS: usize = 0x4;
pub const IDX_DATA: usize = 0x5;
pub const IDX_VERSION: usize = 0x6;
pub const IDX_ABI: usize = 0x7;
pub const IDX_ABI_VERSION: usize = 0x8;

#[derive(Debug)]
pub struct ExecutableHeader {
	pub is_le: bool,
	pub abi: Option<ABI>,
	pub abi_version: u8,
	/// `Elf64_Half`
	pub r#type: ELFType,
	/// `Elf64_Half`
	pub machine: Machine,
	/// `Elf64_Addr`
	pub entry_addr: u64,
	/// `Elf64_Off`
	pub program_header_offset: u64,
	/// `Elf64_Off`
	pub section_header_offset: u64,
	/// `Elf64_Word`
	pub flags: u32,
	/// `Elf64_Half`
	pub elf_header_size: u16,
	/// `Elf64_Half`
	pub program_header_entry_size: u16,
	/// `Elf64_Half`
	pub program_header_entry_count: u16,
	/// `Elf64_Half`
	pub section_header_entry_size: u16,
	/// `Elf64_Half`
	pub section_header_entry_count: u16,
	/// `Elf64_Half`
	pub section_header_string_index: u16,
}

#[derive(Debug)]
pub struct ProgramHeader {
	pub r#type: ProgramType,
	pub flags: u32,
	pub offset: u64,
	pub virtual_addr: u64,
	pub physical_addr: u64,
	pub size_in_file: u64,
	pub size_in_mem: u64,
	pub align: u64,
}

#[derive(Debug)]
pub struct SectionHeader {
	pub name_index: u32,
	pub r#type: SectionType,
	pub flags: u64,
	pub addr: u64,
	pub offset: u64,
	pub size: u64,
	pub link: u32,
	pub info: u32,
	pub addr_align: u64,
	pub entry_size: u64,
}

macro_rules! convertible {
	(
		$name:ident as $rep:ty;
		$($key:ident = $value:expr; $string:expr),* ;
		$($key_range:ident in $value_range:pat),*
		$(=> else $catch_all:ident)? $(,)?
	) => {
		#[repr($rep)]
		#[derive(Debug, Clone, Copy)]
		#[derive_const(PartialEq, Eq)]
		pub enum $name {
			$($key = $value,)*
			$($key_range($rep),)*
			$($catch_all($rep),)?
		}

		impl $name {
			pub const fn name<'a>(self) -> Option<&'a str> {
				match self {
					$(Self::$key => Some($string),)*
					#[allow(unreachable_patterns)]
					_ => None,
				}
			}
		}

		const impl From<$name> for $rep {
			fn from(value: $name) -> Self {
				match value {
					$($name::$key_range(inner) => inner,)*
					$($name::$catch_all(inner) => inner,)?
					// SAFETY: Because `$name` is marked `repr($rep)`, its layout is a `repr(C)` `union`
					// between `repr(C)` structs, each of which has the `$rep` discriminant as its first
					// field, so we can read the discriminant without offsetting the pointer.
					_ => unsafe { *(&raw const value as *const $rep) },
				}
			}
		}

		// $(
		// 	const _: $name = $name::$catch_all(0);

		// 	const impl From<$rep> for $name {
		// 		fn from(value: $rep) -> Self {
		// 			Self::$catch_all(0);
		// 			// SAFETY: `$catch_all` is present in this macro invocation, so failure is infallible.
		// 			unsafe {
		// 				Self::try_from(value).unwrap_unchecked()
		// 			}
		// 		}
		// 	}
		// )?

		const impl TryFrom<$rep> for $name {
			type Error = $rep;

			fn try_from(value: $rep) -> Result<Self, Self::Error> {
				match value {
					$($value => Ok(Self::$key),)*
					$($value_range => Ok(Self::$key_range(value)),)*
					$(unknown => Ok(Self::$catch_all(unknown)),)?
					#[allow(unreachable_patterns)]
					unknown => Err(unknown),
				}
			}
		}
	};
}

convertible!(
	ABI as u8;
	SysV = 0x0; "System V",
	HPUX = 0x1; "HP-UX",
	NetBSD = 0x2; "NetBSD",
	Linux = 0x3; "Linux",
	Solaris = 0x6; "Solaris",
	AIX = 0x7; "AIX",
	IRIX = 0x8; "IRIX",
	FreeBSD = 0x9; "FreeBSD",
	TRU64 = 0xA; "TRU64",
	Modesto = 0xB; "Modesto",
	OpenBSD = 0xC; "OpenBSD",
	ARMEABI = 0x40; "ARM EABI",
	ARM = 0x61; "ARM",
	Standalone = 0xFF; "Standalone [Embedded]";
);

convertible!(
	ELFType as u16;
	None = 0x0; "None",
	Relocatable = 0x1; "Relocatable",
	Executable = 0x2; "[Static] Executable",
	Dynamic = 0x3; "Dynamic",
	CoreDump = 0x4; "Core Dump";
	OSSpecific in 0xFE00..=0xFEFF,
	ProcessorSpecific in 0xFF00..=0xFFFF,
);

convertible!(
	Machine as u16;
	None = 0x0; "None",
	M32 = 0x1; "M32",
	SPARC = 0x2; "SUN SPARC",
	Intel386 = 0x3; "Intel 80386",
	Motorola68K = 0x4; "Motorola 68000",
	Motorola88K = 0x5; "Motorola 88000",
	IntelMCU = 0x6; "Intel MCU",
	Intel860 = 0x7; "Intel 80860",
	MIPSBE = 0x8; "MIPS R3000 [Big Endian]",
	S370 = 0x9; "IBM System/370",
	MIPSLE = 0xA; "MIPS R3000 [Little Endian]",
	PrecisionArchitectureRISC = 0xF; "Precision Architecture RISC",
	VPP500 = 0x11; "Fujitsu VPP500",
	SPARC32Plus = 0x12; "SUN SPARC-32 Plus [v8plus]",
	Intel960 = 0x13; "Intel 80960",
	PowerPC = 0x14; "PowerPC [32-bit]",
	PowerPC64 = 0x15; "PowerPC [64-bit]",
	S390 = 0x16; "IBM System/390",
	SPU = 0x17; "IBM SPU",
	V800 = 0x24; "Renesas V800",
	FR20 = 0x25; "Fujitsu FR20",
	RH32 = 0x26; "TRW RH-32",
	MotorolaRCE = 0x27; "Motorola RCE",
	ARM = 0x28; "ARM [32-bit]",
	DigitalAlpha = 0x29; "Digital Alpha",
	SH = 0x2A; "Hitachi SH",
	SPARCv9 = 0x2B; "SPARC v9",
	TriCore = 0x2C; "Infineon TriCore",
	ARC = 0x2D; "Argonaut RISC Core",
	H8300 = 0x2E; "Hitachi H8/300",
	H8300H = 0x2F; "Hitachi H8/300H",
	H8S = 0x30; "Hitachi H8S",
	H8500 = 0x31; "Hitachi H8/500",
	IntelItanium64 = 0x32; "Intel [Itanium] IA-64",
	MIPSX = 0x33; "Stanford University MIPS-X",
	NXPColdfire = 0x34; "NXP ColdFire",
	Motorola68HC12 = 0x35; "Motorola 68HC12",
	MMA = 0x36; "Fujitsu MMA",
	PCP = 0x37; "PCP",
	NCPU = 0x38; "Cell/B.E.",
	NDR1 = 0x39; "Denso NDR1",
	MotorolaStarCore = 0x3A; "NXP StarCore",
	ME16 = 0x3B; "ME16",
	ST100 = 0x3C; "ST100",
	TinyJ = 0x3D; "TinyJ",
	X86_64 = 0x3E; "x86-64",
	PDSP = 0x3F; "Sony DSP",
	PDP10 = 0x40; "Digital Equipment Corporation PDP-10",
	PDP11 = 0x41; "Digital Equipment Corporation PDP-11",
	FX66 = 0x42; "FX66",
	ST9Plus = 0x43; "ST9+",
	ST7 = 0x44; "ST7",
	Motorola68HC16 = 0x45; "Motorola 68HC16",
	Motorola68HC11 = 0x46; "Motorola 68HC11",
	Motorola68HC08 = 0x47; "Motorola 68HC16",
	Motorola68HC05 = 0x48; "Motorola 68HC16",
	SVx = 0x49; "SVx",
	ST19 = 0x4A; "ST19",
	VAX = 0x4B; "Digital Equipment Corporation VAX",
	CRIS = 0x4C; "CRIS",
	Javelin = 0x4D; "Javelin",
	Firepath = 0x4E; "Firepath",
	ZSP = 0x4F; "ZSP",
	MMIX = 0x50; "MMIX",
	HarvardUniAny = 0x51; "Harvard University Machine-Independent Object File[s]",
	Prism = 0x52; "Prism",
	AVR = 0x53; "AVR",
	FR30 = 0x54; "Fujitsu FR30",
	D10V = 0x55; "D10V",
	D30V = 0x56; "D30V",
	V850 = 0x57; "NEC v850",
	M32R = 0x58; "M32R",
	MN10300 = 0x59; "MN10300",
	MN10200 = 0x5A; "MN10200",
	PicoJava = 0x5B; "picoJava",
	OpenRISC = 0x5C; "OpenRISC",
	ARCCompat = 0x5D; "ARC Compat",
	XTensa = 0x5E; "XTensa",
	VideoCore = 0x5F; "VideoCore",
	GPP = 0x60; "GPP",
	NS32K = 0x61; "National Semiconductors 32000",
	TPC = 0x62; "TPC",
	SNP1K = 0x63; "SNP1K",
	ST200 = 0x64; "ST200",
	IP2K = 0x65; "IP2K",
	MAX = 0x66; "MAX",
	CompactRISC = 0x67; "CompactRISC",
	F2MC16 = 0x68; "Fujitsu F2MC16",
	TexasInstrumentsMSP430 = 0x69; "Texas Instruments msp430",
	Blackfin = 0x6A; "Blackfin",
	S1C33 = 0x6B; "S1C33",
	SEP = 0x6C; "Sharp Embedded Microprocessor",
	ArcaRISC = 0x6D; "Arca RISC",
	UniCore = 0x6E; "UniCore",
	Excess = 0x6F; "eXcess",
	DXP = 0x70; "DXP",
	Nios2 = 0x71; "Nios II",
	CompactRISCCRX = 0x72; "CompactRISC CRX",
	MotorolaXGATE = 0x73; "Motorola XGATE",
	C166 = 0x74; "C166",
	M16C = 0x75; "M16C",
	DsPIC30F = 0x76; "dsPIC30F",
	FCERISC = 0x77; "Freescale Communication Engine RISC",
	M32C = 0x78; "M32C",
	TSK3000 = 0x83; "TSK3000",
	RS08 = 0x84; "Freescale RS08",
	SHARC = 0x85; "SHARC",
	ECOG2 = 0x86; "eCOG2",
	SCore7RISC = 0x87; "Sunplus S+core7 RISC",
	DSP24 = 0x88; "DSP24",
	VideoCore3 = 0x89; "VideoCore III",
	LatticeRISC = 0x8A; "Lattice FPGA RISC",
	SEC17 = 0x8B; "Seiko Epson C17",
	TexasInstrumentsC6000 = 0x8C; "Texas Instruments TMS320C6000",
	TexasInstrumentsC2000 = 0x8D; "Texas Instruments TMS320C2000",
	TexasInstrumentsC5500 = 0x8E; "Texas Instruments TMS320C55x",
	TexasInstrumentsSpecificRISC = 0x8F; "Texas Instruments Specific RISC",
	TexasInstrumentsRealtime = 0x90; "Texas Instruments Realtime Unit",
	MMDSPPlus = 0xA0; "MMDSPPlus",
	CypressM8C = 0xA1; "Cypress M8C",
	R32C = 0xA2; "R32C",
	TriMedia = 0xA3; "TriMedia",
	QDSP6 = 0xA4; "QUALCOMM DSP6",
	Intel8051 = 0xA5; "Intel 8051",
	STxP7x = 0xA6; "STxP7x",
	NDS32RISC = 0xA7; "Compact Code Embedded RISC",
	ECOG1X = 0xA8; "eCOG1X",
	MAXQ30 = 0xA9; "MAXQ30",
	XIM016 = 0xAA; "XIMO16",
	M2000ReconfigurableRISC = 0xAB; "M2000 Reconfigurable RISC",
	CrayNV2 = 0xAC; "Cray NV2",
	RX = 0xAD; "RX",
	METAG = 0xAE; "META",
	Elbrus = 0xAF; "Elbrus",
	ECOG16 = 0xB0; "eCOG16",
	CompactRISCCR16 = 0xB1; "Compact RISC CR16",
	ETPU = 0xB2; "Freescale Extended Time Processing Unit",
	SLE9X = 0xB3; "SLE9X",
	L10M = 0xB4; "Intel L10M",
	K10M = 0xB5; "Intel K10M",
	AArch64 = 0xB7; "ARM [AArch64] [64-bit]",
	AVR32 = 0xB9; "AVR32",
	STM8 = 0xBA; "STM8",
	TILE64 = 0xBB; "TILE64",
	TILEPro = 0xBC; "TILEPro",
	MicroBlaze = 0xBD; "MicroBlaze",
	CUDA = 0xBE; "NVIDIA CUDA",
	TILEGx = 0xBF; "TILE-Gx",
	CloudShield = 0xC0; "CloudShield",
	CoreA1st = 0xC1; "Core-A 1st Generation",
	CoreA2nd = 0xC2; "Core-A 2nd Generation",
	ARCv2 = 0xC3; "ARCv2",
	Open8RISC = 0xC4; "Open8 RISC",
	RL78 = 0xC5; "RL78",
	VideoCore5 = 0xC6; "VideoCore V",
	R78KOR = 0xC7; "78KOR",
	F56800EX = 0xC8; "Freescale 56800EX",
	BA1 = 0xC9; "BA1",
	BA2 = 0xCA; "BA2",
	XCORE = 0xCB; "xCORE",
	MicroPIC = 0xCD; "Microchip PIC",
	IntelGraphicsTech = 0xCE; "Intel Graphics Technology",
	KM32 = 0xD2; "KM211 KM32",
	KMX32 = 0xD3; "KM211 KMX32",
	KMX16 = 0xD4; "KM211 KMX16",
	KMX8 = 0xD5; "KM211 KMX8",
	KVARC = 0xD6; "KM211 KVARC",
	CDP = 0xD7; "CDP",
	COGE = 0xD8; "Cognitive Smart Memory Processor",
	CoolEngine = 0xD9; "CoolEngine",
	NanoradioOptimisedRISC = 0xDA; "Nanoradio Optimised RISC",
	Kalimba = 0xDB; "Kalimba",
	Z80 = 0xDC; "Z80",
	VISIUMCore = 0xDD; "VISIUM Core",
	FT32 = 0xDE; "FT32",
	Moxie = 0xDF; "Moxie",
	AMDGPU = 0xE0; "AMD GPU",
	RISCV = 0xF3; "RISC-V",
	LinuxBPF = 0xF7; "Linux In-Kernel Virtual Machine [BPF]",
	CSKY = 0xFC; "C-SKY",
	LoongArch = 0x102; "LoongArch",
	Alpha = 0x9026; "Alpha";
	=> else Unofficial,
);

// https://www.sco.com/developers/devspecs/mipsabi.pdf
// https://web.archive.org/web/20130523153224/http://www.sparc.org/standards/psABI3rd.pdf
// https://refspecs.linuxbase.org/elf/elf-pa.pdf
convertible!(
	ProgramType as u32;
	Null = 0x0; "Unused Entry",
	Load = 0x1; "Loadable",
	Dynamic = 0x2; "Dynamic Linking Information",
	Interpreter = 0x3; "Interpreter",
	Note = 0x4; "Note[s]",
	_Reserved = 0x5; "Reserved",
	ProgramHeaderTable = 0x6; "Header Table Entry",
	ThreadLocalStorage = 0x7; "Thread Local Storage",
	GNUEhFrame = 0x6474E550; "GNU `.eh_frame_hdr`",
	GNUStack = 0x6474E551; "GNU Stack Note",
	GNUReadOnlyAfterRelocation = 0x6474E552; "[GNU] Read-Only After Relocation",
	GNUProperty = 0x6474E553; "GNU Property",
	GNUSFrame = 0x6474E554; "GNU SFrame",
	SUNBss = 0x6FFFFFFA; "SUN `.bss`",
	SUNStack = 0x6FFFFFFB; "SUN Stack";
	SUNSpecific in 0x6FFFFFFA..=0x6FFFFFFF,
	OSSpecific in 0x60000000..=0x6FFFFFFF,
	ProcessorSpecific in 0x70000000..=0x7FFFFFFF,
);

convertible!(
	SectionType as u64;
	Null = 0x0; "Unused Entry",
	Progbits = 0x1; "Program Data (`@progbits`)",
	SymbolTable = 0x2; "Symbol Table",
	StringTable = 0x3; "String Table",
	RelocationAddends = 0x4; "Relocation Entries [w/ Addends]",
	SymbolHashTable = 0x5; "Symbol hash Table",
	Dynamic = 0x6; "Dynamic Linking Information",
	Note = 0x7; "Note[s] (`@note`)",
	Bss = 0x8; "Null Program Data",
	RelocationNoAddends = 0x9; "Relocation Entries [w/o Addends]",
	_Reserved = 0xA; "Reserved",
	DynamicSymbolTable = 0xB; "Dynamic Symbol Table",
	InitArray = 0xE; "Initialisation Array (`@init_array`)",
	FiniArray = 0xF; "Finalisation Array (`@fini_array`)",
	PreinitArray = 0x10; "Pre-Initialisation Array (`@preinit_array`)",
	SectionGroup = 0x11; "Section Group",
	SymbolTableIndices = 0x12; "Extended Symbol Table Indices",
	RelativeRelocations = 0x13; "Relative Relocation[s]",
	GNUObjectAttributes = 0x6FFFFFF5; "GNU Object Attribute[s]",
	GNUHashTable = 0x6FFFFFF6; "GNU Hash Table",
	GNULibraryList = 0x6FFFFFF7; "GNU [Pre-Link] Library List",
	Checksum = 0x6FFFFFF8; "Checksum",
	SUNMove = 0x6FFFFFFA; "SUN Move",
	SUNCOMDAT = 0x6FFFFFFB; "SUN COMDAT",
	SUNSyminfo = 0x6FFFFFFC; "SUN Syminfo",
	GNUVersionDefinition = 0x6FFFFFFD; "GNU Version Definition",
	GNUVersionNeeds = 0x6FFFFFFE; "GNU Version Needs",
	GNUVersionSymbolTable = 0x6FFFFFFF; "GNU Version Symbol Table";
	OSSpecific in 0x60000000..=0x6FFFFFFF,
	ProcessorSpecific in 0x70000000..=0x7FFFFFFF,
	ApplicationSpecific in 0x80000000..=0x8FFFFFFF,
);
