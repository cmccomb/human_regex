//! Functions for matching repetitions

use super::humanregex::*;
use std::marker::PhantomData as pd;

/// Match at least _n_ of a certain target
/// ```
/// let regex_string = human_regex::at_least(3, "a");
/// assert!(regex_string.to_regex().is_match("aaaa"));
/// assert!(!regex_string.to_regex().is_match("aa"));
/// ```
pub fn at_least<T>(n: u8, target: HumanRegex<T>) -> HumanRegex<SymbolChain> {
    HumanRegex(format!("(?:{}){{{},}}", target, n), pd::<SymbolChain>)
}

/// Match at least _n_ and at most _m_ of a certain target
/// ```
/// let regex_string = human_regex::between(3, 5, "a");
/// assert!(regex_string.to_regex().is_match("aaaa"));
/// assert!(!regex_string.to_regex().is_match("aa"));
/// ```
pub fn between<T>(n: u8, m: u8, target: HumanRegex<T>) -> HumanRegex<SymbolChain> {
    HumanRegex(format!("(?:{}){{{},{}}}", target, n, m), pd::<SymbolChain>)
}

/// Match one or more of a certain target
/// ```
/// let regex_string = human_regex::one_or_more("a");
/// assert!(regex_string.to_regex().is_match("aaaa"));
/// assert!(!regex_string.to_regex().is_match("bb"));
/// ```
pub fn one_or_more<T>(target: HumanRegex<T>) -> HumanRegex<SymbolChain> {
    HumanRegex(format!("(?:{})+", target), pd::<SymbolChain>)
}

/// Match zero or more of a certain target
/// ```
/// let regex_string = human_regex::zero_or_more("a");
/// assert!(regex_string.to_regex().is_match("a"));
/// assert!(regex_string.to_regex().is_match("bb"));
/// ```
pub fn zero_or_more<T>(target: HumanRegex<T>) -> HumanRegex<SymbolChain> {
    HumanRegex(format!("(?:{})*", target), pd::<SymbolChain>)
}

/// Match zero or one of a certain target
/// ```
/// let regex_string = human_regex::zero_or_one("a");
/// assert!(regex_string.to_regex().is_match("a"));
/// assert!(regex_string.to_regex().is_match("bb"));
/// ```
pub fn zero_or_one<T>(target: HumanRegex<T>) -> HumanRegex<SymbolChain> {
    HumanRegex(format!("(?:{})?", target), pd::<SymbolChain>)
}

/// Match exactly _n_ of a certain target
/// ```
/// let regex_string = human_regex::exactly(5, "a");
/// assert!(regex_string.to_regex().is_match("aaaaa"));
/// assert!(!regex_string.to_regex().is_match("aaa"));
/// ```
pub fn exactly<T>(n: u8, target: HumanRegex<T>) -> HumanRegex<SymbolChain> {
    HumanRegex(format!("(?:{}){{{}}}", target, n), pd::<SymbolChain>)
}
