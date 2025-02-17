use std::fs;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use byteorder::{NativeEndian, ReadBytesExt};

use crate::core::datasource;

use super::datasource::{DataSource, SourceError};

pub struct BufReaderSource {
    reader: BufReader<fs::File>,
}

impl BufReaderSource {
    pub fn new(reader: BufReader<fs::File>) -> Self {
        BufReaderSource { reader }
    }

    pub fn report_unused_bytes(&mut self, filename: &PathBuf) {
        let mut buf: Vec<u8> = Vec::new();
        if let Ok(remaining) = self.reader.read_to_end(&mut buf) {
            if remaining > 0 {
                println!("Warning: {} bytes left after loading {filename:?}", remaining);
            }
        }
    }
}

impl DataSource for BufReaderSource {
    fn ok(&self) -> bool {
        true
    }

    fn setError(&mut self) {}

    fn readBytes(&mut self, pBuffer: &mut [u8], length: usize) -> Result<(), SourceError> {
        let count = self.reader.read(pBuffer)?;
        if count != length {
            println!("Error expected {} got {} bytes read", length, count);
        }
        Ok(())
    }

    fn readFloat(&mut self) -> Result<f32, SourceError> {
        Ok(self.reader.read_f32::<NativeEndian>()?)
    }

    fn readShort(&mut self) -> Result<i16, SourceError> {
        Ok(self.reader.read_i16::<NativeEndian>()?)
    }

    fn readInteger(&mut self) -> Result<i32, SourceError> {
        Ok(self.reader.read_i32::<NativeEndian>()?)
    }

    fn readString(&mut self) -> Result<String, SourceError> {
        let length = self.reader.read_i32::<NativeEndian>()?;
        if length <= 0 || length > datasource::maxStringLength {
            return Err(SourceError::FormatError(format!(
                "string length {length} fails sanity check {}",
                datasource::maxStringLength
            )));
        }

        let mut buf: Vec<u8> = vec![0; length as usize];
        self.readBytes(buf.as_mut_slice(), length as usize)?;

        Ok(String::from_utf8(buf)?)
    }
}
