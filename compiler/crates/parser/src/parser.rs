use std::rc::Rc;
use crate::*;
use crate::util::default;

pub struct Parser<'input> {
    tokenizer: Tokenizer<'input>,
    previous_token: (Token, Location),
    token: (Token, Location),
    locations: Vec<Location>,
}

impl<'input> Parser<'input> {
    /// Constructs a parser.
    pub fn new(source: &'input Rc<Source>) -> Self {
        Self {
            tokenizer: Tokenizer::new(source),
            previous_token: (Token::Eof, Location::with_line_and_offset(&source, 1, 0)),
            token: (Token::Eof, Location::with_line_and_offset(&source, 1, 0)),
            locations: vec![],
        }
    }

    fn source(&self) -> &Rc<Source> {
        &self.tokenizer.source
    }

    fn token_location(&mut self) -> Location {
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
        self.locations.pop().unwrap().combine_with_start_of(self.token.1.clone())
    }

    fn add_syntax_error(&self, location: Location, kind: DiagnosticKind, arguments: Vec<Box<DiagnosticArgument>>) {
        self.source().add_diagnostic(Diagnostic::new_syntax_error(location, kind, arguments));
    }

    fn add_warning(&self, location: Location, kind: DiagnosticKind, arguments: Vec<Box<DiagnosticArgument>>) {
        self.source().add_diagnostic(Diagnostic::new_warning(location, kind, arguments));
    }

    fn next(&mut self) -> Result<(), ParserFailure> {
        self.previous_token = self.token.clone();
        self.token = self.tokenizer.scan_ie_div()?;
        Ok(())
    }

    fn next_ie_xml_tag(&mut self) -> Result<(), ParserFailure> {
        self.previous_token = self.token.clone();
        self.token = self.tokenizer.scan_ie_xml_tag()?;
        Ok(())
    }

    fn next_ie_xml_content(&mut self) -> Result<(), ParserFailure> {
        self.previous_token = self.token.clone();
        self.token = self.tokenizer.scan_ie_xml_content()?;
        Ok(())
    }

    fn peek(&self, token: Token) -> bool {
        self.token.0 == token
    }

    fn peek_context_keyword(&self, name: String) -> bool {
        if let Token::Identifier(id) = self.token.0.clone() { id == name } else { false }
    }

    fn peek_reserved_namespace(&self) -> Option<ast::ReservedNamespace> {
        match self.token.0 {
            Token::Public => Some(ast::ReservedNamespace::Public),
            Token::Private => Some(ast::ReservedNamespace::Private),
            Token::Protected => Some(ast::ReservedNamespace::Protected),
            Token::Internal => Some(ast::ReservedNamespace::Internal),
            _ => None,
        }
    }

    fn consume(&mut self, token: Token) -> Result<bool, ParserFailure> {
        if self.token.0 == token {
            self.next()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn consume_identifier(&mut self, reserved_words: bool) -> Result<Option<(String, Location)>, ParserFailure> {
        if let Token::Identifier(id) = self.token.0.clone() {
            let location = self.token.1.clone();
            self.next()?;
            Ok(Some((id, location)))
        } else {
            if reserved_words {
                if let Some(id) = self.token.0.reserved_word_name() {
                    let location = self.token.1.clone();
                    self.next()?;
                    return Ok(Some((id, location)));
                }
            }
            Ok(None)
        }
    }

    fn consume_context_keyword(&mut self, name: String) -> Result<bool, ParserFailure> {
        if let Token::Identifier(id) = self.token.0.clone() {
            if id == name {
                self.next()?;
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    fn expect(&mut self, token: Token) -> Result<(), ParserFailure> {
        if self.token.0 != token {
            self.add_syntax_error(self.token_location(), DiagnosticKind::Expected, diagnostic_arguments![Token(token), Token(self.token.0.clone())]);
            Err(ParserFailure)
        } else {
            self.next()?;
            Ok(())
        }
    }

    fn expect_identifier(&mut self, reserved_words: bool) -> Result<(String, Location), ParserFailure> {
        if let Token::Identifier(id) = self.token.0.clone() {
            let location = self.token.1.clone();
            self.next()?;
            Ok((id, location))
        } else {
            if reserved_words {
                if let Some(id) = self.token.0.reserved_word_name() {
                    let location = self.token.1.clone();
                    self.next()?;
                    return Ok((id, location));
                }
            }
            self.add_syntax_error(self.token_location(), DiagnosticKind::ExpectedIdentifier, diagnostic_arguments![Token(self.token.0.clone())]);
            Err(ParserFailure)
        }
    }

    fn expect_context_keyword(&mut self, name: String) -> Result<(), ParserFailure> {
        if let Token::Identifier(id) = self.token.0.clone() {
            if id == name {
                self.next()?;
                return Ok(());
            }
        }
        self.add_syntax_error(self.token_location(), DiagnosticKind::Expected, diagnostic_arguments![String(name), Token(self.token.0.clone())]);
        Err(ParserFailure)
    }

    /// Expects a greater-than symbol. If the facing token is not greater-than,
    /// but starts with a greater-than symbol, the first character is shifted off
    /// from the facing token.
    fn expect_generics_gt(&mut self) -> Result<(), ParserFailure> {
        match self.token.0 {
            Token::Gt => {
                self.next()?;
                Ok(())
            },
            Token::Ge => {
                self.token.0 = Token::Assign;
                self.token.1.first_offset += 1;
                Ok(())
            },
            Token::RightShift => {
                self.token.0 = Token::Gt;
                self.token.1.first_offset += 1;
                Ok(())
            },
            Token::RightShiftAssign => {
                self.token.0 = Token::Ge;
                self.token.1.first_offset += 1;
                Ok(())
            },
            Token::UnsignedRightShift => {
                self.token.0 = Token::RightShift;
                self.token.1.first_offset += 1;
                Ok(())
            },
            Token::UnsignedRightShiftAssign => {
                self.token.0 = Token::RightShiftAssign;
                self.token.1.first_offset += 1;
                Ok(())
            },
            _ => {
                self.expect(Token::Gt)
            },
        }
    }

    pub fn expect_eof(&mut self) -> Result<(), ParserFailure> {
        self.expect(Token::Eof)
    }

    pub fn parse_expression(&mut self, context: ExpressionContext) -> Result<Rc<ast::Expression>, ParserFailure> {
        if let Some(exp) = self.parse_opt_expression(context)? {
            Ok(exp)
        } else {
            self.add_syntax_error(self.token_location(), DiagnosticKind::ExpectedExpression, diagnostic_arguments![Token(self.token.0.clone())]);
            Err(ParserFailure)
        }
    }

    pub fn parse_opt_expression(&mut self, context: ExpressionContext) -> Result<Option<Rc<ast::Expression>>, ParserFailure> {
        let mut exp: Option<Rc<ast::Expression>> = self.parse_opt_start_expression(context.clone())?;

        // Parse subexpressions
        if let Some(exp) = exp {
            return Ok(Some(self.parse_subexpressions(exp, context.clone())?));
        }
        Ok(None)
    }

    fn parse_opt_start_expression(&mut self, context: ExpressionContext) -> Result<Option<Rc<ast::Expression>>, ParserFailure> {
        if let Token::Identifier(ref id) = self.token.0 {
            let id_location = self.token_location();
            self.next()?;
            let id = Rc::new(ast::Expression {
                location: id_location,
                kind: ast::ExpressionKind::Id(ast::QualifiedIdentifier {
                    attribute: false,
                    qualifier: None,
                    name: ast::IdentifierOrBrackets::Id(id.clone(), id_location),
                }),
            });
            if self.peek(Token::ColonColon) {
                self.push_location(&id_location);
                let id = self.finish_qualified_identifier(false, id)?;
                Ok(Some(Rc::new(ast::Expression {
                    location: self.pop_location(),
                    kind: ast::ExpressionKind::Id(id),
                })))
            } else {
                Ok(Some(id))
            }
        } else if self.peek(Token::Null) {
            self.mark_location();
            self.next();
            Ok(Some(Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::Null,
            })))
        } else if self.peek(Token::False) {
            self.mark_location();
            self.next();
            Ok(Some(Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::Boolean(false),
            })))
        } else if self.peek(Token::True) {
            self.mark_location();
            self.next();
            Ok(Some(Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::Boolean(true),
            })))
        } else if let Token::NumericLiteral(n) = self.token.0 {
            self.mark_location();
            self.next();
            Ok(Some(Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::Numeric(n),
            })))
        } else if let Token::StringLiteral(ref s) = self.token.0 {
            self.mark_location();
            self.next();
            Ok(Some(Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::String(s.clone()),
            })))
        } else if self.peek(Token::This) {
            self.mark_location();
            self.next();
            Ok(Some(Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::This,
            })))
        } else if let Token::RegExpLiteral { ref body, ref flags } = self.token.0 {
            self.mark_location();
            self.next();
            Ok(Some(Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::RegExp { body: body.clone(), flags: flags.clone() },
            })))
        // `@`
        } else if self.peek(Token::Attribute) {
            self.mark_location();
            let id = self.parse_qualified_identifier()?;
            Ok(Some(Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::Id(id),
            })))
        // `public`, `private`, `protected`, `internal`
        } else if let Some(reserved_ns) = self.peek_reserved_namespace() {
            self.mark_location();
            self.duplicate_location();
            self.next();
            let rns = Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::ReservedNamespace(reserved_ns),
            });
            if self.peek(Token::ColonColon) {
                let id = self.finish_qualified_identifier(false, rns)?;
                Ok(Some(Rc::new(ast::Expression {
                    location: self.pop_location(),
                    kind: ast::ExpressionKind::Id(id),
                })))
            } else {
                self.pop_location();
                Ok(Some(rns))
            }
        // `*`
        } else if self.peek(Token::Times) {
            let id_location = self.token_location();
            self.next()?;
            let id = Rc::new(ast::Expression {
                location: id_location,
                kind: ast::ExpressionKind::Id(ast::QualifiedIdentifier {
                    attribute: false,
                    qualifier: None,
                    name: ast::IdentifierOrBrackets::Id("*".into(), id_location),
                }),
            });
            if self.peek(Token::ColonColon) {
                self.push_location(&id_location);
                let id = self.finish_qualified_identifier(false, id)?;
                Ok(Some(Rc::new(ast::Expression {
                    location: self.pop_location(),
                    kind: ast::ExpressionKind::Id(id),
                })))
            } else {
                Ok(Some(id))
            }
        } else {
            Ok(None)
        }
    }

    fn parse_qualified_identifier(&mut self) -> Result<ast::QualifiedIdentifier, ParserFailure> {
        let attribute = self.consume(Token::Attribute)?;
        if attribute && self.peek(Token::LeftBracket) {
            let brackets = self.parse_brackets()?;
            return Ok(ast::QualifiedIdentifier {
                attribute,
                qualifier: None,
                name: ast::IdentifierOrBrackets::Brackets(brackets),
            });
        }

        if let Token::Identifier(ref id) = self.token.0 {
            let id_location = self.token_location();
            self.next()?;
            if self.peek(Token::ColonColon) {
                let id = ast::QualifiedIdentifier {
                    attribute: false,
                    qualifier: None,
                    name: ast::IdentifierOrBrackets::Id(id.clone(), id_location),
                };
                let id = Rc::new(ast::Expression {
                    location: id_location,
                    kind: ast::ExpressionKind::Id(id),
                });
                return self.finish_qualified_identifier(attribute, id);
            } else {
                let id = ast::QualifiedIdentifier {
                    attribute,
                    qualifier: None,
                    name: ast::IdentifierOrBrackets::Id(id.clone(), id_location),
                };
                return Ok(id);
            }
        }

        // `public`, `private`, `protected`, `internal` followed by `::`
        if let Some(reserved_ns) = self.peek_reserved_namespace() {
            let q_location = self.token_location();
            self.next()?;
            let id = Rc::new(ast::Expression {
                location: q_location,
                kind: ast::ExpressionKind::ReservedNamespace(reserved_ns),
            });
            return self.finish_qualified_identifier(attribute, id);
        }

        // (q)::x
        if self.peek(Token::LeftParen) {
            let qual = self.parse_paren_expression()?;
            return self.finish_qualified_identifier(attribute, qual);
        }

        // `*`
        if self.peek(Token::Times) {
            let id_location = self.token_location();
            self.next()?;
            if self.peek(Token::ColonColon) {
                let id = ast::QualifiedIdentifier {
                    attribute: false,
                    qualifier: None,
                    name: ast::IdentifierOrBrackets::Id("*".into(), id_location),
                };
                let id = Rc::new(ast::Expression {
                    location: id_location,
                    kind: ast::ExpressionKind::Id(id),
                });
                return self.finish_qualified_identifier(attribute, id);
            } else {
                let id = ast::QualifiedIdentifier {
                    attribute,
                    qualifier: None,
                    name: ast::IdentifierOrBrackets::Id("*".into(), id_location),
                };
                return Ok(id);
            }
        }

        self.add_syntax_error(self.token_location(), DiagnosticKind::ExpectedIdentifier, diagnostic_arguments![Token(self.token.0.clone())]);
        Err(ParserFailure)
    }

    /// Expects a colon-colon and finishes a qualified identifier.
    fn finish_qualified_identifier(&mut self, attribute: bool, qual: Rc<ast::Expression>) -> Result<ast::QualifiedIdentifier, ParserFailure> {
        self.expect(Token::ColonColon)?;

        // `::` may be followed by one of { IdentifierName, `*`, Brackets }

        // IdentifierName
        if let Token::Identifier(ref id) = self.token.0 {
            let id_location = self.token_location();
            self.next()?;
            Ok(ast::QualifiedIdentifier {
                attribute,
                qualifier: Some(qual),
                name: ast::IdentifierOrBrackets::Id(id.clone(), id_location),
            })
        // IdentifierName (from reserved word)
        } else if let Some(id) = self.token.0.reserved_word_name() {
            let id_location = self.token_location();
            self.next()?;
            Ok(ast::QualifiedIdentifier {
                attribute,
                qualifier: Some(qual),
                name: ast::IdentifierOrBrackets::Id(id, id_location),
            })
        // `*`
        } else if self.peek(Token::Times) {
            let id_location = self.token_location();
            self.next()?;
            Ok(ast::QualifiedIdentifier {
                attribute,
                qualifier: Some(qual),
                name: ast::IdentifierOrBrackets::Id("*".into(), id_location),
            })
        // Brackets
        } else if self.peek(Token::LeftBracket) {
            let brackets = self.parse_brackets()?;
            Ok(ast::QualifiedIdentifier {
                attribute,
                qualifier: Some(qual),
                name: ast::IdentifierOrBrackets::Brackets(brackets),
            })
        } else {
            self.add_syntax_error(self.token_location(), DiagnosticKind::ExpectedIdentifier, diagnostic_arguments![Token(self.token.0.clone())]);
            Err(ParserFailure)
        }
    }

    fn parse_brackets(&mut self) -> Result<Rc<ast::Expression>, ParserFailure> {
        self.expect(Token::LeftBracket);
        let expr = self.parse_expression(ExpressionContext {
            min_precedence: OperatorPrecedence::List,
            allow_in: true,
            ..default()
        });
        self.expect(Token::RightBracket);
        expr
    }

    fn parse_paren_expression(&mut self) -> Result<Rc<ast::Expression>, ParserFailure> {
        self.expect(Token::LeftParen);
        let expr = self.parse_expression(ExpressionContext {
            min_precedence: OperatorPrecedence::AssignmentAndOther,
            allow_in: true,
            ..default()
        });
        self.expect(Token::RightParen);
        expr
    }
}

/// Context used to control the parsing of an expression.
#[derive(Clone)]
pub struct ExpressionContext {
    pub min_precedence: OperatorPrecedence,
    pub allow_in: bool,
    pub allow_assignment: bool,
    pub with_type_annotation: bool,
}

impl Default for ExpressionContext {
    fn default() -> Self {
        Self {
            min_precedence: OperatorPrecedence::List,
            allow_in: true,
            allow_assignment: true,
            with_type_annotation: true,
        }
    }
}