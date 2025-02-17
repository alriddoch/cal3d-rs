use std::string::FromUtf8Error;

use crate::{CalQuaternion, CalVector};

pub(crate) const maxStringLength: i32 = 128;

pub enum SourceError {
    IoError(std::io::Error),
    FormatError(String),
}

impl From<std::io::Error> for SourceError {
    fn from(error: std::io::Error) -> Self {
        SourceError::IoError(error)
    }
}

impl From<FromUtf8Error> for SourceError {
    fn from(error: FromUtf8Error) -> Self {
        SourceError::FormatError(String::from("utf8 conversion error"))
    }
}

pub trait DataSource {
    fn ok(&self) -> bool;
    fn setError(&mut self);
    fn readBytes(&mut self, pBuffer: &mut [u8], length: usize) -> Result<(), SourceError>;
    fn readFloat(&mut self) -> Result<f32, SourceError>;
    fn readShort(&mut self) -> Result<i16, SourceError>;
    fn readInteger(&mut self) -> Result<i32, SourceError>;
    fn readString(&mut self) -> Result<String, SourceError>;
}

pub fn CalVectorFromDataSrc(dataSrc: &mut dyn DataSource) -> Result<CalVector<f32>, SourceError> {
   let x = dataSrc.readFloat()?;
   let y =dataSrc.readFloat()?;
   let z =dataSrc.readFloat()?;
   Ok(CalVector::new(x,y,z))
}
