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
        DiagnosticKind::MalformedDestructuring.id() => "Malformed destructuring".into(),
        DiagnosticKind::UnsupportedDestructuringRest.id() => "Unsupported destructuring rest operation".into(),
        DiagnosticKind::NotAllowedHere.id() => "{1} not allowed here".into(),
        DiagnosticKind::IllegalNullishCoalescingLeftOperand.id() => "Illegal nullish coalescing left operand".into(),
        DiagnosticKind::MalformedRestParameter.id() => "Malformed rest parameter".into(),
        DiagnosticKind::IllegalForInInitializer.id() => "Illegal for..in initializer".into(),
        DiagnosticKind::MultipleForInBindings.id() => "Multiple for..in bindings".into(),
        DiagnosticKind::IllegalBreak.id() => "Illegal break statement".into(),
        DiagnosticKind::IllegalContinue.id() => "Illegal continue statement".into(),
        DiagnosticKind::UndefinedLabel.id() => "Undefined label {1}".into(),
        DiagnosticKind::ExpressionMustNotFollowLineBreak.id() => "Expression must not follow line break".into(),
        DiagnosticKind::TokenMustNotFollowLineBreak.id() => "Token must not follow line break".into(),
        DiagnosticKind::ParentSourceIsNotAFile.id() => "Parent source is not a file".into(),
        DiagnosticKind::FailedToIncludeFile.id() => "Failed to include file".into(),
        DiagnosticKind::UnrecognizedAsDocTag.id() => "Unrecognized ASDoc tag '{1}'".into(),
        DiagnosticKind::FailedParsingAsDocTag.id() => "Failed parsing ASDoc '{1}' tag".into(),
        DiagnosticKind::MalformedMetadataElement.id() => "Malformed meta data element".into(),
        // DiagnosticKind::K.id() => "".into(),
    };
}