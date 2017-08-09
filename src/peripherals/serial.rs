
pub trait Serial {
    /// Get the amount of data in the buffers
    fn available(&self) -> u32;

    /// Get the next byte without removing is from the buffer
    fn peek(&self) -> u8;
    
    /// Get the next byte and remove it from the buffer
    fn read(&mut self) -> u8;
    
    /// Write a byte
    fn write(&mut self, u8) -> u8;
}
