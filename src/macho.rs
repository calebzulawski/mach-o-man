use crate::command::LoadCommand;
use crate::error::Error;
use crate::header::Header;
use std::io::{Read, Seek};

#[derive(PartialEq, Debug)]
pub struct MachO {
    pub header: Header,
    pub commands: Vec<LoadCommand>,
}

impl MachO {
    pub fn from_reader<R: Read + Seek>(r: &mut R) -> Result<Self, Error> {
        let header = Header::from_reader(r)?;
        let mut commands = Vec::new();
        for _ in 0..header.ncmds {
            commands.push(LoadCommand::from_reader(&header, r)?);
        }
        Ok(Self {
            header: header,
            commands: commands,
        })
    }
}
