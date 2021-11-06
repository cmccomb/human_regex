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
    HumanRegex(format!("(?:{})", escape(&*text.to_string())))
}

/// This text is not escaped. You can use it, for instance, to add a regex string directly to the object.
/// ```
/// let regex_string = human_regex::nonescaped_text(r"^\d{2}$");
/// println!("{}", regex_string.to_string());
/// assert!(regex_string.to_regex().is_match("21"));
/// assert!(!regex_string.to_regex().is_match("007"));
/// ```
pub fn nonescaped_text(text: &str) -> HumanRegex {
    HumanRegex(format!("(?:{})", text.to_string()))
}
