use human_regex::{
    digit, exactly, named_capture, nonescaped_text, one_or_more, text, whitespace, word,
    zero_or_more, zero_or_one,
};

fn main() {
    // Define a citation to play with
    let citations_in_mla = "\
    McComb, Christopher, Jonathan Cagan, and Kenneth Kotovsky. \"Lifting the Veil: Drawing insights \
    about design teams from a cognitively-inspired computational model.\" Design Studies 40 (2015): \
    119-142.\
    McComb, Christopher, Jonathan Cagan, and Kenneth Kotovsky. \"Rolling with the punches: An \
    examination of team performance in a design task subject to drastic changes.\" Design Studies \
    36 (2015): 99-121.\
    Raina, Ayush, Christopher McComb, and Jonathan Cagan. \"Learning to design from humans: \
    Imitating human designers through deep learning.\" Journal of Mechanical Design 141 (2019).\
    ";

    // The authors are a bit challenging
    let first_author = exactly(1, one_or_more(word()) + text(", ") + one_or_more(word())).lazy();
    let middle_authors =
        zero_or_more(text(", ") + one_or_more(word()) + text(" ") + one_or_more(word()));
    let last_author =
        zero_or_more(text(", and ") + one_or_more(word()) + text(" ") + one_or_more(word())).lazy();

    let mla_authors = named_capture(
        first_author + middle_authors + last_author + text(". "),
        "authors",
    );

    // The rest is easy
    let mla_title =
        text("\"") + named_capture(one_or_more(nonescaped_text("[^\"]")), "title") + text(".\" ");
    let mla_journal = named_capture(one_or_more(one_or_more(word()) + text(" ")), "journal");
    let mla_volume = zero_or_one(named_capture(one_or_more(digit()), "volume"));
    let mla_year = zero_or_one(text(" (") + named_capture(exactly(4, digit()), "year") + text(")"));
    let mla_pp = zero_or_one(
        text(": ")
            + named_capture(one_or_more(digit()), "lower_page")
            + text("-")
            + named_capture(one_or_more(digit()), "upper_page"),
    );

    // Combine independent pieces
    let mla_regex = one_or_more(
        mla_authors
            + mla_title
            + mla_journal
            + mla_volume
            + mla_year
            + mla_pp
            + text(".")
            + zero_or_more(whitespace()),
    )
    .lazy();

    // Return matches
    for capture in mla_regex.to_regex().captures_iter(citations_in_mla) {
        println!("Full citation: {}", &capture[0]);
        println!("\t- authors: {}", &capture[1]);
        println!("\t- title: {}", &capture[2]);
        println!("\t- journal: {}", &capture[3]);
        println!(
            "\t- volume: {}",
            &capture.get(4).map_or("N/A", |x| { x.as_str() }),
        );
        println!(
            "\t- year: {}",
            &capture.get(5).map_or("N/A", |x| { x.as_str() }),
        );
        println!(
            "\t- pages: from {} to {}",
            &capture.get(6).map_or("N/A", |x| { x.as_str() }),
            &capture.get(7).map_or("N/A", |x| { x.as_str() }),
        );
    }
}
