use std::fs;
use std::io::{BufReader, Read};

use byteorder::{NativeEndian, ReadBytesExt};

use super::datasource::{DataSource, SourceError};

pub struct BufReaderSource {
    reader: BufReader<fs::File>,
}

impl BufReaderSource {
    pub fn new(reader: BufReader<fs::File>) -> Self {
        BufReaderSource { reader }
    }
}

impl DataSource for BufReaderSource {
    fn ok(&self) -> bool {
        false
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

    fn readString(&mut self, strValue: String) -> Result<(), SourceError> {
        todo!();
        Ok(())
    }
}
