/// Represents the error variants for the library.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    OutOfBounds,
}

