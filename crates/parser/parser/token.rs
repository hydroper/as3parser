use crate::ns::*;

/// Represents a lexical token.
#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    Eof,
    Identifier(String),
    StringLiteral(String),
    /// Numeric literal token.
    /// The numeric value is in character representation, which may be parsed
    /// through data type specific methods such as [`NumericLiteral::parse_double()`].
    NumericLiteral(String),
    RegExpLiteral {
        body: String,
        flags: String,
    },

    // Punctuator
    ColonColon,
    /// The `@` token.
    Attribute,
    /// The `..` token.
    Descendants,
    /// The `...` token.
    Ellipsis,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Dot,
    Semicolon,
    Comma,
    Lt,
    Gt,
    /// `<=`
    Le,
    /// `>=`
    Ge,
    Equals,
    NotEquals,
    StrictEquals,
    StrictNotEquals,
    Plus,
    Minus,
    Times,
    Div,
    Remainder,
    Increment,
    Decrement,
    LeftShift,
    RightShift,
    UnsignedRightShift,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
    BitwiseNot,
    LogicalAnd,
    LogicalXor,
    LogicalOr,
    Question,
    Exclamation,
    Colon,
    Assign,
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
    RemainderAssign,
    LeftShiftAssign,
    RightShiftAssign,
    UnsignedRightShiftAssign,
    BitwiseAndAssign,
    BitwiseXorAssign,
    BitwiseOrAssign,
    LogicalAndAssign,
    LogicalXorAssign,
    LogicalOrAssign,
    /// `**`
    Power,
    /// `**=`
    PowerAssign,
    /// `??`
    NullCoalescing,
    /// `??=`
    NullCoalescingAssign,
    /// `?.`
    OptionalChaining,

    // Reserved words
    As,
    Await,
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Default,
    Delete,
    Do,
    Else,
    Extends,
    False,
    Finally,
    For,
    Function,
    If,
    Implements,
    Import,
    In,
    Interface,
    Internal,
    Is,
    New,
    Not,
    Null,
    Package,
    Private,
    Protected,
    Public,
    Return,
    Super,
    Switch,
    This,
    Throw,
    True,
    Try,
    Typeof,
    Use,
    Var,
    Void,
    While,
    With,
    Yield,

    XmlWhitespace,
    XmlLtSlash,
    XmlSlashGt,
    XmlText(String),
    XmlName(String),
    XmlMarkup(String),
    XmlAttributeValue(String),
}

impl ToString for Token {
    /// Converts the token into a readable string.
    ///
    /// The method `Token::to_string` returns the following possible values:
    /// 
    /// * `"end of program"`
    /// * `"identifier"`
    /// * `"string"` for string literal
    /// * `"number"` for numeric literal
    /// * `"regular expression"` for regular expression literal
    /// * `"'keyword'"` for reserved word (including surrounding apostrophes)
    /// * `"'punctuator'"` for punctuator (including surrounding apostrophes)
    /// * `"XML whitespace"`
    /// * `"'</'"`
    /// * `"'/>'"`
    /// * `"XML text"`
    /// * `"XML name"`
    /// * `"XML markup"`
    /// * `"XML attribute value"`
    fn to_string(&self) -> String {
        (match self {
            Token::Eof => "end of program",
            Token::Identifier(_) => "identifier",
            Token::StringLiteral(_) => "string",
            Token::NumericLiteral(_) => "number",
            Token::RegExpLiteral { .. } => "regular expression",

            // Punctuators
            Token::ColonColon => "'::'",
            Token::Attribute => "'@'",
            Token::Descendants => "'..'",
            Token::Ellipsis => "'...'",
            Token::LeftParen => "'('",
            Token::RightParen => "')'",
            Token::LeftBracket => "'['",
            Token::RightBracket => "']'",
            Token::LeftBrace => "'{'",
            Token::RightBrace => "'}'",
            Token::Dot => "'.'",
            Token::Semicolon => "';'",
            Token::Comma => "','",
            Token::Lt => "'<'",
            Token::Gt => "'>'",
            Token::Le => "'<='",
            Token::Ge => "'>='",
            Token::Equals => "'=='",
            Token::NotEquals => "'!='",
            Token::StrictEquals => "'==='",
            Token::StrictNotEquals => "'!=='",
            Token::Plus => "'+'",
            Token::Minus => "'-'",
            Token::Times => "'*'",
            Token::Div => "'/'",
            Token::Remainder => "'%'",
            Token::Increment => "'++'",
            Token::Decrement => "'--'",
            Token::LeftShift => "'<<'",
            Token::RightShift => "'>>'",
            Token::UnsignedRightShift => "'>>>'",
            Token::BitwiseAnd => "'&'",
            Token::BitwiseXor => "'^'",
            Token::BitwiseOr => "'|'",
            Token::BitwiseNot => "'~'",
            Token::LogicalAnd => "'&&'",
            Token::LogicalXor => "'^^'",
            Token::LogicalOr => "'||'",
            Token::Question => "'?'",
            Token::Exclamation => "'!'",
            Token::Colon => "':'",
            Token::Assign => "'='",
            Token::AddAssign => "'+='",
            Token::SubtractAssign => "'-='",
            Token::MultiplyAssign => "'*='",
            Token::DivideAssign => "'/='",
            Token::RemainderAssign => "'%='",
            Token::LeftShiftAssign => "'<<='",
            Token::RightShiftAssign => "'>>='",
            Token::UnsignedRightShiftAssign => "'>>>='",
            Token::BitwiseAndAssign => "'&='",
            Token::BitwiseXorAssign => "'^='",
            Token::BitwiseOrAssign => "'|='",
            Token::LogicalAndAssign => "'&&='",
            Token::LogicalXorAssign => "'^^='",
            Token::LogicalOrAssign => "'||='",
            Token::Power => "'**'",
            Token::PowerAssign => "'**='",
            Token::NullCoalescing => "'??'",
            Token::NullCoalescingAssign => "'??='",
            Token::OptionalChaining => "'?.'",

            // Reserved words
            Token::As => "'as'",
            Token::Await => "'await'",
            Token::Break => "'break'",
            Token::Case => "'case'",
            Token::Catch => "'catch'",
            Token::Class => "'class'",
            Token::Const => "'const'",
            Token::Continue => "'continue'",
            Token::Default => "'default'",
            Token::Delete => "'delete'",
            Token::Do => "'do'",
            Token::Else => "'else'",
            Token::Extends => "'extends'",
            Token::False => "'false'",
            Token::Finally => "'finally'",
            Token::For => "'for'",
            Token::Function => "'function'",
            Token::If => "'if'",
            Token::Implements => "'implements'",
            Token::Import => "'import'",
            Token::In => "'in'",
            Token::Interface => "'interface'",
            Token::Internal => "'internal'",
            Token::Is => "'is'",
            Token::New => "'new'",
            Token::Not => "'not'",
            Token::Null => "'null'",
            Token::Package => "'package'",
            Token::Private => "'private'",
            Token::Protected => "'protected'",
            Token::Public => "'public'",
            Token::Return => "'return'",
            Token::Super => "'super'",
            Token::Switch => "'switch'",
            Token::This => "'this'",
            Token::Throw => "'throw'",
            Token::True => "'true'",
            Token::Try => "'try'",
            Token::Typeof => "'typeof'",
            Token::Use => "'use'",
            Token::Var => "'var'",
            Token::Void => "'void'",
            Token::While => "'while'",
            Token::With => "'with'",
            Token::Yield => "'yield'",

            Token::XmlWhitespace => "XML whitespace",
            Token::XmlLtSlash => "'</'",
            Token::XmlSlashGt => "'/>'",
            Token::XmlText(_) => "XML text",
            Token::XmlName(_) => "XML name",
            Token::XmlMarkup(_) => "XML markup",
            Token::XmlAttributeValue(_) => "XML attribute value",
        }).into()
    }
}

impl Token {
    pub fn is_context_keyword(token: &(Token, Location), keyword: &str) -> bool {
        if let Token::Identifier(name) = &token.0 {
            name == keyword && token.1.character_count() == name.len()
        } else {
            false
        }
    }

    /// Indicates whether the token is a reserved word.
    pub fn is_reserved_word(&self) -> bool {
        self.reserved_word_name().is_some()
    }

    /// Tests whether the token is a reserved word and returns
    /// its *IdentifierName* string.
    pub fn reserved_word_name(&self) -> Option<String> {
        match *self {
            Token::As => Some("as".into()),
            Token::Await => Some("await".into()),
            Token::Break => Some("break".into()),
            Token::Case => Some("case".into()),
            Token::Catch => Some("catch".into()),
            Token::Class => Some("class".into()),
            Token::Const => Some("const".into()),
            Token::Continue => Some("continue".into()),
            Token::Default => Some("default".into()),
            Token::Delete => Some("delete".into()),
            Token::Do => Some("do".into()),
            Token::Else => Some("else".into()),
            Token::Extends => Some("extends".into()),
            Token::False => Some("false".into()),
            Token::Finally => Some("finally".into()),
            Token::For => Some("for".into()),
            Token::Function => Some("function".into()),
            Token::If => Some("if".into()),
            Token::Implements => Some("implements".into()),
            Token::Import => Some("import".into()),
            Token::In => Some("in".into()),
            Token::Interface => Some("interface".into()),
            Token::Internal => Some("internal".into()),
            Token::Is => Some("is".into()),
            Token::New => Some("new".into()),
            Token::Not => Some("not".into()),
            Token::Null => Some("null".into()),
            Token::Package => Some("package".into()),
            Token::Private => Some("private".into()),
            Token::Protected => Some("protected".into()),
            Token::Public => Some("public".into()),
            Token::Return => Some("return".into()),
            Token::Super => Some("super".into()),
            Token::Switch => Some("switch".into()),
            Token::This => Some("this".into()),
            Token::Throw => Some("throw".into()),
            Token::True => Some("true".into()),
            Token::Try => Some("try".into()),
            Token::Typeof => Some("typeof".into()),
            Token::Use => Some("use".into()),
            Token::Var => Some("var".into()),
            Token::Void => Some("void".into()),
            Token::While => Some("while".into()),
            Token::With => Some("with".into()),
            Token::Yield => Some("yield".into()),
            _ => None,
        }
    }

    /// Converts a compound assignment, a logical assignment, or a nullish coalescing assignment to an *Operator* value.
    pub fn compound_assignment(&self) -> Option<Operator> {
        match self {
            Self::AddAssign => Some(Operator::Add),
            Self::SubtractAssign => Some(Operator::Subtract),
            Self::MultiplyAssign => Some(Operator::Multiply),
            Self::DivideAssign => Some(Operator::Divide),
            Self::RemainderAssign => Some(Operator::Remainder),
            Self::PowerAssign => Some(Operator::Power),
            Self::LeftShiftAssign => Some(Operator::ShiftLeft),
            Self::RightShiftAssign => Some(Operator::ShiftRight),
            Self::UnsignedRightShiftAssign => Some(Operator::ShiftRightUnsigned),
            Self::BitwiseAndAssign => Some(Operator::BitwiseAnd),
            Self::BitwiseXorAssign => Some(Operator::BitwiseXor),
            Self::BitwiseOrAssign => Some(Operator::BitwiseOr),
            Self::LogicalAndAssign => Some(Operator::LogicalAnd),
            Self::LogicalXorAssign => Some(Operator::LogicalXor),
            Self::LogicalOrAssign => Some(Operator::LogicalOr),
            Self::NullCoalescingAssign => Some(Operator::NullCoalescing),
            _ => None,
        }
    }

    /// Converts this token into a binary operator, excluding
    /// `not in`, and `is not`.
    pub fn to_binary_operator(&self) -> Option<Operator> {
        match self {
            Self::Times => Some(Operator::Multiply),
            Self::Div => Some(Operator::Divide),
            Self::Remainder => Some(Operator::Remainder),
            Self::Plus => Some(Operator::Add),
            Self::Minus => Some(Operator::Subtract),
            Self::LeftShift => Some(Operator::ShiftLeft),
            Self::RightShift => Some(Operator::ShiftRight),
            Self::UnsignedRightShift => Some(Operator::ShiftRightUnsigned),
            Self::Lt => Some(Operator::Lt),
            Self::Gt => Some(Operator::Gt),
            Self::Le => Some(Operator::Le),
            Self::Ge => Some(Operator::Ge),
            Self::In => Some(Operator::In),
            Self::Is => Some(Operator::Is),
            Self::Equals => Some(Operator::Equals),
            Self::NotEquals => Some(Operator::NotEquals),
            Self::StrictEquals => Some(Operator::StrictEquals),
            Self::StrictNotEquals => Some(Operator::StrictNotEquals),
            Self::BitwiseAnd => Some(Operator::BitwiseAnd),
            Self::BitwiseXor => Some(Operator::BitwiseXor),
            Self::BitwiseOr => Some(Operator::BitwiseOr),
            Self::LogicalAnd => Some(Operator::LogicalAnd),
            Self::LogicalXor => Some(Operator::LogicalXor),
            Self::LogicalOr => Some(Operator::LogicalOr),
            Self::NullCoalescing => Some(Operator::NullCoalescing),
            Self::Power => Some(Operator::Power),
            _  => None,
        }
    }
    
    pub(crate) fn to_attribute(&self, location: &Location) -> Option<Attribute> {
        match self {
            Self::Public => Some(Attribute::Public(location.clone())),
            Self::Private => Some(Attribute::Private(location.clone())),
            Self::Protected => Some(Attribute::Protected(location.clone())),
            Self::Internal => Some(Attribute::Internal(location.clone())),
            Self::Identifier(ref name) => {
                Attribute::from_identifier_name(name, &location)
            },
            _ => None,
        }
    }
}