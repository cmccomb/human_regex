use human_regex::{digit, exactly, named_capture, one_or_more, printable, text};

fn main() {
    // Define a citation to play with
    let citation_in_mla = "McComb, Christopher, Jonathan Cagan, and Kenneth Kotovsky. \
    \"Lifting the Veil: Drawing insights about design teams from a cognitively-inspired \
    computational model.\" Design Studies 40 (2015): 119-142.";

    // Define matches for individual components
    let mla_authors = named_capture(one_or_more(printable()), "authors") + text(". ");
    let mla_title = text("\"") + named_capture(one_or_more(printable()), "title") + text(".\" ");
    let mla_journal = named_capture(one_or_more(printable()), "journal") + text(" ");
    let mla_volume = named_capture(one_or_more(digit()), "volume") + text(" ");
    let mla_year = text("(") + named_capture(exactly(4, digit()), "year") + text("): ");
    let mla_pp = named_capture(one_or_more(digit()), "lower_page")
        + text("-")
        + named_capture(one_or_more(digit()), "upper_page")
        + text(".");

    // Combine independent pieces
    let mla_regex = mla_authors + mla_title + mla_journal + mla_volume + mla_year + mla_pp;

    // Return matches
    for capture in mla_regex.to_regex().captures_iter(citation_in_mla) {
        println!("Full citation: {}", &capture[0]);
        println!("\t- authors: {}", &capture[1]);
        println!("\t- title: {}", &capture[2]);
        println!("\t- journal: {}", &capture[3]);
        println!("\t- volume: {}", &capture[4]);
        println!("\t- year: {}", &capture[5]);
        println!("\t- pages: from {} to {}", &capture[6], &capture[7]);
    }
}
