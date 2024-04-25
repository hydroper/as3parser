use lalrpop_util::lalrpop_mod;
use crate::compilation_unit::*;
use crate::diagnostics::*;
use crate::parser::CharacterValidator;
use crate::util::CharacterReader;

lalrpop_mod!(pub css, "/parser/css.rs");

pub(crate) fn parse_string_content(input: &str, location: &Location) -> String {
    let mut input = CharacterReader::from(input[1..(input.len() - 1)]);
    let mut result = String::new();
    let mut found_invalid_escapes = false;
    while let Some(ch1) = input.next() {
        if ch1 == '\\' {
            let mut digits = String::new();
            loop {
                if !CharacterValidator::is_hex_digit(input.peek_or_zero()) {
                    break;
                }
                digits.push(input.next_or_zero());
            }
            let mv = u32::from_str_radix(&digits, 16).ok().and_then(|mv| char::from_u32(mv));
            if let Some(mv) = mv {
                result.push(mv);
            } else {
                found_invalid_escapes = true;
            }
        } else {
            result.push(ch1);
        }
    }
    if found_invalid_escapes {
        location.compilation_unit().add_diagnostic(Diagnostic::new_syntax_error(location, DiagnosticKind::CssStringContainsInvalidHexEscapes, vec![]));
    }
    result
}