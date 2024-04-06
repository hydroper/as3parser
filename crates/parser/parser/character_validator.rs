//use lazy_regex::{Lazy, Regex, lazy_regex};
use unicode_general_category::{get_general_category, GeneralCategory};

// pub(crate) static CR_OR_CRLF_REGEX: Lazy<Regex> = lazy_regex!(r"\r\n?");

/// The `CharacterValidator` structure defines static methods for character
/// validation.
pub struct CharacterValidator;

impl CharacterValidator {
    /// Returns the count of indentation characters in a string.
    pub fn indent_count(string: &str) -> usize {
        let mut n: usize = 0;
        for ch in string.chars() {
            if !CharacterValidator::is_whitespace(ch) {
                break;
            }
            n += 1;
        }
        n
    }

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
        CharacterValidator::is_dec_digit(ch) || (ch >= '\x41' && ch <= '\x46') || (ch >= '\x61' && ch <= '\x66')
    }

    /// Returns the mathematical value of a hexadecimal digit.
    pub fn hex_digit_mv(ch: char) -> Option<u32> {
        if ch >= 'A' && ch <= 'F' {
            Some((ch as u32) - 0x41 + 10)
        } else if ch >= 'a' && ch <= 'f' {
            Some((ch as u32) - 0x61 + 10)
        } else if ch >= '0' && ch <= '9' {
            Some((ch as u32) - 0x30)
        } else {
            None
        }
    }

    /// Returns the mathematical value of a binary digit.
    pub fn bin_digit_mv(ch: char) -> Option<u32> {
        if ch >= '0' && ch <= '1' {
            Some((ch as u32) - 0x30)
        } else {
            None
        }
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

    pub fn is_xml_whitespace(ch: char) -> bool {
        ch == '\x20' || ch == '\x09' || ch == '\x0A' || ch == '\x0D'
    }
}