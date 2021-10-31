use super::humanregex::HumanRegex;

/// A function for matching any character (except for \n)
/// ```
/// use human_regex::{text, any, exactly};
/// let regex_string = text("h") + exactly(2, any()) + text("l");
/// assert!(regex_string.to_regex().is_match("hurl"));
/// assert!(regex_string.to_regex().is_match("heal"));
/// ```
pub fn any() -> HumanRegex {
    HumanRegex(r".".to_string())
}

/// A function for the digit character class (i.e., the digits 0 through 9)
/// ```
/// use human_regex::{begin, end, one_or_more, digit};
/// let regex_string = begin() + one_or_more(digit()) + end();
/// assert!(regex_string.to_regex().is_match("010101010100100100100101"));
/// assert!(!regex_string.to_regex().is_match("a string that is not composed of digits will fail"));
/// ```
pub fn digit() -> HumanRegex {
    HumanRegex(r"\d".to_string())
}

/// A function for the non-digit character class (i.e., everything BUT the digits 0-9)
/// ```
/// use human_regex::{begin, end, one_or_more, non_digit};
/// let regex_string = begin() + one_or_more(non_digit()) + end();
/// assert!(regex_string.to_regex().is_match("a string without digits will pass"));
/// assert!(!regex_string.to_regex().is_match("a string with digits like 99 will fail"));
/// ```
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
/// ```
/// use human_regex::{one_or_more, text, whitespace};
/// let regex_string = text("at") + one_or_more(whitespace()) + text("least");
/// assert!(!regex_string.to_regex().is_match("atleast"));
/// assert!(regex_string.to_regex().is_match("at least"));
/// assert!(regex_string.to_regex().is_match("at    least"));
/// ```
pub fn whitespace() -> HumanRegex {
    HumanRegex(r"\s".to_string())
}

/// A function for the whitespace character class (i.e., everything BUT space and tab)
/// ```
/// use human_regex::{begin, end, one_or_more, non_whitespace};
/// let regex_string = begin() + one_or_more(non_whitespace()) + end();
/// assert!(regex_string.to_regex().is_match("supercalifragilisticexpialidocious"));
/// assert!(regex_string.to_regex().is_match("a-sluggified-thingamajig"));
/// assert!(!regex_string.to_regex().is_match("something with spaces won't pass"));
/// ```
pub fn non_whitespace() -> HumanRegex {
    HumanRegex(r"\S".to_string())
}

/// A function to match the beginning of text
/// ```
/// use human_regex::{begin, text};
/// let regex_string = begin() + text("hex");
/// assert!(regex_string.to_regex().is_match("hexagon"));
/// assert!(!regex_string.to_regex().is_match("chlorhexadine"));
/// ```
pub fn begin() -> HumanRegex {
    HumanRegex(r"^".to_string())
}

/// A function to match the end of text
/// ```
/// use human_regex::{end, text};
/// let regex_string = text("end") + end();
/// assert!(regex_string.to_regex().is_match("mend"));
/// assert!(!regex_string.to_regex().is_match("endocrinologist"));
/// ```
pub fn end() -> HumanRegex {
    HumanRegex(r"$".to_string())
}

/// A function to match a word boundary
pub fn word_boundary() -> HumanRegex {
    HumanRegex(r"\b".to_string())
}

/// A function to match anything BUT a word boundary
pub fn non_word_boundary() -> HumanRegex {
    HumanRegex(r"\B".to_string())
}

/// A function to match any alphanumeric character ([0-9A-Za-z])
pub fn alphanumeric() -> HumanRegex {
    HumanRegex(r"[[:alnum:]]".to_string())
}

/// A function to match any alphabetic character ([A-Za-z])
pub fn alphabetic() -> HumanRegex {
    HumanRegex(r"[[:alpha:]]".to_string())
}

/// A function to match any lowercase character ([a-z])
pub fn lowercase() -> HumanRegex {
    HumanRegex(r"[[:lower:]]".to_string())
}

/// A function to match any lowercase character ([A-Z])
pub fn uppercase() -> HumanRegex {
    HumanRegex(r"[[:upper:]]".to_string())
}

/// A function to match any digit that would appear in a hexadecimal number ([A-Fa-f0-9])
pub fn hexdigit() -> HumanRegex {
    HumanRegex(r"[[:xdigit:]]".to_string())
}
