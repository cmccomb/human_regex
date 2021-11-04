#[cfg(test)]
mod tests {
    use human_regex as hr;
    use regex::Regex;

    #[test]
    fn find_a_date() {
        let regex_string = hr::beginning()
            + hr::exactly(4, hr::digit())
            + hr::text("-")
            + hr::exactly(2, hr::digit())
            + hr::text("-")
            + hr::exactly(2, hr::digit())
            + hr::end();
        assert!(regex_string.to_regex().is_match("2014-01-01"))
    }

    #[test]
    fn iterating_over_capture_groups() {
        let regex_string = hr::beginning()
            + hr::capture(hr::exactly(4, hr::digit()))
            + hr::text("-")
            + hr::capture(hr::exactly(2, hr::digit()))
            + hr::text("-")
            + hr::capture(hr::exactly(2, hr::digit()))
            + hr::end();
        let text = "2012-03-14, 2013-01-01 and 2014-07-05";
        let match_text = ["M03D14Y2012", "M01D01Y2013", "M07D05Y2014"];
        for (i, cap) in regex_string.to_regex().captures_iter(text).enumerate() {
            assert_eq!(
                format!("M{}D{}Y{}", &cap[2], &cap[3], &cap[1]),
                match_text[i]
            );
        }
    }

    #[test]
    fn replacement_with_named_capture_groups() {
        let regex_string = hr::named_capture(hr::exactly(4, hr::digit()), "y")
            + hr::text("-")
            + hr::named_capture(hr::exactly(2, hr::digit()), "m")
            + hr::text("-")
            + hr::named_capture(hr::exactly(2, hr::digit()), "d");
        let before = "2012-03-14, 2013-01-01 and 2014-07-05";
        let after = regex_string.to_regex().replace_all(before, "$m/$d/$y");
        assert_eq!(after, "03/14/2012, 01/01/2013 and 07/05/2014");
    }
}
