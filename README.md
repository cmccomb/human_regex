| ⚠️ This package is under active development which will include breaking changes. ⚠️ |
| --------------------------------------------------------------------- |
# Regex for Humans
## About
The goal of this crate is simple: give everybody the power of regular expressions without having 
to learn the complicated syntax. It is inspired by [ReadableRegex.jl](https://github.com/jkrumbiegel/ReadableRegex.jl).

## Example usage
### Matching a date
If you want to match a date of the format `2021-10-30`, you would use the following code to generate a regex:
```rust
fn main() {
    use human_regex as hr;
    let regex_string = hr::begin()
        + hr::exactly(4, hr::digit())
        + hr::text("-")
        + hr::exactly(2, hr::digit())
        + hr::text("-")
        + hr::exactly(2, hr::digit())
        + hr::end();
    assert!(regex_string.to_regex().is_match("2021-10-31"))
}
```
