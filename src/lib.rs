#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![warn(clippy::missing_docs_in_private_items)]
#![doc = include_str!("../README.md")]

mod humanregex;
pub use humanregex::HumanRegex;

pub mod shorthand;
pub use shorthand::*;

pub mod repetitions;
pub use repetitions::*;

pub mod logical;
pub use logical::*;

pub mod direct;
pub use direct::*;

pub mod capturing;
pub use capturing::*;

pub mod emptymatches;
pub use emptymatches::*;

pub mod ascii;
pub use ascii::*;

pub mod flags;
pub use flags::*;
