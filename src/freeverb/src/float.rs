use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, Mul, Neg, Sub},
};

pub trait Float:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
    + AddAssign
    + From<f32>
    + Copy
    + Clone
    + PartialEq
    + Default
    + Send
    + Sync
    + Debug
    + 'static
{
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
