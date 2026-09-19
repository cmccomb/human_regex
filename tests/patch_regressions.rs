use human_regex as hr;

fn anchored<T>(pattern: hr::HumanRegex<T>) -> regex::Regex {
    (hr::beginning_of_text() + pattern + hr::end_of_text()).to_regex()
}

fn check_range(start: char, end: char, samples: &[char]) {
    let within = anchored(hr::within_range(start..=end));
    let without = anchored(hr::without_range(start..=end));
    for &c in samples {
        let expected = (start..=end).contains(&c);
        assert_eq!(
            within.is_match(&c.to_string()),
            expected,
            "within({start:?}..={end:?}) matching {c:?}"
        );
        assert_eq!(
            without.is_match(&c.to_string()),
            !expected,
            "without({start:?}..={end:?}) matching {c:?}"
        );
    }
    for input in [String::new(), format!("{start}{end}")] {
        assert!(!within.is_match(&input));
        assert!(!without.is_match(&input));
    }
}

#[test]
fn ranges_escape_ascii_metacharacters_at_either_endpoint() {
    let endpoints = [
        '\0', '\t', '\n', ' ', '#', '$', '&', '(', ')', '*', '+', '-', '.', '?', '[', '\\', ']',
        '^', 'a', 'z', '{', '|', '}', '~', '\x7f',
    ];
    let samples: Vec<char> = (0u8..=127)
        .map(char::from)
        .chain(['é', '中', '🦀'])
        .collect();
    for (index, &start) in endpoints.iter().enumerate() {
        for &end in &endpoints[index..] {
            check_range(start, end, &samples);
        }
    }
}

#[test]
fn ranges_preserve_unicode_and_single_character_semantics() {
    let samples = [
        '\0',
        'a',
        'z',
        'é',
        'ê',
        'ë',
        'α',
        'β',
        'γ',
        'δ',
        '中',
        '\u{d7ff}',
        '\u{e000}',
        '🦀',
        '🦁',
        '🦂',
        '\u{10ffff}',
    ];
    for (start, end) in [
        ('é', 'ë'),
        ('α', 'γ'),
        ('中', '中'),
        ('🦀', '🦂'),
        ('\u{d7ff}', '\u{e000}'),
    ] {
        check_range(start, end, &samples);
    }
}

#[test]
fn whole_unicode_range_preserves_regex_engine_behavior() {
    let samples = ["\0", "a", "中", "🦀", "\u{10ffff}"];
    let within = anchored(hr::within_range('\0'..='\u{10ffff}'));
    for input in samples {
        assert!(within.is_match(input));
    }

    // regex 1.7.1 rejects empty character classes; newer versions allow them.
    // Preserve that engine behavior when the negated range contains no characters.
    let without = regex::Regex::new(&hr::without_range('\0'..='\u{10ffff}').to_string());
    let reference = regex::Regex::new(r"[^\x{0}-\x{10ffff}]");
    assert_eq!(without.is_ok(), reference.is_ok());
    if let Ok(without) = without {
        for input in samples {
            assert!(!without.is_match(input));
        }
    }
}

#[test]
fn escape_all_makes_each_ascii_character_literal() {
    let inputs: Vec<String> = (0u8..=127)
        .map(|c| format!("left{}right", char::from(c)))
        .collect();
    let escaped = hr::escape_all(&inputs);
    for (input, pattern) in inputs.iter().zip(escaped) {
        let compiled = regex::Regex::new(&format!(r"\A(?:{pattern})\z")).unwrap();
        assert!(compiled.is_match(input), "literal {input:?}");
        assert!(!compiled.is_match("leftright"), "literal {input:?}");
        for other in &inputs {
            assert_eq!(
                compiled.is_match(other),
                input == other,
                "literal {input:?}"
            );
        }
    }
}

#[test]
fn escaped_alternatives_match_only_literal_inputs() {
    let options = ["a.b", "c+d", r"\d", "(x|y)", "^start$", "[a-z]", "🦀.é"];
    let compiled = anchored(hr::or(&hr::escape_all(&options)));
    for input in options {
        assert!(compiled.is_match(input), "{input:?}");
    }
    for input in ["axb", "cd", "cccd", "7", "x", "y", "start", "a", "🦀xé", ""] {
        assert!(!compiled.is_match(input), "{input:?}");
    }
    assert_eq!(hr::escape_all::<&str>(&[]), Vec::<String>::new());
    assert_eq!(hr::escape_all(&[""]), vec![String::new()]);
}

#[test]
fn alternation_does_not_accept_colons_or_add_captures() {
    let compiled = anchored(hr::or(&["cat", "dog"]));
    for input in ["cat", "dog"] {
        assert!(compiled.is_match(input));
    }
    for input in [":cat", ":dog", "catdog", ""] {
        assert!(!compiled.is_match(input), "{input:?}");
    }
    assert_eq!(compiled.captures_len(), 1);
    assert_eq!(anchored(hr::or(&["cat"])).captures_len(), 1);
}

#[test]
fn intersections_do_not_accept_colons_or_add_captures() {
    for pattern in [
        hr::and(hr::within_range('a'..='z'), hr::within_range('m'..='z')),
        hr::within_range('a'..='z') & hr::within_range('m'..='z'),
    ] {
        let compiled = anchored(pattern);
        for input in ["m", "z"] {
            assert!(compiled.is_match(input));
        }
        for input in [":m", ":z", "a", "mm", ""] {
            assert!(!compiled.is_match(input), "{input:?}");
        }
        assert_eq!(compiled.captures_len(), 1);
    }
}

#[test]
fn logical_composition_preserves_explicit_capture_numbering() {
    let pattern = hr::capture(hr::or(&["cat", "dog"]))
        + hr::text("-")
        + hr::named_capture(
            hr::and(hr::within_range('a'..='z'), hr::within_range('m'..='z')),
            "letter",
        )
        + hr::capture(hr::digit());
    let compiled = anchored(pattern);
    let captures = compiled.captures("dog-m7").unwrap();
    assert_eq!(captures.len(), 4);
    assert_eq!(&captures[1], "dog");
    assert_eq!(&captures[2], "m");
    assert_eq!(&captures["letter"], "m");
    assert_eq!(&captures[3], "7");
    assert_eq!(compiled.replace("dog-m7", "$3:$letter:$1"), "7:m:dog");
}
