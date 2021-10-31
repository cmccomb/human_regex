use super::humanregex::{fmt, HumanRegex};

/// Match at least _n_ of a certain target
/// ```
/// let regex_string = human_regex::at_least(3, "a");
/// assert!(regex_string.to_regex().is_match("aaaa"));
/// assert!(!regex_string.to_regex().is_match("aa"));
/// ```
pub fn at_least<T>(n: u8, target: T) -> HumanRegex
where
    T: Into<String> + fmt::Display,
{
    HumanRegex(format!("(:?{}){{{},}}", target, n))
}

/// Match at least _n_ and at most _m_ of a certain target
/// ```
/// let regex_string = human_regex::between(3, 5, "a");
/// assert!(regex_string.to_regex().is_match("aaaa"));
/// assert!(!regex_string.to_regex().is_match("aa"));
/// ```
pub fn between<T>(n: u8, m: u8, target: T) -> HumanRegex
where
    T: Into<String> + fmt::Display,
{
    HumanRegex(format!("(:?{}){{{},{}}}", target, n, m))
}

/// Match one or more of a certain target
/// ```
/// let regex_string = human_regex::one_or_more("a");
/// assert!(regex_string.to_regex().is_match("aaaa"));
/// assert!(!regex_string.to_regex().is_match("bb"));
/// ```
pub fn one_or_more<T>(target: T) -> HumanRegex
where
    T: Into<String> + fmt::Display,
{
    HumanRegex(format!("(:?{})+", target))
}

/// Match zero or more of a certain target
/// ```
/// let regex_string = human_regex::zero_or_more("a");
/// assert!(regex_string.to_regex().is_match("a"));
/// assert!(regex_string.to_regex().is_match("bb"));
/// ```
pub fn zero_or_more<T>(target: T) -> HumanRegex
where
    T: Into<String> + fmt::Display,
{
    HumanRegex(format!("(:?{})*", target))
}

/// Match zero or one of a certain target
/// ```
/// let regex_string = human_regex::zero_or_one("a");
/// assert!(regex_string.to_regex().is_match("a"));
/// assert!(regex_string.to_regex().is_match("bb"));
/// ```
pub fn zero_or_one<T>(target: T) -> HumanRegex
where
    T: Into<String> + fmt::Display,
{
    HumanRegex(format!("(:?{})?", target))
}

/// Match zero or one of a certain target
/// ```
/// let regex_string = human_regex::optional("a");
/// assert!(regex_string.to_regex().is_match("a"));
/// assert!(regex_string.to_regex().is_match("bb"));
/// ```
pub fn optional<T>(target: T) -> HumanRegex
where
    T: Into<String> + fmt::Display,
{
    HumanRegex(format!("(:?{})?", target))
}

/// Match exactly _n_ of a certain target
/// ```
/// let regex_string = human_regex::exactly(5, "a");
/// assert!(regex_string.to_regex().is_match("aaaaa"));
/// assert!(!regex_string.to_regex().is_match("aaa"));
/// ```
pub fn exactly<T>(n: u8, target: T) -> HumanRegex
where
    T: Into<String> + fmt::Display,
{
    HumanRegex(format!("(:?{}){{{}}}", target, n))
}
