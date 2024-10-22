use human_regex::{beginning, digit, end, exactly, text};

fn main() {
    // Build the first match pattern
    let regex_string_1 = beginning()
        + exactly(4, digit())
        + text("-")
        + exactly(2, digit())
        + text("-")
        + exactly(2, digit())
        + end();

    // Build the second match pattern
    #[rustfmt::skip]
    let regex_string_2 = beginning() 
        + exactly(4, digit()) 
        + exactly(
            2, 
            text("-") + exactly(2, digit())
        ) 
        + end();

    // Check the match
    println!("{}", regex_string_1.to_regex().is_match("2014-01-01"));

    // Check the match
    println!("{}", regex_string_2.to_regex().is_match("2014-01-01"));
}
