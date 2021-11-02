//! Functions for ASCII character classes

use super::humanregex::HumanRegex;

/// A function to match any alphanumeric character (`[0-9A-Za-z]`)
pub fn alphanumeric() -> HumanRegex {
    HumanRegex(r"[[:alnum:]]".to_string())
}

/// A function to match any alphabetic character (`[A-Za-z]`)
pub fn alphabetic() -> HumanRegex {
    HumanRegex(r"[[:alpha:]]".to_string())
}

/// A function to match any lowercase character (`[a-z]`)
pub fn lowercase() -> HumanRegex {
    HumanRegex(r"[[:lower:]]".to_string())
}

/// A function to match any lowercase character (`[A-Z]`)
pub fn uppercase() -> HumanRegex {
    HumanRegex(r"[[:upper:]]".to_string())
}

/// A function to match any digit that would appear in a hexadecimal number (`[A-Fa-f0-9]`)
pub fn hexdigit() -> HumanRegex {
    HumanRegex(r"[[:xdigit:]]".to_string())
}

/// A function to match any ascii digit ([\x00-\x7F]`)
pub fn ascii() -> HumanRegex {
    HumanRegex(r"[[:ascii:]]".to_string())
}

/// A function to match blank characters (`[\t ]`)
pub fn blank() -> HumanRegex {
    HumanRegex(r"[[:blank:]]".to_string())
}

/// A function to match control characters (`[\x00-\x1F\x7F]`)
pub fn control() -> HumanRegex {
    HumanRegex(r"[[:cntrl:]]".to_string())
}

/// A function to match graphical characters (`[!-~]`)
pub fn graphical() -> HumanRegex {
    HumanRegex(r"[[:graph:]]".to_string())
}

/// A function to match printable characters (`[ -~]`)
pub fn printable() -> HumanRegex {
    HumanRegex(r"[[:print:]]".to_string())
}

/// A function to match punctuation (`[!-/:-@\[-`{-~]`)
pub fn punctuation() -> HumanRegex {
    HumanRegex(r"[[:punct:]]".to_string())
}
