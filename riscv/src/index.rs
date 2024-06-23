use crate::error::Error;

/// Represents an index type restricted by a given valid range.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RangedIndex<const MIN: usize, const MAX: usize>(usize);

impl<const MIN: usize, const MAX: usize> RangedIndex<MIN, MAX> {
    /// Creates a new [RangedIndex].
    pub const fn new() -> Self {
        Self(MIN)
    }

    /// Infallible function that creates a [RangedIndex] from its inner representation.
    ///
    /// Clamps the provided value to the restricted range.
    ///
    /// If the value falls outside the valid range, it will be "clamped" to its nearest in-range
    /// value.
    pub const fn from_inner(val: usize) -> Self {
        match val {
            v if v <= MIN => Self(MIN),
            v if  v >= MAX => Self(MAX),
            v => Self(v),
        }
    }

    /// Converts the [RangedIndex] into its inner representation.
    pub const fn into_inner(self) -> usize {
        self.0
    }
}

impl<const MIN: usize, const MAX: usize> TryFrom<usize> for RangedIndex<MIN, MAX> {
    type Error = Error;

    fn try_from(val: usize) -> Result<Self, Self::Error> {
        match val {
            v if (MIN..=MAX).contains(&v) => Ok(Self(v)),
            _ => Err(Error::OutOfBounds),
        }
    }
}

