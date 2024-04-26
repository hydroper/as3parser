use crate::ns::*;
use num_traits::ToPrimitive;

pub struct CssParser<'input> {
    tokenizer: CssTokenizer<'input>,
    previous_token: (Token, Location),
    token: (Token, Location),
    locations: Vec<Location>,
    expecting_token_error: bool,
}

impl<'input> CssParser<'input> {
    /// Constructs a tokenizer.
    pub fn new(compilation_unit: &'input Rc<CompilationUnit>, options: &ParserOptions) -> Self {
        Self {
            tokenizer: CssTokenizer::new(compilation_unit, options),
            previous_token: (Token::Eof, Location::with_offset(&compilation_unit, 0)),
            token: (Token::Eof, Location::with_offset(&compilation_unit, 0)),
            locations: vec![],
            expecting_token_error: false,
        }
    }

    fn compilation_unit(&self) -> &Rc<CompilationUnit> {
        self.tokenizer.compilation_unit()
    }

    fn token_location(&self) -> Location {
        self.token.1.clone()
    }

    fn mark_location(&mut self) {
        self.locations.push(self.token.1.clone());
    }

    fn duplicate_location(&mut self) {
        self.locations.push(self.locations.last().unwrap().clone());
    }

    fn push_location(&mut self, location: &Location) {
        self.locations.push(location.clone());
    }

    fn pop_location(&mut self) -> Location {
        self.locations.pop().unwrap().combine_with(self.previous_token.1.clone())
    }

    fn add_syntax_error(&self, location: &Location, kind: DiagnosticKind, arguments: Vec<DiagnosticArgument>) {
        if self.compilation_unit().prevent_equal_offset_error(location) {
            return;
        }
        self.compilation_unit().add_diagnostic(Diagnostic::new_syntax_error(location, kind, arguments));
    }

    fn _patch_syntax_error(&self, original: DiagnosticKind, location: &Location, kind: DiagnosticKind, arguments: Vec<DiagnosticArgument>) {
        if self.compilation_unit().diagnostics.borrow().is_empty() {
            return;
        }
        if self.compilation_unit().diagnostics.borrow().last().unwrap().kind == original {
            self.compilation_unit().diagnostics.borrow_mut().pop();
            self.compilation_unit().add_diagnostic(Diagnostic::new_syntax_error(location, kind, arguments));
        }
    }

    /*
    fn add_warning(&self, location: &Location, kind: DiagnosticKind, arguments: Vec<DiagnosticArgument>) {
        self.compilation_unit().add_diagnostic(Diagnostic::new_warning(location, kind, arguments));
    }
    */

    fn next(&mut self) {
        self.previous_token = self.token.clone();
        self.token = self.tokenizer.scan();
    }

    fn peek(&self, token: Token) -> bool {
        self.token.0 == token
    }

    fn peek_identifier(&self) -> Option<(String, Location)> {
        if let Token::Identifier(id) = self.token.0.clone() {
            let location = self.token.1.clone();
            Some((id, location))
        } else {
            None
        }
    }

    fn peek_context_keyword(&self, name: &str) -> bool {
        if let Token::Identifier(id) = self.token.0.clone() { id == name && self.token.1.character_count() == name.len() } else { false }
    }

    fn consume(&mut self, token: Token) -> bool {
        if self.token.0 == token {
            self.next();
            true
        } else {
            false
        }
    }

    fn consume_identifier(&mut self) -> Option<(String, Location)> {
        if let Token::Identifier(id) = self.token.0.clone() {
            let location = self.token.1.clone();
            self.next();
            Some((id, location))
        } else {
            None
        }
    }

    /// Expects a token in non-greedy mode: if it fails, does not skip any token.
    fn expect(&mut self, token: Token) {
        if self.token.0 != token {
            self.expecting_token_error = true;
            self.add_syntax_error(&self.token_location(), DiagnosticKind::Expecting, diagnostic_arguments![Token(token.clone()), Token(self.token.0.clone())]);
        } else {
            self.expecting_token_error = false;
            self.next();
        }
    }

    fn expect_identifier(&mut self) -> (String, Location) {
        if let Token::Identifier(id) = self.token.0.clone() {
            self.expecting_token_error = false;
            let location = self.token.1.clone();
            self.next();
            (id, location)
        } else {
            self.expecting_token_error = true;
            self.add_syntax_error(&self.token_location(), DiagnosticKind::ExpectingIdentifier, diagnostic_arguments![Token(self.token.0.clone())]);
            (INVALIDATED_IDENTIFIER.to_owned(), self.tokenizer.cursor_location())
        }
    }

    pub fn expect_eof(&mut self) {
        self.expect(Token::Eof);
    }

    fn create_invalidated_directive(&self, location: &Location) -> Rc<CssDirective> {
        Rc::new(CssDirective::Invalidated(InvalidatedNode {
            location: location.clone(),
        }))
    }

    fn create_invalidated_property_value(&self, location: &Location) -> Rc<CssPropertyValueNode> {
        Rc::new(CssPropertyValueNode::Invalidated(InvalidatedNode {
            location: location.clone(),
        }))
    }

    fn create_invalidated_selector(&self, location: &Location) -> Rc<CssSelector> {
        Rc::new(CssSelector::Invalidated(InvalidatedNode {
            location: location.clone(),
        }))
    }

    fn create_invalidated_selector_condition(&self, location: &Location) -> Rc<CssSelectorCondition> {
        Rc::new(CssSelectorCondition::Invalidated(InvalidatedNode {
            location: location.clone(),
        }))
    }

    fn create_invalidated_media_query_condition(&self, location: &Location) -> Rc<CssMediaQueryCondition> {
        Rc::new(CssMediaQueryCondition::Invalidated(InvalidatedNode {
            location: location.clone(),
        }))
    }
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