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

| Implemented? | Expression | Description |
| :----------: | :--------: | :---------- | 
| [x] `any()`  |   `.`      | any character except new line (includes new line with s flag)
| [x] `digit()`  |   `\d`     |      digit (\p{Nd}) |
| [x] `non_digit()` |    `\D`    |         not digit
|    [ ]       |`\pN`       |   One-letter name Unicode character class
|    [ ]       |`\p{Greek}` |   Unicode character class (general category or script)
|    [ ]       |`\PN`       |   Negated one-letter name Unicode character class
|    [ ]       |`\P{Greek}` |   negated Unicode character class (general category or script)