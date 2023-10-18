use std::rc::Rc;
use std::str::FromStr;
use conv::ValueFrom;
use crate::{Source, util::CodePointsReader, IntolerableError, Location, character_validation, Diagnostic, DiagnosticKind, Comment, keywords, NumericRangeError};

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

    XmlLtSlash,
    XmlSlashGt,
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

            Token::XmlLtSlash => "'</'",
            Token::XmlSlashGt => "'/>'",
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

    pub fn next(&mut self, reserved_words: bool) -> Result<(Token, Location), IntolerableError> {
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
        let start = self.current_cursor_location();

        final_result_here
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
                content: self.source.text[location.first_offset()..location.last_offset()].to_owned(),
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
                content: self.source.text[location.first_offset()..(location.last_offset() - 2)].to_owned(),
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

    fn consume_identifier_start(&mut self) -> Result<Option<(char, bool)>, IntolerableError> {
        let ch = self.code_points.peek_or_zero();
        if character_validation::is_identifier_start(ch) {
            return Ok(Some((ch, false)));
        }
        if self.code_points.peek_or_zero() == '\\' {
            self.code_points.next();
            return Ok(Some((self.expect_unicode_escape_sequence()?, true)));
        }
        Ok(None)
    }

    fn consume_identifier_part(&mut self) -> Result<Option<(char, bool)>, IntolerableError> {
        let ch = self.code_points.peek_or_zero();
        if character_validation::is_identifier_part(ch) {
            return Ok(Some((ch, false)));
        }
        if self.code_points.peek_or_zero() == '\\' {
            self.code_points.next();
            return Ok(Some((self.expect_unicode_escape_sequence()?, true)));
        }
        Ok(None)
    }

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
                self.consume_underscore_followed_by_dec_digit();
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
                self.consume_underscore_followed_by_dec_digit();
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
                self.consume_underscore_followed_by_dec_digit();
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
                self.consume_underscore_followed_by_dec_digit();
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
            self.consume_underscore_followed_by_hex_digit();
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
            self.consume_underscore_followed_by_bin_digit();
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
}