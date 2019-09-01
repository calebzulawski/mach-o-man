use crate::constants;
use crate::error::Error;
use crate::header::Header;
use std::io::{Read, Seek, SeekFrom};

#[derive(PartialEq, Debug)]
pub enum LoadCommand {
    Uuid([u8; 16]),
    Segment {
        is_64: bool,
        segname: String,
        vmaddr: u32,
        vmsize: u32,
        fileoff: u32,
        filesize: u32,
        maxprot: u32,
        initprot: u32,
        nsects: u32,
        flags: u32,
    },
    Unknown {
        cmd: u32,
        data: Vec<u8>,
    },
}

impl LoadCommand {
    pub fn from_reader<R: Read + Seek>(header: &Header, r: &mut R) -> Result<Self, Error> {
        let cmd = header.magic.read_u32(r)?;
        let size = header.magic.read_u32(r)?;
        if (size < 8)
            || (header.is_32_bit() && size % 4 != 0)
            || (header.is_64_bit() && size % 8 != 0)
        {
            return Err(Error::InvalidLoadCommandSize(size));
        }

        match cmd {
            constants::LC_UUID => {
                let mut data = [0u8; 16];
                r.read_exact(&mut data)?;
                Ok(Self::Uuid(data))
            }
            constants::LC_SEGMENT | constants::LC_SEGMENT_64 => {
                let mut name = [0u8; 16];
                let mut data = [0u32; 8];
                r.read_exact(&mut name)?;
                header.magic.read_u32_into(r, &mut data)?;
                r.seek(SeekFrom::Current(size as i64 - 56))?;
                Ok(Self::Segment {
                    is_64: cmd == constants::LC_SEGMENT_64,
                    segname: String::from_utf8(
                        name.iter().map(|x| *x).take_while(|x| *x != 0).collect(),
                    )?,
                    vmaddr: data[0],
                    vmsize: data[1],
                    fileoff: data[2],
                    filesize: data[3],
                    maxprot: data[4],
                    initprot: data[5],
                    nsects: data[6],
                    flags: data[7],
                })
            }
            _ => {
                let mut data = vec![0u8; (size - 8) as usize];
                r.read_exact(&mut data)?;
                Ok(Self::Unknown {
                    cmd: cmd,
                    data: data,
                })
            }
        }
    }
}
