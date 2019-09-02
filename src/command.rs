use crate::constants;
use crate::error::Error;
use crate::extractor::Extractor;
use crate::header::Header;
use crate::{extract, extractable};
use std::convert::{TryFrom, TryInto};
use std::io::{Read, Seek, SeekFrom};

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
pub enum LoadCommand {
    Uuid(UuidCommand),
    Segment(SegmentCommand),
    Segment64(SegmentCommand64),
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
