
#[derive(Debug, Clone, Copy)]
pub enum DigitalValue {
    High = 1,
    Low = 0,
}

pub trait DigitalOutput {

    /// Write the pin high or low
    fn write(&mut self, state: DigitalValue);

    /// Read the current state of the pin
    fn read(&self) -> DigitalValue;
}

pub trait DigitalInput {

    /// Read the pin
    fn read(&self) -> DigitalValue;
}
