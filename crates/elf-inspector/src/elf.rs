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

// TODO: Add an extra field to the macro naming to do string representation
macro_rules! convertible {
	($name:ident as $rep:ty; $($key:ident = $value:expr),* ; $($key_range:ident in $value_range:pat),* $(=> else $catch_all:ident)? $(,)?) => {
		#[repr($rep)]
		#[derive(Debug, Clone, Copy)]
		#[derive_const(PartialEq, Eq)]
		pub enum $name {
			$($key = $value,)*
			$($key_range($rep),)*
			$($catch_all($rep),)?
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
	SysV = 0x0,
	HPUX = 0x1,
	NetBSD = 0x2,
	Linux = 0x3,
	Solaris = 0x6,
	AIX = 0x7,
	IRIX = 0x8,
	FreeBSD = 0x9,
	TRU64 = 0xA,
	Modesto = 0xB,
	OpenBSD = 0xC,
	ARMEABI = 0x40,
	ARM = 0x61,
	Standalone = 0xFF;
);

convertible!(
	ELFType as u16;
	None = 0x0,
	Relocatable = 0x1,
	Executable = 0x2,
	Dynamic = 0x3,
	CoreDump = 0x4;
	OSSpecific in 0xFE00..=0xFEFF,
	ProcessorSpecific in 0xFF00..=0xFFFF,
);

convertible!(
	Machine as u16;
	None = 0x0,
	M32 = 0x1,
	SPARC = 0x2,
	Intel386 = 0x3,
	Motorola68K = 0x4,
	Motorola88K = 0x5,
	IntelMCU = 0x6,
	Intel860 = 0x7,
	MIPS = 0x8,
	S370 = 0x9,
	MIPSR3000LittleEndian = 0xA,
	PrecisionArchitectureRISC = 0xF,
	VPP500 = 0x11,
	SPARC32Plus = 0x12,
	Intel960 = 0x13,
	PowerPC = 0x14,
	PowerPC64 = 0x15,
	S390 = 0x16,
	SPU = 0x17,
	V800 = 0x24,
	FR20 = 0x25,
	RH32 = 0x26,
	MotorolaRCE = 0x27,
	ARM = 0x28,
	DigitalAlpha = 0x29,
	SH = 0x2A,
	SPARCv9 = 0x2B,
	Tricore = 0x2C,
	ARC = 0x2D,
	H8300 = 0x2E,
	H8300H = 0x2F,
	H8S = 0x30,
	H8500 = 0x31,
	IntelItanium64 = 0x32,
	MIPSX = 0x33,
	MotorolaColdfire = 0x34,
	Motorola68HC12 = 0x35,
	MMA = 0x36,
	PCP = 0x37,
	NCPU = 0x38,
	NDR1 = 0x39,
	MotorolaStarCore = 0x3A,
	ME16 = 0x3B,
	ST100 = 0x3C,
	TinyJ = 0x3D,
	X86_64 = 0x3E,
	PDSP = 0x3F,
	PDP10 = 0x40,
	PDP11 = 0x41,
	FX66 = 0x42,
	ST9Plus = 0x43,
	ST7 = 0x44,
	MotorolaC68HC16 = 0x45,
	MotorolaC68HC11 = 0x46,
	MotorolaC68HC08 = 0x47,
	MotorolaC68HC05 = 0x48,
	SVx = 0x49,
	ST19 = 0x4A,
	VAX = 0x4B,
	CRIS = 0x4C,
	Javelin = 0x4D,
	Firepath = 0x4E,
	ZSP = 0x4F,
	MMIX = 0x50,
	HarvardUniAny = 0x51,
	Prism = 0x52,
	AVR = 0x53,
	FR30 = 0x54,
	D10V = 0x55,
	D30V = 0x56,
	V850 = 0x57,
	M32R = 0x58,
	MN10300 = 0x59,
	MN10200 = 0x5A,
	PicoJava = 0x5B,
	OpenRISC = 0x5C,
	ARCCompat = 0x5D,
	XTensa = 0x5E,
	VideoCore = 0x5F,
	GPP = 0x60,
	NS32K = 0x61,
	TPC = 0x62,
	SNP1K = 0x63,
	ST200 = 0x64,
	IP2K = 0x65,
	MAX = 0x66,
	CompatRISC = 0x67,
	F2MC16 = 0x68,
	TexasInstrumentsMSP430 = 0x69,
	Blackfin = 0x6A,
	S1C33 = 0x6B,
	SEP = 0x6C,
	ArcaRISC = 0x6D,
	UniCore = 0x6E,
	Excess = 0x6F,
	DXP = 0x70,
	NIOS2 = 0x71,
	CompatRISCX = 0x72,
	MotorolaXGATE = 0x73,
	C166 = 0x74,
	M16C = 0x75,
	DsPIC30F = 0x76,
	FCERISC = 0x77,
	M32C = 0x78,
	TSK3000 = 0x83,
	RS08 = 0x84,
	SHARC = 0x85,
	ECOG2 = 0x86,
	SCore7RISC = 0x87,
	DSP24 = 0x88,
	VideoCore3 = 0x89,
	LatticeRISC = 0x8A,
	SEC17 = 0x8B,
	TexasInstrumentsC6000 = 0x8C,
	TexasInstrumentsC2000 = 0x8D,
	TexasInstrumentsC5500 = 0x8E,
	TexasInstrumentsSpecificRISC = 0x8F,
	TexasInstrumentsRealtime = 0x90,
	MMDSPPlus = 0xA0,
	CypressM8C = 0xA1,
	R32C = 0xA2,
	TriMedia = 0xA3,
	QDSP6 = 0xA4,
	Intel8051 = 0xA5,
	STXP7X = 0xA6,
	NDS32RISC = 0xA7,
	ECOG1X = 0xA8,
	MAXQ30 = 0xA9,
	XIM016 = 0xAA,
	M2KReconfigurableRISC = 0xAB,
	CrayNV2 = 0xAC,
	RX = 0xAD,
	METAG = 0xAE,
	Elbrus = 0xAF,
	ECOG16 = 0xB0,
	CompactRISC16 = 0xB1,
	ETPU = 0xB2,
	SLE9X = 0xB3,
	L10M = 0xB4,
	K10M = 0xB5,
	AArch64 = 0xB7,
	AVR32 = 0xB9,
	STM8 = 0xBA,
	TILE64 = 0xBB,
	TILEPro = 0xBC,
	MicroBlaze = 0xBD,
	CUDA = 0xBE,
	TILEGx = 0xBF,
	CloudShield = 0xC0,
	CoreA1st = 0xC1,
	CoreA2nd = 0xC2,
	ARCv2 = 0xC3,
	Open8RISC = 0xC4,
	RL78 = 0xC5,
	VideoCore5 = 0xC6,
	R78KOR = 0xC7,
	F56800EX = 0xC8,
	BA1 = 0xC9,
	BA2 = 0xCA,
	XCORE = 0xCB,
	MicroPIC = 0xCD,
	IntelGraphicsTech = 0xCE,
	KM32 = 0xD2,
	KMX32 = 0xD3,
	KMX16 = 0xD4,
	KMX8 = 0xD5,
	KVARC = 0xD6,
	CDP = 0xD7,
	COGE = 0xD8,
	CoolEngine = 0xD9,
	NanoradioOptimisedRISC = 0xDA,
	Kalimba = 0xDB,
	Z80 = 0xDC,
	VISIUMCore = 0xDD,
	FT32 = 0xDE,
	Moxie = 0xDF,
	AMDGPU = 0xE0,
	RISCV = 0xF3,
	LinuxBPF = 0xF7,
	CSKY = 0xFC,
	LoongArch = 0x102,
	Alpha = 0x9026;
	=> else Unofficial,
);

// https://www.sco.com/developers/devspecs/mipsabi.pdf
// https://web.archive.org/web/20130523153224/http://www.sparc.org/standards/psABI3rd.pdf
// https://refspecs.linuxbase.org/elf/elf-pa.pdf
convertible!(
	ProgramType as u32;
	Null = 0x0,
	Load = 0x1,
	Dynamic = 0x2,
	Interpreter = 0x3,
	Note = 0x4,
	_Reserved = 0x5,
	ProgramHeaderTable = 0x6,
	ThreadLocalStorage = 0x7,
	GNUEhFrame = 0x6474E550,
	GNUStack = 0x6474E551,
	GNUReadOnlyAfterRelocation = 0x6474E552,
	GNUProperty = 0x6474E553,
	GNUSFrame = 0x6474E554,
	SUNBss = 0x6FFFFFFA,
	SUNStack = 0x6FFFFFFB;
	SUNSpecific in 0x6FFFFFFA..=0x6FFFFFFF,
	OSSpecific in 0x60000000..=0x6FFFFFFF,
	ProcessorSpecific in 0x70000000..=0x7FFFFFFF,
);

convertible!(
	SectionType as u64;
	Null = 0x0,
	Progbits = 0x1,
	SymbolTable = 0x2,
	StringTable = 0x3,
	RelocationAddends = 0x4,
	SymbolHashTable = 0x5,
	Dynamic = 0x6,
	Note = 0x7,
	Bss = 0x8,
	RelocationNoAddends = 0x9,
	_Reserved = 0xA,
	DynamicSymbolTable = 0xB,
	InitArray = 0xE,
	FiniArray = 0xF,
	PreinitArray = 0x10,
	SectionGroup = 0x11,
	SymbolTableIndices = 0x12,
	RelativeRelocations = 0x13,
	GNUObjectAttributes = 0x6FFFFFF5,
	GNUHashTable = 0x6FFFFFF6,
	GNULibraryList = 0x6FFFFFF7,
	Checksum = 0x6FFFFFF8,
	SUNMove = 0x6FFFFFFA,
	SUNCOMDAT = 0x6FFFFFFB,
	SUNSyminfo = 0x6FFFFFFC,
	GNUVersionDefinition = 0x6FFFFFFD,
	GNUVersionNeeds = 0x6FFFFFFE,
	GNUVersionSymbolTable = 0x6FFFFFFF;
	OSSpecific in 0x60000000..=0x6FFFFFFF,
	ProcessorSpecific in 0x70000000..=0x7FFFFFFF,
	ApplicationSpecific in 0x80000000..=0x8FFFFFFF,
);
