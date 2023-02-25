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
/// use human_regex::{xor, within_range};
/// let regex_string = xor(within_range('a'..='g'), within_range('b'..='h'));
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
/// use human_regex::{and, within_range, within_set};
/// let regex_string = and(within_range('a'..='y'),within_set(&['x','y','z']));
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

/// See [and]
/// ```
/// use human_regex::{and, within_range, within_set};
/// let regex_string = (within_range('a'..='y') & within_set(&['x','y','z']));
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

/// Removes the characters in the second character class from the characters in the first
/// ```
/// use human_regex::{subtract, within_range, within_set};
/// let regex_string = subtract(within_range('0'..='9'), within_set(&['4']));
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

/// Negation for standard symbol classes.
/// ```
/// use human_regex::{digit};
/// assert_eq!(digit().to_string().replace(r"\d",r"\D"), r"\D");
/// ```
impl std::ops::Not for HumanRegex<SymbolClass<Standard>> {
    type Output = Self;

    fn not(self) -> Self::Output {
        if self
            .to_string()
            .chars()
            .nth(1)
            .expect("Should always be 2 characters in SymbolClass<Standard>")
            .is_lowercase()
        {
            HumanRegex(
                self.to_string()
                    .replace(r"\d", r"\D")
                    .replace(r"\p", r"\P")
                    .replace(r"\w", r"\W")
                    .replace(r"\s", r"\S")
                    .replace(r"\b", r"\B"),
                pd::<SymbolClass<Standard>>,
            )
        } else {
            HumanRegex(
                self.to_string()
                    .replace(r"\D", r"\d")
                    .replace(r"\P", r"\p")
                    .replace(r"\W", r"\w")
                    .replace(r"\S", r"\s")
                    .replace(r"\B", r"\b"),
                pd::<SymbolClass<Standard>>,
            )
        }
    }
}

impl std::ops::Not for HumanRegex<SymbolClass<Custom>> {
    type Output = Self;

    fn not(self) -> Self::Output {
        if self
            .to_string()
            .chars()
            .nth(1)
            .expect("Should always be at least 2 characters in SymbolClass<Custom>")
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
