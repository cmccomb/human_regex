//! Functions for adding flags
// i     case-insensitive: letters match both upper and lower case
// m     multi-line mode: ^ and $ match begin/end of line
// s     allow . to match \n
// u     Unicode support (enabled by default)

use super::humanregex::HumanRegex;

/// Makes all matches case insensitive, matching both upper and lowercase letters.
/// ```
/// use human_regex::{case_insensitive, text};
/// let regex_string = case_insensitive(text("spongebob"));
/// assert!(regex_string.to_regex().is_match("SpOnGeBoB"));
/// assert!(regex_string.to_regex().is_match("spongebob"));
/// assert!(!regex_string.to_regex().is_match("PaTrIcK"));
/// ```
pub fn case_insensitive(target: HumanRegex) -> HumanRegex {
    HumanRegex(format!("(?i:{})", target))
}

/// Enables multiline mode, which will allow `beginning()` and `end()` to match the beginning and end of lines
pub fn multi_line_mode(target: HumanRegex) -> HumanRegex {
    HumanRegex(format!("(?m:{})", target))
}

/// A function that will allow `.` to match newlines (`\n`)
pub fn dot_matches_newline_too(target: HumanRegex) -> HumanRegex {
    HumanRegex(format!("(?s:{})", target))
}

/// A function to disable unicode support
pub fn disable_unicode(target: HumanRegex) -> HumanRegex {
    HumanRegex(format!("(?-u:{})", target))
}
