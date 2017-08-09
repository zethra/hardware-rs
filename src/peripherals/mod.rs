//! Peripherals take pins and perfom functions with them
//!
//! Peripherals must mark the accepted pins as mutable or immutable references
//! according to whether the hardware pin is written to. This will ensure that
//! pins are not used for more than one peripheral.
//!

pub mod digital_io;
pub mod serial;
pub mod time;