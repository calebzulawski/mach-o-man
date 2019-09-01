use crate::error::Error;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::io::{Read, Seek};

#[derive(PartialEq, Debug)]
pub enum Magic {
    MAGIC,
    CIGAM,
    MAGIC_64,
    CIGAM_64,
}

const MH_MAGIC: u32 = 0xfeedface;
const MH_CIGAM: u32 = 0xcefaedfe;
const MH_MAGIC_64: u32 = 0xfeedfacf;
const MH_CIGAM_64: u32 = 0xcffaedfe;

impl Magic {
    fn from_u32(v: u32) -> Result<Self, Error> {
        match v {
            MH_MAGIC => Ok(Magic::MAGIC),
            MH_CIGAM => Ok(Magic::CIGAM),
            MH_MAGIC_64 => Ok(Magic::MAGIC_64),
            MH_CIGAM_64 => Ok(Magic::CIGAM_64),
            value => Err(Error::InvalidMagic(value)),
        }
    }

    fn to_u32(&self) -> u32 {
        match self {
            Self::MAGIC => MH_MAGIC,
            Self::CIGAM => MH_CIGAM,
            Self::MAGIC_64 => MH_MAGIC_64,
            Self::CIGAM_64 => MH_CIGAM_64,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum CpuType {
    I386,
    X86_64,
    ARM,
    ARM64,
    ARM64_32,
    POWERPC,
    POWERPC64,
    Unknown(u32),
}

const CPU_ARCH_ABI64: u32 = 0x01000000;
const CPU_ARCH_ABI64_32: u32 = 0x02000000;
const CPU_TYPE_I386: u32 = 7;
const CPU_TYPE_X86_64: u32 = CPU_TYPE_I386 | CPU_ARCH_ABI64;
const CPU_TYPE_ARM: u32 = 12;
const CPU_TYPE_ARM64: u32 = CPU_TYPE_ARM | CPU_ARCH_ABI64;
const CPU_TYPE_ARM64_32: u32 = CPU_TYPE_ARM | CPU_ARCH_ABI64_32;
const CPU_TYPE_POWERPC: u32 = 18;
const CPU_TYPE_POWERPC64: u32 = CPU_TYPE_POWERPC | CPU_ARCH_ABI64;

impl CpuType {
    fn from_u32(v: u32) -> Self {
        match v {
            CPU_TYPE_I386 => Self::I386,
            CPU_TYPE_x86_64 => Self::X86_64,
            CPU_TYPE_ARM => Self::ARM,
            CPU_TYPE_ARM64 => Self::ARM64,
            CPU_TYPE_ARM64_32 => Self::ARM64_32,
            CPU_TYPE_POWERPC => Self::POWERPC,
            CPU_TYPE_POWERPC64 => Self::POWERPC64,
            unknown => Self::Unknown(unknown),
        }
    }

    fn to_u32(&self) -> u32 {
        match self {
            Self::I386 => CPU_TYPE_I386,
            Self::X86_64 => CPU_TYPE_X86_64,
            Self::ARM => CPU_TYPE_ARM,
            Self::ARM64 => CPU_TYPE_ARM64,
            Self::ARM64_32 => CPU_TYPE_ARM64_32,
            Self::POWERPC => CPU_TYPE_POWERPC,
            Self::POWERPC64 => CPU_TYPE_POWERPC64,
            Self::Unknown(value) => *value,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum CpuSubType {
    Unknown(u32),
}

impl CpuSubType {
    fn from_u32(v: u32) -> Self {
        match v {
            unknown => Self::Unknown(unknown),
        }
    }

    fn to_u32(&self) -> u32 {
        match self {
            Self::Unknown(value) => *value,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Filetype {
    OBJECT,
    EXECUTE,
    FVMLIB,
    CORE,
    PRELOAD,
    DYLIB,
    DYLINKER,
    BUNDLE,
    DYLINK_STUB,
    DSYM,
    KEXT_BUNDLE,
    Unknown(u32),
}

const MH_OBJECT: u32 = 0x1;
const MH_EXECUTE: u32 = 0x2;
const MH_FVMLIB: u32 = 0x3;
const MH_CORE: u32 = 0x4;
const MH_PRELOAD: u32 = 0x5;
const MH_DYLIB: u32 = 0x6;
const MH_DYLINKER: u32 = 0x7;
const MH_BUNDLE: u32 = 0x8;
const MH_DYLINK_STUB: u32 = 0x9;
const MH_DSYM: u32 = 0xa;
const MH_KEXT_BUNDLE: u32 = 0xb;

impl Filetype {
    fn from_u32(v: u32) -> Self {
        match v {
            MH_OBJECT => Self::OBJECT,
            MH_EXECUTE => Self::EXECUTE,
            MH_FVMLIB => Self::FVMLIB,
            MH_CORE => Self::CORE,
            MH_PRELOAD => Self::PRELOAD,
            MH_DYLIB => Self::DYLIB,
            MH_DYLINKER => Self::DYLINKER,
            MH_BUNDLE => Self::BUNDLE,
            MH_DYLINK_STUB => Self::DYLINK_STUB,
            MH_DSYM => Self::DSYM,
            MH_KEXT_BUNDLE => Self::KEXT_BUNDLE,
            unknown => Self::Unknown(unknown),
        }
    }

    fn to_u32(&self) -> u32 {
        match self {
            Self::OBJECT => MH_OBJECT,
            Self::EXECUTE => MH_EXECUTE,
            Self::FVMLIB => MH_FVMLIB,
            Self::CORE => MH_CORE,
            Self::PRELOAD => MH_PRELOAD,
            Self::DYLIB => MH_DYLIB,
            Self::DYLINKER => MH_DYLINKER,
            Self::BUNDLE => MH_BUNDLE,
            Self::DYLINK_STUB => MH_DYLINK_STUB,
            Self::DSYM => MH_DSYM,
            Self::KEXT_BUNDLE => MH_KEXT_BUNDLE,
            Self::Unknown(value) => *value,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Header {
    magic: Magic,
    cputype: CpuType,
    cpusubtype: CpuSubType,
    filetype: Filetype,
    ncmds: u32,
    sizeofcmds: u32,
    flags: u32,
}

impl Header {
    pub fn from_reader<R: Read + Seek>(r: &mut R) -> Result<Header, Error> {
        let magic = Magic::from_u32(r.read_u32::<LittleEndian>()?)?;
        let mut vals: [u32; 6] = [0; 6];
        if magic == Magic::MAGIC || magic == Magic::MAGIC_64 {
            r.read_u32_into::<LittleEndian>(&mut vals)?;
        } else {
            r.read_u32_into::<BigEndian>(&mut vals)?;
        }
        if magic == Magic::MAGIC_64 || magic == Magic::CIGAM_64 {
            r.seek(std::io::SeekFrom::Current(4))?; // skip reserved field
        }
        Ok(Self {
            magic: magic,
            cputype: CpuType::from_u32(vals[0]),
            cpusubtype: CpuSubType::from_u32(vals[1]),
            filetype: Filetype::from_u32(vals[2]),
            ncmds: vals[3],
            sizeofcmds: vals[4],
            flags: vals[5],
        })
    }
}
