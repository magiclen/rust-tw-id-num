#![cfg_attr(not(feature = "rand"), no_std)]

mod check;
mod location_values;

#[cfg(feature = "rand")]
mod generate;

pub use check::*;

#[cfg(feature = "rand")]
pub use generate::*;
