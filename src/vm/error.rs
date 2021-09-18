#[derive(Debug, PartialEq)]

pub enum Error {
    /// Out of bounds while accessing memory at offset.
    OutOfBounds(u16),

    /// PC was somehow negative.
    NegativePc,

    // TODO: Have an offset here so errors show up nicely.
    UnknownInstruction,
}
