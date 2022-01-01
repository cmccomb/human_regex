use human_regex::{exactly, one_or_more, or, punctuation, whitespace, word_boundary};
use stop_words::{get, LANGUAGE};

fn main() {
    // Read in a file
    let document = std::fs::read_to_string("examples/foreword.txt").expect("Cannot read file");

    // Print the contents
    println!("Original text:\n{}", document);

    // Get the stopwords
    let words = get(LANGUAGE::English);

    // Remove punctuation and lowercase the text to make parsing easier
    let lowercase_doc = document.to_ascii_lowercase();
    let regex_for_punctuation = one_or_more(punctuation());
    let text_without_punctuation = regex_for_punctuation
        .to_regex()
        .replace_all(&*lowercase_doc, "");

    // Make a regex to match stopwords with trailing spaces and punctuation
    let regex_for_stop_words =
        word_boundary() + exactly(1, or(&words)) + word_boundary() + one_or_more(whitespace());

    // Remove stop words
    let clean_text = regex_for_stop_words
        .to_regex()
        .replace_all(&*text_without_punctuation, "");
    println!("\nClean text:\n{}", clean_text);
}
