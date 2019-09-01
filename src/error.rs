use std::convert::From;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    InvalidMagic(u32),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::IO(error) => write!(f, "IO error: {}", error),
            Self::InvalidMagic(val) => write!(f, "Invalid magic number: {:x}", val),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::IO(error) => Some(error),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IO(error)
    }
}
