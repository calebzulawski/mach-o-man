use std::convert::From;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    InvalidMagic(u32),
    InvalidLoadCommandSize(u32),
    BadStringParse(Vec<u8>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::IO(error) => write!(f, "IO error: {}", error),
            Self::InvalidMagic(val) => write!(f, "Invalid magic number: {:x}", val),
            Self::InvalidLoadCommandSize(val) => write!(f, "Invalid load command size: {}", val),
            Self::BadStringParse(bytes) => write!(f, "Bad string: {:?}", bytes),
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

impl From<std::string::FromUtf8Error> for Error {
    fn from(error: std::string::FromUtf8Error) -> Self {
        Error::BadStringParse(error.into_bytes())
    }
}
