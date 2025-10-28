use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, Mul, Neg, Sub},
};

/// A trait for the floating point ops needed by the [Freeverb](crate::Freeverb) processor.
pub trait Float:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
    + AddAssign
    + PartialEq
    + Default
    + From<f32>
    + Copy
    + Clone
    + Debug
    + Send
    + Sync
    + 'static
{
    /// Converts the value into an `f32`.
    ///
    /// `f64` doesn't implement `Into<f32>` so an explicit method is needed here.
    fn to_f32(self) -> f32;
}

impl Float for f32 {
    fn to_f32(self) -> f32 {
        self
    }
}

impl Float for f64 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}
