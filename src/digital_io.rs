
pub enum PinState {
    High = 1,
    Low = 0,
}

pub trait DigitalOutput<T> {

    /// Write the pin high or low
    fn write(&mut self, state: PinState);

    /// Read the current state of the pin
    fn read(&self) -> PinState;
}

pub trait DigitalInput<T> {

    /// Read the pin
    fn read(&self) -> PinState;
}
