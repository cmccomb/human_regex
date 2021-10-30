| ⚠️ This package is under active development and doesn't do much yet. |
| -------------------------------------------------------------------------- |
# Regex for Humans
## About
The goal of this crate is simple: give everybody the power of regular expressions without having 
to learn the complicated syntax. It is inspired by [ReadableRegex.jl](https://github.com/jkrumbiegel/ReadableRegex.jl).

## Example usage
### Matching a date
If you want to match a date of the format `2021-10-30`, you would use the following code to generate a regex:
```rust
let hr = human_regex::HumanRegex::new()
    .begin()
    .exactly(4, human_regex::DIGIT)
    .text("-")
    .exactly(2, human_regex::DIGIT)
    .text("-")
    .exactly(2, human_regex::DIGIT)
    .end();
assert!(hr.is_match("2014-01-01"));
```
Specifically, this chunk of code would yield the regex `^\d{4}-\d{2}-\d{2}$`, which is exactly what we want!
