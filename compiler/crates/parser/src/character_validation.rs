use lazy_regex::{Lazy, Regex, lazy_regex};
use unicode_general_category::{get_general_category, GeneralCategory};

pub static CR_OR_CRLF_REGEX: Lazy<Regex> = lazy_regex!(r"\r\n?");

pub fn is_whitespace(ch: char) -> bool {
    if ch == '\x20' || ch == '\x09' || ch == '\x08'
    || ch == '\x0C' || ch == '\u{A0}' {
        return true;
    }
    let category = get_general_category(ch);
    category == GeneralCategory::SpaceSeparator
}

pub fn is_line_terminator(ch: char) -> bool {
    ch == '\x0A' || ch == '\x0D' || ch == '\u{2028}' || ch == '\u{2029}'
}

pub fn is_bin_digit(ch: char) -> bool {
    ch == '\x30' || ch == '\x31'
}

pub fn is_dec_digit(ch: char) -> bool {
    ch >= '\x30' && ch <= '\x39'
}

pub fn is_hex_digit(ch: char) -> bool {
    is_dec_digit(ch) || (ch >= '\x41' && ch <= '\x46') || (ch >= '\x61' && ch <= '\x66')
}

pub fn is_identifier_start(ch: char) -> bool {
    if ch == '\x5f' || ch == '\x24' {
        return true;
    }
    let category = get_general_category(ch);
    [
        GeneralCategory::LowercaseLetter,
        GeneralCategory::UppercaseLetter,
        GeneralCategory::ModifierLetter,
        GeneralCategory::OtherLetter,
        GeneralCategory::TitlecaseLetter,
        GeneralCategory::LetterNumber,
    ].contains(&category)
}

pub fn is_identifier_part(ch: char) -> bool {
    if ch == '\x5f' || ch == '\x24' {
        return true;
    }
    let category = get_general_category(ch);
    [
        GeneralCategory::LowercaseLetter,
        GeneralCategory::UppercaseLetter,
        GeneralCategory::ModifierLetter,
        GeneralCategory::OtherLetter,
        GeneralCategory::TitlecaseLetter,
        GeneralCategory::LetterNumber,
        GeneralCategory::NonspacingMark,
        GeneralCategory::SpacingMark,
        GeneralCategory::ConnectorPunctuation,
        GeneralCategory::DecimalNumber,
    ].contains(&category)
}

pub fn is_xml_name_start(ch: char) -> bool {
    if ch == '\x5f' || ch == ':' {
        return true;
    }
    let category = get_general_category(ch);
    [
        GeneralCategory::LowercaseLetter,
        GeneralCategory::UppercaseLetter,
        GeneralCategory::ModifierLetter,
        GeneralCategory::OtherLetter,
        GeneralCategory::TitlecaseLetter,
        GeneralCategory::LetterNumber,
    ].contains(&category)
}

pub fn is_xml_name_part(ch: char) -> bool {
    if ch == '\x5f' || ch == ':' || ch == '.' || ch == '-' {
        return true;
    }
    let category = get_general_category(ch);
    [
        GeneralCategory::LowercaseLetter,
        GeneralCategory::UppercaseLetter,
        GeneralCategory::ModifierLetter,
        GeneralCategory::OtherLetter,
        GeneralCategory::TitlecaseLetter,
        GeneralCategory::LetterNumber,
        GeneralCategory::DecimalNumber,
    ].contains(&category)
}