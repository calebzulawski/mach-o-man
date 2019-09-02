use crate::constants;
use crate::error::Error;
use crate::extractor::Extractor;
use crate::header::Header;
use crate::{extract, extractable};
use std::convert::{TryFrom, TryInto};
use std::io::{Read, Seek, SeekFrom};

#[derive(PartialEq)]
pub struct NameString {
    name: String,
}

impl NameString {
    fn to_string(self) -> String {
        self.name
    }

    fn as_str(&self) -> &str {
        self.name.as_str()
    }

    fn parse<R: Read>(r: &mut R) -> Result<Self, Error> {
        let mut data = [0u8; 16];
        r.read_exact(&mut data)?;
        Ok(Self {
            name: String::from_utf8(data.iter().map(|x| *x).take_while(|x| *x != 0).collect())?,
        })
    }

    fn from_string(name: String) -> Result<Self, Error> {
        if name.len() > 16 || name.chars().any(|c| !c.is_ascii()) {
            Err(Error::BadString(name))
        } else {
            Ok(Self { name: name })
        }
    }
}

impl std::fmt::Debug for NameString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.name)
    }
}

impl<'a> TryFrom<&mut Extractor<'a>> for NameString {
    type Error = Error;

    fn try_from(e: &mut Extractor<'a>) -> Result<Self, Self::Error> {
        NameString::parse(&mut e.reader)
    }
}

extractable!(UuidCommand { uuid: u128 });

extractable!(SegmentCommand {
    segname: NameString,
    vmaddr: u32,
    vmsize: u32,
    fileoff: u32,
    filesize: u32,
    maxprot: u32,
    initprot: u32,
    nsects: u32,
    flags: u32,
    sections: Vec<Section> = Vec::new(),
});

extractable!(Section {
    sectname: NameString,
    segname: NameString,
    addr: u32,
    size: u32,
    offset: u32,
    align: u32,
    reloff: u32,
    nreloc: u32,
    flags: u32,
    reserved1: u32,
    reserved2: u32,
});

extractable!(SegmentCommand64 {
    segname: NameString,
    vmaddr: u64,
    vmsize: u64,
    fileoff: u64,
    filesize: u64,
    maxprot: u32,
    initprot: u32,
    nsects: u32,
    flags: u32,
    sections: Vec<Section64> = Vec::new(),
});

extractable!(Section64 {
    sectname: NameString,
    segname: NameString,
    addr: u64,
    size: u64,
    offset: u32,
    align: u32,
    reloff: u32,
    nreloc: u32,
    flags: u32,
    reserved1: u32,
    reserved2: u32,
    reserved3: u32,
});

extractable!(SymtabCommand {
    symoff: u32,
    nsyms: u32,
    stroff: u32,
    strsize: u32,
});

extractable!(DysymtabCommand {
    ilocalsym: u32,
    nlocalsym: u32,
    iextdefsym: u32,
    nextdefsym: u32,
    iundefsym: u32,
    nundefsym: u32,
    tocoff: u32,
    ntoc: u32,
    modtaboff: u32,
    nmodtab: u32,
    extrefsymoff: u32,
    nextrefsyms: u32,
    indirectsymoff: u32,
    nindirectsyms: u32,
    extreloff: u32,
    nextrel: u32,
    locreloff: u32,
    nlocrel: u32,
});

extractable!(TwoLevelHintsCommand {
    offset: u32,
    nhints: u32,
});

extractable!(DyldInfoCommand {
    rebase_off: u32,
    rebase_size: u32,
    bind_off: u32,
    bind_size: u32,
    weak_bind_off: u32,
    weak_bind_size: u32,
    lazy_bind_off: u32,
    lazy_bind_size: u32,
    export_off: u32,
    export_size: u32,
});

extractable!(LinkeditDataCommand {
    dataoff: u32,
    datasize: u32,
});

#[derive(PartialEq, Debug)]
pub enum LoadCommand {
    Uuid(UuidCommand),
    Segment(SegmentCommand),
    Segment64(SegmentCommand64),
    Symtab(SymtabCommand),
    Dysymtab(DysymtabCommand),
    TwoLevelHints(TwoLevelHintsCommand),
    DyldInfo(DyldInfoCommand),
    DyldInfoOnly(DyldInfoCommand),
    CodeSignature(LinkeditDataCommand),
    SegmentSplitInfo(LinkeditDataCommand),
    FunctionStarts(LinkeditDataCommand),
    DataInCode(LinkeditDataCommand),
    DylibCodeSignDrs(LinkeditDataCommand),
    LinkerOptimizationHint(LinkeditDataCommand),
    Unknown { cmd: u32, data: Vec<u8> },
}

impl LoadCommand {
    pub fn from_reader<R: Read + Seek>(header: &Header, r: &mut R) -> Result<Self, Error> {
        let initial_position = r.seek(SeekFrom::Current(0))?;
        let mut e = header.magic.get_extractor(r);
        let cmd: u32 = extract!(e);
        let size: u32 = extract!(e);
        if (size < 8)
            || (header.is_32_bit() && size % 4 != 0)
            || (header.is_64_bit() && size % 8 != 0)
        {
            return Err(Error::InvalidLoadCommandSize(size));
        }

        let command = match cmd {
            constants::LC_UUID => LoadCommand::Uuid(extract!(e)),
            constants::LC_SEGMENT => {
                let mut command: SegmentCommand = extract!(e);
                for _ in 0..command.nsects {
                    command.sections.push(extract!(e));
                }
                LoadCommand::Segment(command)
            }
            constants::LC_SEGMENT_64 => {
                let mut command: SegmentCommand64 = extract!(e);
                for _ in 0..command.nsects {
                    command.sections.push(extract!(e));
                }
                LoadCommand::Segment64(command)
            }
            constants::LC_SYMTAB => LoadCommand::Symtab(extract!(e)),
            constants::LC_DYSYMTAB => LoadCommand::Dysymtab(extract!(e)),
            constants::LC_TWOLEVEL_HINTS => LoadCommand::TwoLevelHints(extract!(e)),
            constants::LC_DYLD_INFO => LoadCommand::DyldInfo(extract!(e)),
            constants::LC_DYLD_INFO_ONLY => LoadCommand::DyldInfoOnly(extract!(e)),
            constants::LC_CODE_SIGNATURE => LoadCommand::CodeSignature(extract!(e)),
            constants::LC_SEGMENT_SPLIT_INFO => LoadCommand::SegmentSplitInfo(extract!(e)),
            constants::LC_FUNCTION_STARTS => LoadCommand::FunctionStarts(extract!(e)),
            constants::LC_DATA_IN_CODE => LoadCommand::DataInCode(extract!(e)),
            constants::LC_DYLIB_CODE_SIGN_DRS => LoadCommand::DylibCodeSignDrs(extract!(e)),
            constants::LC_LINKER_OPTIMIZATION_HINT => {
                LoadCommand::LinkerOptimizationHint(extract!(e))
            }
            _ => {
                let mut data = vec![0u8; (size - 8) as usize];
                r.read_exact(&mut data)?;
                Self::Unknown {
                    cmd: cmd,
                    data: data,
                }
            }
        };
        r.seek(SeekFrom::Start(initial_position + size as u64))?;
        Ok(command)
    }
}
