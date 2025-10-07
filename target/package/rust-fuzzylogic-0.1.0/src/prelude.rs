//! Prelude: convenient re-exports for common fuzzy logic items.
//!
//! Bring this module into scope to get the core trait and types
//! without importing each one individually:
//!
//! ```rust
//! use rust_fuzzylogic::prelude::*;
//! ```
//!
//! Note: you still need to expose this module from `lib.rs` with
//! `pub mod prelude;` to make it available to users.

// Core scalar and error types
pub use crate::Float;
pub use crate::error::{FuzzyError, Result};

// Membership trait and built-in shapes
pub use crate::membership::MembershipFn;
pub use crate::membership::trapezoidal::Trapezoidal;
pub use crate::membership::{Gaussian, Triangular};

// Fuzzy Set Operands
pub use crate::ops::FuzzyOps;

// Term wrapper around a boxed membership function
pub use crate::term::Term;

//UniformSampling Functionality
pub use crate::sampler::{Sampler, UniformSampler};
