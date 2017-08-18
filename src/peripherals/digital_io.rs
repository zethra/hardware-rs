use core::ops::Not;
use core::convert::From;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DigitalValue {
    High = 1,
    Low = 0,
}

impl Not for DigitalValue {
    type Output = DigitalValue;

    fn not(self) -> DigitalValue {
        match self {
            DigitalValue::High => DigitalValue::Low,
            DigitalValue::Low => DigitalValue::High,
        }
    }
}

impl From<bool> for DigitalValue {
    fn from(other: bool) -> Self {
        match other {
            true => DigitalValue::High,
            false => DigitalValue::Low,
        }
    }
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
