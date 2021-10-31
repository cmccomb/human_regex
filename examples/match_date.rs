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
