use {Parcel, Error, CharTryFromError, Settings};

use std::char;
use std::io::prelude::*;
#[cfg(feature = "tokio")]
use tokio::prelude::*;

impl Parcel for char
{
    const TYPE_NAME: &'static str = "char";

    fn read(read: &mut Read,
            settings: &Settings) -> Result<Self, Error> {
        let bytes = u32::read(read, settings)?;
        Ok(char::from_u32(bytes).ok_or(CharTryFromError{ })?)
    }

    #[cfg(feature = "tokio")]
    fn read_async(read: &mut AsyncRead)
        -> Box<Future<Item=Self, Error=Error> + Send> {
        unimplemented!();
    }

    fn write(&self, write: &mut Write,
             settings: &Settings) -> Result<(), Error> {
        (*self as u32).write(write, settings)
    }
}

