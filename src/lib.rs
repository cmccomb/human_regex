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

use regex::Regex;
use std::fmt;
use std::ops::Add;

/// A function for matching any character (except for \n)
pub fn any() -> HumanRegex {
    HumanRegex(r".".to_string())
}
/// A function for the digit character class (i.e., the digits 0 through 9)
pub fn digit() -> HumanRegex {
    HumanRegex(r"\d".to_string())
}
/// A function for the non-digit character class (i.e., everything BUT the digits 0-9)
pub fn non_digit() -> HumanRegex {
    HumanRegex(r"\D".to_string())
}
/// A function for the word character class (i.e., all alphanumeric characters plus underscore)
pub fn word() -> HumanRegex {
    HumanRegex(r"\w".to_string())
}
/// A function for the non-word character class (i.e., everything BUT the alphanumeric characters plus underscore)
pub fn non_word() -> HumanRegex {
    HumanRegex(r"\W".to_string())
}
/// A constant for the whitespace character class (i.e., space and tab)
pub fn whitespace() -> HumanRegex {
    HumanRegex(r"\t".to_string())
}
/// A function for the whitespace character class (i.e., everything BUT space and tab)
pub fn non_whitespace() -> HumanRegex {
    HumanRegex(r"\T".to_string())
}

/// A function to match the beginning of a string
pub fn begin() -> HumanRegex {
    HumanRegex(r"^".to_string())
}

/// A function to match the end of a string
pub fn end() -> HumanRegex {
    HumanRegex(r"$".to_string())
}

/// The HumanRegex struct which maintains and updates the regex string
#[derive(Debug)]
pub struct HumanRegex(String);

impl HumanRegex {
    /// Convert to a rust Regex
    pub fn to_regex(&self) -> Regex {
        Regex::new(&*self.0).unwrap()
    }
}

/// Match at least _n_ of a certain target
pub fn at_least<T>(n: u8, target: T) -> HumanRegex
where
    T: Into<String> + fmt::Display,
{
    HumanRegex(format!("{}{{{}}}", target, n))
}

/// Match at least _n_ and at most _m_ of a certain target
pub fn at_least_at_most<T>(n: u8, m: u8, target: T) -> HumanRegex
where
    T: Into<String> + fmt::Display,
{
    HumanRegex(format!("{}{{{},{}}}", target, n, m))
}

/// Match one or more of a certain target
pub fn one_or_more<T>(target: T) -> HumanRegex
where
    T: Into<String> + fmt::Display,
{
    HumanRegex(format!("{}+", target))
}

/// Match zero or more of a certain target
pub fn zero_or_more<T>(target: T) -> HumanRegex
where
    T: Into<String> + fmt::Display,
{
    HumanRegex(format!("{}*", target))
}

/// Match zero or one of a certain target
pub fn zero_or_one<T>(target: T) -> HumanRegex
where
    T: Into<String> + fmt::Display,
{
    HumanRegex(format!("{}?", target))
}

/// Match exactly _n_ of a certain target
pub fn exactly<T>(n: u8, target: T) -> HumanRegex
where
    T: Into<String> + fmt::Display,
{
    HumanRegex(format!("{}{{{}}}", target, n))
}

/// Add generic text to the regex string
pub fn text<T>(text: T) -> HumanRegex
where
    T: Into<String> + fmt::Display,
{
    HumanRegex(text.to_string())
}

impl Add for HumanRegex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let owned_string = format!("{}{}", self.to_string(), rhs.to_string());
        HumanRegex(owned_string)
    }
}

impl fmt::Display for HumanRegex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<HumanRegex> for String {
    fn from(hr: HumanRegex) -> Self {
        hr.to_string()
    }
}
