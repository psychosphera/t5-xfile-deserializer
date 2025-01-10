#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::missing_transmute_annotations)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::from_over_into)]
#![allow(clippy::needless_borrows_for_generic_args)]

extern crate alloc;

#[cfg(feature = "deserializer")]
pub mod deserializer;
use std::io::{Read, Seek, SeekFrom, Write};

#[cfg(feature = "deserializer")]
pub use deserializer::*;

#[cfg(feature = "serializer")]
pub mod serializer;
use serde::{de::DeserializeOwned, Serialize};
#[cfg(feature = "serializer")]
pub use serializer::*;

use t5_xfile_defs::{Error, ErrorKind, Result, XFilePlatform};

use bincode::{
    config::{BigEndian, FixintEncoding, LittleEndian, WithOtherEndian, WithOtherIntEncoding},
    DefaultOptions, Options,
};

macro_rules! file_line_col {
    () => {
        alloc::format!("{}:{}:{}", file!(), line!(), column!())
    };
}
pub(crate) use file_line_col;

macro_rules! size_of {
    ($t:ty) => {
        core::mem::size_of::<$t>()
    };
    ($e:expr) => {
        core::mem::size_of_val($e)
    };
}
pub(crate) use size_of;

type BincodeOptionsLE =
    WithOtherIntEncoding<WithOtherEndian<DefaultOptions, LittleEndian>, FixintEncoding>;
type BincodeOptionsBE =
    WithOtherIntEncoding<WithOtherEndian<DefaultOptions, BigEndian>, FixintEncoding>;

#[derive(Clone)]
enum BincodeOptions {
    LE(BincodeOptionsLE),
    BE(BincodeOptionsBE),
}

impl BincodeOptions {
    fn new(little_endian: bool) -> Self {
        if little_endian {
            BincodeOptions::LE(
                DefaultOptions::new()
                    .with_little_endian()
                    .with_fixint_encoding(),
            )
        } else {
            BincodeOptions::BE(
                DefaultOptions::new()
                    .with_big_endian()
                    .with_fixint_encoding(),
            )
        }
    }

    fn from_platform(platform: XFilePlatform) -> Self {
        Self::new(platform.is_le())
    }

    fn deserialize_from<T: DeserializeOwned>(&self, reader: impl Read) -> bincode::Result<T> {
        match self {
            Self::LE(opts) => opts.deserialize_from(reader),
            Self::BE(opts) => opts.deserialize_from(reader),
        }
    }

    fn serialize_into<T: Serialize>(&self, writer: impl Write, t: T) -> bincode::Result<()> {
        match self {
            Self::LE(opts) => opts.serialize_into(writer, &t),
            Self::BE(opts) => opts.serialize_into(writer, &t),
        }
    }
}

// ============================================================================
/// [`Seek::stream_len`] isn't stable yet, so we implement it manually here
pub(crate) trait StreamLen: Seek {
    fn stream_len(&mut self) -> Result<u64> {
        let pos = self
            .stream_position()
            .map_err(|e| Error::new(file_line_col!(), 0, ErrorKind::Io(e)))?;
        let len = self
            .seek(SeekFrom::End(0))
            .map_err(|e| Error::new(file_line_col!(), 0, ErrorKind::Io(e)))?;
        self.seek(SeekFrom::Start(pos))
            .map_err(|e| Error::new(file_line_col!(), 0, ErrorKind::Io(e)))?;
        Ok(len)
    }
}

impl<T: Seek> StreamLen for T {}
// ============================================================================
