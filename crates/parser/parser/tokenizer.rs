use crate::ns::*;

pub struct Tokenizer<'input> {
    compilation_unit: Rc<CompilationUnit>,
    characters: CharacterReader<'input>,
}

impl<'input> Tokenizer<'input> {
    /// Constructs a tokenizer.
    pub fn new(compilation_unit: &'input Rc<CompilationUnit>, options: &ParserOptions) -> Self {
        let text: &'input str = compilation_unit.text();
        let compilation_unit = compilation_unit.clone();
        let characters: CharacterReader<'input>;
        if let Some(range) = options.byte_range {
            characters = CharacterReader::from_offset(&text[0..range.1], range.0);
        } else {
            characters = CharacterReader::from(text);
        }
        Self {
            compilation_unit,
            characters,
        }
    }

    pub fn compilation_unit(&self) -> &Rc<CompilationUnit> {
        &self.compilation_unit
    }

    pub fn characters(&self) -> &CharacterReader<'input> {
        &self.characters
    }

    fn add_syntax_error(&self, location: &Location, kind: DiagnosticKind, arguments: Vec<Rc<dyn DiagnosticArgument>>) {
        if self.compilation_unit.prevent_equal_offset_error(location) {
            return;
        }
        self.compilation_unit.add_diagnostic(Diagnostic::new_syntax_error(location, kind, arguments));
    }

    /// Scans for an *InputElementDiv* token.
    pub fn scan_ie_div(&mut self) -> (Token, Location) {
        loop {
            let ch = self.characters.peek_or_zero();
            if CharacterValidator::is_whitespace(ch) {
                self.characters.next();
            } else if self.consume_line_terminator() || self.consume_comment() {
                // Consumed line terminator or comment
            } else {
                break;
            }
        }
        if let Some(result) = self.scan_identifier() {
            return result;
        }
        if let Some(result) = self.scan_dot_or_numeric_literal() {
            return result;
        }
        if let Some(result) = self.scan_string_literal(false) {
            return result;
        }
        let start = self.cursor_location();
        match self.characters.peek_or_zero() {
            ',' => {
                // Comma
                self.characters.next();
                let location = start.combine_with(self.cursor_location());
                return (Token::Comma, location);
            },
            '(' => {
                // ParenOpen
                self.characters.next();
                let location = start.combine_with(self.cursor_location());
                return (Token::ParenOpen, location);
            },
            ')' => {
                // ParenClose
                self.characters.next();
                let location = start.combine_with(self.cursor_location());
                return (Token::ParenClose, location);
            },
            '[' => {
                // SquareOpen
                self.characters.next();
                let location = start.combine_with(self.cursor_location());
                return (Token::SquareOpen, location);
            },
            ']' => {
                // SquareClose
                self.characters.next();
                let location = start.combine_with(self.cursor_location());
                return (Token::SquareClose, location);
            },
            '{' => {
                // BlockOpen
                self.characters.next();
                let location = start.combine_with(self.cursor_location());
                return (Token::BlockOpen, location);
            },
            '}' => {
                // BlockClose
                self.characters.next();
                let location = start.combine_with(self.cursor_location());
                return (Token::BlockClose, location);
            },
            ':' => {
                self.characters.next();
                // ColonColon
                if self.characters.peek_or_zero() == ':' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return (Token::ColonColon, location);
                }
                // Colon
                let location = start.combine_with(self.cursor_location());
                return (Token::Colon, location);
            },
            '=' => {
                self.characters.next();
                let ch = self.characters.peek_or_zero();
                // StrictEquals
                if ch == '=' && self.characters.peek_at_or_zero(1) == '=' {
                    self.characters.skip_count_in_place(2);
                    let location = start.combine_with(self.cursor_location());
                    return (Token::StrictEquals, location);
                }
                // Equals
                if ch == '=' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return (Token::Equals, location);
                }
                // Assign
                let location = start.combine_with(self.cursor_location());
                return (Token::Assign, location);
            },
            '!' => {
                self.characters.next();
                let ch = self.characters.peek_or_zero();
                // StrictNotEquals
                if ch == '=' && self.characters.peek_at_or_zero(1) == '=' {
                    self.characters.skip_count_in_place(2);
                    let location = start.combine_with(self.cursor_location());
                    return (Token::StrictNotEquals, location);
                }
                // NotEquals
                if ch == '=' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return (Token::NotEquals, location);
                }
                // Exclamation
                let location = start.combine_with(self.cursor_location());
                return (Token::Exclamation, location);
            },
            '?' => {
                self.characters.next();
                let ch = self.characters.peek_or_zero();
                // OptionalChaining
                if ch == '.' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return (Token::OptionalChaining, location);
                }
                // NullCoalescingAssign
                if ch == '?' && self.characters.peek_at_or_zero(1) == '=' {
                    self.characters.skip_count_in_place(2);
                    let location = start.combine_with(self.cursor_location());
                    return (Token::NullCoalescingAssign, location);
                }
                // NullCoalescing
                if ch == '?' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return (Token::NullCoalescing, location);
                }
                // Question
                let location = start.combine_with(self.cursor_location());
                return (Token::Question, location);
            },
            ';' => {
                // Semicolon
                self.characters.next();
                let location = start.combine_with(self.cursor_location());
                return (Token::Semicolon, location);
            },
            '<' => {
                self.characters.next();
                let ch = self.characters.peek_or_zero();
                // Le
                if ch == '=' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return (Token::Le, location);
                }
                // LeftShiftAssign
                if ch == '<' && self.characters.peek_at_or_zero(1) == '=' {
                    self.characters.skip_count_in_place(2);
                    let location = start.combine_with(self.cursor_location());
                    return (Token::LeftShiftAssign, location);
                }
                // LeftShift
                if ch == '<' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return (Token::LeftShift, location);
                }
                // Lt
                let location = start.combine_with(self.cursor_location());
                return (Token::Lt, location);
            },
            '>' => {
                self.characters.next();
                let ch = self.characters.peek_or_zero();
                // Ge
                if ch == '=' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return (Token::Ge, location);
                }
                // RightShiftAssign
                if ch == '>' && self.characters.peek_at_or_zero(1) == '=' {
                    self.characters.skip_count_in_place(2);
                    let location = start.combine_with(self.cursor_location());
                    return (Token::RightShiftAssign, location);
                }
                // UnsignedRightShiftAssign
                if ch == '>' && self.characters.peek_seq(3) == ">>=" {
                    self.characters.skip_count_in_place(3);
                    let location = start.combine_with(self.cursor_location());
                    return (Token::UnsignedRightShiftAssign, location);
                }
                // UnsignedRightShift
                if ch == '>' && self.characters.peek_at_or_zero(1) == '>' {
                    self.characters.skip_count_in_place(2);
                    let location = start.combine_with(self.cursor_location());
                    return (Token::UnsignedRightShift, location);
                }
                // RightShift
                if ch == '>' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return (Token::RightShift, location);
                }
                // Gt
                let location = start.combine_with(self.cursor_location());
                return (Token::Gt, location);
            },
            '@' => {
                // Attribute
                self.characters.next();
                if let Some(token) = self.scan_string_literal(true) {
                    return token;
                }
                let location = start.combine_with(self.cursor_location());
                return (Token::Attribute, location);
            },
            '+' => {
                self.characters.next();
                let ch = self.characters.peek_or_zero();
                // Increment
                if ch == '+' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return (Token::Increment, location);
                }
                // AddAssign
                if ch == '=' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return (Token::AddAssign, location);
                }
                // Plus
                let location = start.combine_with(self.cursor_location());
                return (Token::Plus, location);
            },
            '-' => {
                self.characters.next();
                let ch = self.characters.peek_or_zero();
                // Decrement
                if ch == '-' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return (Token::Decrement, location);
                }
                // SubtractAssign
                if ch == '=' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return (Token::SubtractAssign, location);
                }
                // Minus
                let location = start.combine_with(self.cursor_location());
                return (Token::Minus, location);
            },
            '*' => {
                self.characters.next();
                let ch = self.characters.peek_or_zero();
                // PowerAssign
                if ch == '*' && self.characters.peek_at_or_zero(1) == '=' {
                    self.characters.skip_count_in_place(2);
                    let location = start.combine_with(self.cursor_location());
                    return (Token::PowerAssign, location);
                }
                // Power
                if ch == '*' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return (Token::Power, location);
                }
                // MultiplyAssign
                if ch == '=' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return (Token::MultiplyAssign, location);
                }
                // Times
                let location = start.combine_with(self.cursor_location());
                return (Token::Times, location);
            },
            '/' => {
                self.characters.next();
                let ch = self.characters.peek_or_zero();
                // DivideAssign
                if ch == '=' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return (Token::DivideAssign, location);
                }
                // Div
                let location = start.combine_with(self.cursor_location());
                return (Token::Div, location);
            },
            '%' => {
                self.characters.next();
                let ch = self.characters.peek_or_zero();
                // RemainderAssign
                if ch == '=' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return (Token::RemainderAssign, location);
                }
                // Percent
                let location = start.combine_with(self.cursor_location());
                return (Token::Percent, location);
            },
            '&' => {
                self.characters.next();
                let ch = self.characters.peek_or_zero();
                // LogicalAndAssign
                if ch == '&' && self.characters.peek_at_or_zero(1) == '=' {
                    self.characters.skip_count_in_place(2);
                    let location = start.combine_with(self.cursor_location());
                    return (Token::LogicalAndAssign, location);
                }
                // LogicalAnd
                if ch == '&' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return (Token::LogicalAnd, location);
                }
                // BitwiseAndAssign
                if ch == '=' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return (Token::BitwiseAndAssign, location);
                }
                // BitwiseAnd
                let location = start.combine_with(self.cursor_location());
                return (Token::Ampersand, location);
            },
            '^' => {
                self.characters.next();
                let ch = self.characters.peek_or_zero();
                // LogicalXorAssign
                if ch == '^' && self.characters.peek_at_or_zero(1) == '=' {
                    self.characters.skip_count_in_place(2);
                    let location = start.combine_with(self.cursor_location());
                    return (Token::LogicalXorAssign, location);
                }
                // LogicalXor
                if ch == '^' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return (Token::LogicalXor, location);
                }
                // BitwiseXorAssign
                if ch == '=' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return (Token::BitwiseXorAssign, location);
                }
                // BitwiseXor
                let location = start.combine_with(self.cursor_location());
                return (Token::Hat, location);
            },
            '|' => {
                self.characters.next();
                let ch = self.characters.peek_or_zero();
                // LogicalOrAssign
                if ch == '|' && self.characters.peek_at_or_zero(1) == '=' {
                    self.characters.skip_count_in_place(2);
                    let location = start.combine_with(self.cursor_location());
                    return (Token::LogicalOrAssign, location);
                }
                // LogicalOr
                if ch == '|' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return (Token::LogicalOr, location);
                }
                // BitwiseOrAssign
                if ch == '=' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return (Token::BitwiseOrAssign, location);
                }
                // BitwiseOr
                let location = start.combine_with(self.cursor_location());
                return (Token::Pipe, location);
            },
            '~' => {
                // BitwiseNot
                self.characters.next();
                let location = start.combine_with(self.cursor_location());
                return (Token::Tilde, location);
            },
            _ => {
                if self.characters.has_remaining() {
                    self.add_unexpected_error();
                    self.characters.next();
                    return self.scan_ie_div();
                // Eof
                } else {
                    return (Token::Eof, start)
                }
            },
        }
    }

    /// Scans regular expression after a `/` or `/=` token has been scanned by
    /// `scan_ie_div`.
    pub fn scan_regexp_literal(&mut self, start: Location, mut body: String) -> (Token, Location) {
        loop {
            let ch = self.characters.peek_or_zero();
            if ch == '/' {
                self.characters.next();
                break;
            } else if ch == '\\' {
                self.characters.next();
                body.push('\\');
                let ch = self.characters.peek_or_zero();
                if self.characters.reached_end() {
                    self.add_unexpected_eof_error(DiagnosticKind::InputEndedBeforeReachingClosingSlashForRegExp);
                    break;
                } else if CharacterValidator::is_line_terminator(ch) {
                    self.add_unexpected_error();
                    self.consume_line_terminator();
                } else {
                    self.characters.next();
                    body.push(ch);
                }
            } else if CharacterValidator::is_line_terminator(ch) {
                body.push('\n');
                self.consume_line_terminator();
            } else if self.characters.reached_end() {
                self.add_unexpected_eof_error(DiagnosticKind::InputEndedBeforeReachingClosingSlashForRegExp);
                break;
            } else {
                body.push(ch);
                self.characters.next();
            }
        }

        let mut flags = String::new();
        while let Some((ch, _)) = self.consume_identifier_part() {
            flags.push(ch);
        }
        
        let location = start.combine_with(self.cursor_location());
        (Token::RegExp { body, flags }, location)
    }

    fn character_ahead_location(&self) -> Location {
        if self.characters.reached_end() {
            return self.cursor_location();
        }
        let offset = self.characters.index();
        let mut next_characters = self.characters.clone();
        next_characters.next().unwrap();
        Location::with_offsets(&self.compilation_unit, offset, next_characters.index())
    }

    pub fn cursor_location(&self) -> Location {
        let offset = self.characters.index();
        Location::with_offset(&self.compilation_unit, offset)
    }

    fn add_unexpected_error(&self) {
        if self.characters.has_remaining() {
            self.add_syntax_error(&self.character_ahead_location(), DiagnosticKind::UnexpectedCharacter, diagarg![self.characters.peek_or_zero().to_string()])
        } else {
            self.add_syntax_error(&self.cursor_location(), DiagnosticKind::UnexpectedEnd, vec![])
        }
    }

    fn add_unexpected_eof_error(&self, kind: DiagnosticKind) {
        self.add_syntax_error(&self.cursor_location(), kind, vec![]);
    }

    // LineTerminator
    fn consume_line_terminator(&mut self) -> bool {
        let ch = self.characters.peek_or_zero();
        if ch == '\x0D' && self.characters.peek_at_or_zero(1) == '\x0A' {
            self.characters.skip_count_in_place(2);
            // self.line_number += 1;
            return true;
        }
        if CharacterValidator::is_line_terminator(ch) {
            self.characters.next();
            // self.line_number += 1;
            return true;
        }
        false
    }

    fn consume_comment(&mut self) -> bool {
        let ch = self.characters.peek_or_zero();
        if ch != '/' {
            return false;
        }
        let ch2 = self.characters.peek_at_or_zero(1);
        if ch2 == '/' {
            let start = self.cursor_location();
            self.characters.skip_count_in_place(2);
            while !CharacterValidator::is_line_terminator(self.characters.peek_or_zero()) && self.characters.has_remaining() {
                self.characters.skip_in_place();
            }
            let location = start.combine_with(self.cursor_location());
            self.consume_line_terminator();

            self.compilation_unit.add_comment(Rc::new(Comment {
                multiline: false,
                content: RefCell::new(self.compilation_unit.text()[(location.first_offset() + 2)..location.last_offset()].to_owned()),
                location: RefCell::new(location),
            }));

            return true;
        }
        if ch2 == '*' {
            let start = self.cursor_location();
            self.characters.skip_count_in_place(2);

            loop {
                if self.characters.peek_or_zero() == '*' && self.characters.peek_at_or_zero(1) == '/' {
                    self.characters.skip_count_in_place(2);
                    break;
                } else if self.consume_line_terminator() {
                    // Consumed LineTerminator
                } else if self.characters.has_remaining() {
                    self.characters.skip_in_place();
                } else {
                    self.add_unexpected_eof_error(DiagnosticKind::InputEndedBeforeReachingClosingSeqForMultiLineComment);
                    break;
                }
            }

            let location = start.combine_with(self.cursor_location());

            let i = location.first_offset() + 2;
            let j = decrease_last_offset(i, location.last_offset(), 2);

            self.compilation_unit.add_comment(Rc::new(Comment {
                multiline: true,
                content: RefCell::new(self.compilation_unit.text()[i..j].to_owned()),
                location: RefCell::new(location),
            }));

            return true;
        }
        false
    }

    fn scan_identifier(&mut self) -> Option<(Token, Location)> {
        let start = self.cursor_location();
        let mut escaped = false;
        let Some((ch, escaped_2)) = self.consume_identifier_start() else {
            return None;
        };
        escaped = escaped || escaped_2;
        let mut name = String::new();
        name.push(ch);
        while let Some((ch, escaped_2)) = self.consume_identifier_part() {
            escaped = escaped || escaped_2;
            name.push(ch);
        }

        let location = start.combine_with(self.cursor_location());
        if !escaped {
            if let Some(token) = As3ReservedWord::token(name.as_ref()) {
                return Some((token, location));
            }
        }
        Some((Token::Identifier(name), location))
    }

    /// Returns a tuple in the form (*character*, *escaped*).
    fn consume_identifier_start(&mut self) -> Option<(char, bool)> {
        let ch = self.characters.peek_or_zero();
        if CharacterValidator::is_identifier_start(ch) {
            self.characters.next();
            return Some((ch, false));
        }
        if self.characters.peek_or_zero() == '\\' {
            self.characters.next();
            return Some((self.expect_unicode_escape_sequence(), true));
        }
        None
    }

    /// Returns a tuple in the form (*character*, *escaped*).
    fn consume_identifier_part(&mut self) -> Option<(char, bool)> {
        let ch = self.characters.peek_or_zero();
        if CharacterValidator::is_identifier_part(ch) {
            self.characters.next();
            return Some((ch, false));
        }
        if self.characters.peek_or_zero() == '\\' {
            self.characters.next();
            return Some((self.expect_unicode_escape_sequence(), true));
        }
        None
    }

    /// Expects UnicodeEscapeSequence starting from `u`.
    fn expect_unicode_escape_sequence(&mut self) -> char {
        let start = self.cursor_location();
        if self.characters.peek_or_zero() != 'u' {
            self.add_unexpected_error();
            return '\x5F';
        }
        self.characters.next();

        // Scan \uXXXX
        if CharacterValidator::is_hex_digit(self.characters.peek_or_zero()) {
            let r = char::from_u32(self.expect_hex_digit() << 12
                | (self.expect_hex_digit() << 8)
                | (self.expect_hex_digit() << 4)
                | self.expect_hex_digit());
            let Some(r) = r else {
                self.add_syntax_error(&start.combine_with(self.cursor_location()), DiagnosticKind::InvalidEscapeValue, vec![]);
                return '\x5F';
            };
            return r;
        }

        // Scan \u{}
        if self.characters.peek_or_zero() != '{' {
            self.add_unexpected_error();
            return '\x5F';
        }
        self.characters.next();
        while CharacterValidator::is_hex_digit(self.characters.peek_or_zero()) {
            self.characters.next();
        }
        if self.characters.peek_or_zero() != '}' {
            self.add_unexpected_error();
            return '\x5F';
        }
        self.characters.next();
        let location = start.combine_with(self.cursor_location());
        let r = u32::from_str_radix(&self.compilation_unit.text()[(start.first_offset + 2)..(location.last_offset - 1)], 16);
        let Ok(r) = r else {
            self.add_syntax_error(&location, DiagnosticKind::InvalidEscapeValue, vec![]);
            return '\x5F';
        };
        let r = char::from_u32(r);
        let Some(r) = r else {
            self.add_syntax_error(&location, DiagnosticKind::InvalidEscapeValue, vec![]);
            return '\x5F';
        };
        r
    }

    fn expect_hex_digit(&mut self) -> u32 {
        let ch = self.characters.peek_or_zero();
        let mv = CharacterValidator::hex_digit_mv(ch);
        if mv.is_none() {
            self.add_unexpected_error();
            return 0x5F;
        }
        self.characters.next();
        mv.unwrap()
    }

    fn scan_dot_or_numeric_literal(&mut self) -> Option<(Token, Location)> {
        let start = self.cursor_location();
        let ch = self.characters.peek_or_zero();
        let mut initial_dot = false;
        if ch == '.' {
            initial_dot = true;
            self.characters.next();

            let seq = self.characters.peek_seq(2);
            // Ellipsis
            if seq == ".." {
                self.characters.skip_count_in_place(2);
                return Some((Token::Ellipsis, start.combine_with(self.cursor_location())));
            }
            let ch = seq.get(..1).map(|ch| ch.chars().next().unwrap()).unwrap_or('\x00');
            // Descendants
            if ch == '.' {
                self.characters.next();
                return Some((Token::Descendants, start.combine_with(self.cursor_location())));
            }
            // Dot
            if !CharacterValidator::is_dec_digit(ch) {
                return Some((Token::Dot, start.combine_with(self.cursor_location())));
            }

            // NumericLiteral
            while CharacterValidator::is_dec_digit(self.characters.peek_or_zero()) {
                self.characters.next();
                self.consume_underscore_followed_by_dec_digit();
            }
        } else if ch == '0' {
            self.characters.next();
            let ch_2 = self.characters.peek_or_zero();

            // HexLiteral
            if ['X', 'x'].contains(&ch_2) {
                self.characters.next();
                return self.scan_hex_literal(start.clone());
            }

            // BinLiteral;
            if ['B', 'b'].contains(&ch_2) {
                self.characters.next();
                return self.scan_bin_literal(start.clone());
            }
        } else if CharacterValidator::is_dec_digit(ch) {
            while CharacterValidator::is_dec_digit(self.characters.peek_or_zero()) {
                self.characters.next();
                self.consume_underscore_followed_by_dec_digit();
            }
        } else {
            return None;
        }

        if !initial_dot && self.characters.peek_or_zero() == '.' {
            self.characters.next();
            if !CharacterValidator::is_dec_digit(self.characters.peek_or_zero()) {
                self.add_unexpected_error();
            }
            while CharacterValidator::is_dec_digit(self.characters.peek_or_zero()) {
                self.characters.next();
                self.consume_underscore_followed_by_dec_digit();
            }
        }

        // Decimal exponent
        if ['E', 'e'].contains(&self.characters.peek_or_zero()) {
            self.characters.next();
            if ['+', '-'].contains(&self.characters.peek_or_zero()) {
                self.characters.next();
            }
            if !CharacterValidator::is_dec_digit(self.characters.peek_or_zero()) {
                self.add_unexpected_error();
            }
            while CharacterValidator::is_dec_digit(self.characters.peek_or_zero()) {
                self.characters.next();
                self.consume_underscore_followed_by_dec_digit();
            }
        }

        let string = self.compilation_unit.text()[start.first_offset..self.characters.index()].to_owned();

        let mut suffix = NumberSuffix::None;
        if self.characters.peek_or_zero() == 'f' || self.characters.peek_or_zero() == 'F' {
            suffix = NumberSuffix::F;
            self.characters.next();
        }
        self.unallow_numeric_suffix();

        let location = start.combine_with(self.cursor_location());

        Some((Token::Number(string, suffix), location))
    }

    fn scan_hex_literal(&mut self, start: Location) -> Option<(Token, Location)> {
        if !CharacterValidator::is_hex_digit(self.characters.peek_or_zero()) {
            self.add_unexpected_error();
        }
        while CharacterValidator::is_hex_digit(self.characters.peek_or_zero()) {
            self.characters.next();
            self.consume_underscore_followed_by_hex_digit();
        }

        let suffix = NumberSuffix::None;
        self.unallow_numeric_suffix();

        let location = start.combine_with(self.cursor_location());
        let s = self.compilation_unit.text()[location.first_offset..location.last_offset].to_owned();
        Some((Token::Number(s, suffix), location))
    }

    fn scan_bin_literal(&mut self, start: Location) -> Option<(Token, Location)> {
        if !CharacterValidator::is_bin_digit(self.characters.peek_or_zero()) {
            self.add_unexpected_error();
        }
        while CharacterValidator::is_bin_digit(self.characters.peek_or_zero()) {
            self.characters.next();
            self.consume_underscore_followed_by_bin_digit();
        }

        let suffix = NumberSuffix::None;
        self.unallow_numeric_suffix();

        let location = start.combine_with(self.cursor_location());
        let s = self.compilation_unit.text()[location.first_offset..location.last_offset].to_owned();
        Some((Token::Number(s, suffix), location))
    }

    fn consume_underscore_followed_by_dec_digit(&mut self) {
        if self.characters.peek_or_zero() == '_' {
            self.characters.next();
            if !CharacterValidator::is_dec_digit(self.characters.peek_or_zero()) {
                self.add_unexpected_error();
            }
            self.characters.next();
        }
    }

    fn consume_underscore_followed_by_hex_digit(&mut self) {
        if self.characters.peek_or_zero() == '_' {
            self.characters.next();
            if !CharacterValidator::is_hex_digit(self.characters.peek_or_zero()) {
                self.add_unexpected_error();
            }
            self.characters.next();
        }
    }

    fn consume_underscore_followed_by_bin_digit(&mut self) {
        if self.characters.peek_or_zero() == '_' {
            self.characters.next();
            if !CharacterValidator::is_bin_digit(self.characters.peek_or_zero()) {
                self.add_unexpected_error();
            }
            self.characters.next();
        }
    }

    fn unallow_numeric_suffix(&self) {
        if CharacterValidator::is_identifier_start(self.characters.peek_or_zero()) {
            self.add_unexpected_error();
        }
    }

    fn scan_string_literal(&mut self, raw: bool) -> Option<(Token, Location)> {
        let delim = self.characters.peek_or_zero();
        if !['"', '\''].contains(&delim) {
            return None;
        }
        let mut start = self.cursor_location();
        // Include the "@" punctuator as part of raw string literals
        if raw {
            start = Location::with_offset(&start.compilation_unit(), start.first_offset() - 1);
        }

        self.characters.next();

        // Triple string literal
        if self.characters.peek_or_zero() == delim && self.characters.peek_at_or_zero(1) == delim {
            self.characters.skip_count_in_place(2);
            return self.scan_triple_string_literal(delim, start, raw);
        }

        let mut value = String::new();

        if raw {
            loop {
                let ch = self.characters.peek_or_zero();
                if ch == delim {
                    self.characters.next();
                    break;
                } else if CharacterValidator::is_line_terminator(ch) {
                    self.add_syntax_error(&self.character_ahead_location(), DiagnosticKind::StringLiteralMustBeTerminatedBeforeLineBreak, vec![]);
                    self.consume_line_terminator();
                } else if !self.characters.has_remaining() {
                    self.add_unexpected_eof_error(DiagnosticKind::InputEndedBeforeReachingClosingQuoteForString);
                    break;
                } else {
                    value.push(ch);
                    self.characters.next();
                }
            }
        } else {
            loop {
                if let Some(s) = self.consume_escape_sequence() {
                    value.push_str(&s);
                } else {
                    let ch = self.characters.peek_or_zero();
                    if ch == delim {
                        self.characters.next();
                        break;
                    } else if CharacterValidator::is_line_terminator(ch) {
                        self.add_syntax_error(&self.character_ahead_location(), DiagnosticKind::StringLiteralMustBeTerminatedBeforeLineBreak, vec![]);
                        self.consume_line_terminator();
                    } else if !self.characters.has_remaining() {
                        self.add_unexpected_eof_error(DiagnosticKind::InputEndedBeforeReachingClosingQuoteForString);
                        break;
                    } else {
                        value.push(ch);
                        self.characters.next();
                    }
                }
            }
        }

        let location = start.combine_with(self.cursor_location());
        Some((Token::String(value), location))
    }

    fn scan_triple_string_literal(&mut self, delim: char, start: Location, raw: bool) -> Option<(Token, Location)> {
        let mut lines: Vec<String> = vec![];
        let mut builder = String::new();

        if raw {
            loop {
                let ch = self.characters.peek_or_zero();
                if ch == delim && self.characters.peek_at_or_zero(1) == delim && self.characters.peek_at_or_zero(2) == delim {
                    self.characters.skip_count_in_place(3);
                    lines.push(builder.clone());
                    break;
                } else if CharacterValidator::is_line_terminator(ch) {
                    lines.push(builder.clone());
                    builder.clear();
                    self.consume_line_terminator();
                } else if !self.characters.has_remaining() {
                    self.add_unexpected_eof_error(DiagnosticKind::InputEndedBeforeReachingClosingQuoteForString);
                    lines.push(builder.clone());
                    builder.clear();
                    break;
                } else {
                    builder.push(ch);
                    self.characters.next();
                }
            }
        } else {
            loop {
                if let Some(s) = self.consume_escape_sequence() {
                    builder.push_str(&s);
                } else {
                    let ch = self.characters.peek_or_zero();
                    if ch == delim && self.characters.peek_at_or_zero(1) == delim && self.characters.peek_at_or_zero(2) == delim {
                        self.characters.skip_count_in_place(3);
                        lines.push(builder.clone());
                        break;
                    } else if CharacterValidator::is_line_terminator(ch) {
                        lines.push(builder.clone());
                        builder.clear();
                        self.consume_line_terminator();
                    } else if !self.characters.has_remaining() {
                        self.add_unexpected_eof_error(DiagnosticKind::InputEndedBeforeReachingClosingQuoteForString);
                        lines.push(builder.clone());
                        builder.clear();
                        break;
                    } else {
                        builder.push(ch);
                        self.characters.next();
                    }
                }
            }
        }

        let location = start.combine_with(self.cursor_location());

        if lines[0].is_empty() && lines.len() > 1 {
            lines.remove(0);
        }

        let last_line = lines.pop().unwrap();

        let base_indent = CharacterValidator::indent_count(&last_line);

        let mut lines: Vec<String> = lines.iter().map(|line| {
            let indent = CharacterValidator::indent_count(line);
            line[usize::min(base_indent, indent)..].to_owned()
        }).collect();

        let last_line = last_line[base_indent..].to_owned();
        if !last_line.is_empty() {
            lines.push(last_line);
        }

        let value = lines.join("\n");
        Some((Token::String(value), location))
    }

    fn consume_escape_sequence(&mut self) -> Option<String> {
        if self.characters.peek_or_zero() != '\\' {
            return None;
        }
        self.characters.next();
        if !self.characters.has_remaining() {
            self.add_unexpected_error();
            return Some("".into());
        }
        if self.consume_line_terminator() {
            return Some("".into());
        }
        let ch = self.characters.peek_or_zero();
        match ch {
            '\'' | '"' | '\\' => {
                self.characters.next();
                Some(ch.into())
            },
            'u' => {
                Some(self.expect_unicode_escape_sequence().into())
            },
            'x' => {
                self.characters.next();
                let v = (self.expect_hex_digit() << 4) | self.expect_hex_digit();
                let v = char::from_u32(v).unwrap();
                Some(v.into())
            },
            'b' => {
                self.characters.next();
                Some('\x08'.into())
            },
            'f' => {
                self.characters.next();
                Some('\x0C'.into())
            },
            'n' => {
                self.characters.next();
                Some('\x0A'.into())
            },
            'r' => {
                self.characters.next();
                Some('\x0D'.into())
            },
            't' => {
                self.characters.next();
                Some('\x09'.into())
            },
            'v' => {
                self.characters.next();
                Some('\x0B'.into())
            },
            '0' => {
                self.characters.next();
                if CharacterValidator::is_dec_digit(self.characters.peek_or_zero()) {
                    self.add_unexpected_error();
                }
                Some('\x00'.into())
            },
            ch => {
                if CharacterValidator::is_dec_digit(ch) {
                    self.add_unexpected_error();
                }
                self.characters.next();
                Some(ch.into())
            },
        }
    }

    /// Scans for an *InputElementXMLTag* token.
    pub fn scan_ie_xml_tag(&mut self) -> (Token, Location) {
        let start = self.cursor_location();
        let ch = self.characters.peek_or_zero();

        // XmlName
        if CharacterValidator::is_xml_name_start(ch) {
            self.characters.next();
            while CharacterValidator::is_xml_name_part(self.characters.peek_or_zero()) {
                self.characters.next();
            }
            let location = start.combine_with(self.cursor_location());
            let name = self.compilation_unit.text()[location.first_offset..location.last_offset].to_owned();
            return (Token::XmlName(name), location);
        }

        // XmlWhitespace
        if CharacterValidator::is_xml_whitespace(ch) {
            while CharacterValidator::is_xml_whitespace(self.characters.peek_or_zero()) {
                if !self.consume_line_terminator() {
                    self.characters.next();
                }
            }
            let location = start.combine_with(self.cursor_location());
            return (Token::XmlWhitespace, location);
        }

        match ch {
            // Assign
            '=' => {
                self.characters.next();
                let location = start.combine_with(self.cursor_location());
                (Token::Assign, location)
            },

            // Gt
            '>' => {
                self.characters.next();
                let location = start.combine_with(self.cursor_location());
                (Token::Gt, location)
            },

            // XmlSlashGt
            '/' => {
                self.characters.next();
                if self.characters.peek_or_zero() != '>' {
                    self.add_unexpected_error();
                    /*
                    while self.characters.has_remaining() {
                        self.characters.next();
                        if self.characters.peek_or_zero() == '>' {
                            self.characters.next();
                            let location = start.combine_with(self.cursor_location());
                            return (Token::XmlSlashGt, location);
                        }
                    }
                    */
                    let location = start.combine_with(self.cursor_location());
                    return (Token::XmlSlashGt, location);
                }
                self.characters.next();
                let location = start.combine_with(self.cursor_location());
                (Token::XmlSlashGt, location)
            },

            // XmlAttributeValue
            '"' | '\'' => {
                let delim = ch;
                self.characters.next();
                while self.characters.peek_or_zero() != delim && self.characters.has_remaining() {
                    if !self.consume_line_terminator() {
                        self.characters.next();
                    }
                }
                if self.characters.reached_end() {
                    self.add_unexpected_eof_error(DiagnosticKind::InputEndedBeforeReachingClosingQuoteForAttributeValue);
                    let value = self.compilation_unit.text()[(start.first_offset + 1)..self.cursor_location().first_offset].to_owned();
                    let location = start.combine_with(self.cursor_location());
                    return (Token::XmlAttributeValue(value), location);
                }
                let value = self.compilation_unit.text()[(start.first_offset + 1)..self.cursor_location().first_offset].to_owned();
                self.characters.next();
                
                let location = start.combine_with(self.cursor_location());
                (Token::XmlAttributeValue(value), location)
            },

            // BlockOpen
            '{' => {
                self.characters.next();
                let location = start.combine_with(self.cursor_location());
                (Token::BlockOpen, location)
            },

            _ => {
                if self.characters.reached_end() {
                    return (Token::Eof, self.cursor_location());
                }
                self.add_unexpected_error();
                self.characters.next();
                self.scan_ie_xml_tag()
            },
        }
    }

    /// Scans for an *InputElementXMLContent* token.
    pub fn scan_ie_xml_content(&mut self) -> (Token, Location) {
        let start = self.cursor_location();
        let ch = self.characters.peek_or_zero();

        match ch {
            '<' => {
                self.characters.next();

                // XmlMarkup
                if let Some(r) = self.scan_xml_markup(start.clone()) {
                    return r;
                }

                // XmlLtSlash
                if self.characters.peek_or_zero() == '/' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return (Token::XmlLtSlash, location);
                }

                // Lt
                let location = start.combine_with(self.cursor_location());
                (Token::Lt, location)
            },
            
            // BlockOpen
            '{' => {
                self.characters.next();
                let location = start.combine_with(self.cursor_location());
                (Token::BlockOpen, location)
            },

            // XmlName
            _ => {
                if self.characters.reached_end() {
                    return (Token::Eof, self.cursor_location());
                }
                loop {
                    let ch = self.characters.peek_or_zero();
                    if ['<', '{'].contains(&ch) {
                        break;
                    }
                    if CharacterValidator::is_line_terminator(ch) {
                        self.consume_line_terminator();
                    } else if self.characters.has_remaining() {
                        self.characters.next();
                    } else {
                        break;
                    }
                }

                let location = start.combine_with(self.cursor_location());
                let content = self.compilation_unit.text()[location.first_offset..location.last_offset].to_owned();
                (Token::XmlText(content), location)
            },
        }
    }

    /// Attempts to scan a XMLMarkup token after a `<` character.
    pub fn scan_xml_markup(&mut self, start: Location) -> Option<(Token, Location)> {
        // XMLComment
        if self.characters.peek_seq(3) == "!--" {
            self.characters.skip_count_in_place(3);
            loop {
                if self.characters.peek_or_zero() == '-' && self.characters.peek_seq(3) == "-->" {
                    self.characters.skip_count_in_place(3);
                    break;
                } else if CharacterValidator::is_line_terminator(self.characters.peek_or_zero()) {
                    self.consume_line_terminator();
                } else if self.characters.reached_end() {
                    self.add_unexpected_eof_error(DiagnosticKind::InputEndedBeforeReachingClosingSeqForXmlComment);
                    break;
                } else {
                    self.characters.next();
                }
            }

            let location = start.combine_with(self.cursor_location());
            let content = self.compilation_unit.text()[location.first_offset..location.last_offset].to_owned();

            return Some((Token::XmlMarkup(content), location));
        }

        // XMLCDATA
        if self.characters.peek_seq(8) == "![CDATA[" {
            self.characters.skip_count_in_place(8);
            loop {
                if self.characters.peek_or_zero() == ']' && self.characters.peek_seq(3) == "]]>" {
                    self.characters.skip_count_in_place(3);
                    break;
                } else if CharacterValidator::is_line_terminator(self.characters.peek_or_zero()) {
                    self.consume_line_terminator();
                } else if self.characters.reached_end() {
                    self.add_unexpected_eof_error(DiagnosticKind::InputEndedBeforeReachingClosingSeqForCData);
                    break;
                } else {
                    self.characters.next();
                }
            }

            let location = start.combine_with(self.cursor_location());
            let content = self.compilation_unit.text()[location.first_offset..location.last_offset].to_owned();

            return Some((Token::XmlMarkup(content), location));
        }

        // XMLPI
        if self.characters.peek_or_zero() == '?' {
            self.characters.next();
            loop {
                if self.characters.peek_or_zero() == '?' && self.characters.peek_at_or_zero(1) == '>' {
                    self.characters.skip_count_in_place(2);
                    break;
                } else if CharacterValidator::is_line_terminator(self.characters.peek_or_zero()) {
                    self.consume_line_terminator();
                } else if self.characters.reached_end() {
                    self.add_unexpected_eof_error(DiagnosticKind::InputEndedBeforeReachingClosingSeqForPi);
                    break;
                } else {
                    self.characters.next();
                }
            }

            let location = start.combine_with(self.cursor_location());
            let content = self.compilation_unit.text()[location.first_offset..location.last_offset].to_owned();

            return Some((Token::XmlMarkup(content), location));
        }

        None
    }
}