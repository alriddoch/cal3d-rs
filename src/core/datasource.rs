pub trait DataSource {
    fn ok(&self) -> bool;
    fn setError(&mut self);
    fn readBytes(&mut self, pBuffer: *mut u8, length: usize) -> bool;
    fn readFloat(&mut self, value: f32) -> bool;
    fn readShort(&mut self, value: i16) -> bool;
    fn readInteger(&mut self, value: i32) -> bool;
    fn readString(&mut self, strValue: String) -> bool;
}
