[![Github CI](https://github.com/cmccomb/human_regex/actions/workflows/tests.yml/badge.svg)](https://github.com/cmccomb/human_regex/actions)
[![Crates.io](https://img.shields.io/crates/v/human_regex.svg)](https://crates.io/crates/human_regex)
[![docs.rs](https://img.shields.io/docsrs/human_regex/latest?logo=rust)](https://docs.rs/human_regex)

# Regex for Humans
The goal of this crate is simple: give everybody the power of regular expressions without having 
to learn the complicated syntax. It is inspired by [ReadableRegex.jl](https://github.com/jkrumbiegel/ReadableRegex.jl).
This crate is a wrapper around the [core Rust regex library](https://crates.io/crates/regex). 

# Example usage
If you want to match a date of the format `2021-10-30`, you could use the following code to generate a regex:
```rust
use human_regex::{beginning, digit, exactly, text, end};
let regex_string = beginning()
    + exactly(4, digit())
    + text("-")
    + exactly(2, digit())
    + text("-")
    + exactly(2, digit())
    + end();
assert!(regex_string.to_regex().is_match("2014-01-01"));
```
The `to_regex()` method returns a [standard Rust regex](https://docs.rs/regex/1.5.4/regex/struct.Regex.html). We can do this another way with slightly less repetition though!
```rust
use human_regex::{beginning, digit, exactly, text, end};
let first_regex_string = text("-") + exactly(2, digit());
let second_regex_string = beginning()
    + exactly(4, digit())
    + exactly(2, first_regex_string)
    + end();
assert!(second_regex_string.to_regex().is_match("2014-01-01"));
```
For a more extensive set of examples, please see [The Cookbook](crate::cookbook).

# Features
This crate currently supports the vast majority of syntax available in the [core Rust regex library](https://crates.io/crates/regex) through a human-readable API.   
The type model that the API is built upon reflects the underlying syntax rules of regular languages/expressions, meaning you get the same instant compiler feedback you're use to in Rust while writing regex.
No more runtime regex panics or unexplained behavior, if it compiles, what can plainly read is what you get.
## Single Character

| Implementation                                         | Expression          | Description                                                   |
|:------------------------------------------------------:|:-------------------:|:--------------------------------------------------------------|
| [`any()`]                                              |         `.`         | any character except new line (includes new line with s flag) |
| [`digit()`]                                            |        `\d`         | digit (`\p{Nd}`)                                              |
| [`non_digit()`]                                        |        `\D`         | not digit                                                     |
| [`unicode_category(UnicodeCategory)`](unicode_category)|        `\p{L}`      | Unicode non-script category                                   |
| [`unicode_script(UnicodeScript)`](unicode_script)      |     `\p{Greek}`     | Unicode script category                                       |

## Character Classes

|      Implementation                         |   Expression   | Description                                                                         |
|:-------------------------------------------:|:--------------:|:------------------------------------------------------------------------------------|
|[`within_set(&['x', 'y', 'z'])`](within_set) |    `[xyz]`     | A character class matching either x, y or z.                                        |
|[`wthout_set(&['x', 'y', 'z'])`](without_set)|    `[^xyz]`    | A character class matching any character except x, y and z.                         |
|[`within_range('a'..='z')`](within_range)    |    `[a-z]`     | A character class matching any character in range a-z.                              |
|[`without_range('a'..='z')`](without_range)  |    `[^a-z]`    | A character class matching any character outside range a-z.                         |
|[`within_set()`](within_set)                 |  `[x[^xyz]]`   | Nested/grouping character class (matching any character except y and z)             |
|[`and(lhs, rhs)`](and)/`lhs & rhs`           |  `[a-y&&xyz]`  | Intersection (a-y AND xyz = xy)                                                     |             
|`within_range()&without_set()`               | `[0-9&&[^4]]`  | Subtraction using intersection and negation (matching 0-9 except 4)                 |    
|[`subtract(lhs, rhs)`](subtract)             |   `[0-9--4]`   | Direct subtraction (matching 0-9 except 4).                                         |             
|[`xor(lhs, rhs)`](xor)                       |  `[a-g~~b-h]`  | Symmetric difference (matching `a` and `h` only).                                   |          
|`within_set(&escape_all())`                  |    `[\[\]]`    | Escaping in character classes (matching `[` or `]`)                                 |         

## Perl Character Classes

|    Implementation  | Expression | Description                                                                |
|:------------------:| :--------: |:---------------------------------------------------------------------------|
|[`digit()`]         |   `\d`     | digit (`\p{Nd}`)                                                           |
|[`non_digit()`]     |   `\D`     | not digit                                                                  |
|[`whitespace()`]    |   `\s`     | whitespace (`\p{White_Space}`)                                             |
|[`non_whitespace()`]|   `\S`     | not whitespace                                                             |
|[`word()`]          |   `\w`     | word character (`\p{Alphabetic} + \p{M} + \d + \p{Pc} + \p{Join_Control}`) |
|[`non_word()`]      |   `\W`     | not word character                                                         |

## ASCII Character Classes

|   Implementation |   Expression   | Description                       |
|:----------------:|:--------------:|:----------------------------------|
|[`alphanumeric()`]| `[[:alnum:]]`  | alphanumeric (`[0-9A-Za-z]`)      |
|[`alphabetic()`]  | `[[:alpha:]]`  | alphabetic (`[A-Za-z]`)           |
|[`ascii()`]       | `[[:ascii:]]`  | ASCII (`[\x00-\x7F]`)             |
|[`blank()`]       | `[[:blank:]]`  | blank (`[\t ]`)                   |
|[`control()`]     | `[[:cntrl:]]`  | control (`[\x00-\x1F\x7F]`)       |
|[`digit()`]       | `[[:digit:]]`  | digits (`[0-9]`)                  |
|[`graphical()`]   | `[[:graph:]]`  | graphical (`[!-~]`)               |
|[`uppercase()`]   | `[[:lower:]]`  | lower case (`[a-z]`)              |
|[`printable()`]   | `[[:print:]]`  | printable (`[ -~]`)               |
|[punctuation()`]  | `[[:punct:]]`  | punctuation (``[!-/:-@\[-`{-~]``) |
|[`whitespace()`]  | `[[:space:]]`  | whitespace (`[\t\n\v\f\r ]`)      |
|[`lowercase()`]   | `[[:upper:]]`  | upper case (`[A-Z]`)              |
|[`word()`]        |  `[[:word:]]`  | word characters (`[0-9A-Za-z_]`)  |
|[`hexdigit()`]    | `[[:xdigit:]]` | hex digit (`[0-9A-Fa-f]`)         |

## Repetitions

|       Implementation                        | Expression | Description                                  |
|:-------------------------------------------:|:----------:|:---------------------------------------------|
|[`zero_or_more(x)`](zero_or_more)            |    `x*`    | zero or more of x (greedy)                   |
|[`one_or_more(x)`](one_or_more)              |    `x+`    | one or more of x (greedy)                    |
|[`zero_or_one(x)`](zero_or_one)              |    `x?`    | zero or one of x (greedy)                    |
|[`zero_or_more(x)`](zero_or_more)            |   `x*?`    | zero or more of x (ungreedy/lazy)            |
|[`one_or_more(x).lazy()`](HumanRegex::lazy)  |   `x+?`    | one or more of x (ungreedy/lazy)             |
|[`zero_or_more(x).lazy()`](HumanRegex::lazy) |   `x??`    | zero or one of x (ungreedy/lazy)             |
|[`between(n, m, x)`](between)                |  `x{n,m}`  | at least n x and at most m x (greedy)        |
|[`at_least(n, x)`](at_least)                 |  `x{n,}`   | at least n x (greedy)                        |
|[`exactly(n, x)`](exactly)                   |   `x{n}`   | exactly n x                                  |
|[`between(n, m, x).lazy()`](HumanRegex::lazy)| `x{n,m}?`  | at least n x and at most m x (ungreedy/lazy) |
|[`at_least(n, x).lazy()`](HumanRegex::lazy)  |  `x{n,}?`  | at least n x (ungreedy/lazy)                 |

## General Operations

|Implementation        | Expression                   | Description                                                         |
|:--------------------:|:----------------------------:|:--------------------------------------------------------------------|
|[`+`](std::ops::Add)  |  `xy`                        | concatenation (x followed by y)                                     |
|[`⎮`](std::ops::BitOr)|    `x⎮y`                    | alternation (x or y, prefer x)                                      |
|[`!`](std::ops::Not)  |`\d->\D`, `[xy]->[^xy]`, etc. | negation (works on any character class, or literal strings of text).|

## Empty matches

|     Implementation    | Expression | Description                                                         |
|:---------------------:|:----------:|:--------------------------------------------------------------------|
|[`beginning()`]        |    `^`     | the beginning of text (or start-of-line with multi-line mode)       |
|[`end()`]              |    `$`     | the end of text (or end-of-line with multi-line mode)               |
|[`beginning_of_text()`]|    `\A`    | only the beginning of text (even with multi-line mode enabled)      |
|[`end_of_text()`]      |    `\z`    | only the end of text (even with multi-line mode enabled)            |
|[`word_boundary()`]    |    `\b`    | a Unicode word boundary (\w on one side and \W, \A, or \z on other) |
|[`non_word_boundary()`]|    `\B`    | not a Unicode word boundary                                         |

## Groupings 

|                   Implementation                  |   Expression    | Description                                             |
|:-------------------------------------------------:|:---------------:|:--------------------------------------------------------|
|[`capture(exp)`](capture)                          |     `(exp)`     | numbered capture group (indexed by opening parenthesis) |
|[`named_capture(exp, name)`](named_capture)        | `(?P<name>exp)` | named (also numbered) capture group                     |
| Handled implicitly through functional composition |    `(?:exp)`    | non-capturing group                                     |
|                     See below                     |   `(?flags)`    | set flags within current group                          |
|                     See below                     | `(?flags:exp)`  | set flags for exp (non-capturing)                       |
   
## Flags 
    
|            Implementation                               | Expression | Description                                                   |
|:-------------------------------------------------------:|:----------:|:--------------------------------------------------------------|
|[`case_insensitive(exp)`](case_insensitive)              |    `i`     | case-insensitive: letters match both upper and lower case     |
|[`multi_line_mode(exp)`](multi_line_mode)                |    `m`     | multi-line mode: `^` and `$` match begin/end of line          |
|[`dot_matches_newline_too(exp)`](dot_matches_newline_too)|    `s`     | allow `.` to match `\n`                                       |
| will not be implemented<sup>1</sup>                     |    `U`     | swap the meaning of `x*` and `x*?`                            |
|[`disable_unicode(exp)`](disable_unicode)                |    `u`     | Unicode support (enabled by default)                          |
| will not be implemented<sup>2</sup>                     |    `x`     | ignore whitespace and allow line comments (starting with `#`) |

1. With the declarative nature of this library, use of this flag would just obfuscate meaning.
2. When using `human_regex`, comments should be added in source code rather than in the regex string.


# The Type System

Other than the declarative expression building, the thing that sets apart HumanRegex from any other Regex library is the
type-checked design. As mentioned above, the type system models the valid states of regular expressions, causing unvalid
expressions to be [nearly](nonescaped_text) impossible to create. This gives you extremely strong guarantees against the
sort of runtime faliures that plague regular expressions in every other language, usually making them an undesirable and
difficult to maintain design choice. With a fully type-modeled API, you can get significantly more reliability than other
regex implementations while retaining all the speed of its finite automata system.  

The system is built upon a thin hierarchy of states using the Typestate pattern. The most general state is the `SymbolChain`, 
which signifies an arbitrary expression with no other guarantees than that it is valid regex. Any operation that removes
guarantees about the actual expression contained within the [`HumanRegex`] struct usually promotes it to this state.  

There are also a state `SymbolClass<T>` signifying that the expression is a matcher for a class of symbols. The three substates
of this are `Standard`, `Custom`, and `ASCII`. Standard and ASCII represent built-in symbol class types. When these built-in
class types get mixed together, such as through use of any of the [`set operations`](logical), they get promoted to the `Custom`
subtype.  

Lastly, there is `LiteralSymbolChain`, which signifies an expression that is a literal grouping of text to be matched.  

The important thing is that the type system allows for making only valid assumptions about the raw expression wrapped by the
[`HumanRegex`] struct. If it is known that the value underlying the wrapper is a standard character class, then it can be
guaranteed at compile time that a string operation that replaces `"\w"` for `"\W"` cannot possibly fail or create invalid regex at
runtime, or that the string operation to add a lazy flag to an expression is guaranteed to find exactly what it is looking for.
