
pub enum PinState {
    High = 1,
    Low = 0,
}

pub trait DigitalOutput<T> {
    fn write(state: PinState);
    fn read() -> PinState;
}

pub trait DigitalInput<T> {
    fn read() -> PinState;
}
