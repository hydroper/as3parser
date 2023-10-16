use std::collections::HashMap;
use maplit::hashmap;
use crate::DiagnosticKind;

pub static MESSAGES: HashMap<i32, String> = hashmap! {
    // DiagnosticKind::K.id() => "".into(),
    DiagnosticKind::UnexpectedOrInvalidToken.id() => "Unexpected or invalid token".into(),
    // DiagnosticKind::K.id() => "".into(),
};