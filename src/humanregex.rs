//! The bread and butter type system and basic operations.

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

/// Represents the state when [HumanRegex] is a wrapper for a quantifier (e.g., an expression that
/// matches a given number of a target). Importantly, these expressions are greedy by default and
/// can be converted to a lazy match with the [lazy] method.
pub struct Quantifier;

/// The HumanRegex struct which maintains and updates the regex string. For most use cases it will
/// never be necessary to instantiate this directly.
#[derive(Debug)]
pub struct HumanRegex<T = SymbolChain>(pub String, pub std::marker::PhantomData<T>);

impl<T> HumanRegex<T> {
    /// Convert to a rust Regex
    pub fn to_regex(&self) -> Regex {
        Regex::new(&self.0).unwrap()
    }
}

impl HumanRegex<Quantifier> {
    /// Add a lazy modifier to quantifier match.
    /// ```
    /// let lazy_regex = human_regex::at_least(2, human_regex::text("asdf")).lazy();
    /// ```
    /// However, some things cannot be made lazy! The following will not compile:
    /// ```ignore
    /// let lazy_regex = human_regex::text("asdf").lazy();
    /// ```
    pub fn lazy(&self) -> HumanRegex<SymbolChain> {
        HumanRegex(format!("{}?", &self.0), pd::<SymbolChain>)
    }
}

/// One of the three fundemental operations on Regular Languages, concatenation!
/// ```
/// use human_regex::{zero_or_one, text};
/// let regex_string = zero_or_one(text("chris")) + text("mccomb");
/// assert!(regex_string.to_regex().is_match("mccomb"));
/// assert!(regex_string.to_regex().is_match("chrismccomb"));
/// ```
impl<T, U> Add<HumanRegex<U>> for HumanRegex<T> {
    type Output = HumanRegex<SymbolChain>;

    fn add(self, rhs: HumanRegex<U>) -> Self::Output {
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
