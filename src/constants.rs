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
