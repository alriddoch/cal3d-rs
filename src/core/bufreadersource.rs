use std::fs;
use std::io::{BufReader, Read};

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

    fn readFloat(&mut self, value: f32) -> Result<(), SourceError> {
        panic!("unimplemented");
        Ok(())
    }

    fn readShort(&mut self, value: i16) -> Result<(), SourceError> {
        panic!("unimplemented");
        Ok(())
    }

    fn readInteger(&mut self, value: i32) -> Result<(), SourceError> {
        panic!("unimplemented");
        Ok(())
    }

    fn readString(&mut self, strValue: String) -> Result<(), SourceError> {
        panic!("unimplemented");
        Ok(())
    }
}
