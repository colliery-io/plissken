//! Complex generic type examples for parser stress testing.
//!
//! This module contains intentionally complex type signatures
//! to test the parser's ability to handle edge cases.

mod nested;
mod bounds;
mod lifetimes;
mod associated;
mod const_generics;
mod pyo3_complex;

pub use nested::*;
pub use bounds::*;
pub use lifetimes::*;
pub use associated::*;
pub use const_generics::*;
pub use pyo3_complex::*;
