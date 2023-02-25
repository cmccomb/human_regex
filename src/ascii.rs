//! Functions for ASCII character classes

use super::humanregex::*;
use std::marker::PhantomData as pd;

/// A function to match any alphanumeric character (`[0-9A-Za-z]`)
pub fn alphanumeric() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:alnum:]]".to_string(), pd::<SymbolClass<Ascii>>)
}

/// A function to match any non-alphanumeric character (`[^0-9A-Za-z]`)
pub fn non_alphanumeric() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:^alnum:]]".to_string(), pd::<SymbolClass<Ascii>>)
}

/// A function to match any alphabetic character (`[A-Za-z]`)
pub fn alphabetic() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:alpha:]]".to_string(), pd::<SymbolClass<Ascii>>)
}

/// A function to match any non-alphabetic character (`[^A-Za-z]`)
pub fn non_alphabetic() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:^alpha:]]".to_string(), pd::<SymbolClass<Ascii>>)
}

/// A function to match any lowercase character (`[a-z]`)
pub fn lowercase() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:lower:]]".to_string(), pd::<SymbolClass<Ascii>>)
}

/// A function to match any non-lowercase character (`[^a-z]`)
pub fn non_lowercase() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:^lower:]]".to_string(), pd::<SymbolClass<Ascii>>)
}

/// A function to match any uppercase character (`[A-Z]`)
pub fn uppercase() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:upper:]]".to_string(), pd::<SymbolClass<Ascii>>)
}

/// A function to match any non-uppercase character (`[^A-Z]`)
pub fn non_uppercase() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:^upper:]]".to_string(), pd::<SymbolClass<Ascii>>)
}

/// A function to match any digit that would appear in a hexadecimal number (`[A-Fa-f0-9]`)
pub fn hexdigit() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:xdigit:]]".to_string(), pd::<SymbolClass<Ascii>>)
}

/// A function to match any digit that wouldn't appear in a hexadecimal number (`[^A-Fa-f0-9]`)
pub fn non_hexdigit() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:^xdigit:]]".to_string(), pd::<SymbolClass<Ascii>>)
}

/// A function to match any ascii digit (`[\x00-\x7F]`)
pub fn ascii() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:ascii:]]".to_string(), pd::<SymbolClass<Ascii>>)
}

/// A function to match any non-ascii digit (`[^\x00-\x7F]`)
pub fn non_ascii() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:^ascii:]]".to_string(), pd::<SymbolClass<Ascii>>)
}

/// A function to match blank characters (`[\t ]`)
pub fn blank() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:blank:]]".to_string(), pd::<SymbolClass<Ascii>>)
}

/// A function to match non-blank characters (`[^\t ]`)
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
pub fn non_punctuation() -> HumanRegex<SymbolClass<Ascii>> {
    HumanRegex(r"[[:^punct:]]".to_string(), pd::<SymbolClass<Ascii>>)
}
