use crate::constants;
use crate::error::Error;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::io::{Read, Seek};

#[derive(PartialEq, Debug)]
pub enum Magic {
    LittleEndian,
    BigEndian,
    LittleEndian64,
    BigEndian64,
}

impl Magic {
    fn from_u32(v: u32) -> Result<Self, Error> {
        match v {
            constants::MH_MAGIC => Ok(Magic::LittleEndian),
            constants::MH_CIGAM => Ok(Magic::BigEndian),
            constants::MH_MAGIC_64 => Ok(Magic::LittleEndian64),
            constants::MH_CIGAM_64 => Ok(Magic::BigEndian64),
            value => Err(Error::InvalidMagic(value)),
        }
    }

    fn to_u32(&self) -> u32 {
        match self {
            Self::LittleEndian => constants::MH_MAGIC,
            Self::BigEndian => constants::MH_CIGAM,
            Self::LittleEndian64 => constants::MH_MAGIC_64,
            Self::BigEndian64 => constants::MH_CIGAM_64,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum CpuType {
    X86,
    X86_64,
    Arm,
    Arm64,
    Arm64_32,
    PowerPC,
    PowerPC64,
    Unknown(u32),
}

impl CpuType {
    fn from_u32(v: u32) -> Self {
        match v {
            constants::CPU_TYPE_I386 => Self::X86,
            constants::CPU_TYPE_X86_64 => Self::X86_64,
            constants::CPU_TYPE_ARM => Self::Arm,
            constants::CPU_TYPE_ARM64 => Self::Arm64,
            constants::CPU_TYPE_ARM64_32 => Self::Arm64_32,
            constants::CPU_TYPE_POWERPC => Self::PowerPC,
            constants::CPU_TYPE_POWERPC64 => Self::PowerPC64,
            unknown => Self::Unknown(unknown),
        }
    }

    fn to_u32(&self) -> u32 {
        match self {
            Self::X86 => constants::CPU_TYPE_I386,
            Self::X86_64 => constants::CPU_TYPE_X86_64,
            Self::Arm => constants::CPU_TYPE_ARM,
            Self::Arm64 => constants::CPU_TYPE_ARM64,
            Self::Arm64_32 => constants::CPU_TYPE_ARM64_32,
            Self::PowerPC => constants::CPU_TYPE_POWERPC,
            Self::PowerPC64 => constants::CPU_TYPE_POWERPC64,
            Self::Unknown(value) => *value,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum CpuSubType {
    Multiple,
    X86,
    X86_64,
    Unknown(u32),
}

impl CpuSubType {
    fn from_u32(v: u32) -> Self {
        match v {
            constants::CPU_SUBTYPE_MULTIPLE => Self::Multiple,
            constants::CPU_SUBTYPE_X86_ALL => Self::X86,
            x if x == constants::CPU_SUBTYPE_X86_ALL | constants::CPU_SUBTYPE_LIB64 => Self::X86_64,
            unknown => Self::Unknown(unknown),
        }
    }

    fn to_u32(&self) -> u32 {
        match self {
            Self::Multiple => constants::CPU_SUBTYPE_MULTIPLE,
            Self::X86 => constants::CPU_SUBTYPE_X86_ALL,
            Self::X86_64 => constants::CPU_SUBTYPE_X86_ALL | constants::CPU_SUBTYPE_LIB64,
            Self::Unknown(value) => *value,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Filetype {
    Object,
    Execute,
    Fvmlib,
    Core,
    Preload,
    Dylib,
    Dylinker,
    Bundle,
    DylinkStub,
    Dsym,
    KextBundle,
    Unknown(u32),
}

impl Filetype {
    fn from_u32(v: u32) -> Self {
        match v {
            constants::MH_OBJECT => Self::Object,
            constants::MH_EXECUTE => Self::Execute,
            constants::MH_FVMLIB => Self::Fvmlib,
            constants::MH_CORE => Self::Core,
            constants::MH_PRELOAD => Self::Preload,
            constants::MH_DYLIB => Self::Dylib,
            constants::MH_DYLINKER => Self::Dylinker,
            constants::MH_BUNDLE => Self::Bundle,
            constants::MH_DYLINK_STUB => Self::DylinkStub,
            constants::MH_DSYM => Self::Dsym,
            constants::MH_KEXT_BUNDLE => Self::KextBundle,
            unknown => Self::Unknown(unknown),
        }
    }

    fn to_u32(&self) -> u32 {
        match self {
            Self::Object => constants::MH_OBJECT,
            Self::Execute => constants::MH_EXECUTE,
            Self::Fvmlib => constants::MH_FVMLIB,
            Self::Core => constants::MH_CORE,
            Self::Preload => constants::MH_PRELOAD,
            Self::Dylib => constants::MH_DYLIB,
            Self::Dylinker => constants::MH_DYLINKER,
            Self::Bundle => constants::MH_BUNDLE,
            Self::DylinkStub => constants::MH_DYLINK_STUB,
            Self::Dsym => constants::MH_DSYM,
            Self::KextBundle => constants::MH_KEXT_BUNDLE,
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
        if magic == Magic::LittleEndian || magic == Magic::LittleEndian64 {
            r.read_u32_into::<LittleEndian>(&mut vals)?;
        } else {
            r.read_u32_into::<BigEndian>(&mut vals)?;
        }
        if magic == Magic::LittleEndian64 || magic == Magic::BigEndian64 {
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
