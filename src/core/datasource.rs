pub enum SourceError {
    IoError(std::io::Error)
}

impl From<std::io::Error> for SourceError {
    fn from(error: std::io::Error) -> Self {
        SourceError::IoError(error)
    }
}

pub trait DataSource {
    fn ok(&self) -> bool;
    fn setError(&mut self);
    fn readBytes(&mut self, pBuffer: &mut [u8], length: usize) -> Result<(), SourceError>;
    fn readFloat(&mut self, value: f32) -> Result<(), SourceError>;
    fn readShort(&mut self, value: i16) -> Result<(), SourceError>;
    fn readInteger(&mut self, value: i32) -> Result<(), SourceError>;
    fn readString(&mut self, strValue: String) -> Result<(), SourceError>;
}
