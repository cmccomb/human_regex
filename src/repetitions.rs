//! Functions for matching repetitions

use super::humanregex::*;
use std::marker::PhantomData as pd;

/// Match at least _n_ of a certain target
/// ```
/// use human_regex::{at_least, text};
/// let regex_string = at_least(3, text("a"));
/// assert!(regex_string.to_regex().is_match("aaaa"));
/// assert!(!regex_string.to_regex().is_match("aa"));
/// ```
pub fn at_least<T>(n: u8, target: HumanRegex<T>) -> HumanRegex<Quantifier> {
    HumanRegex(format!("(?:{}){{{},}}", target, n), pd::<Quantifier>)
}

/// Match at least _n_ and at most _m_ of a certain target
/// ```
/// use human_regex::{between, text};
/// let regex_string = between(3, 5, text("a"));
/// assert!(regex_string.to_regex().is_match("aaaa"));
/// assert!(!regex_string.to_regex().is_match("aa"));
/// ```
pub fn between<T>(n: u8, m: u8, target: HumanRegex<T>) -> HumanRegex<Quantifier> {
    HumanRegex(format!("(?:{}){{{},{}}}", target, n, m), pd::<Quantifier>)
}

/// Match one or more of a certain target
/// ```
/// use human_regex::{one_or_more, text};
/// let regex_string = one_or_more(text("a"));
/// assert!(regex_string.to_regex().is_match("aaaa"));
/// assert!(!regex_string.to_regex().is_match("bb"));
/// ```
pub fn one_or_more<T>(target: HumanRegex<T>) -> HumanRegex<Quantifier> {
    HumanRegex(format!("(?:{})+", target), pd::<Quantifier>)
}

/// Match zero or more of a certain target
/// ```
/// use human_regex::{zero_or_more, text};
/// let regex_string = zero_or_more(text("a"));
/// assert!(regex_string.to_regex().is_match("a"));
/// assert!(regex_string.to_regex().is_match("aaaaa"));
/// assert!(regex_string.to_regex().is_match("bb"));
/// ```
pub fn zero_or_more<T>(target: HumanRegex<T>) -> HumanRegex<Quantifier> {
    HumanRegex(format!("(?:{})*", target), pd::<Quantifier>)
}

/// Match zero or one of a certain target
/// ```
/// use human_regex::{zero_or_one, text};
/// let regex_string = zero_or_one(text("a"));
/// assert!(regex_string.to_regex().is_match("a"));
/// assert!(regex_string.to_regex().is_match("bb"));
/// ```
pub fn zero_or_one<T>(target: HumanRegex<T>) -> HumanRegex<Quantifier> {
    HumanRegex(format!("(?:{})?", target), pd::<Quantifier>)
}

/// Match exactly _n_ of a certain target
/// ```
/// use human_regex::{exactly, text};
/// let regex_string = exactly(5, text("a"));
/// assert!(regex_string.to_regex().is_match("aaaaa"));
/// assert!(!regex_string.to_regex().is_match("aaa"));
/// ```
pub fn exactly<T>(n: u8, target: HumanRegex<T>) -> HumanRegex<Quantifier> {
    HumanRegex(format!("(?:{}){{{}}}", target, n), pd::<Quantifier>)
}
