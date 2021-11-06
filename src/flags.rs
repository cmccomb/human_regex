//! Functions for adding flags
// i     case-insensitive: letters match both upper and lower case
// m     multi-line mode: ^ and $ match begin/end of line
// s     allow . to match \n
// U     swap the meaning of x* and x*?
// u     Unicode support (enabled by default)
// x     ignore whitespace and allow line comments (starting with `#`)

use super::humanregex::HumanRegex;

pub fn case_insensitive(target: HumanRegex) -> HumanRegex {
    HumanRegex(format!("(?i:{})", target))
}

pub fn multi_line_mode(target: HumanRegex) -> HumanRegex {
    HumanRegex(format!("(?m:{})", target))
}

pub fn dot_matches_newline_too(target: HumanRegex) -> HumanRegex {
    HumanRegex(format!("(?s:{})", target))
}

pub fn disable_unicode(target: HumanRegex) -> HumanRegex {
    HumanRegex(format!("(?-u:{})", target))
}

pub fn ignore_whitespace_and_allow_comments(target: HumanRegex) -> HumanRegex {
    HumanRegex(format!("(?x:{})", target))
}
