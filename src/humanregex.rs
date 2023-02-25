use regex::Regex;

pub(crate) use std::fmt;
use std::marker::PhantomData as pd;
use std::ops::Add;

/// Represents the state when [HumanRegex] is a wrapper for a standard single-character class (the kind that starts with a backslash followed by a letter)
pub struct Standard;

/// Represents the state when [HumanRegex] is a wrapper for a custom single-character class (the kind surrounded by one layer of square brackets)
pub struct Custom;

/// Represents the state when [HumanRegex] is a wrapper for a single-character ASCII class (the kind surrounded by colons and two layers of square brackets)
pub struct Ascii;

/// Represents the state when [HumanRegex] is a wrapper for any type of single-character class
pub struct SymbolClass<T>(std::marker::PhantomData<T>);

/// Represents the state when [HumanRegex] is a wrapper for a literal string of characters
pub struct LiteralSymbolChain;

/// Represents the state when [HumanRegex] is a wrapper for any arbitrary regular expression
pub struct SymbolChain;

/// The HumanRegex struct which maintains and updates the regex string.
/// For most use cases it will never be necessary to instantiate this directly.
#[derive(Debug)]
pub struct HumanRegex<T = SymbolChain>(pub String, pub std::marker::PhantomData<T>);

impl<T> HumanRegex<T> {
    /// Convert to a rust Regex
    pub fn to_regex(&self) -> Regex {
        Regex::new(&*self.0).unwrap()
    }

    /// Add a lazy modifier
    pub fn lazy(&self) -> HumanRegex<T> {
        HumanRegex(format!("{}?", &*self.0), pd::<T>)
    }
}

/// ```
/// let regex_string = human_regex::zero_or_one("chris") + human_regex::text("mccomb");
/// assert!(regex_string.to_regex().is_match("mccomb"));
/// assert!(regex_string.to_regex().is_match("chrismccomb"));
/// ```
impl<T> Add for HumanRegex<T> {
    type Output = HumanRegex<SymbolChain>;

    fn add(self, rhs: Self) -> Self::Output {
        HumanRegex(
            format!("{}{}", self.to_string(), rhs.to_string()),
            pd::<SymbolChain>,
        )
    }
}

// Implement the Display trait for HumanRegex
impl<T> fmt::Display for HumanRegex<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Make it possible to create strings from HumanRegex
impl<T> From<HumanRegex<T>> for String {
    fn from(hr: HumanRegex<T>) -> Self {
        hr.to_string()
    }
}
