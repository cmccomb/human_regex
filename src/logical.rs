//! Functions for performing logical operations

use super::humanregex::*;
use std::marker::PhantomData as pd;

/// A function for establishing an OR relationship between two or more possible matches
/// ```
/// use human_regex::{text, logical::or};
/// let regex_string = text("gr") + or(&[text("a"), text("e")]) + text("y");
/// println!("{}", regex_string.to_string());
/// assert!(regex_string.to_regex().is_match("grey"));
/// assert!(regex_string.to_regex().is_match("gray"));
/// assert!(!regex_string.to_regex().is_match("graey"));
/// ```
pub fn or<T>(options: &[T]) -> HumanRegex<SymbolChain>
where
    T: Into<String> + fmt::Display,
{
    let mut regex_string = format!("{}", options[0].to_string());
    for idx in 1..options.len() {
        regex_string = format!("{}|{}", regex_string, options[idx].to_string())
    }
    HumanRegex(format!("(:?{})", regex_string), pd::<SymbolChain>)
}

/// Xor on two [SymbolClass]es, also known as symmetric difference.
///
/// ```
/// use human_regex::xor;
/// let regex_string = xor(&('a'..='g').collect::<Vec<char>>(), &('b'..='h').collect::<Vec<char>>());
/// println!("{}", regex_string);
/// assert!(regex_string.to_regex().is_match("a"));
/// assert!(regex_string.to_regex().is_match("h"));
/// assert!(!regex_string.to_regex().is_match("d"));
/// ```
pub fn xor<T, U>(
    lhs: HumanRegex<SymbolClass<T>>,
    rhs: HumanRegex<SymbolClass<U>>,
) -> HumanRegex<SymbolClass<Custom>> {
    HumanRegex(
        format!("[{}~~{}]", lhs.to_string(), rhs.to_string()),
        pd::<SymbolClass<Custom>>,
    )
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
pub fn and<T, U>(
    lhs: HumanRegex<SymbolClass<T>>,
    rhs: HumanRegex<SymbolClass<U>>,
) -> HumanRegex<SymbolClass<Custom>> {
    lhs & rhs
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
impl<T, U> std::ops::BitAnd<HumanRegex<SymbolClass<U>>> for HumanRegex<SymbolClass<T>> {
    type Output = HumanRegex<SymbolClass<Custom>>;

    fn bitand(self, rhs: HumanRegex<SymbolClass<U>>) -> Self::Output {
        HumanRegex(
            format!("[{}&&{}]", self.to_string(), rhs.to_string()),
            pd::<SymbolClass<Custom>>,
        )
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
pub fn subtract<T, U>(
    from: HumanRegex<SymbolClass<T>>,
    subtract: HumanRegex<SymbolClass<U>>,
) -> HumanRegex<SymbolClass<Custom>> {
    HumanRegex(
        format!("[{}--{}]", from.to_string(), subtract.to_string()),
        pd::<SymbolClass<Custom>>,
    )
}

impl std::ops::Not for HumanRegex<SymbolClass<Standard>> {
    type Output = Self;

    fn not(self) -> Self::Output {
        HumanRegex(
            self.to_string()
                .replace(r"\d", r"\D")
                .replace(r"\d", r"\D")
                .replace(r"\d", r"\D")
                .replace(r"\d", r"\D")
                .replace(r"\d", r"\D")
                .replace(r"\d", r"\D")
                .replace(r"\d", r"\D")
                .replace(r"\d", r"\D")
                .replace(r"\d", r"\D")
                .replace(r"\d", r"\D"),
            pd::<SymbolClass<Standard>>,
        )
    }
}

impl std::ops::Not for HumanRegex<SymbolClass<Custom>> {
    type Output = Self;

    fn not(self) -> Self::Output {
        if self
            .to_string()
            .chars()
            .nth(0)
            .expect("Should always be at least 1 character")
            != '^'
        {
            HumanRegex(
                self.to_string().replace("[", "[^"),
                pd::<SymbolClass<Custom>>,
            )
        } else {
            HumanRegex(
                self.to_string().replace("[^", "["),
                pd::<SymbolClass<Custom>>,
            )
        }
    }
}

impl std::ops::Not for HumanRegex<SymbolClass<Ascii>> {
    type Output = Self;

    fn not(self) -> Self::Output {
        if self
            .to_string()
            .chars()
            .nth(3)
            .expect("Should always be at least 4 characters in SymbolClass<Ascii>")
            != '^'
        {
            HumanRegex(
                self.to_string().replace("[[:", "[[:^"),
                pd::<SymbolClass<Ascii>>,
            )
        } else {
            HumanRegex(
                self.to_string().replace("[[:^", "[[:"),
                pd::<SymbolClass<Ascii>>,
            )
        }
    }
}

impl std::ops::Not for HumanRegex<LiteralSymbolChain> {
    type Output = HumanRegex<SymbolChain>;

    fn not(self) -> Self::Output {
        HumanRegex(
            self.to_string()
                .chars()
                .into_iter()
                .map(|chr| format!("[^{}]", chr))
                .collect::<String>(),
            pd::<SymbolChain>,
        )
    }
}
