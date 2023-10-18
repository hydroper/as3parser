use std::collections::HashMap;
use lazy_static::lazy_static;
use maplit::hashmap;
use crate::DiagnosticKind;

lazy_static! {
    pub static ref MESSAGES: HashMap<i32, String> = hashmap! {
        // DiagnosticKind::K.id() => "".into(),
        DiagnosticKind::UnexpectedOrInvalidToken.id() => "Unexpected or invalid token".into(),
        DiagnosticKind::UnexpectedEnd.id() => "Unexpected end of program".into(),
        DiagnosticKind::FailedProcessingNumericLiteral.id() => "Failed processing numeric literal".into(),
        DiagnosticKind::UnallowedNumericSuffix.id() => "Unallowed numeric suffix".into(),
        // DiagnosticKind::K.id() => "".into(),
    };
}