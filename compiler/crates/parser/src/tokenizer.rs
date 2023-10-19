use std::rc::Rc;
use std::str::FromStr;
use conv::ValueFrom;
use crate::*;
use crate::util::CodePointsReader;

/// Represents a lexical token.
#[derive(Clone, PartialEq)]
pub enum Token {
    Eof,
    Identifier(String),
    StringLiteral(String),
    NumericLiteral(f64),
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
    /// `=>`
    FatArrow,
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
    Instanceof,
    Interface,
    Internal,
    Is,
    New,
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

    XmlWhitespace,
    XmlLtSlash,
    XmlSlashGt,
    XmlText(String),
    XmlName(String),
    XmlMarkup(String),
    XmlAttributeValue(String),
}

impl ToString for Token {
    /// Converts the token into a readable string. Refer to the source code
    /// of this implementation for the possible return values. Updates to the
    /// parser shall not break existing programs that rely on the return values
    /// returned by this method.
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
            Token::FatArrow => "=>",
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
            Token::Instanceof => "'instanceof'",
            Token::Interface => "'interface'",
            Token::Internal => "'internal'",
            Token::Is => "'is'",
            Token::New => "'new'",
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

pub struct Tokenizer<'input> {
    source: Rc<Source>,
    current_line_number: usize,
    code_points: CodePointsReader<'input>,
}

impl<'input> Tokenizer<'input> {
    /// Constructs a tokenizer. The given `source_text` parameter must be the same
    /// as `&source.text()`.
    pub fn new(source: &Rc<Source>, source_text: &'input str) -> Self {
        let source = Rc::clone(source);
        assert!(!source.already_tokenized.get(), "A Source must only be tokenized once.");
        source.already_tokenized.set(true);
        Self {
            source,
            current_line_number: 1,
            code_points: CodePointsReader::from(source_text),
        }
    }

    /// Scans for an InputElementDiv token. If `reserved_words` is false,
    /// all reserved words are taken as identifiers.
    pub fn scan_ie_div(&mut self, reserved_words: bool) -> Result<(Token, Location), IntolerableError> {
        loop {
            let ch = self.code_points.peek_or_zero();
            if character_validation::is_whitespace(ch) {
                self.code_points.next();
            } else if self.consume_line_terminator() || self.consume_comment()? {
                // Consumed line terminator or comment
            } else {
                break;
            }
        }
        if let Some(result) = self.scan_identifier(reserved_words)? {
            return Ok(result);
        }
        if let Some(result) = self.scan_dot_or_numeric_literal()? {
            return Ok(result);
        }
        if let Some(result) = self.scan_string_literal()? {
            return Ok(result);
        }
        let start = self.current_cursor_location();
        match self.code_points.peek_or_zero() {
            ',' => {
                // Comma
                self.code_points.next();
                let location = start.combine_with(self.current_cursor_location());
                return Ok((Token::Comma, location));
            },
            '(' => {
                // LeftParen
                self.code_points.next();
                let location = start.combine_with(self.current_cursor_location());
                return Ok((Token::LeftParen, location));
            },
            ')' => {
                // RightParen
                self.code_points.next();
                let location = start.combine_with(self.current_cursor_location());
                return Ok((Token::RightParen, location));
            },
            '[' => {
                // LeftBracket
                self.code_points.next();
                let location = start.combine_with(self.current_cursor_location());
                return Ok((Token::LeftBracket, location));
            },
            ']' => {
                // RightBracket
                self.code_points.next();
                let location = start.combine_with(self.current_cursor_location());
                return Ok((Token::RightBracket, location));
            },
            '{' => {
                // LeftBrace
                self.code_points.next();
                let location = start.combine_with(self.current_cursor_location());
                return Ok((Token::LeftBrace, location));
            },
            '}' => {
                // RightBrace
                self.code_points.next();
                let location = start.combine_with(self.current_cursor_location());
                return Ok((Token::RightBrace, location));
            },
            ':' => {
                self.code_points.next();
                // ColonColon
                if self.code_points.peek_or_zero() == ':' {
                    self.code_points.next();
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::ColonColon, location));
                }
                // Colon
                let location = start.combine_with(self.current_cursor_location());
                return Ok((Token::Colon, location));
            },
            '=' => {
                self.code_points.next();
                let ch = self.code_points.peek_or_zero();
                // FatArrow
                if ch == '>' {
                    self.code_points.next();
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::FatArrow, location));
                }
                // StrictEquals
                if ch == '=' && self.code_points.peek_at_or_zero(1) == '=' {
                    self.code_points.skip_count_in_place(2);
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::StrictEquals, location));
                }
                // Equals
                if ch == '=' {
                    self.code_points.next();
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::Equals, location));
                }
                // Assign
                let location = start.combine_with(self.current_cursor_location());
                return Ok((Token::Assign, location));
            },
            '!' => {
                self.code_points.next();
                let ch = self.code_points.peek_or_zero();
                // StrictNotEquals
                if ch == '=' && self.code_points.peek_at_or_zero(1) == '=' {
                    self.code_points.skip_count_in_place(2);
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::StrictNotEquals, location));
                }
                // NotEquals
                if ch == '=' {
                    self.code_points.next();
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::NotEquals, location));
                }
                // Exclamation
                let location = start.combine_with(self.current_cursor_location());
                return Ok((Token::Exclamation, location));
            },
            '?' => {
                self.code_points.next();
                let ch = self.code_points.peek_or_zero();
                // OptionalChaining
                if ch == '.' {
                    self.code_points.next();
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::OptionalChaining, location));
                }
                // NullCoalescingAssign
                if ch == '?' && self.code_points.peek_at_or_zero(1) == '=' {
                    self.code_points.skip_count_in_place(2);
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::NullCoalescingAssign, location));
                }
                // NullCoalescing
                if ch == '?' {
                    self.code_points.next();
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::NullCoalescing, location));
                }
                // Question
                let location = start.combine_with(self.current_cursor_location());
                return Ok((Token::Question, location));
            },
            ';' => {
                // Semicolon
                self.code_points.next();
                let location = start.combine_with(self.current_cursor_location());
                return Ok((Token::Semicolon, location));
            },
            '<' => {
                self.code_points.next();
                let ch = self.code_points.peek_or_zero();
                // Le
                if ch == '=' {
                    self.code_points.next();
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::Le, location));
                }
                // LeftShiftAssign
                if ch == '<' && self.code_points.peek_at_or_zero(1) == '=' {
                    self.code_points.skip_count_in_place(2);
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::LeftShiftAssign, location));
                }
                // LeftShift
                if ch == '<' {
                    self.code_points.next();
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::LeftShift, location));
                }
                // Lt
                let location = start.combine_with(self.current_cursor_location());
                return Ok((Token::Lt, location));
            },
            '>' => {
                self.code_points.next();
                let ch = self.code_points.peek_or_zero();
                // Ge
                if ch == '=' {
                    self.code_points.next();
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::Ge, location));
                }
                // RightShiftAssign
                if ch == '>' && self.code_points.peek_at_or_zero(1) == '=' {
                    self.code_points.skip_count_in_place(2);
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::RightShiftAssign, location));
                }
                // UnsignedRightShiftAssign
                if ch == '>' && self.code_points.peek_seq(3) == ">>=" {
                    self.code_points.skip_count_in_place(3);
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::UnsignedRightShiftAssign, location));
                }
                // UnsignedRightShift
                if ch == '>' && self.code_points.peek_at_or_zero(1) == '>' {
                    self.code_points.skip_count_in_place(2);
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::UnsignedRightShift, location));
                }
                // RightShift
                if ch == '<' {
                    self.code_points.next();
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::RightShift, location));
                }
                // Gt
                let location = start.combine_with(self.current_cursor_location());
                return Ok((Token::Gt, location));
            },
            '@' => {
                // Attribute
                self.code_points.next();
                let location = start.combine_with(self.current_cursor_location());
                return Ok((Token::Attribute, location));
            },
            '+' => {
                self.code_points.next();
                let ch = self.code_points.peek_or_zero();
                // Increment
                if ch == '+' {
                    self.code_points.next();
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::Increment, location));
                }
                // AddAssign
                if ch == '=' {
                    self.code_points.next();
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::AddAssign, location));
                }
                // Plus
                let location = start.combine_with(self.current_cursor_location());
                return Ok((Token::Plus, location));
            },
            '-' => {
                self.code_points.next();
                let ch = self.code_points.peek_or_zero();
                // Decrement
                if ch == '-' {
                    self.code_points.next();
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::Decrement, location));
                }
                // SubtractAssign
                if ch == '=' {
                    self.code_points.next();
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::SubtractAssign, location));
                }
                // Minus
                let location = start.combine_with(self.current_cursor_location());
                return Ok((Token::Minus, location));
            },
            '*' => {
                self.code_points.next();
                let ch = self.code_points.peek_or_zero();
                // PowerAssign
                if ch == '*' && self.code_points.peek_at_or_zero(1) == '=' {
                    self.code_points.skip_count_in_place(2);
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::PowerAssign, location));
                }
                // Power
                if ch == '*' {
                    self.code_points.next();
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::Power, location));
                }
                // MultiplyAssign
                if ch == '=' {
                    self.code_points.next();
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::MultiplyAssign, location));
                }
                // Times
                let location = start.combine_with(self.current_cursor_location());
                return Ok((Token::Times, location));
            },
            '/' => {
                self.code_points.next();
                let ch = self.code_points.peek_or_zero();
                // DivideAssign
                if ch == '=' {
                    self.code_points.next();
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::DivideAssign, location));
                }
                // Div
                let location = start.combine_with(self.current_cursor_location());
                return Ok((Token::Div, location));
            },
            '%' => {
                self.code_points.next();
                let ch = self.code_points.peek_or_zero();
                // RemainderAssign
                if ch == '=' {
                    self.code_points.next();
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::RemainderAssign, location));
                }
                // Remainder
                let location = start.combine_with(self.current_cursor_location());
                return Ok((Token::Remainder, location));
            },
            '&' => {
                self.code_points.next();
                let ch = self.code_points.peek_or_zero();
                // LogicalAndAssign
                if ch == '&' && self.code_points.peek_at_or_zero(1) == '=' {
                    self.code_points.skip_count_in_place(2);
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::LogicalAndAssign, location));
                }
                // LogicalAnd
                if ch == '&' {
                    self.code_points.next();
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::LogicalAnd, location));
                }
                // BitwiseAndAssign
                if ch == '=' {
                    self.code_points.next();
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::BitwiseAndAssign, location));
                }
                // BitwiseAnd
                let location = start.combine_with(self.current_cursor_location());
                return Ok((Token::BitwiseAnd, location));
            },
            '^' => {
                self.code_points.next();
                let ch = self.code_points.peek_or_zero();
                // LogicalXorAssign
                if ch == '^' && self.code_points.peek_at_or_zero(1) == '=' {
                    self.code_points.skip_count_in_place(2);
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::LogicalXorAssign, location));
                }
                // LogicalXor
                if ch == '^' {
                    self.code_points.next();
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::LogicalXor, location));
                }
                // BitwiseXorAssign
                if ch == '=' {
                    self.code_points.next();
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::BitwiseXorAssign, location));
                }
                // BitwiseXor
                let location = start.combine_with(self.current_cursor_location());
                return Ok((Token::BitwiseXor, location));
            },
            '|' => {
                self.code_points.next();
                let ch = self.code_points.peek_or_zero();
                // LogicalOrAssign
                if ch == '|' && self.code_points.peek_at_or_zero(1) == '=' {
                    self.code_points.skip_count_in_place(2);
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::LogicalOrAssign, location));
                }
                // LogicalOr
                if ch == '|' {
                    self.code_points.next();
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::LogicalOr, location));
                }
                // BitwiseOrAssign
                if ch == '=' {
                    self.code_points.next();
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::BitwiseOrAssign, location));
                }
                // BitwiseOr
                let location = start.combine_with(self.current_cursor_location());
                return Ok((Token::BitwiseOr, location));
            },
            '~' => {
                // BitwiseNot
                self.code_points.next();
                let location = start.combine_with(self.current_cursor_location());
                return Ok((Token::BitwiseNot, location));
            },
            _ => {
                if self.code_points.has_remaining() {
                    self.add_unexpected_error();
                    return Err(IntolerableError);
                // Eof
                } else {
                    return Ok((Token::Eof, start))
                }
            },
        }
    }

    /// Scans regular expression after a `/` or `/=` token has been scanned by
    /// `scan_ie_div`.
    pub fn scan_regexp_literal(&mut self, start: Location) -> Result<(Token, Location), IntolerableError> {
        let mut body = String::new();
        loop {
            let ch = self.code_points.peek_or_zero();
            if ch == '/' {
                self.code_points.next();
                break;
            } else if ch == '\\' {
                self.code_points.next();
                body.push('\\');
                let ch = self.code_points.peek_or_zero();
                if self.code_points.reached_end() {
                    self.add_unexpected_error();
                    return Err(IntolerableError);
                } else if character_validation::is_line_terminator(ch) {
                    self.add_unexpected_error();
                }
                self.consume_line_terminator();
                body.push(ch);
            } else if character_validation::is_line_terminator(ch) {
                body.push('\n');
                self.consume_line_terminator();
            } else if self.code_points.reached_end() {
                self.add_unexpected_error();
                return Err(IntolerableError);
            } else {
                body.push(ch);
                self.code_points.next();
            }
        }

        let mut flags = String::new();
        while let Some((ch, _)) = self.consume_identifier_part()? {
            flags.push(ch);
        }
        
        let location = start.combine_with(self.current_cursor_location());
        Ok((Token::RegExpLiteral { body, flags }, location))
    }

    /// Current line number, counted from one.
    pub fn current_line_number(&self) -> usize {
        self.current_line_number
    }

    fn current_character_ahead_location(&self) -> Location {
        let offset = self.code_points.index();
        let mut next_code_points = self.code_points.clone();
        next_code_points.next();
        Location::with_line_and_offsets(&self.source, self.current_line_number, offset, next_code_points.index() + 1)
    }

    fn current_cursor_location(&self) -> Location {
        let offset = self.code_points.index();
        Location::with_line_and_offset(&self.source, self.current_line_number, offset)
    }

    fn add_unexpected_error(&self) {
        if self.code_points.has_remaining() {
            self.source.add_diagnostic(Diagnostic::new_syntax_error(self.current_character_ahead_location(), DiagnosticKind::UnexpectedOrInvalidToken, vec![]))
        } else {
            self.source.add_diagnostic(Diagnostic::new_syntax_error(self.current_cursor_location(), DiagnosticKind::UnexpectedEnd, vec![]))
        }
    }

    // LineTerminator
    fn consume_line_terminator(&mut self) -> bool {
        let ch = self.code_points.peek_or_zero();
        if ch == '\x0D' && self.code_points.peek_at_or_zero(1) == '\x0A' {
            self.code_points.skip_count_in_place(2);
            self.source.line_number_offsets.borrow_mut().push(self.code_points.index());
            self.current_line_number += 1;
            return true;
        }
        if character_validation::is_line_terminator(ch) {
            self.code_points.next();
            self.source.line_number_offsets.borrow_mut().push(self.code_points.index());
            self.current_line_number += 1;
            return true;
        }
        false
    }

    fn consume_comment(&mut self) -> Result<bool, IntolerableError> {
        let ch = self.code_points.peek_or_zero();
        if ch != '/' {
            return Ok(false);
        }
        let ch2 = self.code_points.peek_at_or_zero(1);
        if ch2 == '/' {
            let start = self.current_cursor_location();
            self.code_points.skip_count_in_place(2);
            while !character_validation::is_line_terminator(self.code_points.peek_or_zero()) && self.code_points.has_remaining() {
                self.code_points.skip_in_place();
            }
            let location = start.combine_with(self.current_cursor_location());
            self.consume_line_terminator();

            self.source.comments.borrow_mut().push(Comment {
                multiline: false,
                content: self.source.text[(location.first_offset() + 2)..location.last_offset()].to_owned(),
                location,
            });

            return Ok(true);
        }
        if ch2 == '*' {
            let start = self.current_cursor_location();
            self.code_points.skip_count_in_place(2);

            loop {
                if self.code_points.peek_or_zero() == '*' && self.code_points.peek_at_or_zero(1) == '/' {
                    self.code_points.skip_count_in_place(2);
                    break;
                } else if self.consume_line_terminator() {
                    // Consumed LineTerminator
                } else if self.code_points.has_remaining() {
                    self.code_points.skip_in_place();
                } else {
                    self.add_unexpected_error();
                    return Err(IntolerableError);
                }
            }

            let location = start.combine_with(self.current_cursor_location());

            self.source.comments.borrow_mut().push(Comment {
                multiline: true,
                content: self.source.text[(location.first_offset() + 2)..(location.last_offset() - 2)].to_owned(),
                location,
            });

            return Ok(true);
        }
        Ok(false)
    }

    fn scan_identifier(&mut self, reserved_words: bool) -> Result<Option<(Token, Location)>, IntolerableError> {
        let start = self.current_cursor_location();
        let mut escaped = false;
        let Some((ch, escaped_2)) = self.consume_identifier_start()? else {
            return Ok(None);
        };
        escaped = escaped || escaped_2;
        let mut name = String::new();
        name.push(ch);
        while let Some((ch, escaped_2)) = self.consume_identifier_part()? {
            escaped = escaped || escaped_2;
            name.push(ch);
        }
        let location = start.combine_with(self.current_cursor_location());
        if reserved_words && !escaped {
            if let Some(token) = keywords::reserved_word_token(name.as_ref()) {
                return Ok(Some((token, location)));
            }
        }
        Ok(Some((Token::Identifier(name), location)))
    }

    /// Returns a tuple in the form (*character*, *escaped*).
    fn consume_identifier_start(&mut self) -> Result<Option<(char, bool)>, IntolerableError> {
        let ch = self.code_points.peek_or_zero();
        if character_validation::is_identifier_start(ch) {
            self.code_points.next();
            return Ok(Some((ch, false)));
        }
        if self.code_points.peek_or_zero() == '\\' {
            self.code_points.next();
            return Ok(Some((self.expect_unicode_escape_sequence()?, true)));
        }
        Ok(None)
    }

    /// Returns a tuple in the form (*character*, *escaped*).
    fn consume_identifier_part(&mut self) -> Result<Option<(char, bool)>, IntolerableError> {
        let ch = self.code_points.peek_or_zero();
        if character_validation::is_identifier_part(ch) {
            self.code_points.next();
            return Ok(Some((ch, false)));
        }
        if self.code_points.peek_or_zero() == '\\' {
            self.code_points.next();
            return Ok(Some((self.expect_unicode_escape_sequence()?, true)));
        }
        Ok(None)
    }

    /// Expects UnicodeEscapeSequence starting from `u`.
    fn expect_unicode_escape_sequence(&mut self) -> Result<char, IntolerableError> {
        let start = self.current_cursor_location();
        if self.code_points.peek_or_zero() != 'u' {
            self.add_unexpected_error();
            return Err(IntolerableError);
        }
        self.code_points.next();

        // Scan \uXXXX
        if character_validation::is_hex_digit(self.code_points.peek_or_zero()) {
            let r = char::from_u32(self.expect_hex_digit()? << 12
                | (self.expect_hex_digit()? << 8)
                | (self.expect_hex_digit()? << 4)
                | self.expect_hex_digit()?);
            let Some(r) = r else {
                self.source.add_diagnostic(Diagnostic::new_syntax_error(start.combine_with(self.current_cursor_location()), DiagnosticKind::UnexpectedOrInvalidToken, vec![]));
                return Err(IntolerableError);
            };
            return Ok(r);
        }

        // Scan \u{}
        if self.code_points.peek_or_zero() != '{' {
            self.add_unexpected_error();
            return Err(IntolerableError);
        }
        self.code_points.next();
        while character_validation::is_hex_digit(self.code_points.peek_or_zero()) {
            self.code_points.next();
        }
        if self.code_points.peek_or_zero() != '}' {
            self.add_unexpected_error();
            return Err(IntolerableError);
        }
        self.code_points.next();
        let location = start.combine_with(self.current_cursor_location());
        let r = u32::from_str_radix(&self.source.text[(start.first_offset + 2)..(location.last_offset - 1)], 16);
        let Ok(r) = r else {
            self.source.add_diagnostic(Diagnostic::new_syntax_error(location, DiagnosticKind::UnexpectedOrInvalidToken, vec![]));
            return Err(IntolerableError);
        };
        let r = char::from_u32(r);
        let Some(r) = r else {
            self.source.add_diagnostic(Diagnostic::new_syntax_error(location, DiagnosticKind::UnexpectedOrInvalidToken, vec![]));
            return Err(IntolerableError);
        };
        Ok(r)
    }

    fn expect_hex_digit(&mut self) -> Result<u32, IntolerableError> {
        let ch = self.code_points.peek_or_zero();
        if !character_validation::is_hex_digit(ch) {
            self.add_unexpected_error();
            return Err(IntolerableError);
        }
        self.code_points.next();
        Ok(ch as u32)
    }

    fn scan_dot_or_numeric_literal(&mut self) -> Result<Option<(Token, Location)>, IntolerableError> {
        let start = self.current_cursor_location();
        let ch = self.code_points.peek_or_zero();
        let mut initial_dot = false;
        if ch == '.' {
            initial_dot = true;
            self.code_points.next();

            let seq = self.code_points.peek_seq(2);
            // Ellipsis
            if seq == ".." {
                self.code_points.skip_count_in_place(2);
                return Ok(Some((Token::Ellipsis, start.combine_with(self.current_cursor_location()))));
            }
            let ch = seq.get(..1).map(|ch| ch.chars().next().unwrap()).unwrap_or('\x00');
            // Descendants
            if ch == '.' {
                self.code_points.next();
                return Ok(Some((Token::Descendants, start.combine_with(self.current_cursor_location()))));
            }
            // Dot
            if !character_validation::is_dec_digit(ch) {
                return Ok(Some((Token::Dot, start.combine_with(self.current_cursor_location()))));
            }

            // NumericLiteral
            while character_validation::is_dec_digit(self.code_points.peek_or_zero()) {
                self.code_points.next();
                self.consume_underscore_followed_by_dec_digit()?;
            }
        } else if ch == '0' {
            self.code_points.next();
            let ch_2 = self.code_points.peek_or_zero();

            // HexLiteral
            if ['X', 'x'].contains(&ch_2) {
                self.code_points.next();
                return self.scan_hex_literal(start.clone());
            }

            // BinLiteral;
            if ['B', 'b'].contains(&ch_2) {
                self.code_points.next();
                return self.scan_bin_literal(start.clone());
            }
        } else if character_validation::is_dec_digit(ch) {
            self.code_points.next();
            while character_validation::is_dec_digit(self.code_points.peek_or_zero()) {
                self.code_points.next();
                self.consume_underscore_followed_by_dec_digit()?;
            }
        } else {
            return Ok(None);
        }

        if !initial_dot && self.code_points.peek_or_zero() == '.' {
            self.code_points.next();
            if !character_validation::is_dec_digit(self.code_points.peek_or_zero()) {
                self.add_unexpected_error();
                return Err(IntolerableError);
            }
            while character_validation::is_dec_digit(self.code_points.peek_or_zero()) {
                self.code_points.next();
                self.consume_underscore_followed_by_dec_digit()?;
            }
        }

        // Decimal exponent
        if ['E', 'e'].contains(&self.code_points.peek_or_zero()) {
            self.code_points.next();
            if ['+', '-'].contains(&self.code_points.peek_or_zero()) {
                self.code_points.next();
            }
            if !character_validation::is_dec_digit(self.code_points.peek_or_zero()) {
                self.add_unexpected_error();
                return Err(IntolerableError);
            }
            while character_validation::is_dec_digit(self.code_points.peek_or_zero()) {
                self.code_points.next();
                self.consume_underscore_followed_by_dec_digit()?;
            }
        }

        self.unallow_numeric_suffix();

        let location = start.combine_with(self.current_cursor_location());
        let string = self.source.text[location.first_offset..location.last_offset].to_owned().replace('_', "");

        let Ok(v) = f64::from_str(&string) else {
            self.source.add_diagnostic(Diagnostic::new_syntax_error(location, DiagnosticKind::FailedProcessingNumericLiteral, vec![]));
            return Err(IntolerableError);
        };

        Ok(Some((Token::NumericLiteral(v), location)))
    }

    fn scan_hex_literal(&mut self, start: Location) -> Result<Option<(Token, Location)>, IntolerableError> {
        if !character_validation::is_hex_digit(self.code_points.peek_or_zero()) {
            self.add_unexpected_error();
            return Err(IntolerableError);
        }
        while character_validation::is_hex_digit(self.code_points.peek_or_zero()) {
            self.code_points.next();
            self.consume_underscore_followed_by_hex_digit()?;
        }

        self.unallow_numeric_suffix();

        let location = start.combine_with(self.current_cursor_location());

        let s = self.source.text[(location.first_offset + 2)..location.last_offset].replace('_', "");
        let n = u64::from_str_radix(&s, 16);
        let n = n.map_err(|_| NumericRangeError)
            .and_then(|n| f64::value_from(n).map_err(|_| NumericRangeError));

        let Ok(n) = n else {
            self.source.add_diagnostic(Diagnostic::new_syntax_error(location, DiagnosticKind::FailedProcessingNumericLiteral, vec![]));
            return Err(IntolerableError);
        };

        Ok(Some((Token::NumericLiteral(n), location)))
    }

    fn scan_bin_literal(&mut self, start: Location) -> Result<Option<(Token, Location)>, IntolerableError> {
        if !character_validation::is_bin_digit(self.code_points.peek_or_zero()) {
            self.add_unexpected_error();
            return Err(IntolerableError);
        }
        while character_validation::is_bin_digit(self.code_points.peek_or_zero()) {
            self.code_points.next();
            self.consume_underscore_followed_by_bin_digit()?;
        }

        self.unallow_numeric_suffix();

        let location = start.combine_with(self.current_cursor_location());

        let s = self.source.text[(location.first_offset + 2)..location.last_offset].replace('_', "");
        let n = u64::from_str_radix(&s, 2);
        let n = n.map_err(|_| NumericRangeError)
            .and_then(|n| f64::value_from(n).map_err(|_| NumericRangeError));

        let Ok(n) = n else {
            self.source.add_diagnostic(Diagnostic::new_syntax_error(location, DiagnosticKind::FailedProcessingNumericLiteral, vec![]));
            return Err(IntolerableError);
        };

        Ok(Some((Token::NumericLiteral(n), location)))
    }

    fn consume_underscore_followed_by_dec_digit(&mut self) -> Result<(), IntolerableError> {
        if self.code_points.peek_or_zero() == '_' {
            self.code_points.next();
            if !character_validation::is_dec_digit(self.code_points.peek_or_zero()) {
                self.add_unexpected_error();
                return Err(IntolerableError);
            }
            self.code_points.next();
        }
        Ok(())
    }

    fn consume_underscore_followed_by_hex_digit(&mut self) -> Result<(), IntolerableError> {
        if self.code_points.peek_or_zero() == '_' {
            self.code_points.next();
            if !character_validation::is_hex_digit(self.code_points.peek_or_zero()) {
                self.add_unexpected_error();
                return Err(IntolerableError);
            }
            self.code_points.next();
        }
        Ok(())
    }

    fn consume_underscore_followed_by_bin_digit(&mut self) -> Result<(), IntolerableError> {
        if self.code_points.peek_or_zero() == '_' {
            self.code_points.next();
            if !character_validation::is_bin_digit(self.code_points.peek_or_zero()) {
                self.add_unexpected_error();
                return Err(IntolerableError);
            }
            self.code_points.next();
        }
        Ok(())
    }

    fn unallow_numeric_suffix(&self) {
        if character_validation::is_identifier_start(self.code_points.peek_or_zero()) {
            self.add_unexpected_error();
        }
    }

    fn scan_string_literal(&mut self) -> Result<Option<(Token, Location)>, IntolerableError> {
        let delim = self.code_points.peek_or_zero();
        if !['"', '\''].contains(&delim) {
            return Ok(None);
        }
        let start = self.current_cursor_location();
        self.code_points.next();

        // Triple string literal
        if self.code_points.peek_or_zero() == delim && self.code_points.peek_at_or_zero(1) == delim {
            self.code_points.skip_count_in_place(2);
            return self.scan_triple_string_literal(delim, start);
        }

        let mut builder = String::new();

        loop {
            if let Some(s) = self.consume_escape_sequence()? {
                builder.push_str(&s);
            } else {
                let ch = self.code_points.peek_or_zero();
                if ch == delim {
                    self.code_points.next();
                    break;
                } else if character_validation::is_line_terminator(ch) {
                    self.source.add_diagnostic(Diagnostic::new_syntax_error(self.current_character_ahead_location(), DiagnosticKind::UnallowedLineBreak, vec![]));
                    self.consume_line_terminator();
                } else if !self.code_points.has_remaining() {
                    self.add_unexpected_error();
                    return Err(IntolerableError);
                } else {
                    builder.push(ch);
                    self.code_points.next();
                }
            }
        }

        let location = start.combine_with(self.current_cursor_location());
        let value = self.source.text[(location.first_offset + 1)..(location.last_offset - 1)].to_owned();

        Ok(Some((Token::StringLiteral(value), location)))
    }

    fn scan_triple_string_literal(&mut self, delim: char, start: Location) -> Result<Option<(Token, Location)>, IntolerableError> {
        let mut lines: Vec<String> = vec![];
        let mut builder = String::new();

        let initial_line_break = self.consume_line_terminator();

        loop {
            if let Some(s) = self.consume_escape_sequence()? {
                builder.push_str(&s);
            } else {
                let ch = self.code_points.peek_or_zero();
                if ch == delim && self.code_points.peek_at_or_zero(1) == delim && self.code_points.peek_at_or_zero(2) == delim {
                    self.code_points.skip_count_in_place(3);
                    lines.push(builder.clone());
                    break;
                } else if character_validation::is_line_terminator(ch) {
                    lines.push(builder.clone());
                    builder.clear();
                    self.consume_line_terminator();
                } else if !self.code_points.has_remaining() {
                    self.add_unexpected_error();
                    return Err(IntolerableError);
                } else {
                    builder.push(ch);
                    self.code_points.next();
                }
            }
        }

        let location = start.combine_with(self.current_cursor_location());
        let last_line = if initial_line_break && lines.len() > 1 {
            lines.pop().unwrap()
        } else {
            "".to_owned()
        };

        let base_indent = character_validation::indent_count(&last_line);

        let mut lines: Vec<String> = lines.iter().map(|line| {
            let indent = character_validation::indent_count(line);
            line[usize::min(base_indent, indent)..].to_owned()
        }).collect();

        let last_line = last_line[base_indent..].to_owned();
        if !last_line.is_empty() {
            lines.push(last_line);
        }

        let value = lines.join("\n");
        Ok(Some((Token::StringLiteral(value), location)))
    }

    fn consume_escape_sequence(&mut self) -> Result<Option<String>, IntolerableError> {
        if self.code_points.peek_or_zero() != '\\' {
            return Ok(None);
        }
        self.code_points.next();
        if !self.code_points.has_remaining() {
            self.add_unexpected_error();
            return Err(IntolerableError);
        }
        if self.consume_line_terminator() {
            return Ok(Some("".into()));
        }
        let ch = self.code_points.peek_or_zero();
        match ch {
            '\'' | '"' | '\\' => {
                self.code_points.next();
                Ok(Some(ch.into()))
            },
            'u' => {
                Ok(Some(self.expect_unicode_escape_sequence()?.into()))
            },
            'x' => {
                self.code_points.next();
                let v = (self.expect_hex_digit()? << 4) | self.expect_hex_digit()?;
                let v = char::from_u32(v).unwrap();
                Ok(Some(v.into()))
            },
            'b' => {
                self.code_points.next();
                Ok(Some('\x08'.into()))
            },
            'f' => {
                self.code_points.next();
                Ok(Some('\x0C'.into()))
            },
            'n' => {
                self.code_points.next();
                Ok(Some('\x0A'.into()))
            },
            'r' => {
                self.code_points.next();
                Ok(Some('\x0D'.into()))
            },
            't' => {
                self.code_points.next();
                Ok(Some('\x09'.into()))
            },
            'v' => {
                self.code_points.next();
                Ok(Some('\x0B'.into()))
            },
            '0' => {
                self.code_points.next();
                if character_validation::is_dec_digit(self.code_points.peek_or_zero()) {
                    self.add_unexpected_error();
                }
                Ok(Some('\x00'.into()))
            },
            ch => {
                if character_validation::is_dec_digit(ch) {
                    self.add_unexpected_error();
                }
                self.code_points.next();
                Ok(Some(ch.into()))
            },
        }
    }

    /// Scans for an InputElementXMLTag token.
    pub fn scan_ie_xml_tag(&mut self) -> Result<(Token, Location), IntolerableError> {
        let start = self.current_cursor_location();
        let ch = self.code_points.peek_or_zero();

        // XmlName
        if character_validation::is_xml_name_start(ch) {
            self.code_points.next();
            while character_validation::is_xml_name_part(self.code_points.peek_or_zero()) {
                self.code_points.next();
            }
            let location = start.combine_with(self.current_cursor_location());
            let name = self.source.text[location.first_offset..location.last_offset].to_owned();
            return Ok((Token::XmlName(name), location));
        }

        // XmlWhitespace
        if character_validation::is_xml_whitespace(ch) {
            while character_validation::is_xml_whitespace(self.code_points.peek_or_zero()) {
                if !self.consume_line_terminator() {
                    self.code_points.next();
                }
            }
            let location = start.combine_with(self.current_cursor_location());
            return Ok((Token::XmlWhitespace, location));
        }

        match ch {
            // Assign
            '=' => {
                self.code_points.next();
                let location = start.combine_with(self.current_cursor_location());
                Ok((Token::Assign, location))
            },

            // Gt
            '>' => {
                self.code_points.next();
                let location = start.combine_with(self.current_cursor_location());
                Ok((Token::Gt, location))
            },

            // XmlSlashGt
            '/' => {
                self.code_points.next();
                if self.code_points.peek_or_zero() != '>' {
                    self.add_unexpected_error();
                    return Err(IntolerableError);
                }
                self.code_points.next();
                let location = start.combine_with(self.current_cursor_location());
                Ok((Token::XmlSlashGt, location))
            },

            // XmlAttributeValue
            '"' | '\'' => {
                let delim = ch;
                self.code_points.next();
                while self.code_points.peek_or_zero() != delim && self.code_points.has_remaining() {
                    self.code_points.next();
                }
                if self.code_points.reached_end() {
                    self.add_unexpected_error();
                    return Err(IntolerableError)
                }
                let value = self.source.text[(start.first_offset + 1)..self.current_cursor_location().first_offset].to_owned();
                self.code_points.next();
                
                let location = start.combine_with(self.current_cursor_location());
                Ok((Token::XmlAttributeValue(value), location))
            },

            // LeftBrace
            '{' => {
                self.code_points.next();
                let location = start.combine_with(self.current_cursor_location());
                Ok((Token::LeftBrace, location))
            },

            _ => {
                self.add_unexpected_error();
                Err(IntolerableError)
            },
        }
    }

    /// Scans for an InputElementXMLContent token.
    pub fn scan_ie_xml_content(&mut self) -> Result<(Token, Location), IntolerableError> {
        let start = self.current_cursor_location();
        let ch = self.code_points.peek_or_zero();

        match ch {
            '<' => {
                self.code_points.next();

                // XmlMarkup
                if let Some(r) = self.scan_xml_markup(start.clone())? {
                    return Ok(r);
                }

                // XmlLtSlash
                if self.code_points.peek_or_zero() == '/' {
                    self.code_points.next();
                    let location = start.combine_with(self.current_cursor_location());
                    return Ok((Token::XmlLtSlash, location));
                }

                // Lt
                let location = start.combine_with(self.current_cursor_location());
                Ok((Token::Lt, location))
            },
            
            // LeftBrace
            '{' => {
                self.code_points.next();
                let location = start.combine_with(self.current_cursor_location());
                Ok((Token::LeftBrace, location))
            },

            // XmlName
            _ => {
                loop {
                    let ch = self.code_points.peek_or_zero();
                    if ['<', '{'].contains(&ch) {
                        break;
                    }
                    if character_validation::is_line_terminator(ch) {
                        self.consume_line_terminator();
                    } else if self.code_points.reached_end() {
                        self.add_unexpected_error();
                        return Err(IntolerableError);
                    } else {
                        self.code_points.next();
                    }
                }

                let location = start.combine_with(self.current_cursor_location());
                let content = self.source.text[location.first_offset..location.last_offset].to_owned();
                Ok((Token::XmlText(content), location))
            },
        }
    }

    /// Attempts to scan a XMLMarkup token after a `<` character.
    pub fn scan_xml_markup(&mut self, start: Location) -> Result<Option<(Token, Location)>, IntolerableError> {
        // XMLComment
        if self.code_points.peek_seq(2) == "!--" {
            self.code_points.skip_count_in_place(3);
            loop {
                if self.code_points.peek_or_zero() == '-' && self.code_points.peek_seq(3) == "-->" {
                    self.code_points.skip_count_in_place(3);
                    break;
                } else if character_validation::is_line_terminator(self.code_points.peek_or_zero()) {
                    self.consume_line_terminator();
                } else if self.code_points.reached_end() {
                    self.add_unexpected_error();
                    return Err(IntolerableError);
                } else {
                    self.code_points.next();
                }
            }

            let location = start.combine_with(self.current_cursor_location());
            let content = self.source.text[location.first_offset..location.last_offset].to_owned();

            return Ok(Some((Token::XmlMarkup(content), location)));
        }

        // XMLCDATA
        if self.code_points.peek_seq(8) == "![CDATA[" {
            self.code_points.skip_count_in_place(8);
            loop {
                if self.code_points.peek_or_zero() == ']' && self.code_points.peek_seq(3) == "]]>" {
                    self.code_points.skip_count_in_place(3);
                    break;
                } else if character_validation::is_line_terminator(self.code_points.peek_or_zero()) {
                    self.consume_line_terminator();
                } else if self.code_points.reached_end() {
                    self.add_unexpected_error();
                    return Err(IntolerableError);
                } else {
                    self.code_points.next();
                }
            }

            let location = start.combine_with(self.current_cursor_location());
            let content = self.source.text[location.first_offset..location.last_offset].to_owned();

            return Ok(Some((Token::XmlMarkup(content), location)));
        }

        // XMLPI
        if self.code_points.peek_or_zero() == '?' {
            self.code_points.next();
            loop {
                if self.code_points.peek_or_zero() == '?' && self.code_points.peek_at_or_zero(1) == '>' {
                    self.code_points.skip_count_in_place(2);
                    break;
                } else if character_validation::is_line_terminator(self.code_points.peek_or_zero()) {
                    self.consume_line_terminator();
                } else if self.code_points.reached_end() {
                    self.add_unexpected_error();
                    return Err(IntolerableError);
                } else {
                    self.code_points.next();
                }
            }

            let location = start.combine_with(self.current_cursor_location());
            let content = self.source.text[location.first_offset..location.last_offset].to_owned();

            return Ok(Some((Token::XmlMarkup(content), location)));
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_n_per_n() {
        let _n = "n".to_owned();
        let source = Source::new(None, "n * n".into(), &CompilerOptions::new());
        let mut tokenizer = Tokenizer::new(&source, &source.text());
        assert!(matches!(tokenizer.scan_ie_div(true), Ok((Token::Identifier(_n), _))));
        assert!(matches!(tokenizer.scan_ie_div(true), Ok((Token::Times, _))));
        assert!(matches!(tokenizer.scan_ie_div(true), Ok((Token::Identifier(_n), _))));
    }

    #[test]
    fn tokenize_comments() {
        let _n = "n".to_owned();
        let source = Source::new(None, "
            // Single-line comment
            /* Multi-line comment */
        ".into(), &CompilerOptions::new());
        let mut tokenizer = Tokenizer::new(&source, &source.text());
        assert!(matches!(tokenizer.scan_ie_div(true), Ok((Token::Eof, _))));
        assert_eq!(source.comments().borrow()[0].content(), " Single-line comment");
        assert_eq!(source.comments().borrow()[1].content(), " Multi-line comment ");
    }

    #[test]
    fn tokenize_strings() {
        let _string1 = "Some content".to_owned();
        let _string2 = "Another\ncontent".to_owned();
        let source = Source::new(None, r###"
            "Some content"
            """
            Another
            content
            """
        "###.into(), &CompilerOptions::new());
        let mut tokenizer = Tokenizer::new(&source, &source.text());
        assert!(matches!(tokenizer.scan_ie_div(true), Ok((Token::StringLiteral(_string1), _))));
        assert!(matches!(tokenizer.scan_ie_div(true), Ok((Token::StringLiteral(_string2), _))));
    }
}