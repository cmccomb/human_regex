use super::humanregex::{fmt, HumanRegex};

/// A function for establishing an OR relationship between two possible matches
/// ```
/// let regex_string = human_regex::or(&["a", "b", "c"]);
/// assert!(regex_string.to_regex().is_match("a"));
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
