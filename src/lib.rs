pub mod membership;

//Temporary Module Decleration to avoid error 
pub mod builder;
pub mod defuzz;
pub mod error;
pub mod ops;
pub mod rule;
pub mod sampler;
pub mod system;
pub mod term;
pub mod variable;

//pub use rust_fuzzylogic::triangular::Triangular;

//type definitions
#[cfg(all(feature = "f32", feature = "f64"))]
compile_error!("Enable only one of 'f32' or 'f64'.");

#[cfg(feature = "f32")]
pub type Float = f32;

#[cfg(not(feature = "f32"))] // default: f64
pub type Float = f64;

#[cfg(feature = "serde")]
pub use serde::{Deserialize, Serialize};