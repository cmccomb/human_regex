//! Functions for performing logical operations

use super::humanregex::HumanRegex;
use std::fmt;

/// A function for establishing an OR relationship between two or more possible matches
/// ```
/// use human_regex::{text, logical::or};
/// let regex_string = text("gr") + or(&[text("a"), text("e")]) + text("y");
/// println!("{}", regex_string.to_string());
/// assert!(regex_string.to_regex().is_match("grey"));
/// assert!(regex_string.to_regex().is_match("gray"));
/// assert!(!regex_string.to_regex().is_match("graey"));
/// ```
pub fn or<T>(options: &[T]) -> HumanRegex
where
    T: Into<String> + fmt::Display,
{
    let mut regex_string = format!("{}", options[0].to_string());
    for idx in 1..options.len() {
        regex_string = format!("{}|{}", regex_string, options[idx].to_string())
    }
    HumanRegex(format!("[(:?{})]", regex_string))
}

/// Negated [or] relationship between two or more possible matches
/// ```
/// use human_regex::{text, logical::nor};
/// let regex_string = text("gr") + nor(&[text("a"), text("e")]) + text("y");
/// println!("{}", regex_string.to_string());
/// assert!(!regex_string.to_regex().is_match("grey"));
/// assert!(!regex_string.to_regex().is_match("gray"));
/// assert!(regex_string.to_regex().is_match("groy"));
/// ```
pub fn nor<T>(options: &[T]) -> HumanRegex
where
    T: Into<String> + fmt::Display,
{
    HumanRegex(or(options).to_string().replacen("[", "[^", 1))
}

/// A function for establishing an AND relationship between two or more possible matches
/// ```
/// use human_regex::{beginning, end, text, logical::{and,or}, shorthand::within_range};
/// let regex_string = beginning() + and(&vec![within_range('a'..='y'),or(&['x','y','z'])]) + end();
/// println!("{}", regex_string);
/// assert!(regex_string.to_regex().is_match("x"));
/// assert!(regex_string.to_regex().is_match("y"));
/// assert!(!regex_string.to_regex().is_match("z"));
/// ```
pub fn and<T>(options: &[T]) -> HumanRegex
where
    T: Into<String> + fmt::Display,
{
    let mut regex_string = format!("{}", options[0].to_string());
    for idx in 1..options.len() {
        regex_string = format!("[{}&&{}]", regex_string, options[idx].to_string())
    }
    HumanRegex(format!("(:?{})", regex_string))
}
/// Allows the use of `&` as a syntax sugar for [and]
/// ```
/// use human_regex::{beginning, end, text, logical::or, shorthand::within_range};
/// let regex_string = beginning() + (within_range('a'..='y') & or(&['x','y','z'])) + end();
/// println!("{}", regex_string);
/// assert!(regex_string.to_regex().is_match("x"));
/// assert!(regex_string.to_regex().is_match("y"));
/// assert!(!regex_string.to_regex().is_match("z"));
/// ```
impl std::ops::BitAnd for HumanRegex {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        and(&vec![self, rhs])
    }
}
