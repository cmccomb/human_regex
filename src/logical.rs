use super::humanregex::{fmt, HumanRegex};

/// A function for establishing an OR relationship between two possible matches
/// ```
/// use human_regex::{text, or};
/// let regex_string = text("gr") + or(&["a", "e"]) + text("y");
/// assert!(regex_string.to_regex().is_match("grey"));
/// assert!(regex_string.to_regex().is_match("gray"));
/// assert!(!regex_string.to_regex().is_match("graey"));
/// ```
pub fn or<T>(options: &[T]) -> HumanRegex
where
    T: Into<String> + fmt::Display,
{
    let mut regex_string = format!("({})", options[0].to_string());
    for idx in 1..options.len() {
        regex_string = format!("{}|({})", regex_string, options[idx].to_string())
    }
    HumanRegex(regex_string)
}
