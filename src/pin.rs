

/// Represents a physical pin in the hardware that can only be assigned to one thing
///
/// Hardware crates should have a stuct of avaiable pins. References to these pins can then be
/// given out to peripherals that need them. Depending on whether the peripheral takes a mutable
/// or immutable reference, rust's borrow checker ensures that only one peripheral will have
/// mutable access to a pin
///
/// The single field is an identifier for that pin, such as a pin number. The type can depend on
/// what the hardware uses to identify pins
///

#[derive(Debug)]
pub struct Pin<T> (pub T);
