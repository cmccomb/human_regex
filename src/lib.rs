#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![warn(clippy::missing_docs_in_private_items)]

//! # Regex for Humans
//! The goal of this crate is simple: give everybody the power of regular expressions without having
//! to learn the complicated syntax. It is inspired by [ReadableRegex.jl](https://github.com/jkrumbiegel/ReadableRegex.jl).

use regex::Regex;

/// A unit struct for the digit character class (i.e., the digits 0 through 9)
struct Digit;
/// A unit struct for the non-digit character class (i.e., everything BUT the digits 0-9)
struct NonDigit;
/// A unit struct for the word character class (i.e., all alphanumeric characters plus underscore)
struct Word;
/// A unit struct for the non-word character class (i.e., everything BUT the alphanumeric characters plus underscore)
struct NonWord;
/// A unit structure for the whitespace character class (i.e., space and tab)
struct Whitespace;
/// A unit structure for the whitespace character class (i.e., everything BUT space and tab)
struct NonWhitespace;

/// The HumanRegex struct which maintains and updates the regex string
#[derive(Default)]
pub struct HumanRegex {
    /// The internally-maintained true regex string
    pub regex_string: String,
}

impl HumanRegex {
    /// Generates a new human regex directly from a regex string
    pub fn new(regex_string: &str) -> Self {
        HumanRegex {
            regex_string: String::from(regex_string),
        }
    }

    /// Checks whether or not a string matches with the constructed regex
    pub fn is_match(&self, string_to_match: &str) -> bool {
        let re = Regex::new(&*self.regex_string).unwrap();
        re.is_match(string_to_match)
    }
}
