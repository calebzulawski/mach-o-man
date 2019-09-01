#[allow(non_snake_case)]
#[allow(dead_code)]

// CPU types
pub(crate) const CPU_ARCH_ABI64: u32 = 0x01000000;
pub(crate) const CPU_ARCH_ABI64_32: u32 = 0x02000000;
pub(crate) const CPU_TYPE_I386: u32 = 7;
pub(crate) const CPU_TYPE_X86_64: u32 = CPU_TYPE_I386 | CPU_ARCH_ABI64;
pub(crate) const CPU_TYPE_ARM: u32 = 12;
pub(crate) const CPU_TYPE_ARM64: u32 = CPU_TYPE_ARM | CPU_ARCH_ABI64;
pub(crate) const CPU_TYPE_ARM64_32: u32 = CPU_TYPE_ARM | CPU_ARCH_ABI64_32;
pub(crate) const CPU_TYPE_POWERPC: u32 = 18;
pub(crate) const CPU_TYPE_POWERPC64: u32 = CPU_TYPE_POWERPC | CPU_ARCH_ABI64;

// CPU subtypes
pub(crate) const CPU_SUBTYPE_LIB64: u32 = 0x80000000;
pub(crate) const CPU_SUBTYPE_MULTIPLE: u32 = 0xFFFFFFFF;
pub(crate) const CPU_SUBTYPE_X86_ALL: u32 = 3;

// Magic numbers
pub(crate) const MH_MAGIC: u32 = 0xfeedface;
pub(crate) const MH_CIGAM: u32 = 0xcefaedfe;
pub(crate) const MH_MAGIC_64: u32 = 0xfeedfacf;
pub(crate) const MH_CIGAM_64: u32 = 0xcffaedfe;

// File types
pub(crate) const MH_OBJECT: u32 = 0x1;
pub(crate) const MH_EXECUTE: u32 = 0x2;
pub(crate) const MH_FVMLIB: u32 = 0x3;
pub(crate) const MH_CORE: u32 = 0x4;
pub(crate) const MH_PRELOAD: u32 = 0x5;
pub(crate) const MH_DYLIB: u32 = 0x6;
pub(crate) const MH_DYLINKER: u32 = 0x7;
pub(crate) const MH_BUNDLE: u32 = 0x8;
pub(crate) const MH_DYLINK_STUB: u32 = 0x9;
pub(crate) const MH_DSYM: u32 = 0xa;
pub(crate) const MH_KEXT_BUNDLE: u32 = 0xb;

// Load command types
pub(crate) const LC_REQ_DYLD: u32 = 0x80000000;
pub(crate) const LC_SEGMENT: u32 = 0x1;
pub(crate) const LC_SYMTAB: u32 = 0x2;
pub(crate) const LC_SYMSEG: u32 = 0x3;
pub(crate) const LC_THREAD: u32 = 0x4;
pub(crate) const LC_UNIXTHREAD: u32 = 0x5;
pub(crate) const LC_LOADFVMLIB: u32 = 0x6;
pub(crate) const LC_IDFVMLIB: u32 = 0x7;
pub(crate) const LC_IDENT: u32 = 0x8;
pub(crate) const LC_FVMFILE: u32 = 0x9;
pub(crate) const LC_PREPAGE: u32 = 0xa;
pub(crate) const LC_DYSYMTAB: u32 = 0xb;
pub(crate) const LC_LOAD_DYLIB: u32 = 0xc;
pub(crate) const LC_ID_DYLIB: u32 = 0xd;
pub(crate) const LC_LOAD_DYLINKER: u32 = 0xe;
pub(crate) const LC_ID_DYLINKER: u32 = 0xf;
pub(crate) const LC_PREBOUND_DYLINKER: u32 = 0x10;
pub(crate) const LC_ROUTINES: u32 = 0x11;
pub(crate) const LC_SUB_FRAMEWORK: u32 = 0x12;
pub(crate) const LC_SUB_UMBRELLA: u32 = 0x13;
pub(crate) const LC_SUB_CLIENT: u32 = 0x14;
pub(crate) const LC_SUB_LIBRARY: u32 = 0x15;
pub(crate) const LC_TWOLEVEL_HINTS: u32 = 0x16;
pub(crate) const LC_PREBIND_CKSUM: u32 = 0x17;

pub(crate) const LC_LOAD_WEAK_DYLIBL: u32 = 0x18 | LC_REQ_DYLD;
pub(crate) const LC_SEGMENT_64: u32 = 0x19;
pub(crate) const LC_ROUTINES_64: u32 = 0x1a;
pub(crate) const LC_UUID: u32 = 0x1b;
pub(crate) const LC_RPATH: u32 = 0x1c | LC_REQ_DYLD;
pub(crate) const LC_CODE_SIGNATURE: u32 = 0x1d;
pub(crate) const LC_SEGMENT_SPLIT_INFO: u32 = 0x1e;
pub(crate) const LC_REEXPORT_DYLIB: u32 = 0x1f | LC_REQ_DYLD;
pub(crate) const LC_LAZY_LOAD_DYLIB: u32 = 0x20;
pub(crate) const LC_ENCRYPTION_INFO: u32 = 0x21;
pub(crate) const LC_DYLD_INFO: u32 = 0x22;
pub(crate) const LC_DYLD_INFO_ONLY: u32 = 0x22 | LC_REQ_DYLD;
pub(crate) const LC_LOAD_UPWARD_DYLIB: u32 = 0x23 | LC_REQ_DYLD;
pub(crate) const LC_VERSION_MIN_MACOSX: u32 = 0x24;
pub(crate) const LC_VERSION_MIN_IPHONEOS: u32 = 0x25;
pub(crate) const LC_FUNCTION_STARTS: u32 = 0x26;
pub(crate) const LC_DYLD_ENVIRONMENT: u32 = 0x27;
pub(crate) const LC_MAIN: u32 = 0x28 | LC_REQ_DYLD;
pub(crate) const LC_DATA_IN_CODE: u32 = 0x29;
pub(crate) const LC_SOURCE_VERSION: u32 = 0x2A;
pub(crate) const LC_DYLIB_CODE_SIGN_DRS: u32 = 0x2B;
pub(crate) const LC_ENCRYPTION_INFO_64: u32 = 0x2C;
pub(crate) const LC_LINKER_OPTION: u32 = 0x2D;
pub(crate) const LC_LINKER_OPTIMIZATION_HINT: u32 = 0x2E;
pub(crate) const LC_VERSION_MIN_TVOS: u32 = 0x2F;
pub(crate) const LC_VERSION_MIN_WATCHOS: u32 = 0x30;
pub(crate) const LC_NOTE: u32 = 0x31;
pub(crate) const LC_BUILD_VERSION: u32 = 0x32;
pub(crate) const LC_DYLD_EXPORTS_TRIE: u32 = 0x33 | LC_REQ_DYLD;
pub(crate) const LC_DYLD_CHAINED_FIXUPS: u32 = 0x34 | LC_REQ_DYLD;
