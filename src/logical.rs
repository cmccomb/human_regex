//! Functions for performing logical operations

use super::humanregex::*;
use crate::{any, exactly, none, text, zero_or_one};
use std::{marker::PhantomData as pd, ops::BitOr};

/// A function for establishing an OR relationship between two or more possible matches.
///
/// This explicit function is meant to be used to quickly "or" together RegExes of the same type.
/// If you need to perform an OR operation on expressions of differing types, use the [BitOr] ("|") operator.
/// ```
/// use human_regex::{text, logical::or};
/// let regex_string = text("gr") + or(&[text("a"), text("e")]) + text("y");
/// println!("{}", regex_string.to_string());
/// assert!(regex_string.to_regex().is_match("grey"));
/// assert!(regex_string.to_regex().is_match("gray"));
/// assert!(!regex_string.to_regex().is_match("graey"));
/// ```
pub fn or<T>(expressions: &[HumanRegex<T>]) -> HumanRegex<SymbolChain> {
    let mut regex_string = format!("{}", expressions[0].to_string());
    for expression in expressions.iter().skip(1) {
        regex_string = format!("{}|{}", regex_string, expression.to_string())
    }
    HumanRegex(format!("(?:{})", regex_string), pd::<SymbolChain>)
}

/// Binary OR on any two expressions.
///
/// If you need to do a batch OR, try [or].
/// ```
/// use human_regex::{text, one_or_more};
/// let expr = text("w") + (one_or_more(text("o"))+text("w"))|(text("oah"));
/// assert!(expr.to_regex().is_match("wow"));
/// assert!(expr.to_regex().is_match("woooooow"));
/// assert!(expr.to_regex().is_match("woah"));
/// ```
impl<T, U> BitOr<HumanRegex<U>> for HumanRegex<T> {
    type Output = HumanRegex<SymbolChain>;
    fn bitor(self, rhs: HumanRegex<U>) -> Self::Output {
        HumanRegex(format!("(?:{}|{})", self, rhs), pd::<SymbolChain>)
    }
}

/// Xor on two SymbolClasses, also known as symmetric difference.
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
/// use human_regex::{and, within_range, within_set, text};
/// let regex_string = and(within_range('a'..='y'),within_set(&[text("xyz")]));
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
/// use human_regex::{and, within_range, within_set, text};
/// let regex_string = (within_range('a'..='y') & within_set(&[text("xyz")]));
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
/// use human_regex::{subtract, within_range, within_set, text};
/// let regex_string = subtract(within_range('0'..='9'), within_set(&[text('4')]));
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
/// assert_eq!((!digit()).to_string(), r"\D");
/// ```
impl std::ops::Not for HumanRegex<SymbolClass<Standard>> {
    type Output = Self;

    fn not(self) -> Self::Output {
        if self.to_string() == any().to_string() {
            return none();
        }
        if self.to_string() == none().to_string() {
            return any();
        }
        if self
            .to_string()
            .chars()
            .nth(1)
            .expect("All classes shorter than 2 characters filtered above")
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

/// Negation for custom symbol classes.
/// ```
/// use human_regex::{digit};
///
/// assert_eq!((!digit()).to_string(), r"\D");
/// ```
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
                self.to_string().replace('[', "[^"),
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

/// Negation for ASCII symbol classes.
/// ```
/// use human_regex::{punctuation};
///
/// assert_eq!((!punctuation()).to_string(), r"[[:^punct:]]");
/// ```
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

/// Negation for literal text.
/// ```
/// use human_regex::{text};
///
/// assert_eq!((!text(r"&\abc")).to_string(), r"[^\&][^\\][^a][^b][^c]");
/// ```
impl std::ops::Not for HumanRegex<LiteralSymbolChain> {
    type Output = HumanRegex<SymbolChain>;

    fn not(self) -> Self::Output {
        HumanRegex(
            (zero_or_one(text(r"\")) + exactly(1, any()))
                .to_regex()
                .captures_iter(&self.to_string())
                .map(|x| format!("[^{}]", &x[0]))
                .collect::<String>(),
            pd::<SymbolChain>,
        )
    }
}
