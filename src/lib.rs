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
//! use human_regex as hr;
//! let regex_string = hr::begin()
//!     + hr::exactly(4, hr::digit())
//!     + hr::text("-")
//!     + hr::exactly(2, hr::digit())
//!     + hr::text("-")
//!     + hr::exactly(2, hr::digit())
//!     + hr::end();
//! assert!(regex_string.to_regex().is_match("2014-01-01"))
//! ```
//! We can do this another way with slightly less repetition though!
//! ```rust
//! use human_regex as hr;
//! let first_regex_string = hr::text("-") + hr::exactly(2, hr::digit());
//! let second_regex_string = hr::begin()
//!     + hr::exactly(4, hr::digit())
//!     + hr::exactly(2, first_regex_string)
//!     + hr::end();
//! println!("{}", second_regex_string);
//! assert!(second_regex_string.to_regex().is_match("2014-01-01"))
//! ```

mod shorthand;
pub use shorthand::{
    any, begin, digit, end, non_digit, non_whitespace, non_word, text, whitespace, word,
};

mod humanregex;
pub use humanregex::{fmt, HumanRegex};

mod repetitions;
pub use repetitions::{
    at_least, at_least_at_most, exactly, one_or_more, optional, zero_or_more, zero_or_one,
};

mod logical;
pub use logical::or;
