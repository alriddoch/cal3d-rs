use std::fs;
use std::io::BufReader;

use super::datasource::DataSource;

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

    fn readBytes(&mut self, pBuffer: *mut u8, length: usize) -> bool {
        panic!("unimplemented");
        false
    }

    fn readFloat(&mut self, value: f32) -> bool {
        panic!("unimplemented");
        false
    }

    fn readShort(&mut self, value: i16) -> bool {
        panic!("unimplemented");
        false
    }

    fn readInteger(&mut self, value: i32) -> bool {
        panic!("unimplemented");
        false
    }

    fn readString(&mut self, strValue: String) -> bool {
        panic!("unimplemented");
        false
    }
}
