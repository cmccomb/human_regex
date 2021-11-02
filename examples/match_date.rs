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
