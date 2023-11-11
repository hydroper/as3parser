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
        DiagnosticKind::UnallowedLineBreak.id() => "Unallowed line break".into(),
        DiagnosticKind::Expected.id() => "Expected {1} before {2}".into(),
        DiagnosticKind::ExpectedIdentifier.id() => "Expected identifier before {1}".into(),
        DiagnosticKind::ExpectedExpression.id() => "Expected expression before {1}".into(),
        DiagnosticKind::ExpectedXmlName.id() => "Expected XML name before {1}".into(),
        DiagnosticKind::ExpectedXmlAttributeValue.id() => "Expected XML attribute value before {1}".into(),
        DiagnosticKind::MalformedArrowFunctionElement.id() => "Malformed arrow function element".into(),
        DiagnosticKind::WrongParameterPosition.id() => "Wrong parameter position".into(),
        DiagnosticKind::DuplicateRestParameter.id() => "Duplicate rest parameter".into(),
        // DiagnosticKind::K.id() => "".into(),
    };
}