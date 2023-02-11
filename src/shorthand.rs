//! Functions for general purpose matches

use super::humanregex::HumanRegex;

/// A function for matching any character (except for \n)
/// ```
/// use human_regex::{text, any, exactly};
/// let regex_string = text("h") + exactly(2, any()) + text("l");
/// assert!(regex_string.to_regex().is_match("hurl"));
/// assert!(regex_string.to_regex().is_match("heal"));
/// ```
pub fn any() -> HumanRegex {
    HumanRegex(r".".to_string())
}

/// A function for the digit character class (i.e., the digits 0 through 9)
/// ```
/// use human_regex::{beginning, end, one_or_more, digit};
/// let regex_string = beginning() + one_or_more(digit()) + end();
/// assert!(regex_string.to_regex().is_match("010101010100100100100101"));
/// assert!(!regex_string.to_regex().is_match("a string that is not composed of digits will fail"));
/// ```
pub fn digit() -> HumanRegex {
    HumanRegex(r"\d".to_string())
}

/// A function for the non-digit character class (i.e., everything BUT the digits 0-9)
/// ```
/// use human_regex::{beginning, end, one_or_more, non_digit};
/// let regex_string = beginning() + one_or_more(non_digit()) + end();
/// assert!(regex_string.to_regex().is_match("a string without digits will pass"));
/// assert!(!regex_string.to_regex().is_match("a string with digits like 99 will fail"));
/// ```
pub fn non_digit() -> HumanRegex {
    HumanRegex(r"\D".to_string())
}

/// A function for the word character class (i.e., all alphanumeric characters plus underscore)
pub fn word() -> HumanRegex {
    HumanRegex(r"\w".to_string())
}

/// A function for the non-word character class (i.e., everything BUT the alphanumeric characters plus underscore)
pub fn non_word() -> HumanRegex {
    HumanRegex(r"\W".to_string())
}

/// A constant for the whitespace character class (i.e., space and tab)
/// ```
/// use human_regex::{one_or_more, text, whitespace};
/// let regex_string = text("at") + one_or_more(whitespace()) + text("least");
/// assert!(!regex_string.to_regex().is_match("atleast"));
/// assert!(regex_string.to_regex().is_match("at least"));
/// assert!(regex_string.to_regex().is_match("at    least"));
/// ```
pub fn whitespace() -> HumanRegex {
    HumanRegex(r"\s".to_string())
}

/// A function for the whitespace character class (i.e., everything BUT space and tab)
/// ```
/// use human_regex::{beginning, end, one_or_more, non_whitespace};
/// let regex_string = beginning() + one_or_more(non_whitespace()) + end();
/// assert!(regex_string.to_regex().is_match("supercalifragilisticexpialidocious"));
/// assert!(regex_string.to_regex().is_match("a-sluggified-thingamajig"));
/// assert!(!regex_string.to_regex().is_match("something with spaces won't pass"));
/// ```
pub fn non_whitespace() -> HumanRegex {
    HumanRegex(r"\S".to_string())
}

/// Matches anything within a range of characters
///```
/// use human_regex::{beginning, end, within_range};
/// let regex_string = beginning() + within_range('a'..='d') + end();
/// println!("{}", beginning());
/// assert!(regex_string.to_regex().is_match("c"));
/// assert!(!regex_string.to_regex().is_match("h"));
///```
pub fn within_range(range: std::ops::RangeInclusive<char>) -> HumanRegex {
    HumanRegex(format!("[{}-{}]", range.start(), range.end()))
}
/// Matches anything outside of a range of characters
///```
/// use human_regex::{beginning, end, without_range};
/// let regex_string = beginning() + without_range('a'..='d') + end();
/// println!("{}", beginning());
/// assert!(regex_string.to_regex().is_match("h"));
/// assert!(!regex_string.to_regex().is_match("c"));
///```
pub fn without_range(range: std::ops::RangeInclusive<char>) -> HumanRegex {
    HumanRegex(format!("[^{}-{}]", range.start(), range.end()))
}

/// An enum covering all Unicode character categories
///
/// Used in the [unicode_category] function.
#[allow(missing_docs)] // variants are self documenting
pub enum UnicodeCategory {
    Letter,
    LowercaseLetter,
    UppercaseLetter,
    TitlecaseLetter,
    CasedLetter,
    ModifierLetter,
    OtherLetter,
    Mark,
    NonSpacingMark,
    SpaceCombiningMark,
    EnclosingMark,
    Separator,
    SpaceSeparator,
    LineSeparator,
    ParagraphSeparator,
    Symbol,
    MathSymbol,
    CurrencySymbol,
    ModifierSymbol,
    OtherSymbol,
    Number,
    DecimalDigitNumber,
    LetterNumber,
    OtherNumber,
    Punctuation,
    DashPunctuation,
    OpenPunctuation,
    ClosePunctuation,
    InitialPunctuation,
    FinalPunctuation,
    ConnectorPunctuation,
    OtherPunctuation,
    Other,
    Control,
    Format,
    PrivateUse,
    Surrogate,
    Unassigned,
}

/// A function for matching Unicode character categories. For matching script categories see [unicode_script].
/// ```
/// use human_regex::{beginning, end, one_or_more, unicode_category, UnicodeCategory};
/// let regex_string = beginning()
///     + one_or_more(unicode_category(UnicodeCategory::CurrencySymbol))
///     + end();
/// assert!(regex_string.to_regex().is_match("$¥₹"));
/// assert!(!regex_string.to_regex().is_match("normal words"));
/// ```
pub fn unicode_category(category: UnicodeCategory) -> HumanRegex {
    HumanRegex(match category {
        UnicodeCategory::Letter => r"\p{Letter}".to_string(),
        UnicodeCategory::LowercaseLetter => r"\p{Lowercase_Letter}".to_string(),
        UnicodeCategory::UppercaseLetter => r"\p{Uppercase_Letter}".to_string(),
        UnicodeCategory::TitlecaseLetter => r"\p{Titlecase_Letter}".to_string(),
        UnicodeCategory::CasedLetter => r"\p{Cased_Letter}".to_string(),
        UnicodeCategory::ModifierLetter => r"\p{Modifier_Letter}".to_string(),
        UnicodeCategory::OtherLetter => r"\p{Other_Letter}".to_string(),
        UnicodeCategory::Mark => r"\p{Mark}".to_string(),
        UnicodeCategory::NonSpacingMark => r"\p{NonSpacing_Mark}".to_string(),
        UnicodeCategory::SpaceCombiningMark => r"\p{SpaceCombining_Mark}".to_string(),
        UnicodeCategory::EnclosingMark => r"\p{Enclosing_Mark}".to_string(),
        UnicodeCategory::Separator => r"\p{Separator}".to_string(),
        UnicodeCategory::SpaceSeparator => r"\p{Space_Separator}".to_string(),
        UnicodeCategory::LineSeparator => r"\p{Line_Separator}".to_string(),
        UnicodeCategory::ParagraphSeparator => r"\p{Paragraph_Separator}".to_string(),
        UnicodeCategory::Symbol => r"\p{Symbol}".to_string(),
        UnicodeCategory::MathSymbol => r"\p{Math_Symbol}".to_string(),
        UnicodeCategory::CurrencySymbol => r"\p{Currency_Symbol}".to_string(),
        UnicodeCategory::ModifierSymbol => r"\p{Modifier_Symbol}".to_string(),
        UnicodeCategory::OtherSymbol => r"\p{Other_Symbol}".to_string(),
        UnicodeCategory::Number => r"\p{Number}".to_string(),
        UnicodeCategory::DecimalDigitNumber => r"\p{DecimalDigit_Number}".to_string(),
        UnicodeCategory::LetterNumber => r"\p{Letter_Number}".to_string(),
        UnicodeCategory::OtherNumber => r"\p{Other_Number}".to_string(),
        UnicodeCategory::Punctuation => r"\p{Punctuation}".to_string(),
        UnicodeCategory::DashPunctuation => r"\p{Dash_Punctuation}".to_string(),
        UnicodeCategory::OpenPunctuation => r"\p{Open_Punctuation}".to_string(),
        UnicodeCategory::ClosePunctuation => r"\p{Close_Punctuation}".to_string(),
        UnicodeCategory::InitialPunctuation => r"\p{Initial_Punctuation}".to_string(),
        UnicodeCategory::FinalPunctuation => r"\p{Final_Punctuation}".to_string(),
        UnicodeCategory::ConnectorPunctuation => r"\p{Connector_Punctuation}".to_string(),
        UnicodeCategory::OtherPunctuation => r"\p{Other_Punctuation}".to_string(),
        UnicodeCategory::Other => r"\p{Other}".to_string(),
        UnicodeCategory::Control => r"\p{Control}".to_string(),
        UnicodeCategory::Format => r"\p{Format}".to_string(),
        UnicodeCategory::PrivateUse => r"\p{Private_Use}".to_string(),
        UnicodeCategory::Surrogate => r"\p{Surrogate}".to_string(),
        UnicodeCategory::Unassigned => r"\p{Unassigned}".to_string(),
    })
}

/// A function for not matching Unicode character categories. For matching script categories see [non_unicode_script].
/// ```
/// use human_regex::{beginning, end, one_or_more, non_unicode_category, UnicodeCategory};
/// let regex_string = beginning()
///     + one_or_more(non_unicode_category(UnicodeCategory::CurrencySymbol))
///     + end();
/// assert!(regex_string.to_regex().is_match("normal words"));
/// assert!(!regex_string.to_regex().is_match("$¥₹"));
/// ```
pub fn non_unicode_category(category: UnicodeCategory) -> HumanRegex {
    HumanRegex(unicode_category(category).to_string().replace(r"\p", r"\P"))
}

/// An enum for covering all Unicode script categories
///
/// Used in the [unicode_script] function
#[allow(missing_docs)] // variants are self documenting
pub enum UnicodeScript {
    Common,
    Arabic,
    Armenian,
    Bengali,
    Bopomofo,
    Braille,
    Buhid,
    CandianAboriginal,
    Cherokee,
    Cyrillic,
    Devanagari,
    Ethiopic,
    Georgian,
    Greek,
    Gujarati,
    Gurkmukhi,
    Han,
    Hangul,
    Hanunoo,
    Hebrew,
    Hirigana,
    Inherited,
    Kannada,
    Katakana,
    Khmer,
    Lao,
    Latin,
    Limbu,
    Malayalam,
    Mongolian,
    Myanmar,
    Ogham,
    Oriya,
    Runic,
    Sinhala,
    Syriac,
    Tagalog,
    Tagbanwa,
    TaiLe,
    Tamil,
    Telugu,
    Thaana,
    Thai,
    Tibetan,
    Yi,
}

/// A function for matching Unicode characters belonging to a certain script category. For matching other categories see [unicode_category].
/// ```
/// use human_regex::{beginning, end, one_or_more, unicode_script, UnicodeScript};
/// let regex_string = beginning()
///     + one_or_more(unicode_script(UnicodeScript::Han))
///     + end();
/// assert!(regex_string.to_regex().is_match("蟹"));
/// assert!(!regex_string.to_regex().is_match("latin text"));
/// ```
pub fn unicode_script(category: UnicodeScript) -> HumanRegex {
    HumanRegex(match category {
        UnicodeScript::Common => r"\p{Common}".to_string(),
        UnicodeScript::Arabic => r"\p{Arabic}".to_string(),
        UnicodeScript::Armenian => r"\p{Armenian}".to_string(),
        UnicodeScript::Bengali => r"\p{Bengali}".to_string(),
        UnicodeScript::Bopomofo => r"\p{Bopomofo}".to_string(),
        UnicodeScript::Braille => r"\p{Braille}".to_string(),
        UnicodeScript::Buhid => r"\p{Buhid}".to_string(),
        UnicodeScript::CandianAboriginal => r"\p{CandianAboriginal}".to_string(),
        UnicodeScript::Cherokee => r"\p{Cherokee}".to_string(),
        UnicodeScript::Cyrillic => r"\p{Cyrillic}".to_string(),
        UnicodeScript::Devanagari => r"\p{Devanagari}".to_string(),
        UnicodeScript::Ethiopic => r"\p{Ethiopic}".to_string(),
        UnicodeScript::Georgian => r"\p{Georgian}".to_string(),
        UnicodeScript::Greek => r"\p{Greek}".to_string(),
        UnicodeScript::Gujarati => r"\p{Gujarati}".to_string(),
        UnicodeScript::Gurkmukhi => r"\p{Gurkmukhi}".to_string(),
        UnicodeScript::Han => r"\p{Han}".to_string(),
        UnicodeScript::Hangul => r"\p{Hangul}".to_string(),
        UnicodeScript::Hanunoo => r"\p{Hanunoo}".to_string(),
        UnicodeScript::Hebrew => r"\p{Hebrew}".to_string(),
        UnicodeScript::Hirigana => r"\p{Hirigana}".to_string(),
        UnicodeScript::Inherited => r"\p{Inherited}".to_string(),
        UnicodeScript::Kannada => r"\p{Kannada}".to_string(),
        UnicodeScript::Katakana => r"\p{Katakana}".to_string(),
        UnicodeScript::Khmer => r"\p{Khmer}".to_string(),
        UnicodeScript::Lao => r"\p{Lao}".to_string(),
        UnicodeScript::Latin => r"\p{Latin}".to_string(),
        UnicodeScript::Limbu => r"\p{Limbu}".to_string(),
        UnicodeScript::Malayalam => r"\p{Malayalam}".to_string(),
        UnicodeScript::Mongolian => r"\p{Mongolian}".to_string(),
        UnicodeScript::Myanmar => r"\p{Myanmar}".to_string(),
        UnicodeScript::Ogham => r"\p{Ogham}".to_string(),
        UnicodeScript::Oriya => r"\p{Oriya}".to_string(),
        UnicodeScript::Runic => r"\p{Runic}".to_string(),
        UnicodeScript::Sinhala => r"\p{Sinhala}".to_string(),
        UnicodeScript::Syriac => r"\p{Syriac}".to_string(),
        UnicodeScript::Tagalog => r"\p{Tagalog}".to_string(),
        UnicodeScript::Tagbanwa => r"\p{Tagbanwa}".to_string(),
        UnicodeScript::TaiLe => r"\p{TaiLe}".to_string(),
        UnicodeScript::Tamil => r"\p{Tamil}".to_string(),
        UnicodeScript::Telugu => r"\p{Telugu}".to_string(),
        UnicodeScript::Thaana => r"\p{Thaana}".to_string(),
        UnicodeScript::Thai => r"\p{Thai}".to_string(),
        UnicodeScript::Tibetan => r"\p{Tibetan}".to_string(),
        UnicodeScript::Yi => r"\p{Yi}".to_string(),
    })
}

/// A function for matching Unicode characters not belonging to a certain script category. For matching other categories see [non_unicode_category].
/// ```
/// use human_regex::{beginning, end, one_or_more, non_unicode_script, UnicodeScript};
/// let regex_string = beginning()
///     + one_or_more(non_unicode_script(UnicodeScript::Han))
///     + end();
/// assert!(regex_string.to_regex().is_match("latin text"));
/// assert!(!regex_string.to_regex().is_match("蟹"));
/// ```
pub fn non_unicode_script(category: UnicodeScript) -> HumanRegex {
    HumanRegex(unicode_script(category).to_string().replace(r"\p", r"\P"))
}
