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
//!

use regex::Regex;
use std::fmt;
use std::ops::Add;

/// The HumanRegex struct which maintains and updates the regex string
#[derive(Debug)]
pub struct HumanRegex(String);

impl HumanRegex {
    /// Convert to a rust Regex
    pub fn to_regex(&self) -> Regex {
        Regex::new(&*self.0).unwrap()
    }
}

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
    HumanRegex(r"\s".to_string())
}

/// A function for the whitespace character class (i.e., everything BUT space and tab)
pub fn non_whitespace() -> HumanRegex {
    HumanRegex(r"\S".to_string())
}

/// A function to match the beginning of text
/// ```
/// let regex_string = human_regex::begin() + human_regex::text("hex");
/// assert!(regex_string.to_regex().is_match("hexagon"));
/// assert!(!regex_string.to_regex().is_match("chlorhexadine"));
/// ```
pub fn begin() -> HumanRegex {
    HumanRegex(r"^".to_string())
}

/// A function to match the end of text
/// ```
/// let regex_string = human_regex::text("end") + human_regex::end();
/// assert!(regex_string.to_regex().is_match("mend"));
/// assert!(!regex_string.to_regex().is_match("endocrinologist"));
/// ```
pub fn end() -> HumanRegex {
    HumanRegex(r"$".to_string())
}

/// Match at least _n_ of a certain target
/// ```
/// let regex_string = human_regex::at_least(3, "a");
/// assert!(regex_string.to_regex().is_match("aaaa"));
/// assert!(!regex_string.to_regex().is_match("aa"));
/// ```
pub fn at_least<T>(n: u8, target: T) -> HumanRegex
where
    T: Into<String> + fmt::Display,
{
    HumanRegex(format!("({}){{{},}}", target, n))
}

/// Match at least _n_ and at most _m_ of a certain target
/// ```
/// let regex_string = human_regex::at_least_at_most(3, 5, "a");
/// assert!(regex_string.to_regex().is_match("aaaa"));
/// assert!(!regex_string.to_regex().is_match("aa"));
/// ```
pub fn at_least_at_most<T>(n: u8, m: u8, target: T) -> HumanRegex
where
    T: Into<String> + fmt::Display,
{
    HumanRegex(format!("({}){{{},{}}}", target, n, m))
}

/// Match one or more of a certain target
/// ```
/// let regex_string = human_regex::one_or_more("a");
/// assert!(regex_string.to_regex().is_match("aaaa"));
/// assert!(!regex_string.to_regex().is_match("bb"));
/// ```
pub fn one_or_more<T>(target: T) -> HumanRegex
where
    T: Into<String> + fmt::Display,
{
    HumanRegex(format!("({})+", target))
}

/// Match zero or more of a certain target
/// ```
/// let regex_string = human_regex::zero_or_more("a");
/// assert!(regex_string.to_regex().is_match("a"));
/// assert!(regex_string.to_regex().is_match("bb"));
/// ```
pub fn zero_or_more<T>(target: T) -> HumanRegex
where
    T: Into<String> + fmt::Display,
{
    HumanRegex(format!("({})*", target))
}

/// Match zero or one of a certain target
/// ```
/// let regex_string = human_regex::zero_or_one("a");
/// assert!(regex_string.to_regex().is_match("a"));
/// assert!(regex_string.to_regex().is_match("bb"));
/// ```
pub fn zero_or_one<T>(target: T) -> HumanRegex
where
    T: Into<String> + fmt::Display,
{
    HumanRegex(format!("({})?", target))
}

/// Match zero or one of a certain target
/// ```
/// let regex_string = human_regex::optional("a");
/// assert!(regex_string.to_regex().is_match("a"));
/// assert!(regex_string.to_regex().is_match("bb"));
/// ```
pub fn optional<T>(target: T) -> HumanRegex
where
    T: Into<String> + fmt::Display,
{
    HumanRegex(format!("({})?", target))
}

/// Match exactly _n_ of a certain target
/// ```
/// let regex_string = human_regex::exactly(5, "a");
/// assert!(regex_string.to_regex().is_match("aaaaa"));
/// assert!(!regex_string.to_regex().is_match("aaa"));
/// ```
pub fn exactly<T>(n: u8, target: T) -> HumanRegex
where
    T: Into<String> + fmt::Display,
{
    HumanRegex(format!("({}){{{}}}", target, n))
}

/// Add generic text to the regex string
/// ```
/// let regex_string = human_regex::text("asdf");
/// assert!(regex_string.to_regex().is_match("asdf"));
/// assert!(!regex_string.to_regex().is_match("asddf"));
/// ```
pub fn text<T>(text: T) -> HumanRegex
where
    T: Into<String> + fmt::Display,
{
    HumanRegex(text.to_string())
}

/// A function for establishing an OR relationship between two possible matches
/// ```
/// let regex_string = human_regex::or(&["a", "b", "c"]);
/// assert!(regex_string.to_regex().is_match("a"));
/// ```
pub fn or<T>(options: &[T]) -> HumanRegex
where
    T: Into<String> + fmt::Display,
{
    let mut regex_string = format!("({})", options[0].to_string());
    for idx in 1..options.len() {
        regex_string = format!("{}|({})", regex_string, options[idx].to_string())
    }
    HumanRegex(regex_string)
}

/// ```
/// let regex_string = human_regex::optional("chris") + human_regex::text("mccomb");
/// assert!(regex_string.to_regex().is_match("mccomb"));
/// assert!(regex_string.to_regex().is_match("chrismccomb"));
/// ```
impl Add for HumanRegex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        HumanRegex(format!("{}{}", self.to_string(), rhs.to_string()))
    }
}

// Implement the Display trait for HumanRegex
impl fmt::Display for HumanRegex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Make it possible to create strings from HumanRegex
impl From<HumanRegex> for String {
    fn from(hr: HumanRegex) -> Self {
        hr.to_string()
    }
}
