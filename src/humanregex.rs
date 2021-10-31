use regex::Regex;

pub use std::fmt;
use std::ops::Add;

/// The HumanRegex struct which maintains and updates the regex string.
/// For most use cases it will never be necessary to instantiate this directly.
#[derive(Debug)]
pub struct HumanRegex(pub String);

impl HumanRegex {
    /// Convert to a rust Regex
    pub fn to_regex(&self) -> Regex {
        Regex::new(&*self.0).unwrap()
    }

    /// Add a lazy modifier
    pub fn lazy(&self) -> HumanRegex {
        HumanRegex(format!("{}?", &*self.0))
    }
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
