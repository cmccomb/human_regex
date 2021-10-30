#[cfg(test)]
mod tests {

    #[test]
    fn check_direct_regex() {
        let check_match = human_regex::HumanRegex::from_regex_string(r"^\d{4}-\d{2}-\d{2}$");
        assert!(check_match.is_match("2014-01-01"))
    }
    #[test]
    fn check_default() {
        let check_match = human_regex::HumanRegex::default();
        assert!(check_match.is_match(""))
    }
    #[test]
    fn check_new() {
        let check_match = human_regex::HumanRegex::new();
        assert!(check_match.is_match(""))
    }
    #[test]
    fn check_exactly_plus_text() {
        let check_match = human_regex::HumanRegex::new()
            .begin()
            .exactly(4, human_regex::DIGIT)
            .text("-")
            .exactly(2, human_regex::DIGIT)
            .text("-")
            .exactly(2, human_regex::DIGIT)
            .end();
        assert!(check_match.is_match("2014-01-01"))
    }
}
