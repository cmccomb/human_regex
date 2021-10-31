use super::humanregex::{fmt, HumanRegex};
use regex::escape;

/// A function for matching any character (except for \n)
pub fn any() -> HumanRegex {
    HumanRegex(r".".to_string())
}

/// A function for the digit character class (i.e., the digits 0 through 9)
pub fn digit() -> HumanRegex {
    HumanRegex(r"\d".to_string())
}

/// A function for the non-digit character class (i.e., everything BUT the digits 0-9)
pub fn non_digit() -> HumanRegex {
    HumanRegex(r"\D".to_string())
}

/// A function for the word character class (i.e., all alphanumeric characters plus underscore)
pub fn word() -> HumanRegex {
    HumanRegex(r"\w".to_string())
}

/// A function for the non-word character class (i.e., everything BUT the alphanumeric characters plus underscore)
pub fn non_word() -> HumanRegex {
    HumanRegex(r"\W".to_string())
}

/// A constant for the whitespace character class (i.e., space and tab)
pub fn whitespace() -> HumanRegex {
    HumanRegex(r"\s".to_string())
}

/// A function for the whitespace character class (i.e., everything BUT space and tab)
pub fn non_whitespace() -> HumanRegex {
    HumanRegex(r"\S".to_string())
}

/// A function to match the beginning of text
/// ```
/// let regex_string = human_regex::begin() + human_regex::text("hex");
/// assert!(regex_string.to_regex().is_match("hexagon"));
/// assert!(!regex_string.to_regex().is_match("chlorhexadine"));
/// ```
pub fn begin() -> HumanRegex {
    HumanRegex(r"^".to_string())
}

/// A function to match the end of text
/// ```
/// let regex_string = human_regex::text("end") + human_regex::end();
/// assert!(regex_string.to_regex().is_match("mend"));
/// assert!(!regex_string.to_regex().is_match("endocrinologist"));
/// ```
pub fn end() -> HumanRegex {
    HumanRegex(r"$".to_string())
}

/// Add generic text to the regex string. Text that is added through this function is automatically escaped.
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
