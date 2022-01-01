//! Functions for capturing matches

use super::humanregex::HumanRegex;

/// Add a numbered capturing group around an expression
/// ```
/// use human_regex::{capture, digit, exactly, text};
/// let regex_string = capture(exactly(4, digit()))
///     + text("-")
///     + capture(exactly(2, digit()))
///     + text("-")
///     + capture(exactly(2, digit()));
///
/// let caps = regex_string.to_regex().captures("2010-03-14").unwrap();
///
/// assert_eq!("2010", caps.get(1).unwrap().as_str());
/// assert_eq!("03", caps.get(2).unwrap().as_str());
/// assert_eq!("14", caps.get(3).unwrap().as_str());
/// ```

pub fn capture(target: HumanRegex) -> HumanRegex {
    HumanRegex(format!("({})", target))
}

/// Add a named capturing group around an expression
/// ```
/// use human_regex::{named_capture, digit, exactly, text};
/// let regex_string = named_capture(exactly(4, digit()), "year")
///     + text("-")
///     + named_capture(exactly(2, digit()), "month")
///     + text("-")
///     + named_capture(exactly(2, digit()), "day");
///
/// let caps = regex_string.to_regex().captures("2010-03-14").unwrap();
/// assert_eq!("2010", &caps["year"]);
/// assert_eq!("03", &caps["month"]);
/// assert_eq!("14", &caps["day"]);
/// ```
pub fn named_capture(target: HumanRegex, name: &str) -> HumanRegex {
    HumanRegex(format!("(?P<{}>{})", name, target))
}
