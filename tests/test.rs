#[cfg(test)]
mod tests {
    #[test]
    fn match_date() {
        use human_regex as hr;
        let regex_string = hr::begin()
            + hr::exactly(4, hr::digit())
            + hr::text("-")
            + hr::exactly(2, hr::digit())
            + hr::text("-")
            + hr::exactly(2, hr::digit())
            + hr::end();
        assert!(regex_string.to_regex().is_match("2014-01-01"))
    }

    #[test]
    #[should_panic]
    fn match_date_should_panic() {
        use human_regex as hr;
        let check_match = hr::begin()
            + hr::exactly(4, hr::digit())
            + hr::text("-")
            + hr::exactly(2, hr::digit())
            + hr::text("-")
            + hr::exactly(2, hr::digit());
        assert!(check_match.to_regex().is_match("01-01-2014"))
    }
}
