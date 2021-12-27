[![Github CI](https://github.com/cmccomb/human_regex/actions/workflows/tests.yml/badge.svg)](https://github.com/cmccomb/human_regex/actions)
[![Crates.io](https://img.shields.io/crates/v/human_regex.svg)](https://crates.io/crates/human_regex)
[![docs.rs](https://img.shields.io/docsrs/human_regex/latest?logo=rust)](https://docs.rs/human_regex)

# Regex for Humans
The goal of this crate is simple: give everybody the power of regular expressions without having 
to learn the complicated syntax. It is inspired by [ReadableRegex.jl](https://github.com/jkrumbiegel/ReadableRegex.jl).
This crate is a wrapper around the [core Rust regex library](https://crates.io/crates/regex). 

# Example usage
If you want to match a date of the format `2021-10-30`, you would use the following code to generate a regex:
```rust
use human_regex::{beginning, digit, end, exactly, text};

fn main() {
    let regex_string = beginning()
        + exactly(4, digit())
        + text("-")
        + exactly(2, digit())
        + text("-")
        + exactly(2, digit())
        + end();
    println!("{}", regex_string.to_regex().is_match("2014-01-01"))
}
```
The `to_regex()` method returns a [standard Rust regex](https://docs.rs/regex/1.5.4/regex/struct.Regex.html).

# Roadmap
The eventual goal of this crate is to support all the syntax in the [core Rust regex library](https://crates.io/crates/regex) through a human-readable API. Here is where we currently stand:

## Single Character (3 of 7)

| Implemented?  | Expression  | Description                                                    |
|:-------------:|:-----------:|:---------------------------------------------------------------| 
|    `any()`    |     `.`     | any character except new line (includes new line with s flag)  |
|   `digit()`   |    `\d`     | digit (\p{Nd})                                                 |
| `non_digit()` |    `\D`     | not digit                                                      |
|               |    `\pN`    | One-letter name Unicode character class                        |
|               | `\p{Greek}` | Unicode character class (general category or script)           |
|               |    `\PN`    | Negated one-letter name Unicode character class                |
|               | `\P{Greek}` | negated Unicode character class (general category or script)   |

## Character Classes (4 of 11)

|      Implemented?      |   Expression   | Description                                                               |
|:----------------------:|:--------------:|:--------------------------------------------------------------------------|
| `or(&['x', 'y', 'z'])` | `[xyz]`        | A character class matching either x, y or z (union).                      |
|                        |    `[^xyz]`    | A character class matching any character except x, y and z.               |
|                        |    `[a-z]`     | A character class matching any character in range a-z.                    |
|       See below        | `[[:alpha:]]`  | ASCII character class ([A-Za-z])                                          |                
|                        | `[[:^alpha:]]` | Negated ASCII character class ([^A-Za-z])                                 |               
|         `or()`         |  `[x[^xyz]]`   | Nested/grouping character class (matching any character except y and z)   |
|                        |  `[a-y&&xyz]`  | Intersection (matching x or y)                                            |             
|                        | `[0-9&&[^4]]`  | Subtraction using intersection and negation (matching 0-9 except 4)       |    
|                        |   `[0-9--4]`   | Direct subtraction (matching 0-9 except 4)                                |             
|                        |  `[a-g~~b-h]`  | Symmetric difference (matching `a` and `h` only)                          |          
|                        |    `[\[\]]`    | Escaping in character classes (matching [ or ])                           |         

## Perl Character Classes

| Implemented?       | Expression | Description                                                              |
| :---------------:  | :--------: |:-------------------------------------------------------------------------| 
| `digit()`          |   `\d`     | digit (\p{Nd})                                                           |
| `non_digit()`      |   `\D`     | not digit                                                                |
| `whitespace()`     |   `\s`     | whitespace (\p{White_Space})                                             |
| `non_whitespace()` |   `\S`     | not whitespace                                                           |
| `word()`           |   `\w`     | word character (\p{Alphabetic} + \p{M} + \d + \p{Pc} + \p{Join_Control}) |
| `non_word()`       |   `\W`     | not word character                                                       |

## ASCII Character Classes

|   Implemented?    |   Expression   | Description                     |
|:-----------------:|:--------------:|:--------------------------------|
| `alphanumeric()`  | `[[:alnum:]]`  | alphanumeric ([0-9A-Za-z])      |
|  `alphabetic()`   | `[[:alpha:]]`  | alphabetic ([A-Za-z])           |
|     `ascii()`     | `[[:ascii:]]`  | ASCII ([\x00-\x7F])             |
|     `blank()`     | `[[:blank:]]`  | blank ([\t ])                   |
|    `control()`    | `[[:cntrl:]]`  | control ([\x00-\x1F\x7F])       |
|     `digit()`     | `[[:digit:]]`  | digits ([0-9])                  |
|   `graphical()`   | `[[:graph:]]`  | graphical ([!-~])               |
|   `uppercase()`   | `[[:lower:]]`  | lower case ([a-z])              |
|   `printable()`   | `[[:print:]]`  | printable ([ -~])               |
|  `punctuation()`  | `[[:punct:]]`  | punctuation ([!-/:-@\[-`{-~])   |
|  `whitespace()`   | `[[:space:]]`  | whitespace ([\t\n\v\f\r ])      |
|   `lowercase()`   | `[[:upper:]]`  | upper case ([A-Z])              |
|     `word()`      |  `[[:word:]]`  | word characters ([0-9A-Za-z_])  |
|   `hexdigit()`    | `[[:xdigit:]]` | hex digit ([0-9A-Fa-f])         |

## Repetitions

|       Implemented?        | Expression | Description                                  |
|:-------------------------:|:----------:|:---------------------------------------------|
|     `zero_or_more(x)`     |    `x*`    | zero or more of x (greedy)                   |
|     `one_or_more(x)`      |    `x+`    | one or more of x (greedy)                    |
|     `zero_or_one(x)`      |    `x?`    | zero or one of x (greedy)                    |
|     `zero_or_more(x)`     |   `x*?`    | zero or more of x (ungreedy/lazy)            |
|  `one_or_more(x).lazy()`  |   `x+?`    | one or more of x (ungreedy/lazy)             |
| `zero_or_more(x).lazy()`  |   `x??`    | zero or one of x (ungreedy/lazy)             |
|    `between(n, m, x)`     |  `x{n,m}`  | at least n x and at most m x (greedy)        |
|     `at_least(n, x)`      |  `x{n,}`   | at least n x (greedy)                        |
|      `exactly(n, x)`      |   `x{n}`   | exactly n x                                  |
| `between(n, m, x).lazy()` | `x{n,m}?`  | at least n x and at most m x (ungreedy/lazy) |
|  `at_least(n, x).lazy()`  |  `x{n,}?`  | at least n x (ungreedy/lazy)                 |

## Composites

| Implemented? | Expression | Description                     |
|:------------:|:----------:|:--------------------------------|
|      `+`     |  `xy`      | concatenation (x followed by y) |
|    `or()`    |    `x\| y`                              | alternation (x or y, prefer x)  |

## Empty matches

|     Implemented?      | Expression | Description                                                         |
|:---------------------:|:----------:|:--------------------------------------------------------------------|
|     `beginning()`     |    `^`     | the beginning of text (or start-of-line with multi-line mode)       |
|        `end()`        |    `$`     | the end of text (or end-of-line with multi-line mode)               |
| `beginning_of_text()` |    `\A`    | only the beginning of text (even with multi-line mode enabled)      |
|    `end_of_text()`    |    `\z`    | only the end of text (even with multi-line mode enabled)            |
|   `word_boundary()`   |    `\b`    | a Unicode word boundary (\w on one side and \W, \A, or \z on other) |
| `non_word_boundary()` |    `\B`    | not a Unicode word boundary                                         |

## Groupings (1 of 5)

|                   Implemented?                    |   Expression    | Description                                             |
|:-------------------------------------------------:|:---------------:|:--------------------------------------------------------|
|                  `capture(exp)`                   |     `(exp)`     | numbered capture group (indexed by opening parenthesis) |
|            `named_capture(exp, name)`             | `(?P<name>exp)` | named (also numbered) capture group                     |
| Handled implicitly through functional composition |    `(?:exp)`    | non-capturing group                                     |
|                                                   |   `(?flags)`    | set flags within current group                          |
|                     See below                     | `(?flags:exp)`  | set flags for exp (non-capturing)                       |
   
## Flags (0 of 6)
    
|      Implemented?       | Expression | Description                                                   |
|:-----------------------:|:----------:|:--------------------------------------------------------------|
| `case_insensitive(exp)` |    `i`     | case-insensitive: letters match both upper and lower case     |
|                         |    `m`     | multi-line mode: `^` and `$` match begin/end of line          |
|                         |    `s`     | allow `.` to match `\n`                                       |
|                         |    `U`     | swap the meaning of `x*` and `x*`?                            |
|                         |    `u`     | Unicode support (enabled by default)                          |
|                         |    `x`     | ignore whitespace and allow line comments (starting with `#`) |
