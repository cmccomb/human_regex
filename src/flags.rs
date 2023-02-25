//! Functions for adding flags
// i     case-insensitive: letters match both upper and lower case
// m     multi-line mode: ^ and $ match begin/end of line
// s     allow . to match \n
// u     Unicode support (enabled by default)

use super::humanregex::*;
use std::marker::PhantomData as pd;

/// Makes all matches case insensitive, matching both upper and lowercase letters.
/// ```
/// use human_regex::{case_insensitive, text};
/// let regex_string = case_insensitive(text("spongebob"));
/// assert!(regex_string.to_regex().is_match("SpOnGeBoB"));
/// assert!(regex_string.to_regex().is_match("spongebob"));
/// assert!(!regex_string.to_regex().is_match("PaTrIcK"));
/// ```
pub fn case_insensitive<T>(target: HumanRegex<T>) -> HumanRegex<SymbolChain> {
    HumanRegex(format!("(?i:{})", target), pd::<SymbolChain>)
}

/// Enables multiline mode, which will allow `beginning()` and `end()` to match the beginning and end of lines
pub fn multi_line_mode<T>(target: HumanRegex<T>) -> HumanRegex<SymbolChain> {
    HumanRegex(format!("(?m:{})", target), pd::<SymbolChain>)
}

/// A function that will allow `.` to match newlines (`\n`)
pub fn dot_matches_newline_too<T>(target: HumanRegex<T>) -> HumanRegex<SymbolChain> {
    HumanRegex(format!("(?s:{})", target), pd::<SymbolChain>)
}

/// A function to disable unicode support
pub fn disable_unicode<T>(target: HumanRegex<T>) -> HumanRegex<SymbolChain> {
    HumanRegex(format!("(?-u:{})", target), pd::<SymbolChain>)
}
