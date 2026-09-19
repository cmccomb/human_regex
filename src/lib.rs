#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![doc = include_str!("../README.md")]

pub mod cookbook;

mod humanregex;
#[doc(inline)]
pub use humanregex::HumanRegex;

pub mod shorthand;
#[doc(inline)]
pub use shorthand::*;

pub mod repetitions;
#[doc(inline)]
pub use repetitions::*;

pub mod logical;
#[doc(inline)]
pub use logical::*;

pub mod direct;
#[doc(inline)]
pub use direct::*;

pub mod capturing;
#[doc(inline)]
pub use capturing::*;

pub mod emptymatches;
#[doc(inline)]
pub use emptymatches::*;

pub mod ascii;
#[doc(inline)]
pub use ascii::*;

pub mod flags;
#[doc(inline)]
pub use flags::*;
