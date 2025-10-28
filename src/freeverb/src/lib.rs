//! A Rust implementation of the Freeverb reverb algorithm.
//!
//! Freeverb was originally written in C++ by "Jezar at Dreampoint", and was released into the public
//! domain in June 2000. It is now widely used in various incarnations in multiple software packages.
//!
//! - The orignal C++ source code can be found [here](https://freeverb3-vst.sourceforge.io).
//! - For an analysis of the algorithm see
//!   [here](https://ccrma.stanford.edu/~jos/pasp/Freeverb.html).

mod all_pass;
mod comb;
mod delay_line;
mod float;
mod freeverb;
mod tuning;

pub use self::{float::Float, freeverb::Freeverb};
