//! Functions for ASCII character classes

use super::humanregex::HumanRegex;

/// A function to match any alphanumeric character (`[0-9A-Za-z]`)
pub fn alphanumeric() -> HumanRegex {
    HumanRegex(r"[[:alnum:]]".to_string())
}

/// A function to match any non-alphanumeric character (`[^0-9A-Za-z]`)
pub fn non_alphanumeric() -> HumanRegex {
    HumanRegex(r"[[:^alnum:]]".to_string())
}

/// A function to match any alphabetic character (`[A-Za-z]`)
pub fn alphabetic() -> HumanRegex {
    HumanRegex(r"[[:alpha:]]".to_string())
}

/// A function to match any non-alphabetic character (`[^A-Za-z]`)
pub fn non_alphabetic() -> HumanRegex {
    HumanRegex(r"[[:^alpha:]]".to_string())
}

/// A function to match any lowercase character (`[a-z]`)
pub fn lowercase() -> HumanRegex {
    HumanRegex(r"[[:lower:]]".to_string())
}

/// A function to match any non-lowercase character (`[^a-z]`)
pub fn non_lowercase() -> HumanRegex {
    HumanRegex(r"[[:^lower:]]".to_string())
}

/// A function to match any uppercase character (`[A-Z]`)
pub fn uppercase() -> HumanRegex {
    HumanRegex(r"[[:upper:]]".to_string())
}

/// A function to match any non-uppercase character (`[^A-Z]`)
pub fn non_uppercase() -> HumanRegex {
    HumanRegex(r"[[:^upper:]]".to_string())
}

/// A function to match any digit that would appear in a hexadecimal number (`[A-Fa-f0-9]`)
pub fn hexdigit() -> HumanRegex {
    HumanRegex(r"[[:xdigit:]]".to_string())
}

/// A function to match any digit that wouldn't appear in a hexadecimal number (`[^A-Fa-f0-9]`)
pub fn non_hexdigit() -> HumanRegex {
    HumanRegex(r"[[:^xdigit:]]".to_string())
}

/// A function to match any ascii digit (`[\x00-\x7F]`)
pub fn ascii() -> HumanRegex {
    HumanRegex(r"[[:ascii:]]".to_string())
}

/// A function to match any non-ascii digit (`[^\x00-\x7F]`)
pub fn non_ascii() -> HumanRegex {
    HumanRegex(r"[[:^ascii:]]".to_string())
}

/// A function to match blank characters (`[\t ]`)
pub fn blank() -> HumanRegex {
    HumanRegex(r"[[:blank:]]".to_string())
}

/// A function to match non-blank characters (`[^\t ]`)
pub fn non_blank() -> HumanRegex {
    HumanRegex(r"[[:^blank:]]".to_string())
}

/// A function to match control characters (`[\x00-\x1F\x7F]`)
pub fn control() -> HumanRegex {
    HumanRegex(r"[[:cntrl:]]".to_string())
}

/// A function to match non-control characters (`[^\x00-\x1F\x7F]`)
pub fn non_control() -> HumanRegex {
    HumanRegex(r"[[:^cntrl:]]".to_string())
}

/// A function to match graphical characters (`[!-~]`)
pub fn graphical() -> HumanRegex {
    HumanRegex(r"[[:graph:]]".to_string())
}

/// A function to match non-graphical characters (`[^!-~]`)
pub fn non_graphical() -> HumanRegex {
    HumanRegex(r"[[:^graph:]]".to_string())
}

/// A function to match printable characters (`[ -~]`)
pub fn printable() -> HumanRegex {
    HumanRegex(r"[[:print:]]".to_string())
}

/// A function to match unprintable characters (`[^ -~]`)
pub fn non_printable() -> HumanRegex {
    HumanRegex(r"[[:^print:]]".to_string())
}

/// A function to match punctuation (`[!-/:-@\[-`{-~]`)
pub fn punctuation() -> HumanRegex {
    HumanRegex(r"[[:punct:]]".to_string())
}

/// A function to match non-punctuation (`[^!-/:-@\[-`{-~]`)
pub fn non_punctuation() -> HumanRegex {
    HumanRegex(r"[[:^punct:]]".to_string())
}
