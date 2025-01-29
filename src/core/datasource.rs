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
    fn readFloat(&mut self) -> Result<f32, SourceError>;
    fn readShort(&mut self) -> Result<i16, SourceError>;
    fn readInteger(&mut self) -> Result<i32, SourceError>;
    fn readString(&mut self, strValue: String) -> Result<(), SourceError>;
}
