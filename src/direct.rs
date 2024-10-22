//! Functions for directly matching text or adding known regex strings

use super::humanregex::*;
use regex::escape;
use std::marker::PhantomData as pd;

/// Add matching text to the regex string. Text that is added through this function is automatically escaped.
/// ```
/// let regex_string = human_regex::text("asdf");
/// assert!(regex_string.to_regex().is_match("asdf"));
/// assert!(!regex_string.to_regex().is_match("asddf"));
/// ```
pub fn text<T>(text: T) -> HumanRegex<LiteralSymbolChain>
where
    T: Into<String> + fmt::Display,
{
    HumanRegex(escape(&text.to_string()), pd::<LiteralSymbolChain>)
}

/// Escapes an entire collection of text for use in something like an [or](super::logical::or)
/// or an [and](super::logical::and) expression.
///
/// See the [cookbook's](super::cookbook) stop words example for an example of the utility of this function.
/// ```
/// use human_regex::direct::escape_all;
/// let escaped_vec = escape_all(&vec!["et-al", "either|or", "[full-scale]"]);
/// assert_eq!(escaped_vec[0].to_string(), r"et\-al");
/// assert_eq!(escaped_vec[1].to_string(), r"either\|or");
/// assert_eq!(escaped_vec[2].to_string(), r"\[full\-scale\]");
///```
pub fn escape_all<T>(options: &[T]) -> Vec<HumanRegex<LiteralSymbolChain>>
where
    T: Into<String> + fmt::Display,
{
    options
        .iter()
        .map(|string| HumanRegex(escape(&string.to_string()), pd::<LiteralSymbolChain>))
        .collect()
}

/// This text is not escaped. You can use it, for instance, to add a regex string directly to the object.
/// While this is not recommended, as it violates the compile-time guarantees of the type system, sometimes
/// an escape hatch is necessary. Use with caution.
/// ```
/// let regex_string = human_regex::nonescaped_text(r"^\d{2}$");
/// println!("{}", regex_string.to_string());
/// assert!(regex_string.to_regex().is_match("21"));
/// assert!(!regex_string.to_regex().is_match("007"));
/// ```
pub fn nonescaped_text(text: &str) -> HumanRegex<SymbolChain> {
    HumanRegex(format!("(?:{})", text.to_string()), pd::<SymbolChain>)
}
