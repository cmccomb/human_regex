#[cfg(test)]
mod tests {
    #[test]
    fn check_direct_regex() {
        let check_match = human_regex::HumanRegex::new(r"^\d{4}-\d{2}-\d{2}$");
        assert!(check_match.is_match("2014-01-01"))
    }
    #[test]
    fn check_default() {
        let check_match = human_regex::HumanRegex::default();
        assert!(check_match.is_match(""))
    }
}
