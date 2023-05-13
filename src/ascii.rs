//! Functions for ASCII character classes

use super::humanregex::*;
use std::marker::PhantomData as pd;

/// A function to match any alphanumeric character (`[0-9A-Za-z]`)
/// ```
/// use std::ops::Not;
/// let regex_string = human_regex::alphanumeric();
/// assert!(regex_string.to_regex().is_match("[").not());
/// assert!(regex_string.to_regex().is_match("a"));
/// assert!(regex_string.to_regex().is_match("A"));
/// assert!(regex_string.to_regex().is_match("1"));
/// ```
pub fn alphanumeric() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:alnum:]]".to_string(), pd::<SymbolClass<Ascii>>)
}

/// A function to match any non-alphanumeric character (`[^0-9A-Za-z]`)
/// ```
/// use std::ops::Not;
/// let regex_string = human_regex::non_alphanumeric();
/// assert!(regex_string.to_regex().is_match("["));
/// assert!(regex_string.to_regex().is_match("a").not());
/// assert!(regex_string.to_regex().is_match("A").not());
/// assert!(regex_string.to_regex().is_match("1").not());
/// ```
pub fn non_alphanumeric() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:^alnum:]]".to_string(), pd::<SymbolClass<Ascii>>)
}

/// A function to match any alphabetic character (`[A-Za-z]`)
/// ```
/// use std::ops::Not;
/// let regex_string = human_regex::alphabetic();
/// assert!(regex_string.to_regex().is_match("[").not());
/// assert!(regex_string.to_regex().is_match("a"));
/// assert!(regex_string.to_regex().is_match("A"));
/// assert!(regex_string.to_regex().is_match("1").not());
/// ```
pub fn alphabetic() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:alpha:]]".to_string(), pd::<SymbolClass<Ascii>>)
}

/// A function to match any non-alphabetic character (`[^A-Za-z]`)
/// ```
/// use std::ops::Not;
/// let regex_string = human_regex::non_alphabetic();
/// assert!(regex_string.to_regex().is_match("["));
/// assert!(regex_string.to_regex().is_match("a").not());
/// assert!(regex_string.to_regex().is_match("A").not());
/// assert!(regex_string.to_regex().is_match("1"));
/// ```
pub fn non_alphabetic() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:^alpha:]]".to_string(), pd::<SymbolClass<Ascii>>)
}

/// A function to match any lowercase character (`[a-z]`)
/// ```
/// use std::ops::Not;
/// let regex_string = human_regex::lowercase();
/// assert!(regex_string.to_regex().is_match("[").not());
/// assert!(regex_string.to_regex().is_match("a"));
/// assert!(regex_string.to_regex().is_match("A").not());
/// assert!(regex_string.to_regex().is_match("1").not());
/// ```
pub fn lowercase() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:lower:]]".to_string(), pd::<SymbolClass<Ascii>>)
}

/// A function to match any non-lowercase character (`[^a-z]`)
/// ```
/// use std::ops::Not;
/// let regex_string = human_regex::non_lowercase();
/// assert!(regex_string.to_regex().is_match("["));
/// assert!(regex_string.to_regex().is_match("a").not());
/// assert!(regex_string.to_regex().is_match("A"));
/// assert!(regex_string.to_regex().is_match("1"));
/// ```
pub fn non_lowercase() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:^lower:]]".to_string(), pd::<SymbolClass<Ascii>>)
}

/// A function to match any uppercase character (`[A-Z]`)
/// ```
/// use std::ops::Not;
/// let regex_string = human_regex::uppercase();
/// assert!(regex_string.to_regex().is_match("[").not());
/// assert!(regex_string.to_regex().is_match("a").not());
/// assert!(regex_string.to_regex().is_match("A"));
/// assert!(regex_string.to_regex().is_match("1").not());
/// ```
pub fn uppercase() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:upper:]]".to_string(), pd::<SymbolClass<Ascii>>)
}

/// A function to match any non-uppercase character (`[^A-Z]`)
/// ```
/// use std::ops::Not;
/// let regex_string = human_regex::non_uppercase();
/// assert!(regex_string.to_regex().is_match("["));
/// assert!(regex_string.to_regex().is_match("a"));
/// assert!(regex_string.to_regex().is_match("A").not());
/// assert!(regex_string.to_regex().is_match("1"));
/// ```
pub fn non_uppercase() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:^upper:]]".to_string(), pd::<SymbolClass<Ascii>>)
}

/// A function to match any digit that would appear in a hexadecimal number (`[A-Fa-f0-9]`)
/// ```
/// use std::ops::Not;
/// let regex_string = human_regex::hexdigit();
/// assert!(regex_string.to_regex().is_match("[").not());
/// assert!(regex_string.to_regex().is_match("a"));
/// assert!(regex_string.to_regex().is_match("A"));
/// assert!(regex_string.to_regex().is_match("g").not());
/// assert!(regex_string.to_regex().is_match("G").not());
/// assert!(regex_string.to_regex().is_match("1"));
/// ```
pub fn hexdigit() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:xdigit:]]".to_string(), pd::<SymbolClass<Ascii>>)
}

/// A function to match any digit that wouldn't appear in a hexadecimal number (`[^A-Fa-f0-9]`)
/// ```
/// use std::ops::Not;
/// let regex_string = human_regex::non_hexdigit();
/// assert!(regex_string.to_regex().is_match("["));
/// assert!(regex_string.to_regex().is_match("a").not());
/// assert!(regex_string.to_regex().is_match("A").not());
/// assert!(regex_string.to_regex().is_match("g"));
/// assert!(regex_string.to_regex().is_match("G"));
/// assert!(regex_string.to_regex().is_match("1").not());
/// ```
pub fn non_hexdigit() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:^xdigit:]]".to_string(), pd::<SymbolClass<Ascii>>)
}

/// A function to match any ascii digit (`[\x00-\x7F]`)
/// ```
/// use std::ops::Not;
/// let regex_string = human_regex::ascii();
/// assert!(regex_string.to_regex().is_match("["));
/// assert!(regex_string.to_regex().is_match("a"));
/// assert!(regex_string.to_regex().is_match("A"));
/// assert!(regex_string.to_regex().is_match("1"));
/// assert!(regex_string.to_regex().is_match("!"));
/// assert!(regex_string.to_regex().is_match("¡").not());
/// ```
pub fn ascii() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:ascii:]]".to_string(), pd::<SymbolClass<Ascii>>)
}

/// A function to match any non-ascii digit (`[^\x00-\x7F]`)
/// ```
/// use std::ops::Not;
/// let regex_string = human_regex::non_ascii();
/// assert!(regex_string.to_regex().is_match("[").not());
/// assert!(regex_string.to_regex().is_match("a").not());
/// assert!(regex_string.to_regex().is_match("G").not());
/// assert!(regex_string.to_regex().is_match("1").not());
/// assert!(regex_string.to_regex().is_match("!").not());
/// assert!(regex_string.to_regex().is_match("¡"));
/// ```
pub fn non_ascii() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:^ascii:]]".to_string(), pd::<SymbolClass<Ascii>>)
}

/// A function to match blank characters (`[\t ]`)
/// ```
/// use std::ops::Not;
/// let regex_string = human_regex::blank();
/// assert!(regex_string.to_regex().is_match("[").not());
/// assert!(regex_string.to_regex().is_match("a").not());
/// assert!(regex_string.to_regex().is_match("A").not());
/// assert!(regex_string.to_regex().is_match("1").not());
/// assert!(regex_string.to_regex().is_match("¡").not());
/// assert!(regex_string.to_regex().is_match("!").not());
/// assert!(regex_string.to_regex().is_match(" "));
/// ```
pub fn blank() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:blank:]]".to_string(), pd::<SymbolClass<Ascii>>)
}

/// A function to match non-blank characters (`[^\t ]`)
/// ```
/// use std::ops::Not;
/// let regex_string = human_regex::non_blank();
/// assert!(regex_string.to_regex().is_match("["));
/// assert!(regex_string.to_regex().is_match("a"));
/// assert!(regex_string.to_regex().is_match("A"));
/// assert!(regex_string.to_regex().is_match("1"));
/// assert!(regex_string.to_regex().is_match("¡"));
/// assert!(regex_string.to_regex().is_match("!"));
/// assert!(regex_string.to_regex().is_match(" ").not());
/// ```
pub fn non_blank() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:^blank:]]".to_string(), pd::<SymbolClass<Ascii>>)
}

/// A function to match control characters (`[\x00-\x1F\x7F]`)
pub fn control() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:cntrl:]]".to_string(), pd::<SymbolClass<Ascii>>)
}

/// A function to match non-control characters (`[^\x00-\x1F\x7F]`)
pub fn non_control() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:^cntrl:]]".to_string(), pd::<SymbolClass<Ascii>>)
}

/// A function to match graphical characters (`[!-~]`)
pub fn graphical() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:graph:]]".to_string(), pd::<SymbolClass<Ascii>>)
}

/// A function to match non-graphical characters (`[^!-~]`)
pub fn non_graphical() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:^graph:]]".to_string(), pd::<SymbolClass<Ascii>>)
}

/// A function to match printable characters (`[ -~]`)
pub fn printable() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:print:]]".to_string(), pd::<SymbolClass<Ascii>>)
}

/// A function to match unprintable characters (`[^ -~]`)
pub fn non_printable() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:^print:]]".to_string(), pd::<SymbolClass<Ascii>>)
}

/// A function to match punctuation (`[!-/:-@\[-`{-~]`)
pub fn punctuation() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:punct:]]".to_string(), pd::<SymbolClass<Ascii>>)
}

/// A function to match non-punctuation (`[^!-/:-@\[-`{-~]`)
/// ```
/// use std::ops::Not;
/// let regex_string = human_regex::non_punctuation();
/// assert!(regex_string.to_regex().is_match("[").not());
/// assert!(regex_string.to_regex().is_match("a"));
/// assert!(regex_string.to_regex().is_match("A"));
/// assert!(regex_string.to_regex().is_match("1"));
/// assert!(regex_string.to_regex().is_match("¡"));
/// assert!(regex_string.to_regex().is_match("!").not());
/// assert!(regex_string.to_regex().is_match(" "));
/// ```
pub fn non_punctuation() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:^punct:]]".to_string(), pd::<SymbolClass<Ascii>>)
}
