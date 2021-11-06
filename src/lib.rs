#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![warn(clippy::missing_docs_in_private_items)]

//! # A Regex for Humans
//! The goal of this crate is simple: give everybody the power of regular expressions without having
//! to learn the complicated syntax. It is inspired by [ReadableRegex.jl](https://github.com/jkrumbiegel/ReadableRegex.jl).

//! # Example usage
//! ## Matching a date
//! If you want to match a date of the format `2021-10-30`, you could use the following code to generate a regex:
//! ```rust
//! use human_regex::{beginning, digit, exactly, text, end};
//! let regex_string = beginning()
//!     + exactly(4, digit())
//!     + text("-")
//!     + exactly(2, digit())
//!     + text("-")
//!     + exactly(2, digit())
//!     + end();
//! assert!(regex_string.to_regex().is_match("2014-01-01"))
//! ```
//! We can do this another way with slightly less repetition though!
//! ```rust
//! use human_regex::{beginning, digit, exactly, text, end};
//! let first_regex_string = text("-") + exactly(2, digit());
//! let second_regex_string = beginning()
//!     + exactly(4, digit())
//!     + exactly(2, first_regex_string)
//!     + end();
//! assert!(second_regex_string.to_regex().is_match("2014-01-01"))
//! ```
//! ##

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
