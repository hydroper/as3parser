use num_traits::ToPrimitive;
use crate::compilation_unit::*;
use crate::diagnostics::*;
use crate::parser::CharacterValidator;
use crate::util::CharacterReader;

fn _parse_string_content(input: &str, location: &Location) -> String {
    let mut input = CharacterReader::from(&input[1..(input.len() - 1)]);
    let mut result = String::new();
    while let Some(ch1) = input.next() {
        let i = location.first_offset() + 1 + input.index() - 1;
        if ch1 == '\\' {
            let mut digits = String::new();
            loop {
                if !CharacterValidator::is_hex_digit(input.peek_or_zero()) {
                    break;
                }
                digits.push(input.next_or_zero());
            }
            let mv = u32::from_str_radix(&digits, 16).ok().and_then(|mv| char::from_u32(mv));
            let j = location.first_offset() + 1 + input.index();
            if let Some(mv) = mv {
                result.push(mv);
            } else {
                location.compilation_unit().add_diagnostic(Diagnostic::new_syntax_error(&Location::with_offsets(&location.compilation_unit(), i, j), DiagnosticKind::CssInvalidHexEscape, diagnostic_arguments![String(digits)]));
            }
        } else {
            result.push(ch1);
        }
    }
    result
}

fn _rgb_bytes_to_integer(r: f64, g: f64, b: f64) -> u32 {
    (_calc_rgb_byte(r) << 16) | (_calc_rgb_byte(g) << 8) | _calc_rgb_byte(b)
}

fn _calc_rgb_byte(value: f64) -> u32 {
    // Integer
    if value.round() == value {
        value.round().to_u32().unwrap_or(0).clamp(0, 255)
    // Float
    } else {
        (value * 255.0).round().to_u32().unwrap_or(0).clamp(0, 255)
    }
}