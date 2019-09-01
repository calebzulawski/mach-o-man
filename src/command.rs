use crate::error::Error;
use crate::header::Header;
use std::io::{Read, Seek};

#[derive(PartialEq, Debug)]
pub enum LoadCommand {
    Unknown { command: u32, data: Vec<u8> },
}

impl LoadCommand {
    pub fn from_reader<R: Read + Seek>(header: &Header, r: &mut R) -> Result<Self, Error> {
        let command = header.magic.read_u32(r)?;
        let size = header.magic.read_u32(r)?;
        if (size < 8)
            || (header.is_32_bit() && size % 4 != 0)
            || (header.is_64_bit() && size % 8 != 0)
        {
            return Err(Error::InvalidLoadCommandSize(size));
        }
        let mut data = vec![0u8; (size - 8) as usize];
        r.read_exact(&mut data)?;
        Ok(Self::Unknown {
            command: command,
            data: data,
        })
    }
}
