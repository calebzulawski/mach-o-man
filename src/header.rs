use crate::constants;
use crate::error::Error;
use crate::extract;
use crate::extractor::Extractor;

use std::convert::{TryFrom, TryInto};
use std::io::{Read, Seek};

#[derive(PartialEq, Debug)]
pub enum Magic {
    LittleEndian,
    BigEndian,
    LittleEndian64,
    BigEndian64,
}

impl Magic {
    pub(crate) fn get_extractor<'a, R: Read + Seek>(&self, r: &'a mut R) -> Extractor<'a> {
        match self {
            Self::LittleEndian | Self::LittleEndian64 => Extractor::little_endian(r),
            Self::BigEndian | Self::BigEndian64 => Extractor::big_endian(r),
        }
    }

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

impl TryFrom<&mut Extractor<'_>> for Magic {
    type Error = Error;

    fn try_from(e: &mut Extractor) -> Result<Self, Self::Error> {
        Ok(Magic::from_u32(e.try_into()?)?)
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

impl TryFrom<&mut Extractor<'_>> for CpuType {
    type Error = Error;

    fn try_from(e: &mut Extractor) -> Result<Self, Self::Error> {
        Ok(CpuType::from_u32(e.try_into()?))
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

impl TryFrom<&mut Extractor<'_>> for CpuSubType {
    type Error = Error;

    fn try_from(e: &mut Extractor) -> Result<Self, Self::Error> {
        Ok(CpuSubType::from_u32(e.try_into()?))
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

impl TryFrom<&mut Extractor<'_>> for Filetype {
    type Error = Error;

    fn try_from(e: &mut Extractor) -> Result<Self, Self::Error> {
        Ok(Filetype::from_u32(e.try_into()?))
    }
}

#[derive(PartialEq, Debug)]
pub struct Header {
    pub magic: Magic,
    pub cputype: CpuType,
    pub cpusubtype: CpuSubType,
    pub filetype: Filetype,
    pub ncmds: u32,
    pub sizeofcmds: u32,
    pub flags: u32,
}

impl Header {
    pub fn from_reader<R: Read + Seek>(r: &mut R) -> Result<Self, Error> {
        let magic = Magic::from_u32((&mut Extractor::little_endian(r)).try_into()?)?;
        let mut e = magic.get_extractor(r);
        let header = Self {
            magic: magic,
            cputype: extract!(e),
            cpusubtype: extract!(e),
            filetype: extract!(e),
            ncmds: extract!(e),
            sizeofcmds: extract!(e),
            flags: extract!(e),
        };
        if header.is_64_bit() {
            r.seek(std::io::SeekFrom::Current(4))?; // skip reserved field
        }
        Ok(header)
    }

    pub fn is_32_bit(&self) -> bool {
        self.magic == Magic::LittleEndian || self.magic == Magic::BigEndian
    }

    pub fn is_64_bit(&self) -> bool {
        self.magic == Magic::LittleEndian64 || self.magic == Magic::BigEndian64
    }

    pub fn is_little_endian(&self) -> bool {
        self.magic == Magic::LittleEndian || self.magic == Magic::LittleEndian64
    }

    pub fn is_big_endian(&self) -> bool {
        self.magic == Magic::BigEndian || self.magic == Magic::BigEndian64
    }
}
