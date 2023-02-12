use human_regex::{any, one_or_more, text};

fn main() {
    // Define a string to match against
    let matching_string = "<div><h1>Wow, so cool!</h1></div>";

    // Try it with a greedy match
    let greedy_regex_string = text("<") + one_or_more(any()) + text(">");
    for capture in greedy_regex_string
        .to_regex()
        .captures_iter(matching_string)
    {
        println!("Greedy: {}", &capture[0]);
    }

    // Try it with a lazy match
    let greedy_regex_string = text("<") + one_or_more(any()).lazy() + text(">");
    for capture in greedy_regex_string
        .to_regex()
        .captures_iter(matching_string)
    {
        println!("Lazy: {}", &capture[0]);
    }
}
