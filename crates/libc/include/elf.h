#pragma once
#include <internal/__defs.h>
#include <stdint.h>

#define ELFMAG1 'E'
#define ELFMAG3 'F'
#define ELFMAG2 'L'
#define ELF64_R_INFO(sym, type) ((((Elf64_Xword) (sym)) << 32) + (type))
#define ELF32_ST_INFO(bind, type) (((bind) << 4) + ((type) & 0xF))
#define ELF32_R_INFO(sym, type) (((sym) << 8) + ((type) & 0xFF))
#define ELF32_M_INFO(sym, size) (((sym) << 8) + (unsigned char) (size))
#define ELF32_ST_BIND(val) (((unsigned char) (val)) >> 4)
#define ELF64_R_TYPE(i) ((i) & 0xFFFFFFFF)
#define ELF64_R_SYM(i) ((i) >> 32)
#define ELF32_M_SYM(info) ((info) >> 8)
#define ELF32_ST_VISIBILITY(o) ((o) & 0x03)
#define ELF32_M_SIZE(info) ((unsigned char) (info))
#define ELF32_ST_TYPE(val) ((val) & 0xF)
#define ELF32_R_TYPE(val) ((val) & 0xFF)
#define ELF32_R_SYM(val) ((val) >> 8)
#define PF_X (1 << 0)
#define SHF_WRITE (1 << 0)
#define PF_W (1 << 1)
#define SHF_ALLOC (1 << 1)
#define SHF_TLS (1 << 10)
#define PF_R (1 << 2)
#define SHF_EXECINSTR (1 << 2)
#define SHF_MERGE (1 << 4)
#define SHF_STRINGS (1 << 5)
#define SHF_INFO_LINK (1 << 6)
#define SHF_LINK_ORDER (1 << 7)
#define SHF_OS_NONCONFORMING (1 << 8)
#define SHF_GROUP (1 << 9)
#define EI_NIDENT (16)
#define DT_ADDRTAGIDX(tag) (DT_ADDRRNGHI - (tag))
#define DT_IA_64_PLT_RESERVE (DT_LOPROC + 0)
#define DT_PPC64_GLINK (DT_LOPROC + 0)
#define DT_PPC_GOT (DT_LOPROC + 0)
#define DT_PPC64_OPD (DT_LOPROC + 1)
#define DT_PPC64_OPDSZ (DT_LOPROC + 2)
#define DT_VALTAGIDX(tag) (DT_VALRNGHI - (tag))
#define DT_VERSIONTAGIDX(tag) (DT_VERNEEDNUM - (tag))
#define PT_IA_64_ARCHEXT (PT_LOPROC + 0)
#define PT_IA_64_UNWIND (PT_LOPROC + 1)
#define SHT_IA_64_EXT (SHT_LOPROC + 0)
#define SHT_IA_64_UNWIND (SHT_LOPROC + 1)
#define DT_NULL 0
#define EI_MAG0 0
#define ELFCLASSNONE 0
#define ELFDATANONE 0
#define ELFOSABI_NONE 0
#define ELFOSABI_SYSV 0
#define ELF_NOTE_OS_LINUX 0
#define EM_NONE 0
#define ET_NONE 0
#define EV_NONE 0
#define PT_NULL 0
#define R_386_NONE 0
#define R_390_NONE 0
#define R_PPC_NONE 0
#define R_X86_64_NONE 0
#define SHN_UNDEF 0
#define SHT_NULL 0
#define STB_LOCAL 0
#define STN_UNDEF 0
#define STT_NOTYPE 0
#define STV_DEFAULT 0
#define SYMINFO_NONE 0
#define R_IA64_NONE 0x00
#define DF_1_NOW 0x00000001
#define DF_ORIGIN 0x00000001 /* Object may use
DF_ORIGIN */
#define DF_P1_LAZYLOAD 0x00000001
#define DTF_1_PARINIT 0x00000001
#define EF_S390_HIGH_GPRS 0x00000001
#define DF_1_GLOBAL 0x00000002
#define DF_P1_GROUPPERM 0x00000002
#define DF_SYMBOLIC 0x00000002 /* Symbol resolutions
start with this object */
#define DTF_1_CONFEXP 0x00000002
#define DF_1_GROUP 0x00000004
#define DF_TEXTREL 0x00000004 /* Object contains text
relocations */
#define DF_1_NODELETE 0x00000008
#define DF_BIND_NOW 0x00000008 /* No lazy binding for
this object */
#define EF_IA_64_MASKOS 0x0000000F
#define DF_1_LOADFLTR 0x00000010
#define DF_STATIC_TLS 0x00000010 /* Module uses the static
TLS model */
#define EF_IA_64_ABI64 0x00000010
#define DF_1_INITFIRST 0x00000020
#define DF_1_NOOPEN 0x00000040
#define DF_1_ORIGIN 0x00000080
#define DF_1_DIRECT 0x00000100
#define DF_1_TRANS 0x00000200
#define DF_1_INTERPOSE 0x00000400
#define DF_1_NODEFLIB 0x00000800
#define DF_1_NODUMP 0x00001000
#define DF_1_CONFALT 0x00002000
#define DF_1_ENDFILTEE 0x00004000
#define DF_1_DISPRELDNE 0x00008000
#define SYMINFO_FLG_DIRECT 0x0001
#define DF_1_DISPRELPND 0x00010000
#define SYMINFO_FLG_PASSTHRU 0x0002
#define SYMINFO_FLG_COPY 0x0004
#define SYMINFO_FLG_LAZYLOAD 0x0008
#define EF_CPU32 0x00810000
#define PF_MASKOS 0x0FF00000
#define SHF_MASKOS 0x0FF00000
#define GRP_COMDAT 0x1
#define SHF_IA_64_SHORT 0x10000000
#define SHF_IA_64_NORECOV 0x20000000
#define R_IA64_IMM14 0x21
#define R_IA64_IMM22 0x22
#define R_IA64_IMM64 0x23
#define R_IA64_DIR32MSB 0x24
#define R_IA64_DIR32LSB 0x25
#define R_IA64_DIR64MSB 0x26
#define R_IA64_DIR64LSB 0x27
#define R_IA64_GPREL22 0x2A
#define R_IA64_GPREL64I 0x2B
#define R_IA64_GPREL32MSB 0x2C
#define R_IA64_GPREL32LSB 0x2D
#define R_IA64_GPREL64MSB 0x2E
#define R_IA64_GPREL64LSB 0x2F
#define R_IA64_LTOFF22 0x32
#define R_IA64_LTOFF64I 0x33
#define R_IA64_PLTOFF22 0x3A
#define R_IA64_PLTOFF64I 0x3B
#define R_IA64_PLTOFF64MSB 0x3E
#define R_IA64_PLTOFF64LSB 0x3F
#define R_IA64_FPTR64I 0x43
#define R_IA64_FPTR32MSB 0x44
#define R_IA64_FPTR32LSB 0x45
#define R_IA64_FPTR64MSB 0x46
#define R_IA64_FPTR64LSB 0x47
#define R_IA64_PCREL60B 0x48
#define R_IA64_PCREL21B 0x49
#define R_IA64_PCREL21M 0x4A
#define R_IA64_PCREL21F 0x4B
#define R_IA64_PCREL32MSB 0x4C
#define R_IA64_PCREL32LSB 0x4D
#define R_IA64_PCREL64MSB 0x4E
#define R_IA64_PCREL64LSB 0x4F
#define R_IA64_LTOFF_FPTR22 0x52
#define R_IA64_LTOFF_FPTR64I 0x53
#define R_IA64_LTOFF_FPTR32MSB 0x54
#define R_IA64_LTOFF_FPTR32LSB 0x55
#define R_IA64_LTOFF_FPTR64MSB 0x56
#define R_IA64_LTOFF_FPTR64LSB 0x57
#define R_IA64_SEGREL32MSB 0x5C
#define R_IA64_SEGREL32LSB 0x5D
#define R_IA64_SEGREL64MSB 0x5E
#define R_IA64_SEGREL64LSB 0x5F
#define PT_LOOS 0x60000000
#define SHT_LOOS 0x60000000
#define DT_LOOS 0x6000000D
#define R_IA64_SECREL32MSB 0x64
#define PT_GNU_EH_FRAME 0x6474E550
#define PT_GNU_STACK 0x6474E551
#define PT_GNU_RELRO 0x6474E552
#define R_IA64_SECREL32LSB 0x65
#define R_IA64_SECREL64MSB 0x66
#define R_IA64_SECREL64LSB 0x67
#define R_IA64_REL32MSB 0x6C
#define R_IA64_REL32LSB 0x6D
#define R_IA64_REL64MSB 0x6E
#define R_IA64_REL64LSB 0x6F
#define DT_HIOS 0x6FFFF000
#define DT_VALRNGLO 0x6FFFFD00
#define DT_GNU_PRELINKED 0x6FFFFDF5
#define DT_GNU_CONFLICTSZ 0x6FFFFDF6
#define DT_GNU_LIBLISTSZ 0x6FFFFDF7
#define DT_CHECKSUM 0x6FFFFDF8
#define DT_PLTPADSZ 0x6FFFFDF9
#define DT_MOVEENT 0x6FFFFDFA
#define DT_MOVESZ 0x6FFFFDFB
#define DT_FEATURE_1 0x6FFFFDFC
#define DT_POSFLAG_1 0x6FFFFDFD
#define DT_SYMINSZ 0x6FFFFDFE
#define DT_SYMINENT 0x6FFFFDFF
#define DT_VALRNGHI 0x6FFFFDFF
#define DT_ADDRRNGLO 0x6FFFFE00
#define DT_GNU_HASH 0x6FFFFEF5
#define DT_TLSDESC_PLT 0x6FFFFEF6
#define DT_TLSDESC_GOT 0x6FFFFEF7
#define DT_GNU_CONFLICT 0x6FFFFEF8
#define DT_GNU_LIBLIST 0x6FFFFEF9
#define DT_CONFIG 0x6FFFFEFA
#define DT_DEPAUDIT 0x6FFFFEFB
#define DT_AUDIT 0x6FFFFEFC
#define DT_PLTPAD 0x6FFFFEFD
#define DT_MOVETAB 0x6FFFFEFE
#define DT_ADDRRNGHI 0x6FFFFEFF
#define DT_SYMINFO 0x6FFFFEFF
#define DT_VERSYM 0x6FFFFFF0
#define SHT_GNU_ATTRIBUTES 0x6FFFFFF5
#define SHT_GNU_HASH 0x6FFFFFF6
#define SHT_GNU_LIBLIST 0x6FFFFFF7
#define SHT_CHECKSUM 0x6FFFFFF8
#define DT_RELACOUNT 0x6FFFFFF9
#define DT_RELCOUNT 0x6FFFFFFA
#define DT_FLAGS_1 0x6FFFFFFB
#define DT_VERDEF 0x6FFFFFFC
#define DT_VERDEFNUM 0x6FFFFFFD
#define SHT_GNU_verdef 0x6FFFFFFD
#define DT_VERNEED 0x6FFFFFFE
#define SHT_GNU_verneed 0x6FFFFFFE
#define DT_VERNEEDNUM 0x6FFFFFFF
#define PT_HIOS 0x6FFFFFFF
#define SHT_GNU_versym 0x6FFFFFFF
#define SHT_HIOS 0x6FFFFFFF
#define DT_LOPROC 0x70000000
#define PT_LOPROC 0x70000000
#define SHT_LOPROC 0x70000000
#define R_IA64_LTV32MSB 0x74
#define R_IA64_LTV32LSB 0x75
#define R_IA64_LTV64MSB 0x76
#define R_IA64_LTV64LSB 0x77
#define R_IA64_PCREL21BI 0x79
#define R_IA64_PCREL22 0x7A
#define R_IA64_PCREL64I 0x7B
#define ELFMAG0 0x7F
#define DT_AUXILIARY 0x7FFFFFFD
#define DT_FILTER 0x7FFFFFFF
#define DT_HIPROC 0x7FFFFFFF
#define PT_HIPROC 0x7FFFFFFF
#define SHT_HIPROC 0x7FFFFFFF
#define R_IA64_IPLTMSB 0x80
#define PF_IA_64_NORECOV 0x80000000
#define SHT_LOUSER 0x80000000
#define R_IA64_IPLTLSB 0x81
#define R_IA64_COPY 0x84
#define R_IA64_SUB 0x85
#define R_IA64_LTOFF22X 0x86
#define R_IA64_LDXMOV 0x87
#define SHT_HIUSER 0x8FFFFFFF
#define R_IA64_TPREL14 0x91
#define R_IA64_TPREL22 0x92
#define R_IA64_TPREL64I 0x93
#define R_IA64_TPREL64MSB 0x96
#define R_IA64_TPREL64LSB 0x97
#define R_IA64_LTOFF_TPREL22 0x9A
#define R_IA64_DTPMOD64MSB 0xA6
#define R_IA64_DTPMOD64LSB 0xA7
#define R_IA64_LTOFF_DTPMOD22 0xAA
#define R_IA64_DTPREL14 0xB1
#define R_IA64_DTPREL22 0xB2
#define R_IA64_DTPREL64I 0xB3
#define R_IA64_DTPREL32MSB 0xB4
#define R_IA64_DTPREL32LSB 0xB5
#define R_IA64_DTPREL64MSB 0xB6
#define R_IA64_DTPREL64LSB 0xB7
#define R_IA64_LTOFF_DTPREL22 0xBA
#define PF_MASKPROC 0xF0000000
#define SHF_MASKPROC 0xF0000000
#define ET_LOOS 0xFE00
#define ET_HIOS 0xFEFF
#define ET_LOPROC 0xFF00
#define SHN_LOPROC 0xFF00
#define SHN_LORESERVE 0xFF00
#define SYMINFO_BT_LOWRESERVE 0xFF00
#define EF_IA_64_ARCH 0xFF000000
#define SHN_HIPROC 0xFF1F
#define SHN_LOOS 0xFF20
#define SHN_HIOS 0xFF3F
#define SHN_ABS 0xFFF1
#define SHN_COMMON 0xFFF2
#define SYMINFO_BT_PARENT 0xFFFE
#define ET_HIPROC 0xFFFF
#define PN_XNUM 0xFFFF
#define SHN_HIRESERVE 0xFFFF
#define SHN_XINDEX 0xFFFF
#define SYMINFO_BT_SELF 0xFFFF
#define DT_IA_64_NUM 1
#define DT_NEEDED 1
#define DT_PPC_NUM 1
#define EI_MAG1 1
#define ELFCLASS32 1
#define ELFDATA2LSB 1
#define ELF_NOTE_OS_GNU 1
#define ET_REL 1
#define EV_CURRENT 1
#define NT_GNU_ABI_TAG 1
#define PT_LOAD 1
#define R_386_32 1
#define R_390_8 1
#define R_PPC_ADDR32 1
#define R_X86_64_64 1
#define SHT_PROGBITS 1
#define STB_GLOBAL 1
#define STT_OBJECT 1
#define STV_INTERNAL 1
#define SYMINFO_CURRENT 1
#define DT_STRSZ 10
#define R_386_GOTPC 10
#define R_390_GLOB_DAT 10
#define R_PPC_REL24 10
#define R_X86_64_32 10
#define SHT_SHLIB 10
#define STB_LOOS 10
#define STT_GNU_IFUNC 10
#define STT_LOOS 10
#define R_PPC64_TPREL16_HIGHESTA 100
#define R_PPC64_DTPREL16_DS 101
#define R_PPC64_DTPREL16_LO_DS 102
#define R_PPC64_DTPREL16_HIGHER 103
#define R_PPC64_DTPREL16_HIGHERA 104
#define R_PPC64_DTPREL16_HIGHEST 105
#define R_PPC64_DTPREL16_HIGHESTA 106
#define DT_ADDRNUM 11
#define DT_SYMENT 11
#define R_386_32PLT 11
#define R_390_JMP_SLOT 11
#define R_PPC_REL14 11
#define R_X86_64_32S 11
#define SHT_DYNSYM 11
#define DT_INIT 12
#define DT_VALNUM 12
#define R_390_RELATIVE 12
#define R_PPC_REL14_BRTAKEN 12
#define R_X86_64_16 12
#define STB_HIOS 12
#define STT_HIOS 12
#define DT_FINI 13
#define R_390_GOTOFF32 13
#define R_PPC_REL14_BRNTAKEN 13
#define R_X86_64_PC16 13
#define STB_LOPROC 13
#define STT_LOPROC 13
#define DT_SONAME 14
#define R_386_TLS_TPOFF 14
#define R_390_GOTPC 14
#define R_PPC_GOT16 14
#define R_X86_64_8 14
#define SHT_INIT_ARRAY 14
#define DT_RPATH 15
#define R_386_TLS_IE 15
#define R_390_GOT16 15
#define R_PPC_GOT16_LO 15
#define R_X86_64_PC8 15
#define SHT_FINI_ARRAY 15
#define STB_HIPROC 15
#define STT_HIPROC 15
#define DT_SYMBOLIC 16
#define DT_VERSIONTAGNUM 16
#define R_386_TLS_GOTIE 16
#define R_390_PC16 16
#define R_PPC_GOT16_HI 16
#define R_X86_64_DTPMOD64 16
#define SHT_PREINIT_ARRAY 16
#define DT_REL 17
#define R_386_TLS_LE 17
#define R_390_PC16DBL 17
#define R_PPC_GOT16_HA 17
#define R_X86_64_DTPOFF64 17
#define SHT_GROUP 17
#define DT_RELSZ 18
#define R_386_TLS_GD 18
#define R_390_PLT16DBL 18
#define R_PPC_PLTREL24 18
#define R_X86_64_TPOFF64 18
#define SHT_SYMTAB_SHNDX 18
#define DT_RELENT 19
#define R_386_TLS_LDM 19
#define R_390_PC32DBL 19
#define R_PPC_COPY 19
#define R_X86_64_TLSGD 19
#define DT_PLTRELSZ 2
#define EI_MAG2 2
#define ELFCLASS64 2
#define ELFDATA2MSB 2
#define ELF_NOTE_OS_SOLARIS2 2
#define ET_EXEC 2
#define EV_NUM 2
#define PT_DYNAMIC 2
#define R_386_PC32 2
#define R_390_12 2
#define R_PPC_ADDR24 2
#define R_X86_64_PC32 2
#define SHT_SYMTAB 2
#define STB_WEAK 2
#define STT_FUNC 2
#define STV_HIDDEN 2
#define SYMINFO_NUM 2
#define DT_PLTREL 20
#define EM_PPC 20
#define R_386_16 20
#define R_390_PLT32DBL 20
#define R_PPC_GLOB_DAT 20
#define R_X86_64_TLSLD 20
#define DT_DEBUG 21
#define EM_PPC64 21
#define R_386_PC16 21
#define R_390_GOTPCDBL 21
#define R_PPC_JMP_SLOT 21
#define R_X86_64_DTPOFF32 21
#define DT_TEXTREL 22
#define EM_S390 22
#define R_386_8 22
#define R_390_64 22
#define R_PPC_RELATIVE 22
#define R_X86_64_GOTTPOFF 22
#define DT_JMPREL 23
#define R_386_PC8 23
#define R_390_PC64 23
#define R_PPC_LOCAL24PC 23
#define R_X86_64_TPOFF32 23
#define DT_BIND_NOW 24
#define R_386_TLS_GD_32 24
#define R_390_GOT64 24
#define R_PPC_UADDR32 24
#define R_X86_64_PC64 24
#define R_PPC64_JMP_IREL 247
#define R_PPC64_IRELATIVE 248
#define R_PPC_IRELATIVE 248
#define R_PPC64_REL16 249
#define R_PPC_REL16 249
#define DT_INIT_ARRAY 25
#define R_386_TLS_GD_PUSH 25
#define R_390_PLT64 25
#define R_PPC_UADDR16 25
#define R_X86_64_GOTOFF64 25
#define R_PPC64_REL16_LO 250
#define R_PPC_REL16_LO 250
#define R_PPC64_REL16_HI 251
#define R_PPC_REL16_HI 251
#define R_PPC64_REL16_HA 252
#define R_PPC_REL16_HA 252
#define R_PPC_TOC16 255
#define DT_FINI_ARRAY 26
#define R_386_TLS_GD_CALL 26
#define R_390_GOTENT 26
#define R_PPC_REL32 26
#define R_X86_64_GOTPC32 26
#define DT_INIT_ARRAYSZ 27
#define R_386_TLS_GD_POP 27
#define R_390_GOTOFF16 27
#define R_PPC_PLT32 27
#define R_X86_64_GOT64 27
#define DT_FINI_ARRAYSZ 28
#define R_386_TLS_LDM_32 28
#define R_390_GOTOFF64 28
#define R_PPC_PLTREL32 28
#define R_X86_64_GOTPCREL64 28
#define DT_RUNPATH 29
#define R_386_TLS_LDM_PUSH 29
#define R_390_GOTPLT12 29
#define R_PPC_PLT16_LO 29
#define R_X86_64_GOTPC64 29
#define DT_EXTRANUM 3
#define DT_PLTGOT 3
#define DT_PPC64_NUM 3
#define EI_MAG3 3
#define ELFCLASSNUM 3
#define ELFDATANUM 3
#define ELFOSABI_LINUX 3
#define ELF_NOTE_OS_FREEBSD 3
#define EM_386 3
#define ET_DYN 3
#define PT_INTERP 3
#define R_386_GOT32 3
#define R_390_16 3
#define R_PPC_ADDR16 3
#define R_X86_64_GOT32 3
#define SHT_STRTAB 3
#define STB_NUM 3
#define STT_SECTION 3
#define STV_PROTECTED 3
#define DT_FLAGS 30
#define R_386_TLS_LDM_CALL 30
#define R_390_GOTPLT16 30
#define R_PPC_PLT16_HI 30
#define R_X86_64_GOTPLT64 30
#define R_386_TLS_LDM_POP 31
#define R_390_GOTPLT32 31
#define R_PPC_PLT16_HA 31
#define R_X86_64_PLTOFF64 31
#define DT_ENCODING 32
#define DT_PREINIT_ARRAY 32
#define R_386_TLS_LDO_32 32
#define R_390_GOTPLT64 32
#define R_PPC_SDAREL16 32
#define R_X86_64_SIZE32 32
#define DT_PREINIT_ARRAYSZ 33
#define R_386_TLS_IE_32 33
#define R_390_GOTPLTENT 33
#define R_PPC_SECTOFF 33
#define R_X86_64_SIZE64 33
#define DT_NUM 34
#define R_386_TLS_LE_32 34
#define R_390_PLTOFF16 34
#define R_PPC_SECTOFF_LO 34
#define R_X86_64_GOTPC32_TLSDESC 34
#define R_386_TLS_DTPMOD32 35
#define R_390_PLTOFF32 35
#define R_PPC_SECTOFF_HI 35
#define R_X86_64_TLSDESC_CALL 35
#define R_386_TLS_DTPOFF32 36
#define R_390_PLTOFF64 36
#define R_PPC_SECTOFF_HA 36
#define R_X86_64_TLSDESC 36
#define R_386_TLS_TPOFF32 37
#define R_390_TLS_LOAD 37
#define R_PPC64_ADDR30 37
#define R_X86_64_IRELATIVE 37
#define R_390_TLS_GDCALL 38
#define R_PPC64_ADDR64 38
#define R_X86_64_NUM 38
#define R_386_TLS_GOTDESC 39
#define R_390_TLS_LDCALL 39
#define R_PPC64_ADDR16_HIGHER 39
#define DT_HASH 4
#define EI_CLASS 4
#define ET_CORE 4
#define PT_NOTE 4
#define R_386_PLT32 4
#define R_390_32 4
#define R_PPC_ADDR16_LO 4
#define R_X86_64_PLT32 4
#define SELFMAG 4
#define SHT_RELA 4
#define STT_FILE 4
#define EM_ARM 40
#define R_386_TLS_DESC_CALL 40
#define R_390_TLS_GD32 40
#define R_PPC64_ADDR16_HIGHERA 40
#define R_386_TLS_DESC 41
#define R_390_TLS_GD64 41
#define R_PPC64_ADDR16_HIGHEST 41
#define R_386_IRELATIVE 42
#define R_390_TLS_GOTIE12 42
#define R_PPC64_ADDR16_HIGHESTA 42
#define R_386_NUM 43
#define R_390_TLS_GOTIE32 43
#define R_PPC64_UADDR64 43
#define R_390_TLS_GOTIE64 44
#define R_PPC64_REL64 44
#define R_390_TLS_LDM32 45
#define R_PPC64_PLT64 45
#define R_390_TLS_LDM64 46
#define R_PPC64_PLTREL64 46
#define R_390_TLS_IE32 47
#define R_PPC64_TOC16 47
#define R_390_TLS_IE64 48
#define R_PPC64_TOC16_LO 48
#define R_390_TLS_IEENT 49
#define R_PPC64_TOC16_HI 49
#define DT_STRTAB 5
#define EI_DATA 5
#define ET_NUM 5
#define PT_SHLIB 5
#define R_386_COPY 5
#define R_390_PC32 5
#define R_PPC_ADDR16_HI 5
#define R_X86_64_COPY 5
#define SHT_HASH 5
#define STT_COMMON 5
#define EM_IA_64 50
#define R_390_TLS_LE32 50
#define R_PPC64_TOC16_HA 50
#define R_390_TLS_LE64 51
#define R_PPC64_TOC 51
#define R_390_TLS_LDO32 52
#define R_PPC64_PLTGOT16 52
#define R_390_TLS_LDO64 53
#define R_PPC64_PLTGOT16_LO 53
#define R_390_TLS_DTPMOD 54
#define R_PPC64_PLTGOT16_HI 54
#define R_390_TLS_DTPOFF 55
#define R_PPC64_PLTGOT16_HA 55
#define R_390_TLS_TPOFF 56
#define R_PPC64_ADDR16_DS 56
#define R_390_20 57
#define R_PPC64_ADDR16_LO_DS 57
#define R_390_GOT20 58
#define R_PPC64_GOT16_DS 58
#define R_390_GOTPLT20 59
#define R_PPC64_GOT16_LO_DS 59
#define DT_SYMTAB 6
#define EI_VERSION 6
#define PT_PHDR 6
#define R_386_GLOB_DAT 6
#define R_390_GOT12 6
#define R_PPC_ADDR16_HA 6
#define R_X86_64_GLOB_DAT 6
#define SHT_DYNAMIC 6
#define STT_TLS 6
#define R_390_TLS_GOTIE20 60
#define R_PPC64_PLT16_LO_DS 60
#define R_390_NUM 61
#define R_PPC64_SECTOFF_DS 61
#define EM_X86_64 62
#define R_PPC64_SECTOFF_LO_DS 62
#define R_PPC64_TOC16_DS 63
#define R_PPC64_TOC16_LO_DS 64
#define R_PPC64_PLTGOT16_DS 65
#define R_PPC64_PLTGOT16_LO_DS 66
#define R_PPC64_TLS 67
#define R_PPC_TLS 67
#define R_PPC64_DTPMOD64 68
#define R_PPC_DTPMOD32 68
#define R_PPC64_TPREL16 69
#define R_PPC_TPREL16 69
#define DT_RELA 7
#define EI_OSABI 7
#define PT_TLS 7
#define R_386_JMP_SLOT 7
#define R_390_GOT32 7
#define R_PPC_ADDR14 7
#define R_X86_64_JUMP_SLOT 7
#define SHT_NOTE 7
#define STT_NUM 7
#define R_PPC64_TPREL16_LO 70
#define R_PPC_TPREL16_LO 70
#define R_PPC64_TPREL16_HI 71
#define R_PPC_TPREL16_HI 71
#define R_PPC64_TPREL16_HA 72
#define R_PPC_TPREL16_HA 72
#define R_PPC64_TPREL64 73
#define R_PPC_TPREL32 73
#define R_PPC64_DTPREL16 74
#define R_PPC_DTPREL16 74
#define R_PPC64_DTPREL16_LO 75
#define R_PPC_DTPREL16_LO 75
#define R_PPC64_DTPREL16_HI 76
#define R_PPC_DTPREL16_HI 76
#define R_PPC64_DTPREL16_HA 77
#define R_PPC_DTPREL16_HA 77
#define R_PPC64_DTPREL64 78
#define R_PPC_DTPREL32 78
#define R_PPC64_GOT_TLSGD16 79
#define R_PPC_GOT_TLSGD16 79
#define DT_RELASZ 8
#define EI_ABIVERSION 8
#define PT_NUM 8
#define R_386_RELATIVE 8
#define R_390_PLT32 8
#define R_PPC_ADDR14_BRTAKEN 8
#define R_X86_64_RELATIVE 8
#define SHT_NOBITS 8
#define R_PPC64_GOT_TLSGD16_LO 80
#define R_PPC_GOT_TLSGD16_LO 80
#define R_PPC64_GOT_TLSGD16_HI 81
#define R_PPC_GOT_TLSGD16_HI 81
#define R_PPC64_GOT_TLSGD16_HA 82
#define R_PPC_GOT_TLSGD16_HA 82
#define R_PPC64_GOT_TLSLD16 83
#define R_PPC_GOT_TLSLD16 83
#define R_PPC64_GOT_TLSLD16_LO 84
#define R_PPC_GOT_TLSLD16_LO 84
#define R_PPC64_GOT_TLSLD16_HI 85
#define R_PPC_GOT_TLSLD16_HI 85
#define R_PPC64_GOT_TLSLD16_HA 86
#define R_PPC_GOT_TLSLD16_HA 86
#define R_PPC64_GOT_TPREL16_DS 87
#define R_PPC_GOT_TPREL16 87
#define R_PPC64_GOT_TPREL16_LO_DS 88
#define R_PPC_GOT_TPREL16_LO 88
#define R_PPC64_GOT_TPREL16_HI 89
#define R_PPC_GOT_TPREL16_HI 89
#define DT_RELAENT 9
#define EI_PAD 9
#define R_386_GOTOFF 9
#define R_390_COPY 9
#define R_PPC_ADDR14_BRNTAKEN 9
#define R_X86_64_GOTPCREL 9
#define SHT_REL 9
#define R_PPC64_GOT_TPREL16_HA 90
#define R_PPC_GOT_TPREL16_HA 90
#define R_PPC64_GOT_DTPREL16_DS 91
#define R_PPC_GOT_DTPREL16 91
#define R_PPC64_GOT_DTPREL16_LO_DS 92
#define R_PPC_GOT_DTPREL16_LO 92
#define R_PPC64_GOT_DTPREL16_HI 93
#define R_PPC_GOT_DTPREL16_HI 93
#define R_PPC64_GOT_DTPREL16_HA 94
#define R_PPC_GOT_DTPREL16_HA 94
#define R_PPC64_TPREL16_DS 95
#define R_PPC64_TPREL16_LO_DS 96
#define R_PPC64_TPREL16_HIGHER 97
#define R_PPC64_TPREL16_HIGHERA 98
#define R_PPC64_TPREL16_HIGHEST 99
#define ELF64_M_INFO(sym, size) ELF32_M_INFO(sym, size)
#define ELF64_M_SIZE(info) ELF32_M_SIZE(info)
#define ELF64_M_SYM(info) ELF32_M_SYM(info)
#define ELF64_ST_BIND(val) ELF32_ST_BIND(val)
#define ELF64_ST_INFO(bind, type) ELF32_ST_INFO((bind), (type))
#define ELF64_ST_TYPE(val) ELF32_ST_TYPE(val)
#define ELF64_ST_VISIBILITY(o) ELF32_ST_VISIBILITY(o)
#define ELF_NOTE_GNU "GNU"
#define ELF_NOTE_ABI NT_GNU_ABI_TAG
#define R_PPC64_ADDR14 R_PPC_ADDR14
#define R_PPC64_ADDR14_BRNTAKEN R_PPC_ADDR14_BRNTAKEN
#define R_PPC64_ADDR14_BRTAKEN R_PPC_ADDR14_BRTAKEN
#define R_PPC64_ADDR16 R_PPC_ADDR16
#define R_PPC64_ADDR16_HA R_PPC_ADDR16_HA
#define R_PPC64_ADDR16_HI R_PPC_ADDR16_HI
#define R_PPC64_ADDR16_LO R_PPC_ADDR16_LO
#define R_PPC64_ADDR24 R_PPC_ADDR24
#define R_PPC64_ADDR32 R_PPC_ADDR32
#define R_PPC64_COPY R_PPC_COPY
#define R_PPC64_GLOB_DAT R_PPC_GLOB_DAT
#define R_PPC64_GOT16 R_PPC_GOT16
#define R_PPC64_GOT16_HA R_PPC_GOT16_HA
#define R_PPC64_GOT16_HI R_PPC_GOT16_HI
#define R_PPC64_GOT16_LO R_PPC_GOT16_LO
#define R_PPC64_JMP_SLOT R_PPC_JMP_SLOT
#define R_PPC64_NONE R_PPC_NONE
#define R_PPC64_PLT16_HA R_PPC_PLT16_HA
#define R_PPC64_PLT16_HI R_PPC_PLT16_HI
#define R_PPC64_PLT16_LO R_PPC_PLT16_LO
#define R_PPC64_PLT32 R_PPC_PLT32
#define R_PPC64_PLTREL32 R_PPC_PLTREL32
#define R_PPC64_REL14 R_PPC_REL14
#define R_PPC64_REL14_BRNTAKEN R_PPC_REL14_BRNTAKEN
#define R_PPC64_REL14_BRTAKEN R_PPC_REL14_BRTAKEN
#define R_PPC64_REL24 R_PPC_REL24
#define R_PPC64_REL32 R_PPC_REL32
#define R_PPC64_RELATIVE R_PPC_RELATIVE
#define R_PPC64_SECTOFF R_PPC_SECTOFF
#define R_PPC64_SECTOFF_HA R_PPC_SECTOFF_HA
#define R_PPC64_SECTOFF_HI R_PPC_SECTOFF_HI
#define R_PPC64_SECTOFF_LO R_PPC_SECTOFF_LO
#define R_PPC64_UADDR16 R_PPC_UADDR16
#define R_PPC64_UADDR32 R_PPC_UADDR32
#define ELFMAG "\177ELF"
typedef uint32_t Elf32_Addr;
typedef uint64_t Elf64_Addr;
typedef uint32_t Elf32_Word;
typedef uint32_t Elf64_Word;
typedef int32_t Elf32_Sword;
typedef int32_t Elf64_Sword;
typedef uint64_t Elf32_Xword;
typedef uint64_t Elf64_Xword;
typedef int64_t Elf32_Sxword;
typedef int64_t Elf64_Sxword;
typedef uint32_t Elf32_Off;
typedef uint64_t Elf64_Off;

typedef struct {
	Elf32_Word p_type;	 /* Segment type */
	Elf32_Off p_offset;	 /* Segment file offset */
	Elf32_Addr p_vaddr;	 /* Segment virtual address */
	Elf32_Addr p_paddr;	 /* Segment physical address */
	Elf32_Word p_filesz; /* Segment size in file */
	Elf32_Word p_memsz;	 /* Segment size in memory */
	Elf32_Word p_flags;	 /* Segment flags */
	Elf32_Word p_align;	 /* Segment alignment */
} Elf32_Phdr;

typedef struct {
	Elf64_Word p_type;	  /* Segment type */
	Elf64_Word p_flags;	  /* Segment flags */
	Elf64_Off p_offset;	  /* Segment file offset */
	Elf64_Addr p_vaddr;	  /* Segment virtual address */
	Elf64_Addr p_paddr;	  /* Segment physical address */
	Elf64_Xword p_filesz; /* Segment size in file */
	Elf64_Xword p_memsz;  /* Segment size in memory */
	Elf64_Xword p_align;  /* Segment alignment */
} Elf64_Phdr;

typedef uint16_t Elf32_Half;
typedef uint16_t Elf64_Half;
typedef uint16_t Elf32_Section;
typedef uint16_t Elf64_Section;

typedef struct {
	Elf32_Word n_namesz;
	Elf32_Word n_descsz;
	Elf32_Word n_type;
} Elf32_Nhdr;

typedef struct {
	Elf64_Word n_namesz;
	Elf64_Word n_descsz;
	Elf64_Word n_type;
} Elf64_Nhdr;

typedef struct {
	Elf64_Word st_name;
	unsigned char st_info;
	unsigned char st_other;
	Elf64_Section st_shndx;
	Elf64_Addr st_value;
	Elf64_Xword st_size;
} Elf64_Sym;

typedef struct {
	Elf32_Word st_name;
	Elf32_Addr st_value;
	Elf32_Word st_size;
	unsigned char st_info;
	unsigned char st_other;
	Elf32_Section st_shndx;
} Elf32_Sym;

typedef struct {
	Elf64_Addr r_offset;
	Elf64_Xword r_info;
} Elf64_Rel;

typedef struct {
	Elf32_Addr r_offset;
	Elf32_Word r_info;
} Elf32_Rel;

typedef struct {
	Elf64_Addr r_offset;
	Elf64_Xword r_info;
	Elf64_Sxword r_addend;
} Elf64_Rela;

typedef struct {
	Elf32_Addr r_offset;
	Elf32_Word r_info;
	Elf32_Sword r_addend;
} Elf32_Rela;

typedef struct {
	Elf32_Half vd_version;
	Elf32_Half vd_flags;
	Elf32_Half vd_ndx;
	Elf32_Half vd_cnt;
	Elf32_Word vd_hash;
	Elf32_Word vd_aux;
	Elf32_Word vd_next;
} Elf32_Verdef;

typedef struct {
	Elf64_Half vd_version;
	Elf64_Half vd_flags;
	Elf64_Half vd_ndx;
	Elf64_Half vd_cnt;
	Elf64_Word vd_hash;
	Elf64_Word vd_aux;
	Elf64_Word vd_next;
} Elf64_Verdef;

typedef struct {
	Elf64_Word vda_name;
	Elf64_Word vda_next;
} Elf64_Verdaux;

typedef struct {
	Elf32_Word vda_name;
	Elf32_Word vda_next;
} Elf32_Verdaux;

typedef struct {
	Elf32_Half vn_version;
	Elf32_Half vn_cnt;
	Elf32_Word vn_file;
	Elf32_Word vn_aux;
	Elf32_Word vn_next;
} Elf32_Verneed;

typedef struct {
	Elf64_Half vn_version;
	Elf64_Half vn_cnt;
	Elf64_Word vn_file;
	Elf64_Word vn_aux;
	Elf64_Word vn_next;
} Elf64_Verneed;

typedef struct {
	Elf32_Word vna_hash;
	Elf32_Half vna_flags;
	Elf32_Half vna_other;
	Elf32_Word vna_name;
	Elf32_Word vna_next;
} Elf32_Vernaux;

typedef struct {
	Elf64_Word vna_hash;
	Elf64_Half vna_flags;
	Elf64_Half vna_other;
	Elf64_Word vna_name;
	Elf64_Word vna_next;
} Elf64_Vernaux;

typedef struct {
	unsigned char e_ident[EI_NIDENT];
	Elf64_Half e_type;
	Elf64_Half e_machine;
	Elf64_Word e_version;
	Elf64_Addr e_entry;
	Elf64_Off e_phoff;
	Elf64_Off e_shoff;
	Elf64_Word e_flags;
	Elf64_Half e_ehsize;
	Elf64_Half e_phentsize;
	Elf64_Half e_phnum;
	Elf64_Half e_shentsize;
	Elf64_Half e_shnum;
	Elf64_Half e_shstrndx;
} Elf64_Ehdr;

typedef struct {
	unsigned char e_ident[EI_NIDENT];
	Elf32_Half e_type;
	Elf32_Half e_machine;
	Elf32_Word e_version;
	Elf32_Addr e_entry;
	Elf32_Off e_phoff;
	Elf32_Off e_shoff;
	Elf32_Word e_flags;
	Elf32_Half e_ehsize;
	Elf32_Half e_phentsize;
	Elf32_Half e_phnum;
	Elf32_Half e_shentsize;
	Elf32_Half e_shnum;
	Elf32_Half e_shstrndx;
} Elf32_Ehdr;

typedef struct {
	Elf32_Word sh_name;
	Elf32_Word sh_type;
	Elf32_Word sh_flags;
	Elf32_Addr sh_addr;
	Elf32_Off sh_offset;
	Elf32_Word sh_size;
	Elf32_Word sh_link;
	Elf32_Word sh_info;
	Elf32_Word sh_addralign;
	Elf32_Word sh_entsize;
} Elf32_Shdr;

typedef struct {
	Elf64_Word sh_name;
	Elf64_Word sh_type;
	Elf64_Xword sh_flags;
	Elf64_Addr sh_addr;
	Elf64_Off sh_offset;
	Elf64_Xword sh_size;
	Elf64_Word sh_link;
	Elf64_Word sh_info;
	Elf64_Xword sh_addralign;
	Elf64_Xword sh_entsize;
} Elf64_Shdr;

typedef struct {
	Elf32_Sword d_tag;

	union {
		Elf32_Word d_val;
		Elf32_Addr d_ptr;
	} d_un;
} Elf32_Dyn;

typedef struct {
	Elf64_Sxword d_tag;

	union {
		Elf64_Xword d_val;
		Elf64_Addr d_ptr;
	} d_un;
} Elf64_Dyn;

/* Figure 3.11: Auxillary Vector Types */
#define AT_NULL 0 /* a_un: ignored */
#define AT_IGNORE 1 /* a_un: ignored */
#define AT_EXECFD 2 /* a_un: a_val */
#define AT_PHDR 3 /* a_un: a_ptr */
#define AT_PHENT 4 /* a_un: a_val */
#define AT_PHNUM 5 /* a_un: a_val */
#define AT_PAGESZ 6 /* a_un: a_val */
#define AT_BASE 7 /* a_un: a_ptr */
#define AT_FLAGS 8 /* a_un: a_val */
#define AT_ENTRY 9 /* a_un: a_ptr */
#define AT_NOTELF 10 /* a_un: a_val */
#define AT_UID 11 /* a_un: a_val */
#define AT_EUID 12 /* a_un: a_val */
#define AT_GID 13 /* a_un: a_val */
#define AT_EGID 14 /* a_un: a_val */
#define AT_PLATFORM 15 /* a_un: a_ptr */
#define AT_HWCAP 16 /* a_un: a_val */
#define AT_CLKTCK 17 /* a_un: a_val */
#define AT_SECURE 23 /* a_un: a_val */
#define AT_BASE_PLATFORM 24 /* a_un: a_ptr */
#define AT_RANDOM 25 /* a_un: a_ptr */
#define AT_HWCAP2 26 /* a_un: a_val */
#define AT_EXECFN 31 /* a_un: a_ptr */

typedef struct {
	int a_type;

	union {
		long a_val;
		void *a_ptr;
		void (*a_fnc)();
	} a_un;
} auxv_t;
