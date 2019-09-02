use crate::error::Error;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::convert::TryFrom;
use std::io::{Read, Seek};

pub(crate) enum Endianness {
    Big,
    Little,
}

pub(crate) trait ReadAndSeek: Read + Seek {}
impl<T: Read + Seek> ReadAndSeek for T {}

pub(crate) struct Extractor<'a> {
    pub endianness: Endianness,
    pub reader: &'a mut dyn ReadAndSeek,
}

impl<'a> Extractor<'a> {
    pub fn little_endian(reader: &'a mut dyn ReadAndSeek) -> Self {
        Self {
            endianness: Endianness::Little,
            reader: reader,
        }
    }

    pub fn big_endian(reader: &'a mut dyn ReadAndSeek) -> Self {
        Self {
            endianness: Endianness::Big,
            reader: reader,
        }
    }
}

impl<'a> TryFrom<&mut Extractor<'a>> for u32 {
    type Error = Error;

    fn try_from(e: &mut Extractor<'a>) -> Result<Self, Self::Error> {
        Ok(match e.endianness {
            Endianness::Big => e.reader.read_u32::<BigEndian>()?,
            Endianness::Little => e.reader.read_u32::<LittleEndian>()?,
        })
    }
}

impl<'a> TryFrom<&mut Extractor<'a>> for u64 {
    type Error = Error;

    fn try_from(e: &mut Extractor<'a>) -> Result<Self, Self::Error> {
        Ok(match e.endianness {
            Endianness::Big => e.reader.read_u64::<BigEndian>()?,
            Endianness::Little => e.reader.read_u64::<LittleEndian>()?,
        })
    }
}

impl<'a> TryFrom<&mut Extractor<'a>> for u128 {
    type Error = Error;

    fn try_from(e: &mut Extractor<'a>) -> Result<Self, Self::Error> {
        Ok(match e.endianness {
            Endianness::Big => e.reader.read_u128::<BigEndian>()?,
            Endianness::Little => e.reader.read_u128::<LittleEndian>()?,
        })
    }
}

#[macro_export]
macro_rules! extractable {
    // used for initializing fields that aren't automagically extracted
    (@init $extractor:ident) => ( $extractor.try_into()? );
    (@init $extractor:ident $init:expr) => ( $init );

    // used for seeking past padding
    (@pad $extractor:ident) => ();
    (@pad $extractor:ident $reserved:expr) => ($extractor.reader.seek(SeekFrom::Current($reserved))?);

    // the entrypoint
    ($name:ident {
        $($field:ident : $type:ty $( = $init:expr )* ),*
        $(,)*
        $(=> pad $reserved:expr)*
    }
    ) => (
        // create the struct
        #[derive(PartialEq, Debug)]
        pub struct $name {
            $( pub $field : $type ),*
        }

        // create the extraction implementation
        impl<'a> std::convert::TryFrom<&mut crate::extractor::Extractor<'a>> for $name {
            type Error = crate::error::Error;
            fn try_from(e: &mut crate::extractor::Extractor<'a>) -> Result<Self, Self::Error> {
                use std::convert::TryInto;
                use std::io::Seek;
                // extract the fields
                let value = Self {
                    $( $field: extractable!(@init e $($init)*) ),*
                };
                // skip past any reserved space
                extractable!(@pad e $($reserved)*);
                Ok(value)
            }
        }
    );
}

#[macro_export]
macro_rules! extract {
    ($extractor:ident) => {
        (&mut $extractor).try_into()?
    };
}
