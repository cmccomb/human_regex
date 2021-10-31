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

| Implemented?       | Expression | Description |
| :---------------:  | :------------: | :---------- |
x*        zero or more of x (greedy)
x+        one or more of x (greedy)
x?        zero or one of x (greedy)
x*?       zero or more of x (ungreedy/lazy)
x+?       one or more of x (ungreedy/lazy)
x??       zero or one of x (ungreedy/lazy)
x{n,m}    at least n x and at most m x (greedy)
x{n,}     at least n x (greedy)
x{n}      exactly n x
x{n,m}?   at least n x and at most m x (ungreedy/lazy)
x{n,}?    at least n x (ungreedy/lazy)
x{n}?     exactly n x

## Composites

| Implemented?       |   Expression   |      Description                |
| :---------------:  | :------------: | :------------------------------ |
|    `+`             |   `xy`         | concatenation (x followed by y) |
| `or()`             |   `x\|y`        | alternation (x or y, prefer x)  |

