//! Functions for directly matching text or adding known regex strings

use super::humanregex::{fmt, HumanRegex};
use regex::escape;

/// Add matching text to the regex string. Text that is added through this function is automatically escaped.
/// ```
/// let regex_string = human_regex::text("asdf");
/// assert!(regex_string.to_regex().is_match("asdf"));
/// assert!(!regex_string.to_regex().is_match("asddf"));
/// ```
pub fn text<T>(text: T) -> HumanRegex
where
    T: Into<String> + fmt::Display,
{
    HumanRegex(escape(&*text.to_string()))
}

/// Add a regex string directly to the regex string. This text is not escaped.
/// ```
/// let regex_string = human_regex::direct_regex(r"^\d{2}$");
/// println!("{}", regex_string.to_string());
/// assert!(regex_string.to_regex().is_match("21"));
/// assert!(!regex_string.to_regex().is_match("007"));
/// ```
pub fn direct_regex(text: &str) -> HumanRegex {
    HumanRegex(text.to_string())
}
