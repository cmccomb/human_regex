| ⚠️ This package is under active development which will include breaking changes. ⚠️ |
| --------------------------------------------------------------------- |
# Regex for Humans
The goal of this crate is simple: give everybody the power of regular expressions without having 
to learn the complicated syntax. It is inspired by [ReadableRegex.jl](https://github.com/jkrumbiegel/ReadableRegex.jl).
This crate is a wrapper around the [core Rust regex library](https://crates.io/crates/regex). 

# Example usage
## Matching a date
If you want to match a date of the format `2021-10-30`, you would use the following code to generate a regex:
```rust
use human_regex::{begin, digit, end, exactly, text};

fn main() {
    let regex_string = begin()
        + exactly(4, digit())
        + text("-")
        + exactly(2, digit())
        + text("-")
        + exactly(2, digit())
        + end();
    println!("{}", regex_string.to_regex().is_match("2014-01-01"))
}
```

# Roadmap
The eventual goal of this crate is to support all of the syntax in the [core Rust regex library](https://crates.io/crates/regex) through a human-readable API. Here is where we currently stand:

## Character Classes
### Single Character

| Implemented?  | Expression | Description |
| :----------:  | :--------: | :---------- | 
| `any()`       |   `.`      | any character except new line (includes new line with s flag) |
| `digit()`     |   `\d`     | digit (\p{Nd}) |
| `non_digit()` |    `\D`    | not digit |
|               |`\pN`       | One-letter name Unicode character class |
|               |`\p{Greek}` | Unicode character class (general category or script) |
|               |`\PN`       | Negated one-letter name Unicode character class |
|               |`\P{Greek}` | negated Unicode character class (general category or script) |

### Perl Character Classes

| Implemented?       | Expression | Description |
| :---------------:  | :--------: | :---------- | 
| `digit()`          |   `\d`     | digit (\p{Nd}) |
| `non_digit()`      |   `\D`     | not digit |
| `whitespace()`     |   `\s`     | whitespace (\p{White_Space}) |
| `non_whitespace()` |   `\S`     | not whitespace |
| `word()`           |   `\w`     | word character (\p{Alphabetic} + \p{M} + \d + \p{Pc} + \p{Join_Control}) |
| `non_word()`       |   `\W`     | not word character |

### ASCII Character Classes

| Implemented?       | Expression | Description |
| :---------------:  | :------------: | :---------- |
|                    | `[[:alnum:]]`  | alphanumeric ([0-9A-Za-z]) |
|                    | `[[:alpha:]]`  | alphabetic ([A-Za-z]) |
|                    | `[[:ascii:]]`  | ASCII ([\x00-\x7F]) |
|                    | `[[:blank:]]`  | blank ([\t ]) |
|                    | `[[:cntrl:]]`  | control ([\x00-\x1F\x7F]) |
| `digit()`          | `[[:digit:]]`  | digits ([0-9]) |
|                    | `[[:graph:]]`  | graphical ([!-~]) |
|                    | `[[:lower:]]`  | lower case ([a-z]) |
|                    | `[[:print:]]`  | printable ([ -~]) |
|                    | `[[:punct:]]`  | punctuation ([!-/:-@\[-`{-~]) |
|                    | `[[:space:]]`  | whitespace ([\t\n\v\f\r ]) |
|                    | `[[:upper:]]`  | upper case ([A-Z]) |
|  `word()`          | `[[:word:]]`   | word characters ([0-9A-Za-z_]) |
|                    | `[[:xdigit:]]` | hex digit ([0-9A-Fa-f]) |

## Repetitions

| Implemented?             | Expression | Description |
| :----------------------: | :------------: | :---------- |
| `zero_or_more(x)`        |    `x*`        | zero or more of x (greedy) |
| `one_or_more(x)`         |    `x+`        | one or more of x (greedy) |
| `zero_or_one(x)`         |    `x?`        | zero or one of x (greedy) |
| `zero_or_more(x)`        |    `x*?`       | zero or more of x (ungreedy/lazy) |
| `one_or_more(x).lazy()`  |    `x+?`       | one or more of x (ungreedy/lazy) |
| `zero_or_more(x).lazy()` |    `x??`       | zero or one of x (ungreedy/lazy) |
| `at_least_at_most(n, m, x)` |    `x{n,m}`    | at least n x and at most m x (greedy) |
| `at_least(n, x)`         | `x{n,}`        | at least n x (greedy) |
| `exactly(n, x)`          | `x{n}`         | exactly n x |
| `at_least_at_most(n, m, x).lazy()`| `x{n,m}?`  | at least n x and at most m x (ungreedy/lazy) |
| `at_least(n, x).lazy()`  | `x{n,}?`   | at least n x (ungreedy/lazy) |

## Composites

| Implemented?       |   Expression   |      Description                |
| :---------------:  | :------------: | :------------------------------ |
|    `+`             |   `xy`         | concatenation (x followed by y) |
| `or()`             |   `x\|y`        | alternation (x or y, prefer x)  |

## Empty matches

| Implemented?       |   Expression   |      Description                |
| :---------------:  | :------------: | :------------------------------ |
| `begin()` | `^` |     the beginning of text (or start-of-line with multi-line mode) |
| `end()` | `$`  |   the end of text (or end-of-line with multi-line mode) |
| |`\A`  |  only the beginning of text (even with multi-line mode enabled) |
| | `\z` |   only the end of text (even with multi-line mode enabled) |
| |`\b`   | a Unicode word boundary (\w on one side and \W, \A, or \z on other) |
| | `\B`  |  not a Unicode word boundary |

## Groupings and Flags

| Implemented?       |   Expression   |      Description                |
| :---------------:  | :------------: | :------------------------------ |
| | `(exp)`         | numbered capture group (indexed by opening parenthesis) |
| | `(?P<name>exp)` | named (also numbered) capture group |
| | `(?:exp)`       | non-capturing group |
| | `(?flags)`      | set flags within current group |
| | `(?flags:exp)`  | set flags for exp (non-capturing) |

| Implemented?       |   Expression   |      Description                |
| :---------------:  | :------------: | :------------------------------ |
| | `i` |    case-insensitive: letters match both upper and lower case |
| | `m` |     multi-line mode: `^` and `$` match begin/end of line |
| | `s` |     allow `.` to match `\n` |
| | `U` |     swap the meaning of `x*` and `x*`? |
| | `u` |     Unicode support (enabled by default) |
| | `x` |     ignore whitespace and allow line comments (starting with `#`) |