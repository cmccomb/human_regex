#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![warn(clippy::missing_docs_in_private_items)]

//! # Regex for Humans
//! The goal of this crate is simple: give everybody the power of regular expressions without having
//! to learn the complicated syntax. It is inspired by [ReadableRegex.jl](https://github.com/jkrumbiegel/ReadableRegex.jl).

use regex::Regex;

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
