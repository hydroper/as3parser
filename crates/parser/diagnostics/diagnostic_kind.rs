#[repr(i32)]
#[derive(Eq, PartialEq, Clone, Copy)]
pub enum DiagnosticKind {
    UnexpectedOrInvalidToken = 1024,
    UnexpectedEnd = 1025,
    UnallowedNumericSuffix = 1026,
    UnallowedLineBreak = 1027,
    Expected = 1028,
    ExpectedIdentifier = 1029,
    ExpectedExpression = 1030,
    ExpectedXmlName = 1031,
    ExpectedXmlAttributeValue = 1032,
    IllegalNullishCoalescingLeftOperand = 1033,
    WrongParameterPosition = 1034,
    DuplicateRestParameter = 1035,
    NotAllowedHere = 1036,
    MalformedRestParameter = 1037,
    IllegalForInInitializer = 1038,
    MultipleForInBindings = 1039,
    UndefinedLabel = 1040,
    IllegalContinue = 1041,
    IllegalBreak = 1042,
    ExpressionMustNotFollowLineBreak = 1043,
    TokenMustNotFollowLineBreak = 1044,
    ExpectedStringLiteral = 1045,
    DuplicateAttribute = 1046,
    DuplicateAccessModifier = 1047,
    ExpectedDirectiveKeyword = 1048,
    UnallowedAttribute = 1049,
    UseDirectiveMustContainPublic = 1050,
    MalformedEnumMember = 1051,
    FunctionMayNotBeGenerator = 1052,
    FunctionMayNotBeAsynchronous = 1053,
    FunctionMustNotContainBody = 1054,
    FunctionMustContainBody = 1055,
    FunctionMustNotContainAnnotations = 1056,
    NestedClassesNotAllowed = 1057,
    DirectiveNotAllowedInInterface = 1058,
    FailedParsingAsDocTag = 1059,
    UnrecognizedAsDocTag = 1060,
    UnrecognizedProxy = 1061,
    EnumMembersMustBeConst = 1062,
    ConstructorMustNotSpecifyResultType = 1063,
    UnrecognizedMetadataSyntax = 1064,
}

impl DiagnosticKind {
    pub fn id(&self) -> i32 {
        *self as i32
    }
}