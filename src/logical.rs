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
/// use human_regex::{text, nor};
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
/// use human_regex::{text, and, or, within};
/// let regex_string = and(&vec![within('a'..='y'),or(&['x','y','z'])]);
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
/// use human_regex::{text, or, within};
/// let regex_string = (within('a'..='y') & or(&['x','y','z']));
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

/// Subtracts the second argument from the first
///
/// If you would like to use ranges, collect them into a Vec<T>.
/// ```
/// use human_regex::subtract;
/// let regex_string = subtract(&('0'..='9').collect::<Vec<char>>(), &['4']);
/// println!("{}", regex_string);
/// assert!(regex_string.to_regex().is_match("3"));
/// assert!(regex_string.to_regex().is_match("9"));
/// assert!(!regex_string.to_regex().is_match("4"));
/// ```
pub fn subtract<T>(from: &[T], subtract: &[T]) -> HumanRegex
where
    T: Into<String> + fmt::Display,
{
    HumanRegex(format!("[{}--{}]", or(from), or(subtract)))
    // I really don't like this implementation, but it's the only
    // "type safe" way to do it for now. I plan on completely overhauling
    // the HumanRegex type system to include things like "BracketedExpression",
    // which will implement Into<HumanRegex>, and that will be what's used as
    // arguments here in the future.
}
