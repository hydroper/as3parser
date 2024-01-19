use std::{cell::Cell, collections::HashMap, rc::Rc};
use lazy_regex::*;
use crate::*;
use crate::ast::XmlElement;
use crate::util::default;

pub struct Parser<'input> {
    tokenizer: Tokenizer<'input>,
    previous_token: (Token, Location),
    token: (Token, Location),
    locations: Vec<Location>,
    activations: Vec<Activation>,
}

impl<'input> Parser<'input> {
    /// Constructs a parser.
    pub fn new(source: &'input Rc<Source>) -> Self {
        Self {
            tokenizer: Tokenizer::new(source),
            previous_token: (Token::Eof, Location::with_line_and_offset(&source, 1, 0)),
            token: (Token::Eof, Location::with_line_and_offset(&source, 1, 0)),
            locations: vec![],
            activations: vec![],
        }
    }

    fn source(&self) -> &Rc<Source> {
        &self.tokenizer.source
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
        self.locations.pop().unwrap().combine_with_start_of(self.token.1.clone())
    }

    fn add_syntax_error(&self, location: &Location, kind: DiagnosticKind, arguments: Vec<DiagnosticArgument>) {
        self.source().add_diagnostic(Diagnostic::new_syntax_error(location, kind, arguments));
    }

    /*
    fn add_warning(&self, location: &Location, kind: DiagnosticKind, arguments: Vec<DiagnosticArgument>) {
        self.source().add_diagnostic(Diagnostic::new_warning(location, kind, arguments));
    }
    */

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

    fn peek_identifier(&self, reserved_words: bool) -> Result<Option<(String, Location)>, ParserFailure> {
        if let Token::Identifier(id) = self.token.0.clone() {
            let location = self.token.1.clone();
            Ok(Some((id, location)))
        } else {
            if reserved_words {
                if let Some(id) = self.token.0.reserved_word_name() {
                    let location = self.token.1.clone();
                    return Ok(Some((id, location)));
                }
            }
            Ok(None)
        }
    }

    fn peek_context_keyword(&self, name: &str) -> bool {
        if let Token::Identifier(id) = self.token.0.clone() { id == name && self.token.1.character_count() == name.len() } else { false }
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

    fn consume_and_ie_xml_tag(&mut self, token: Token) -> Result<bool, ParserFailure> {
        if self.token.0 == token {
            self.next_ie_xml_tag()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn consume_and_ie_xml_content(&mut self, token: Token) -> Result<bool, ParserFailure> {
        if self.token.0 == token {
            self.next_ie_xml_content()?;
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

    fn consume_context_keyword(&mut self, name: &str) -> Result<bool, ParserFailure> {
        if let Token::Identifier(id) = self.token.0.clone() {
            if id == name && self.token.1.character_count() == name.len() {
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
            self.add_syntax_error(&self.token_location(), DiagnosticKind::Expected, diagnostic_arguments![Token(token), Token(self.token.0.clone())]);
            Err(ParserFailure)
        } else {
            self.next()?;
            Ok(())
        }
    }

    fn expect_and_ie_xml_tag(&mut self, token: Token) -> Result<(), ParserFailure> {
        if self.token.0 != token {
            self.add_syntax_error(&self.token_location(), DiagnosticKind::Expected, diagnostic_arguments![Token(token), Token(self.token.0.clone())]);
            Err(ParserFailure)
        } else {
            self.next_ie_xml_tag()?;
            Ok(())
        }
    }

    fn expect_and_ie_xml_content(&mut self, token: Token) -> Result<(), ParserFailure> {
        if self.token.0 != token {
            self.add_syntax_error(&self.token_location(), DiagnosticKind::Expected, diagnostic_arguments![Token(token), Token(self.token.0.clone())]);
            Err(ParserFailure)
        } else {
            self.next_ie_xml_content()?;
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
            self.add_syntax_error(&self.token_location(), DiagnosticKind::ExpectedIdentifier, diagnostic_arguments![Token(self.token.0.clone())]);
            Err(ParserFailure)
        }
    }

    fn expect_context_keyword(&mut self, name: &str) -> Result<(), ParserFailure> {
        if let Token::Identifier(id) = self.token.0.clone() {
            if id == name && self.token.1.character_count() == name.len() {
                self.next()?;
                return Ok(());
            }
        }
        self.add_syntax_error(&self.token_location(), DiagnosticKind::Expected, diagnostic_arguments![String(name.into()), Token(self.token.0.clone())]);
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
            self.add_syntax_error(&self.token_location(), DiagnosticKind::ExpectedExpression, diagnostic_arguments![Token(self.token.0.clone())]);
            Err(ParserFailure)
        }
    }

    pub fn parse_opt_expression(&mut self, context: ExpressionContext) -> Result<Option<Rc<ast::Expression>>, ParserFailure> {
        let exp: Option<Rc<ast::Expression>> = self.parse_opt_start_expression(context.clone())?;

        // Parse subexpressions
        if let Some(exp) = exp {
            return Ok(Some(self.parse_subexpressions(exp, context.clone())?));
        }
        Ok(None)
    }

    fn parse_subexpressions(&mut self, mut base: Rc<ast::Expression>, context: ExpressionContext) -> Result<Rc<ast::Expression>, ParserFailure> {
        loop {
            if self.peek(Token::FatArrow) && context.min_precedence.includes(&OperatorPrecedence::AssignmentAndOther) {
                base = self.parse_arrow_function(base.location.clone(), ArrowFunctionContext {
                    left: Some(base),
                    right_context: context.clone(),
                    ..default()
                })?;
            } else if self.consume(Token::Dot)? {
                base = self.parse_dot_subexpression(base)?;
            } else if self.consume(Token::OptionalChaining)? {
                base = self.parse_optional_chaining_subexpression(base)?;
            } else if self.peek(Token::LeftBracket) {
                let asdoc = self.parse_asdoc()?;
                self.next()?;
                self.push_location(&base.location);
                let key = self.parse_expression(ExpressionContext { allow_in: true, min_precedence: OperatorPrecedence::List, ..default() })?;
                self.expect(Token::RightBracket)?;
                base = Rc::new(ast::Expression {
                    location: self.pop_location(),
                    kind: ast::ExpressionKind::BracketsMember { base, key, asdoc }
                });
            } else if self.consume(Token::Descendants)? {
                self.push_location(&base.location);
                let id = self.parse_qualified_identifier()?;
                base = Rc::new(ast::Expression {
                    location: self.pop_location(),
                    kind: ast::ExpressionKind::Descendants { base, id },
                });
            } else if self.peek(Token::LeftParen) {
                self.push_location(&base.location);
                let arguments = self.parse_arguments()?;
                base = Rc::new(ast::Expression {
                    location: self.pop_location(),
                    kind: ast::ExpressionKind::Call { base, arguments },
                });
            } else if self.peek(Token::Increment) && !self.previous_token.1.line_break(&self.token.1) {
                self.push_location(&base.location);
                self.next()?;
                base = Rc::new(ast::Expression {
                    location: self.pop_location(),
                    kind: ast::ExpressionKind::Unary { base, operator: Operator::PostIncrement },
                });
            } else if self.peek(Token::Decrement) && !self.previous_token.1.line_break(&self.token.1) {
                self.push_location(&base.location);
                self.next()?;
                base = Rc::new(ast::Expression {
                    location: self.pop_location(),
                    kind: ast::ExpressionKind::Unary { base, operator: Operator::PostDecrement },
                });
            } else if self.peek(Token::Exclamation) && !self.previous_token.1.line_break(&self.token.1) {
                self.push_location(&base.location);
                self.next()?;
                base = Rc::new(ast::Expression {
                    location: self.pop_location(),
                    kind: ast::ExpressionKind::Unary { base, operator: Operator::NonNull },
                });
            // `not in`, `not instanceof`
            } else if self.peek_context_keyword("not") && context.min_precedence.includes(&OperatorPrecedence::Relational) && !self.previous_token.1.line_break(&self.token.1) {
                self.push_location(&base.location);
                self.next()?;
                if self.consume(Token::Instanceof)? {
                    base = self.parse_binary_operator(base, Operator::NotInstanceof, OperatorPrecedence::Relational.add_one().unwrap(), context.clone())?;
                } else {
                    self.expect(Token::In)?;
                    base = self.parse_binary_operator(base, Operator::NotIn, OperatorPrecedence::Relational.add_one().unwrap(), context.clone())?;
                }
            // ConditionalExpression
            } else if self.peek(Token::Question) && context.min_precedence.includes(&OperatorPrecedence::AssignmentAndOther) {
                self.push_location(&base.location);
                self.next()?;
                let consequent = self.parse_expression(ExpressionContext {
                    min_precedence: OperatorPrecedence::AssignmentAndOther,
                    with_type_annotation: false,
                    ..context.clone()
                })?;
                self.expect(Token::Colon)?;
                let alternative = self.parse_expression(ExpressionContext {
                    min_precedence: OperatorPrecedence::AssignmentAndOther,
                    with_type_annotation: true,
                    ..context.clone()
                })?;
                base = Rc::new(ast::Expression {
                    location: self.pop_location(),
                    kind: ast::ExpressionKind::Conditional { test: base, consequent, alternative },
                });
            } else if let Some((required_precedence, operator, right_precedence)) = self.check_binary_operator(context.clone()) {
                if context.min_precedence.includes(&required_precedence) {
                    self.next()?;
                    base = self.parse_binary_operator(base, operator, right_precedence, context.clone())?;
                } else {
                    break;
                }
            // AssignmentExpression
            } else if self.peek(Token::Assign) && context.min_precedence.includes(&OperatorPrecedence::AssignmentAndOther) && context.allow_assignment {
                self.push_location(&base.location);
                self.next()?;
                let left = if matches!(base.kind, ast::ExpressionKind::ArrayInitializer { .. }) || matches!(base.kind, ast::ExpressionKind::ObjectInitializer { .. }) {
                    ast::AssignmentLeft::Destructuring(self.exp_to_destructuring(base)?)
                } else {
                    ast::AssignmentLeft::Expression(base)
                };
                let right = self.parse_expression(ExpressionContext {
                    min_precedence: OperatorPrecedence::AssignmentAndOther,
                    ..context.clone()
                })?;
                base = Rc::new(ast::Expression {
                    location: self.pop_location(),
                    kind: ast::ExpressionKind::Assignment { left, compound: None, right },
                });
            // CompoundAssignment and LogicalAssignment
            } else if let Some(compound) = self.token.0.compound_assignment() {
                if context.min_precedence.includes(&OperatorPrecedence::AssignmentAndOther) && context.allow_assignment {
                    self.push_location(&base.location);
                    self.next()?;
                    let left = if matches!(base.kind, ast::ExpressionKind::ArrayInitializer { .. }) || matches!(base.kind, ast::ExpressionKind::ObjectInitializer { .. }) {
                        ast::AssignmentLeft::Destructuring(self.exp_to_destructuring(base)?)
                    } else {
                        ast::AssignmentLeft::Expression(base)
                    };
                    let right = self.parse_expression(ExpressionContext {
                        min_precedence: OperatorPrecedence::AssignmentAndOther,
                        ..context.clone()
                    })?;
                    base = Rc::new(ast::Expression {
                        location: self.pop_location(),
                        kind: ast::ExpressionKind::Assignment { left, compound: Some(compound), right },
                    });
                } else {
                    break;
                }
            } else if self.peek(Token::Comma) && context.min_precedence.includes(&OperatorPrecedence::List) {
                self.push_location(&base.location);
                self.next()?;
                let right = self.parse_expression(ExpressionContext {
                    min_precedence: OperatorPrecedence::AssignmentAndOther,
                    ..context.clone()
                })?;
                base = Rc::new(ast::Expression {
                    location: self.pop_location(),
                    kind: ast::ExpressionKind::Sequence(base, right),
                });
            } else if self.peek(Token::Colon) && context.with_type_annotation && context.min_precedence.includes(&OperatorPrecedence::Postfix) {
                self.push_location(&base.location);
                self.next()?;
                let type_annotation = self.parse_type_expression()?;
                base = Rc::new(ast::Expression {
                    location: self.pop_location(),
                    kind: ast::ExpressionKind::WithTypeAnnotation { base, type_annotation },
                });
            } else {
                break;
            }
        }

        Ok(base)
    }

    fn parse_binary_operator(&mut self, base: Rc<ast::Expression>, mut operator: Operator, right_precedence: OperatorPrecedence, context: ExpressionContext) -> Result<Rc<ast::Expression>, ParserFailure> {
        // The left operand of a null-coalescing operation must not be
        // a logical AND, XOR or OR operation.
        if operator == Operator::NullCoalescing {
            if let ast::ExpressionKind::Unary { base, operator } = &base.kind {
                if [Operator::LogicalAnd, Operator::LogicalXor, Operator::LogicalOr].contains(&operator) {
                    self.add_syntax_error(&base.location.clone(), DiagnosticKind::IllegalNullishCoalescingLeftOperand, vec![]);
                }
            }
        }

        if operator == Operator::Is && self.consume_context_keyword("not")? {
            operator = Operator::IsNot;
        }

        self.push_location(&base.location);
        let right = self.parse_expression(ExpressionContext {
            min_precedence: right_precedence,
            ..context
        })?;
        Ok(Rc::new(ast::Expression {
            location: self.pop_location(),
            kind: ast::ExpressionKind::Binary { left: base, operator, right },
        }))
    }

    /// Returns either None or Some((required_precedence, operator, right_precedence))
    fn check_binary_operator(&self, context: ExpressionContext) -> Option<(OperatorPrecedence, Operator, OperatorPrecedence)> {
        if let Some(operator) = self.token.0.to_binary_operator() {
            if operator == Operator::In && !context.allow_in {
                return None;
            }
            let (precedence, associativity) = operator.binary_position().unwrap();
            // If associativity is left-to-right, right precedence is `required_precedence` plus one
            let mut right_precedence = precedence;
            if associativity == BinaryAssociativity::LeftToRight {
                right_precedence = right_precedence.add_one().unwrap();
            }

            // Right precedence is bitwise OR for nullish coalescing
            if operator == Operator::NullCoalescing {
                right_precedence = OperatorPrecedence::BitwiseOr;
            }

            Some((precedence, operator, right_precedence))
        } else {
            None
        }
    }

    fn parse_optional_chaining_subexpression(&mut self, base: Rc<ast::Expression>) -> Result<Rc<ast::Expression>, ParserFailure> {
        self.push_location(&base.location);
        self.duplicate_location();
        let mut operation = Rc::new(ast::Expression {
            location: base.location.clone(),
            kind: ast::ExpressionKind::OptionalChainingHost,
        });
        if self.peek(Token::LeftParen) {
            let arguments = self.parse_arguments()?;
            operation = Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::Call { base: operation, arguments }
            });
        } else if self.consume(Token::LeftBracket)? {
            let key = self.parse_expression(ExpressionContext { allow_in: true, min_precedence: OperatorPrecedence::List, ..default() })?;
            self.expect(Token::RightBracket)?;
            operation = Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::BracketsMember { base: operation, key, asdoc: None }
            });
        } else {
            let id = self.parse_qualified_identifier()?;
            operation = Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::DotMember { base: operation, id }
            });
        }

        // Parse postfix subexpressions
        operation = self.parse_subexpressions(operation, ExpressionContext {
            min_precedence: OperatorPrecedence::Postfix,
            ..default()
        })?;

        Ok(Rc::new(ast::Expression {
            location: self.pop_location(),
            kind: ast::ExpressionKind::OptionalChaining { base, operations: operation },
        }))
    }

    fn parse_dot_subexpression(&mut self, base: Rc<ast::Expression>) -> Result<Rc<ast::Expression>, ParserFailure> {
        self.push_location(&base.location);
        if self.peek(Token::LeftParen) {
            let paren_exp = self.parse_paren_list_expression()?;
            if !matches!(paren_exp.kind, ast::ExpressionKind::Sequence(_, _)) && self.peek(Token::ColonColon) {
                let id = self.finish_qualified_identifier(false, paren_exp.clone())?;
                Ok(Rc::new(ast::Expression {
                    location: self.pop_location(),
                    kind: ast::ExpressionKind::DotMember { base, id }
                }))
            } else {
                Ok(Rc::new(ast::Expression {
                    location: self.pop_location(),
                    kind: ast::ExpressionKind::Filter { base, condition: paren_exp }
                }))
            }
        } else if self.consume(Token::Lt)? {
            let mut arguments = vec![];
            arguments.push(self.parse_type_expression()?);
            while self.consume(Token::Comma)? {
                arguments.push(self.parse_type_expression()?);
            }
            self.expect_generics_gt()?;
            Ok(Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::WithTypeArguments { base, arguments }
            }))
        } else {
            let id = self.parse_qualified_identifier()?;
            Ok(Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::DotMember { base, id }
            }))
        }
    }

    fn parse_arrow_function(&mut self, start: Location, context: ArrowFunctionContext) -> Result<Rc<ast::Expression>, ParserFailure> {
        self.expect(Token::FatArrow)?;
        self.push_location(&start);

        // Enter activation
        self.activations.push(Activation::new());

        // Reinterpret left side
        let mut params: Vec<ast::FunctionParam> = vec![];
        let mut return_annotation: Option<Rc<ast::TypeExpression>> = None;
        if let Some(left) = context.left {
            if let ast::ExpressionKind::WithTypeAnnotation { base, type_annotation } = &left.kind {
                params = self.exp_to_function_params(base.clone())?;
                return_annotation = Some(type_annotation.clone());
            } else {
                params = self.exp_to_function_params(left.clone())?;
            }
        }

        // Validate parameter list
        self.validate_function_parameter_list(&params)?;

        // Body
        let body: ast::FunctionBody = if self.peek(Token::LeftBrace) {
            ast::FunctionBody::Block(self.parse_block(DirectiveContext::Default)?)
        } else {
            ast::FunctionBody::Expression(self.parse_expression(ExpressionContext {
                min_precedence: OperatorPrecedence::AssignmentAndOther,
                ..context.right_context
            })?)
        };

        // Exit activation
        let activation = self.activations.pop().unwrap();

        Ok(Rc::new(ast::Expression {
            location: self.pop_location(),
            kind: ast::ExpressionKind::ArrowFunction(Rc::new(ast::FunctionCommon {
                flags: if activation.uses_yield { ast::FunctionFlags::YIELD } else { ast::FunctionFlags::empty() }
                    | if activation.uses_await { ast::FunctionFlags::AWAIT } else { ast::FunctionFlags::empty() },
                params,
                return_annotation,
                body: Some(body),
            })),
        }))
    }

    fn exp_to_function_params(&mut self, exp: Rc<ast::Expression>) -> Result<Vec<ast::FunctionParam>, ParserFailure> {
        if let ast::ExpressionKind::EmptyParen = exp.kind {
            Ok(vec![])
        } else if let ast::ExpressionKind::Paren(exp) = &exp.kind {
            self.seq_exp_to_function_params(exp.clone())
        } else {
            self.seq_exp_to_function_params(exp.clone())
        }
    }

    fn seq_exp_to_function_params(&mut self, exp: Rc<ast::Expression>) -> Result<Vec<ast::FunctionParam>, ParserFailure> {
        if let ast::ExpressionKind::Sequence(left, right) = &exp.kind {
            let mut params = self.seq_exp_to_function_params(left.clone())?;
            params.push(self.exp_to_function_param(right.clone())?);
            Ok(params)
        } else {
            Ok(vec![self.exp_to_function_param(exp.clone())?])
        }
    }

    fn exp_to_function_param(&mut self, exp: Rc<ast::Expression>) -> Result<ast::FunctionParam, ParserFailure> {
        if let ast::ExpressionKind::Rest(subexp) = &exp.kind {
            Ok(ast::FunctionParam {
                location: exp.location.clone(),
                kind: ast::FunctionParamKind::Rest,
                binding: ast::VariableBinding {
                    pattern: self.exp_to_destructuring(subexp.clone())?,
                    init: None,
                },
            })
        } else if let ast::ExpressionKind::Assignment { left, compound, right } = &exp.kind {
            let left = match left {
                ast::AssignmentLeft::Destructuring(destructuring) => destructuring.clone(),
                ast::AssignmentLeft::Expression(exp) => self.exp_to_destructuring(exp.clone())?,
            };
            if compound.is_some() {
                self.add_syntax_error(&exp.location.clone(), DiagnosticKind::MalformedArrowFunctionElement, vec![]);
                return Err(ParserFailure);
            }
            Ok(ast::FunctionParam {
                location: exp.location.clone(),
                kind: ast::FunctionParamKind::Optional,
                binding: ast::VariableBinding {
                    pattern: left,
                    init: Some(right.clone()),
                },
            })
        } else {
            Ok(ast::FunctionParam {
                location: exp.location.clone(),
                kind: ast::FunctionParamKind::Required,
                binding: ast::VariableBinding {
                    pattern: self.exp_to_destructuring(exp.clone())?,
                    init: None,
                },
            })
        }
    }

    fn exp_to_destructuring(&mut self, exp: Rc<ast::Expression>) -> Result<Rc<ast::Destructuring>, ParserFailure> {
        if let ast::ExpressionKind::WithTypeAnnotation { base, type_annotation } = &exp.kind {
            self.exp_to_destructuring_1(base.clone(), Some(type_annotation.clone()), exp.location.clone())
        } else {
            self.exp_to_destructuring_1(exp.clone(), None, exp.location.clone())
        }
    }

    fn exp_to_destructuring_1(&mut self, exp: Rc<ast::Expression>, type_annotation: Option<Rc<ast::TypeExpression>>, location: Location) -> Result<Rc<ast::Destructuring>, ParserFailure> {
        if let ast::ExpressionKind::Unary { base, operator } = &exp.kind {
            if *operator == Operator::NonNull {
                return self.exp_to_destructuring_2(base.clone(), true, type_annotation, location);
            }
        }
        self.exp_to_destructuring_2(exp.clone(), false, type_annotation, location)
    }

    fn exp_to_destructuring_2(&mut self, exp: Rc<ast::Expression>, non_null: bool, type_annotation: Option<Rc<ast::TypeExpression>>, location: Location) -> Result<Rc<ast::Destructuring>, ParserFailure> {
        let destructuring_kind: ast::DestructuringKind;
        match &exp.kind {
            ast::ExpressionKind::Id(id) => {
                if let Some(name) = id.to_identifier() {
                    destructuring_kind = ast::DestructuringKind::Binding { name };
                } else {
                    self.add_syntax_error(&exp.location.clone(), DiagnosticKind::MalformedDestructuring, vec![]);
                    return Err(ParserFailure);
                }
            },
            ast::ExpressionKind::ArrayInitializer { elements, asdoc: _ } => {
                destructuring_kind = self.array_initializer_to_destructuring_kind(elements.clone())?;
            },
            ast::ExpressionKind::ObjectInitializer { fields } => {
                destructuring_kind = self.object_initializer_to_destructuring_kind(fields.clone())?;
            },
            _ => {
                self.add_syntax_error(&exp.location.clone(), DiagnosticKind::MalformedDestructuring, vec![]);
                return Err(ParserFailure);
            },
        }
        Ok(Rc::new(ast::Destructuring {
            location,
            kind: destructuring_kind,
            non_null,
            type_annotation,
        }))
    }

    fn array_initializer_to_destructuring_kind(&mut self, elements: Vec<Option<Rc<ast::Expression>>>) -> Result<ast::DestructuringKind, ParserFailure> {
        let mut result_items: Vec<Option<ast::ArrayDestructuringItem>> = vec![];
        for element in elements {
            if element.is_none() {
                result_items.push(None);
                continue;
            }
            let element = element.unwrap();
            if let ast::ExpressionKind::Rest(subexp) = &element.kind {
                result_items.push(Some(ast::ArrayDestructuringItem::Rest(self.exp_to_destructuring(subexp.clone())?, element.location.clone())));
                continue;
            }
            result_items.push(Some(ast::ArrayDestructuringItem::Pattern(self.exp_to_destructuring(element.clone())?)));
        }
        Ok(ast::DestructuringKind::Array(result_items))
    }

    fn object_initializer_to_destructuring_kind(&mut self, fields: Vec<Rc<ast::ObjectField>>) -> Result<ast::DestructuringKind, ParserFailure> {
        let mut result_fields: Vec<Rc<ast::RecordDestructuringField>> = vec![];
        for field in fields {
            let ast::ObjectField::Field { ref key, destructuring_non_null, ref value } = *field else {
                self.add_syntax_error(&field.location(), DiagnosticKind::UnsupportedDestructuringRest, vec![]);
                continue;
            };
            let alias = if let Some(v) = value { Some(self.exp_to_destructuring(v.clone())?) } else { None };
            result_fields.push(Rc::new(ast::RecordDestructuringField {
                location: field.location(),
                key: key.clone(),
                non_null: destructuring_non_null,
                alias,
            }));
        }
        Ok(ast::DestructuringKind::Record(result_fields))
    }

    /// Ensures the parameter list consists of zero or more required parameters followed by
    /// zero or more optional parameters optionally followed by a rest parameter.
    fn validate_function_parameter_list(&mut self, params: &Vec<ast::FunctionParam>) -> Result<(), ParserFailure> {
        let mut least_kind = ast::FunctionParamKind::Required; 
        let mut has_rest = false;
        for param in params {
            if !least_kind.may_be_followed_by(param.kind) {
                self.add_syntax_error(&param.location.clone(), DiagnosticKind::WrongParameterPosition, vec![]);
            }
            least_kind = param.kind;
            if param.kind == ast::FunctionParamKind::Rest && has_rest {
                self.add_syntax_error(&param.location.clone(), DiagnosticKind::DuplicateRestParameter, vec![]);
            }
            has_rest = param.kind == ast::FunctionParamKind::Rest;
        }
        Ok(())
    }

    fn parse_opt_start_expression(&mut self, context: ExpressionContext) -> Result<Option<Rc<ast::Expression>>, ParserFailure> {
        if let Token::Identifier(id) = self.token.0.clone() {
            let id_location = self.token_location();
            self.next()?;

            // EmbedExpression
            if let Token::StringLiteral(string_value) = &self.token.0 {
                if id == "embed" && self.previous_token.1.character_count() == "embed".len() {
                    return Ok(Some(self.finish_embed_expression(id_location, string_value.clone())?));
                }
            }

            let id = Rc::new(ast::Expression {
                location: id_location.clone(),
                kind: ast::ExpressionKind::Id(ast::QualifiedIdentifier {
                    attribute: false,
                    qualifier: None,
                    name: ast::IdentifierOrBrackets::Id(id, id_location.clone()),
                }),
            });
            if self.peek(Token::ColonColon) {
                self.push_location(&id_location.clone());
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
            self.next()?;
            Ok(Some(Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::Null,
            })))
        } else if self.peek(Token::False) {
            self.mark_location();
            self.next()?;
            Ok(Some(Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::Boolean(false),
            })))
        } else if self.peek(Token::True) {
            self.mark_location();
            self.next()?;
            Ok(Some(Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::Boolean(true),
            })))
        } else if let Token::NumericLiteral(n) = self.token.0 {
            self.mark_location();
            self.next()?;
            Ok(Some(Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::Numeric(n),
            })))
        } else if let Token::StringLiteral(ref s) = self.token.0.clone() {
            self.mark_location();
            self.next()?;
            Ok(Some(Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::String(s.clone()),
            })))
        } else if self.peek(Token::This) {
            self.mark_location();
            self.next()?;
            Ok(Some(Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::This,
            })))
        } else if let Token::RegExpLiteral { ref body, ref flags } = self.token.0.clone() {
            self.mark_location();
            self.next()?;
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
            self.next()?;
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
        // Parentheses
        } else if self.peek(Token::LeftParen) {
            return Ok(Some(self.parse_paren_list_expr_or_qual_id()?));
        // `*`
        } else if self.peek(Token::Times) {
            let id_location = self.token_location();
            self.next()?;
            let id = Rc::new(ast::Expression {
                location: id_location.clone(),
                kind: ast::ExpressionKind::Id(ast::QualifiedIdentifier {
                    attribute: false,
                    qualifier: None,
                    name: ast::IdentifierOrBrackets::Id("*".into(), id_location.clone()),
                }),
            });
            if self.peek(Token::ColonColon) {
                self.push_location(&id_location.clone());
                let id = self.finish_qualified_identifier(false, id)?;
                Ok(Some(Rc::new(ast::Expression {
                    location: self.pop_location(),
                    kind: ast::ExpressionKind::Id(id),
                })))
            } else {
                Ok(Some(id))
            }
        // XMLList, XMLElement, XMLMarkup
        } else if self.peek(Token::Lt) {
            if let Some(token) = self.tokenizer.scan_xml_markup(self.token_location())? {
                self.token = token;
            }
            let start = self.token_location();
            if let Token::XmlMarkup(content) = &self.token.0.clone() {
                self.mark_location();
                self.next()?;
                Ok(Some(Rc::new(ast::Expression {
                    location: self.pop_location(),
                    kind: ast::ExpressionKind::XmlMarkup(content.clone()),
                })))
            } else {
                Ok(Some(self.parse_xml_element_or_xml_list(start)?))
            }
        // `...`
        } else if self.peek(Token::Ellipsis) && context.min_precedence.includes(&OperatorPrecedence::AssignmentAndOther) {
            self.mark_location();
            self.next()?;
            let expr = self.parse_expression(ExpressionContext {
                allow_in: true,
                min_precedence: OperatorPrecedence::AssignmentAndOther.add_one().unwrap(),
                ..default()
            })?;
            Ok(Some(Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::Rest(expr),
            })))
        // ArrayInitializer
        } else if self.peek(Token::LeftBracket) {
            Ok(Some(self.parse_array_initializer()?))
        // NewExpression, VectorInitializer
        } else if self.peek(Token::New) && context.min_precedence.includes(&OperatorPrecedence::Unary) {
            let start = self.token_location();
            self.next()?;
            if self.peek(Token::Lt) {
                Ok(Some(self.parse_vector_initializer(start)?))
            } else {
                Ok(Some(self.parse_new_expression(start)?))
            }
        } else if self.peek(Token::LeftBrace) {
            Ok(Some(self.parse_object_initializer()?))
        } else if self.peek(Token::Function) {
            Ok(Some(self.parse_function_expression()?))
        // SuperExpression
        } else if self.peek(Token::Super) && context.min_precedence.includes(&OperatorPrecedence::Postfix) {
            Ok(Some(self.parse_super_expression_followed_by_property_operator()?))
        // AwaitExpression
        } else if self.peek(Token::Await) && context.min_precedence.includes(&OperatorPrecedence::Postfix) {
            self.mark_location();
            let operator_token = self.token.clone();
            self.next()?;
            let base = self.parse_expression(ExpressionContext {
                allow_in: true,
                min_precedence: OperatorPrecedence::Unary,
                ..default()
            })?;
            if let Some(activation) = self.activations.last_mut() {
                activation.uses_await = true;
            } else {
                self.add_syntax_error(&operator_token.1, DiagnosticKind::NotAllowedHere, diagnostic_arguments![Token(operator_token.0)]);
            }
            Ok(Some(Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::Unary { base, operator: Operator::Await }
            })))
        // YieldExpression
        } else if self.peek(Token::Yield) && context.min_precedence.includes(&OperatorPrecedence::AssignmentAndOther) {
            self.mark_location();
            let operator_token = self.token.clone();
            self.next()?;
            let base = self.parse_expression(ExpressionContext {
                allow_in: true,
                min_precedence: OperatorPrecedence::AssignmentAndOther,
                ..default()
            })?;
            if let Some(activation) = self.activations.last_mut() {
                activation.uses_yield = true;
            } else {
                self.add_syntax_error(&operator_token.1, DiagnosticKind::NotAllowedHere, diagnostic_arguments![Token(operator_token.0)]);
            }
            Ok(Some(Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::Unary { base, operator: Operator::Yield }
            })))
        // Miscellaneous prefix unary expressions
        } else if let Some((operator, subexp_precedence)) = self.check_prefix_operator() {
            if context.min_precedence.includes(&OperatorPrecedence::Unary) {
                self.mark_location();
                self.next()?;
                let base = self.parse_expression(ExpressionContext { min_precedence: subexp_precedence, ..default() })?;
                Ok(Some(Rc::new(ast::Expression {
                    location: self.pop_location(),
                    kind: ast::ExpressionKind::Unary { base, operator }
                })))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    fn check_prefix_operator(&self) -> Option<(Operator, OperatorPrecedence)> {
        match self.token.0 {
            Token::Delete => Some((Operator::Delete, OperatorPrecedence::Postfix)),
            Token::Void => Some((Operator::Void, OperatorPrecedence::Unary)),
            Token::Typeof => Some((Operator::Typeof, OperatorPrecedence::Unary)),
            Token::Increment => Some((Operator::PreIncrement, OperatorPrecedence::Postfix)),
            Token::Decrement => Some((Operator::PreDecrement, OperatorPrecedence::Postfix)),
            Token::Plus => Some((Operator::Positive, OperatorPrecedence::Unary)),
            Token::Minus => Some((Operator::Negative, OperatorPrecedence::Unary)),
            Token::BitwiseNot => Some((Operator::BitwiseNot, OperatorPrecedence::Unary)),
            Token::Exclamation => Some((Operator::LogicalNot, OperatorPrecedence::Unary)),
            _ => None,
        }
    }

    fn parse_function_expression(&mut self) -> Result<Rc<ast::Expression>, ParserFailure> {
        self.mark_location();
        self.next()?;
        let mut name = None;
        if let Token::Identifier(id) = self.token.0.clone() {
            name = Some((id, self.token.1.clone()));
            self.next()?;
        }
        let (common, _) = self.parse_function_common(true, DirectiveContext::Default)?;
        Ok(Rc::new(ast::Expression {
            location: self.pop_location(),
            kind: ast::ExpressionKind::Function {
                name,
                common,
            },
        }))
    }

    fn parse_function_common(&mut self, function_expr: bool, block_context: DirectiveContext) -> Result<(Rc<ast::FunctionCommon>, Option<ast::GenericsWhere>), ParserFailure> {
        self.expect(Token::LeftParen)?;
        let mut params: Vec<ast::FunctionParam> = vec![];
        while !self.peek(Token::RightParen) {
            self.mark_location();
            let rest = self.consume(Token::Ellipsis)?;
            let binding = self.parse_variable_binding(true)?;
            let has_initializer = binding.init.is_some();
            let location = self.pop_location();
            if rest && has_initializer {
                self.add_syntax_error(&location.clone(), DiagnosticKind::MalformedRestParameter, vec![]);
            }
            let param = ast::FunctionParam {
                location,
                binding,
                kind: if rest {
                    ast::FunctionParamKind::Rest
                } else if has_initializer {
                    ast::FunctionParamKind::Optional
                } else {
                    ast::FunctionParamKind::Required
                },
            };
            params.push(param);
            if !self.consume(Token::Comma)? {
                break;
            }
        }
        self.expect(Token::RightParen)?;
        self.validate_function_parameter_list(&params)?;

        let return_annotation = if self.consume(Token::Colon)? { Some(self.parse_type_expression()?) } else { None };
        let where_clause = if !function_expr { self.parse_generics_where_clause()? } else { None };

        // Enter activation
        self.activations.push(Activation::new());

        // Body
        let body = if self.peek(Token::LeftBrace) {
            Some(ast::FunctionBody::Block(self.parse_block(block_context)?))
        } else {
            None
        };

        // Body is required by function expressions
        if body.is_none() && function_expr {
            self.expect(Token::LeftBrace)?;
        }

        // Exit activation
        let activation = self.activations.pop().unwrap();

        Ok((Rc::new(ast::FunctionCommon {
            flags: if activation.uses_yield { ast::FunctionFlags::YIELD } else { ast::FunctionFlags::empty() }
                | if activation.uses_await { ast::FunctionFlags::AWAIT } else { ast::FunctionFlags::empty() },
            params,
            return_annotation,
            body,
        }), where_clause))
    }

    fn parse_object_initializer(&mut self) -> Result<Rc<ast::Expression>, ParserFailure> {
        self.mark_location();
        self.expect(Token::LeftBrace)?;
        let mut fields: Vec<Rc<ast::ObjectField>> = vec![];
        while !self.peek(Token::RightBrace) {
            fields.push(self.parse_object_field()?);
            if !self.consume(Token::Comma)? {
                break;
            }
        }
        self.expect(Token::RightBrace)?;

        Ok(Rc::new(ast::Expression {
            location: self.pop_location(),
            kind: ast::ExpressionKind::ObjectInitializer { fields },
        }))
    }

    fn parse_object_field(&mut self) -> Result<Rc<ast::ObjectField>, ParserFailure> {
        if self.peek(Token::Ellipsis) {
            self.mark_location();
            self.next()?;
            let subexp = self.parse_expression(ExpressionContext {
                allow_in: true,
                min_precedence: OperatorPrecedence::AssignmentAndOther.add_one().unwrap(),
                ..default()
            })?;
            return Ok(Rc::new(ast::ObjectField::Rest(subexp, self.pop_location())));
        }

        // Parse the key
        let key = self.parse_object_key()?;

        let destructuring_non_null = self.consume(Token::Exclamation)?;
        let mut value = None;

        if self.consume(Token::Colon)? {
            value = Some(self.parse_expression(ExpressionContext {
                allow_in: true,
                min_precedence: OperatorPrecedence::AssignmentAndOther,
                ..default()
            })?);
        } else if !matches!(key.0, ast::ObjectKey::Id(_)) {
            self.expect(Token::Colon)?;
        }

        Ok(Rc::new(ast::ObjectField::Field {
            key,
            destructuring_non_null,
            value,
        }))
    }

    fn parse_vector_initializer(&mut self, start: Location) -> Result<Rc<ast::Expression>, ParserFailure> {
        self.push_location(&start);
        self.expect(Token::Lt)?;
        let element_type = self.parse_type_expression()?;
        self.expect_generics_gt()?;

        let mut elements: Vec<Rc<ast::Expression>> = vec![];
        while !self.peek(Token::RightBracket) {
            elements.push(self.parse_expression(ExpressionContext {
                allow_in: true,
                min_precedence: OperatorPrecedence::AssignmentAndOther,
                ..default()
            })?);
            if !self.consume(Token::Comma)? {
                break;
            }
        }
        self.expect(Token::RightBracket)?;
        Ok(Rc::new(ast::Expression {
            location: self.pop_location(),
            kind: ast::ExpressionKind::VectorInitializer { element_type, elements },
        }))
    }

    fn parse_new_expression(&mut self, start: Location) -> Result<Rc<ast::Expression>, ParserFailure> {
        self.push_location(&start);
        let base = self.parse_new_subexpression()?;
        let arguments = if self.peek(Token::LeftParen) { Some(self.parse_arguments()?) } else { None };
        Ok(Rc::new(ast::Expression {
            location: self.pop_location(),
            kind: ast::ExpressionKind::New { base, arguments },
        }))
    }

    fn parse_new_expression_start(&mut self) -> Result<Rc<ast::Expression>, ParserFailure> {
        if self.peek(Token::New) {
            let start = self.token_location();
            self.next()?;
            self.parse_new_expression(start)
        } else if self.peek(Token::Super) {
            self.parse_super_expression_followed_by_property_operator()
        } else {
            self.parse_primary_expression()
        }
    }

    fn parse_super_expression_followed_by_property_operator(&mut self) -> Result<Rc<ast::Expression>, ParserFailure> {
        self.mark_location();
        self.duplicate_location();
        self.next()?;
        let arguments = if self.peek(Token::LeftParen) { Some(self.parse_arguments()?) } else { None };
        let super_expr = Rc::new(ast::Expression {
            location: self.pop_location(),
            kind: ast::ExpressionKind::Super(arguments),
        });

        if self.consume(Token::LeftBracket)? {
            let key = self.parse_expression(ExpressionContext { allow_in: true, min_precedence: OperatorPrecedence::List, ..default() })?;
            self.expect(Token::RightBracket)?;
            Ok(Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::BracketsMember { base: super_expr, key, asdoc: None },
            }))
        } else {
            self.expect(Token::Dot)?;
            let id = self.parse_qualified_identifier()?;
            Ok(Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::DotMember { base: super_expr, id },
            }))
        }
    }

    fn parse_arguments(&mut self) -> Result<Vec<Rc<ast::Expression>>, ParserFailure> {
        self.expect(Token::LeftParen)?;
        let mut arguments = vec![];
        while !self.peek(Token::RightParen) {
            arguments.push(self.parse_expression(ExpressionContext {
                allow_in: true,
                min_precedence: OperatorPrecedence::AssignmentAndOther,
                ..default()
            })?);
            if !self.consume(Token::Comma)? {
                break;
            }
        }
        self.expect(Token::RightParen)?;
        Ok(arguments)
    }

    fn parse_new_subexpression(&mut self) -> Result<Rc<ast::Expression>, ParserFailure> {
        let mut base = self.parse_new_expression_start()?;
        loop {
            if self.consume(Token::LeftBracket)? {
                self.push_location(&base.location);
                let key = self.parse_expression(ExpressionContext { allow_in: true, min_precedence: OperatorPrecedence::List, ..default() })?;
                self.expect(Token::RightBracket)?;
                base = Rc::new(ast::Expression {
                    location: self.pop_location(),
                    kind: ast::ExpressionKind::BracketsMember { base, key, asdoc: None },
                });
            } else if self.consume(Token::Dot)? {
                self.push_location(&base.location);
                let id = self.parse_qualified_identifier()?;
                base = Rc::new(ast::Expression {
                    location: self.pop_location(),
                    kind: ast::ExpressionKind::DotMember { base, id },
                });
            } else {
                break;
            }
        }
        Ok(base)
    }

    fn parse_primary_expression(&mut self) -> Result<Rc<ast::Expression>, ParserFailure> {
        if let Token::Identifier(id) = self.token.0.clone() {
            let id_location = self.token_location();
            self.next()?;

            // EmbedExpression
            if let Token::StringLiteral(string_value) = &self.token.0.clone() {
                if id == "embed" && self.previous_token.1.character_count() == "embed".len() {
                    return self.finish_embed_expression(id_location, string_value.clone());
                }
            }

            let id = Rc::new(ast::Expression {
                location: id_location.clone(),
                kind: ast::ExpressionKind::Id(ast::QualifiedIdentifier {
                    attribute: false,
                    qualifier: None,
                    name: ast::IdentifierOrBrackets::Id(id, id_location.clone()),
                }),
            });
            if self.peek(Token::ColonColon) {
                self.push_location(&id_location.clone());
                let id = self.finish_qualified_identifier(false, id)?;
                Ok(Rc::new(ast::Expression {
                    location: self.pop_location(),
                    kind: ast::ExpressionKind::Id(id),
                }))
            } else {
                Ok(id)
            }
        } else if self.peek(Token::Null) {
            self.mark_location();
            self.next()?;
            Ok(Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::Null,
            }))
        } else if self.peek(Token::False) {
            self.mark_location();
            self.next()?;
            Ok(Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::Boolean(false),
            }))
        } else if self.peek(Token::True) {
            self.mark_location();
            self.next()?;
            Ok(Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::Boolean(true),
            }))
        } else if let Token::NumericLiteral(n) = self.token.0.clone() {
            self.mark_location();
            self.next()?;
            Ok(Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::Numeric(n),
            }))
        } else if let Token::StringLiteral(ref s) = self.token.0.clone() {
            self.mark_location();
            self.next()?;
            Ok(Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::String(s.clone()),
            }))
        } else if self.peek(Token::This) {
            self.mark_location();
            self.next()?;
            Ok(Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::This,
            }))
        } else if let Token::RegExpLiteral { ref body, ref flags } = self.token.0.clone() {
            self.mark_location();
            self.next()?;
            Ok(Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::RegExp { body: body.clone(), flags: flags.clone() },
            }))
        // `@`
        } else if self.peek(Token::Attribute) {
            self.mark_location();
            let id = self.parse_qualified_identifier()?;
            Ok(Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::Id(id),
            }))
        // `public`, `private`, `protected`, `internal`
        } else if let Some(reserved_ns) = self.peek_reserved_namespace() {
            self.mark_location();
            self.duplicate_location();
            self.next()?;
            let rns = Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::ReservedNamespace(reserved_ns),
            });
            if self.peek(Token::ColonColon) {
                let id = self.finish_qualified_identifier(false, rns)?;
                Ok(Rc::new(ast::Expression {
                    location: self.pop_location(),
                    kind: ast::ExpressionKind::Id(id),
                }))
            } else {
                self.pop_location();
                Ok(rns)
            }
        // Parentheses
        } else if self.peek(Token::LeftParen) {
            Ok(self.parse_paren_list_expr_or_qual_id()?)
        // `*`
        } else if self.peek(Token::Times) {
            let id_location = self.token_location();
            self.next()?;
            let id = Rc::new(ast::Expression {
                location: id_location.clone(),
                kind: ast::ExpressionKind::Id(ast::QualifiedIdentifier {
                    attribute: false,
                    qualifier: None,
                    name: ast::IdentifierOrBrackets::Id("*".into(), id_location.clone()),
                }),
            });
            if self.peek(Token::ColonColon) {
                self.push_location(&id_location.clone());
                let id = self.finish_qualified_identifier(false, id)?;
                Ok(Rc::new(ast::Expression {
                    location: self.pop_location(),
                    kind: ast::ExpressionKind::Id(id),
                }))
            } else {
                Ok(id)
            }
        // XMLList, XMLElement, XMLMarkup
        } else if self.peek(Token::Lt) {
            if let Some(token) = self.tokenizer.scan_xml_markup(self.token_location())? {
                self.token = token;
            }
            let start = self.token_location();
            if let Token::XmlMarkup(content) = &self.token.0.clone() {
                self.mark_location();
                self.next()?;
                Ok(Rc::new(ast::Expression {
                    location: self.pop_location(),
                    kind: ast::ExpressionKind::XmlMarkup(content.clone()),
                }))
            } else {
                Ok(self.parse_xml_element_or_xml_list(start)?)
            }
        // ArrayInitializer
        } else if self.peek(Token::LeftBracket) {
            Ok(self.parse_array_initializer()?)
        } else if self.peek(Token::LeftBrace) {
            Ok(self.parse_object_initializer()?)
        } else if self.peek(Token::Function) {
            Ok(self.parse_function_expression()?)
        } else {
            self.add_syntax_error(&self.token_location(), DiagnosticKind::ExpectedExpression, diagnostic_arguments![Token(self.token.0.clone())]);
            Err(ParserFailure)
        }
    }

    fn finish_embed_expression(&mut self, start: Location, source: String) -> Result<Rc<ast::Expression>, ParserFailure> {
        self.push_location(&start);
        self.next()?;
        let type_annotation = if self.consume(Token::Colon)? {
            Some(self.parse_type_expression()?)
        } else {
            None
        };
        return Ok(Rc::new(ast::Expression {
            location: self.pop_location(),
            kind: ast::ExpressionKind::Embed {
                source,
                type_annotation,
            },
        }));
    }

    fn parse_array_initializer(&mut self) -> Result<Rc<ast::Expression>, ParserFailure> {
        self.mark_location();

        // ASDoc
        let asdoc = self.parse_asdoc()?;

        self.expect(Token::LeftBracket)?;

        let mut elements: Vec<Option<Rc<ast::Expression>>> = vec![];

        while !self.peek(Token::RightBracket) {
            let mut ellipses = false;
            while self.consume(Token::Comma)? {
                elements.push(None);
                ellipses = true;
            }
            if !ellipses  {
                elements.push(Some(self.parse_expression(ExpressionContext {
                    allow_in: true,
                    min_precedence: OperatorPrecedence::AssignmentAndOther,
                    ..default()
                })?));
            }
            if !self.consume(Token::Comma)? {
                break;
            }
        }
        self.expect(Token::RightBracket)?;
        Ok(Rc::new(ast::Expression {
            location: self.pop_location(),
            kind: ast::ExpressionKind::ArrayInitializer { asdoc, elements },
        }))
    }

    fn parse_xml_element_or_xml_list(&mut self, start: Location) -> Result<Rc<ast::Expression>, ParserFailure> {
        self.next_ie_xml_tag()?;
        if self.consume_and_ie_xml_content(Token::Gt)? {
            self.push_location(&start);
            let content = self.parse_xml_content()?;
            self.expect_and_ie_xml_tag(Token::XmlLtSlash)?;
            self.expect(Token::Gt)?;
            return Ok(Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::XmlList(content),
            }));
        }

        self.push_location(&start);
        let element = self.parse_xml_element(start, true)?;
        return Ok(Rc::new(ast::Expression {
            location: self.pop_location(),
            kind: ast::ExpressionKind::XmlElement(element),
        }));
    }

    /// Parses XMLElement starting from its XMLTagContent.
    fn parse_xml_element(&mut self, start: Location, ends_at_ie_div: bool) -> Result<ast::XmlElement, ParserFailure> {
        self.push_location(&start);
        let opening_tag_name = self.parse_xml_tag_name()?;
        let mut attributes: Vec<ast::XmlAttributeOrExpression> = vec![];
        while self.consume_and_ie_xml_tag(Token::XmlWhitespace)? {
            if self.consume(Token::LeftBrace)? {
                let expr = self.parse_expression(ExpressionContext { allow_in: true, min_precedence: OperatorPrecedence::AssignmentAndOther, ..default() })?;
                self.expect_and_ie_xml_tag(Token::RightBrace)?;
                attributes.push(ast::XmlAttributeOrExpression::Expression(expr));
            } else if matches!(self.token.0, Token::XmlName(_)) {
                let name = self.parse_xml_name()?;
                self.consume_and_ie_xml_tag(Token::XmlWhitespace)?;
                self.expect_and_ie_xml_tag(Token::Assign)?;
                self.consume_and_ie_xml_tag(Token::XmlWhitespace)?;
                let value: ast::XmlAttributeValueOrExpression;
                if self.consume(Token::LeftBrace)? {
                    let expr = self.parse_expression(ExpressionContext { allow_in: true, min_precedence: OperatorPrecedence::AssignmentAndOther, ..default() })?;
                    self.expect_and_ie_xml_tag(Token::RightBrace)?;
                    value = ast::XmlAttributeValueOrExpression::Expression(expr);
                } else {
                    value = ast::XmlAttributeValueOrExpression::Value(self.parse_xml_attribute_value()?);
                }
                attributes.push(ast::XmlAttributeOrExpression::Attribute(ast::XmlAttribute { name, value }));
            } else {
                break;
            }
        }

        let mut content: Vec<ast::XmlElementContent> = vec![];
        let mut closing_tag_name: Option<ast::XmlTagName> = None;

        let is_empty;

        if ends_at_ie_div {
            is_empty = self.consume(Token::XmlSlashGt)?;
        } else {
            is_empty = self.consume_and_ie_xml_content(Token::XmlSlashGt)?;
        }

        if !is_empty {
            self.expect_and_ie_xml_content(Token::Gt)?;
            content = self.parse_xml_content()?;
            self.expect_and_ie_xml_tag(Token::XmlLtSlash)?;
            closing_tag_name = Some(self.parse_xml_tag_name()?);
            self.consume_and_ie_xml_tag(Token::XmlWhitespace)?;
            if ends_at_ie_div {
                self.expect(Token::Gt)?;
            } else {
                self.expect_and_ie_xml_content(Token::Gt)?;
            }
        }

        Ok(XmlElement {
            location: self.pop_location(),
            opening_tag_name,
            attributes,
            content,
            closing_tag_name,
        })
    }

    fn parse_xml_attribute_value(&mut self) -> Result<String, ParserFailure> {
        if let Token::XmlAttributeValue(value) = self.token.0.clone() {
            self.next_ie_xml_tag()?;
            return Ok(value);
        } else {
            self.add_syntax_error(&self.token_location(), DiagnosticKind::ExpectedXmlAttributeValue, diagnostic_arguments![Token(self.token.0.clone())]);
            Err(ParserFailure)
        }
    }

    fn parse_xml_tag_name(&mut self) -> Result<ast::XmlTagName, ParserFailure> {
        if self.consume(Token::LeftBrace)? {
            let expr = self.parse_expression(ExpressionContext {
                allow_in: true,
                min_precedence: OperatorPrecedence::AssignmentAndOther,
                ..default()
            })?;
            self.expect_and_ie_xml_tag(Token::RightBrace)?;
            Ok(ast::XmlTagName::Expression(expr))
        } else {
            Ok(ast::XmlTagName::Name(self.parse_xml_name()?))
        }
    }

    fn parse_xml_name(&mut self) -> Result<(String, Location), ParserFailure> {
        if let Token::XmlName(name) = self.token.0.clone() {
            let name_location = self.token_location();
            self.next_ie_xml_tag()?;
            return Ok((name, name_location));
        } else {
            self.add_syntax_error(&self.token_location(), DiagnosticKind::ExpectedXmlName, diagnostic_arguments![Token(self.token.0.clone())]);
            Err(ParserFailure)
        }
    }

    /// Parses XMLContent until a `</` token.
    fn parse_xml_content(&mut self) -> Result<Vec<ast::XmlElementContent>, ParserFailure> {
        let mut content = vec![];
        while !self.peek(Token::XmlLtSlash) {
            if self.consume(Token::LeftBrace)? {
                let expr = self.parse_expression(ExpressionContext { allow_in: true, min_precedence: OperatorPrecedence::AssignmentAndOther, ..default() })?;
                self.expect_and_ie_xml_content(Token::RightBrace)?;
                content.push(ast::XmlElementContent::Expression(expr));
            } else if let Token::XmlMarkup(markup) = self.token.0.clone() {
                let location = self.token_location();
                self.next_ie_xml_content()?;
                content.push(ast::XmlElementContent::Markup(markup, location));
            } else if let Token::XmlText(text) = self.token.0.clone() {
                let location = self.token_location();
                self.next_ie_xml_content()?;
                content.push(ast::XmlElementContent::Text(text, location));
            } else if self.consume_and_ie_xml_tag(Token::Lt)? {
                let start = self.token_location();
                let element = self.parse_xml_element(start, false)?;
                content.push(ast::XmlElementContent::Element(element));
            } else {
                self.expect_and_ie_xml_content(Token::XmlLtSlash)?;
            }
        }
        Ok(content)
    }

    fn finish_paren_list_expr_or_qual_id(&mut self, start: Location, left: Rc<ast::Expression>) -> Result<Rc<ast::Expression>, ParserFailure> {
        if self.peek(Token::ColonColon) && !matches!(left.kind, ast::ExpressionKind::Sequence(_, _)) {
            self.push_location(&start);
            let id = self.finish_qualified_identifier(false, left)?;
            return Ok(Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::Id(id),
            }));
        }
        self.push_location(&start);
        return Ok(Rc::new(ast::Expression {
            location: self.pop_location(),
            kind: ast::ExpressionKind::Paren(left),
        }));
    }

    /// Parses either a ParenListExpression, (), or a QualifiedIdentifier
    fn parse_paren_list_expr_or_qual_id(&mut self) -> Result<Rc<ast::Expression>, ParserFailure> {
        let start = self.token_location();
        self.expect(Token::LeftParen)?;

        if self.peek(Token::RightParen) {
            self.push_location(&start);
            self.next()?;
            return Ok(Rc::new(ast::Expression {
                location: self.pop_location(),
                kind: ast::ExpressionKind::EmptyParen,
            }));
        }

        let expr = self.parse_expression(ExpressionContext {
            min_precedence: OperatorPrecedence::List,
            allow_in: true,
            ..default()
        })?;

        self.expect(Token::RightParen)?;
        self.finish_paren_list_expr_or_qual_id(start, expr)
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

        // IdentifierName
        if let Token::Identifier(id) = self.token.0.clone() {
            let id_location = self.token_location();
            self.next()?;
            if self.peek(Token::ColonColon) {
                let id = ast::QualifiedIdentifier {
                    attribute: false,
                    qualifier: None,
                    name: ast::IdentifierOrBrackets::Id(id, id_location.clone()),
                };
                let id = Rc::new(ast::Expression {
                    location: id_location.clone(),
                    kind: ast::ExpressionKind::Id(id),
                });
                return self.finish_qualified_identifier(attribute, id);
            } else {
                let id = ast::QualifiedIdentifier {
                    attribute,
                    qualifier: None,
                    name: ast::IdentifierOrBrackets::Id(id, id_location.clone()),
                };
                return Ok(id);
            }
        }

        // `public`, `private`, `protected`, `internal` followed by `::`
        if let Some(reserved_ns) = self.peek_reserved_namespace() {
            let q_location = self.token_location();
            self.next()?;
            if !self.peek(Token::ColonColon) {
                let id = ast::QualifiedIdentifier {
                    attribute,
                    qualifier: None,
                    name: ast::IdentifierOrBrackets::Id(reserved_ns.to_string(), q_location),
                };
                return Ok(id);
            }
            let id = Rc::new(ast::Expression {
                location: q_location,
                kind: ast::ExpressionKind::ReservedNamespace(reserved_ns),
            });
            return self.finish_qualified_identifier(attribute, id);
        }

        // IdentifierName (from reserved word)
        if let Some(id) = self.token.0.reserved_word_name() {
            let id_location = self.token_location();
            self.next()?;
            if self.peek(Token::ColonColon) {
                let id = ast::QualifiedIdentifier {
                    attribute: false,
                    qualifier: None,
                    name: ast::IdentifierOrBrackets::Id(id, id_location.clone()),
                };
                let id = Rc::new(ast::Expression {
                    location: id_location.clone(),
                    kind: ast::ExpressionKind::Id(id),
                });
                return self.finish_qualified_identifier(attribute, id);
            } else {
                let id = ast::QualifiedIdentifier {
                    attribute,
                    qualifier: None,
                    name: ast::IdentifierOrBrackets::Id(id, id_location.clone()),
                };
                return Ok(id);
            }
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
                    name: ast::IdentifierOrBrackets::Id("*".into(), id_location.clone()),
                };
                let id = Rc::new(ast::Expression {
                    location: id_location.clone(),
                    kind: ast::ExpressionKind::Id(id),
                });
                return self.finish_qualified_identifier(attribute, id);
            } else {
                let id = ast::QualifiedIdentifier {
                    attribute,
                    qualifier: None,
                    name: ast::IdentifierOrBrackets::Id("*".into(), id_location.clone()),
                };
                return Ok(id);
            }
        }

        self.add_syntax_error(&self.token_location(), DiagnosticKind::ExpectedIdentifier, diagnostic_arguments![Token(self.token.0.clone())]);
        Err(ParserFailure)
    }

    fn parse_non_attribute_qualified_identifier(&mut self) -> Result<ast::NonAttributeQualifiedIdentifier, ParserFailure> {
        // IdentifierName
        if let Token::Identifier(id) = self.token.0.clone() {
            let id_location = self.token_location();
            self.next()?;
            if self.peek(Token::ColonColon) {
                let id = ast::QualifiedIdentifier {
                    attribute: false,
                    qualifier: None,
                    name: ast::IdentifierOrBrackets::Id(id, id_location.clone()),
                };
                let id = Rc::new(ast::Expression {
                    location: id_location.clone(),
                    kind: ast::ExpressionKind::Id(id),
                });
                return self.finish_non_attribute_qualified_identifier(id);
            } else {
                let id = ast::NonAttributeQualifiedIdentifier {
                    qualifier: None,
                    name: ast::IdentifierOrBrackets::Id(id, id_location.clone()),
                };
                return Ok(id);
            }
        }

        // `public`, `private`, `protected`, `internal` followed by `::`
        if let Some(reserved_ns) = self.peek_reserved_namespace() {
            let q_location = self.token_location();
            self.next()?;
            if !self.peek(Token::ColonColon) {
                let id = ast::NonAttributeQualifiedIdentifier {
                    qualifier: None,
                    name: ast::IdentifierOrBrackets::Id(reserved_ns.to_string(), q_location),
                };
                return Ok(id);
            }
            let id = Rc::new(ast::Expression {
                location: q_location,
                kind: ast::ExpressionKind::ReservedNamespace(reserved_ns),
            });
            return self.finish_non_attribute_qualified_identifier(id);
        }

        // IdentifierName (from reserved word)
        if let Some(id) = self.token.0.reserved_word_name() {
            let id_location = self.token_location();
            self.next()?;
            if self.peek(Token::ColonColon) {
                let id = ast::QualifiedIdentifier {
                    attribute: false,
                    qualifier: None,
                    name: ast::IdentifierOrBrackets::Id(id, id_location.clone()),
                };
                let id = Rc::new(ast::Expression {
                    location: id_location.clone(),
                    kind: ast::ExpressionKind::Id(id),
                });
                return self.finish_non_attribute_qualified_identifier(id);
            } else {
                let id = ast::NonAttributeQualifiedIdentifier {
                    qualifier: None,
                    name: ast::IdentifierOrBrackets::Id(id, id_location.clone()),
                };
                return Ok(id);
            }
        }

        // (q)::x
        if self.peek(Token::LeftParen) {
            let qual = self.parse_paren_expression()?;
            return self.finish_non_attribute_qualified_identifier(qual);
        }

        // `*`
        if self.peek(Token::Times) {
            let id_location = self.token_location();
            self.next()?;
            if self.peek(Token::ColonColon) {
                let id = ast::QualifiedIdentifier {
                    attribute: false,
                    qualifier: None,
                    name: ast::IdentifierOrBrackets::Id("*".into(), id_location.clone()),
                };
                let id = Rc::new(ast::Expression {
                    location: id_location.clone(),
                    kind: ast::ExpressionKind::Id(id),
                });
                return self.finish_non_attribute_qualified_identifier(id);
            } else {
                let id = ast::NonAttributeQualifiedIdentifier {
                    qualifier: None,
                    name: ast::IdentifierOrBrackets::Id("*".into(), id_location.clone()),
                };
                return Ok(id);
            }
        }

        self.add_syntax_error(&self.token_location(), DiagnosticKind::ExpectedIdentifier, diagnostic_arguments![Token(self.token.0.clone())]);
        Err(ParserFailure)
    }

    /// Expects a colon-colon and finishes a qualified identifier.
    fn finish_qualified_identifier(&mut self, attribute: bool, qual: Rc<ast::Expression>) -> Result<ast::QualifiedIdentifier, ParserFailure> {
        self.expect(Token::ColonColon)?;

        // `::` may be followed by one of { IdentifierName, `*`, Brackets }

        // IdentifierName
        if let Token::Identifier(id) = self.token.0.clone() {
            let id_location = self.token_location();
            self.next()?;
            Ok(ast::QualifiedIdentifier {
                attribute,
                qualifier: Some(qual),
                name: ast::IdentifierOrBrackets::Id(id, id_location),
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
            self.add_syntax_error(&self.token_location(), DiagnosticKind::ExpectedIdentifier, diagnostic_arguments![Token(self.token.0.clone())]);
            Err(ParserFailure)
        }
    }

    /// Expects a colon-colon and finishes a non attribute qualified identifier.
    fn finish_non_attribute_qualified_identifier(&mut self, qual: Rc<ast::Expression>) -> Result<ast::NonAttributeQualifiedIdentifier, ParserFailure> {
        self.expect(Token::ColonColon)?;

        // `::` may be followed by one of { IdentifierName, `*`, Brackets }

        // IdentifierName
        if let Token::Identifier(id) = self.token.0.clone() {
            let id_location = self.token_location();
            self.next()?;
            Ok(ast::NonAttributeQualifiedIdentifier {
                qualifier: Some(qual),
                name: ast::IdentifierOrBrackets::Id(id, id_location),
            })
        // IdentifierName (from reserved word)
        } else if let Some(id) = self.token.0.reserved_word_name() {
            let id_location = self.token_location();
            self.next()?;
            Ok(ast::NonAttributeQualifiedIdentifier {
                qualifier: Some(qual),
                name: ast::IdentifierOrBrackets::Id(id, id_location),
            })
        // `*`
        } else if self.peek(Token::Times) {
            let id_location = self.token_location();
            self.next()?;
            Ok(ast::NonAttributeQualifiedIdentifier {
                qualifier: Some(qual),
                name: ast::IdentifierOrBrackets::Id("*".into(), id_location),
            })
        // Brackets
        } else if self.peek(Token::LeftBracket) {
            let brackets = self.parse_brackets()?;
            Ok(ast::NonAttributeQualifiedIdentifier {
                qualifier: Some(qual),
                name: ast::IdentifierOrBrackets::Brackets(brackets),
            })
        } else {
            self.add_syntax_error(&self.token_location(), DiagnosticKind::ExpectedIdentifier, diagnostic_arguments![Token(self.token.0.clone())]);
            Err(ParserFailure)
        }
    }

    fn parse_brackets(&mut self) -> Result<Rc<ast::Expression>, ParserFailure> {
        self.expect(Token::LeftBracket)?;
        let expr = self.parse_expression(ExpressionContext {
            min_precedence: OperatorPrecedence::List,
            allow_in: true,
            ..default()
        });
        self.expect(Token::RightBracket)?;
        expr
    }

    fn parse_paren_expression(&mut self) -> Result<Rc<ast::Expression>, ParserFailure> {
        self.expect(Token::LeftParen)?;
        let expr = self.parse_expression(ExpressionContext {
            min_precedence: OperatorPrecedence::AssignmentAndOther,
            allow_in: true,
            ..default()
        });
        self.expect(Token::RightParen)?;
        expr
    }

    fn parse_paren_list_expression(&mut self) -> Result<Rc<ast::Expression>, ParserFailure> {
        self.expect(Token::LeftParen)?;
        let expr = self.parse_expression(ExpressionContext {
            min_precedence: OperatorPrecedence::List,
            allow_in: true,
            ..default()
        });
        self.expect(Token::RightParen)?;
        expr
    }

    fn parse_type_expression(&mut self) -> Result<Rc<ast::TypeExpression>, ParserFailure> {
        self.parse_type_expression_with_context(false)
    }

    fn parse_type_expression_with_context(&mut self, between_union: bool) -> Result<Rc<ast::TypeExpression>, ParserFailure> {
        // Allow a `|` prefix
        if !between_union {
            self.consume(Token::BitwiseOr)?;
        }

        let start = self.token_location();
        let (mut base, wrap_nullable) = self.parse_type_expression_start()?;

        loop {
            if self.consume(Token::Dot)? {
                self.push_location(&base.location);
                if self.consume(Token::Lt)? {
                    let mut arguments = vec![self.parse_type_expression()?];
                    while self.consume(Token::Comma)? {
                        arguments.push(self.parse_type_expression()?);
                    }
                    self.expect_generics_gt()?;
                    base = Rc::new(ast::TypeExpression {
                        location: self.pop_location(),
                        kind: ast::TypeExpressionKind::WithTypeArguments { base, arguments },
                    });
                } else {
                    let id = self.parse_non_attribute_qualified_identifier()?;
                    base = Rc::new(ast::TypeExpression {
                        location: self.pop_location(),
                        kind: ast::TypeExpressionKind::DotMember { base, id },
                    });
                }
            } else if self.consume(Token::Question)? {
                self.push_location(&base.location);
                base = Rc::new(ast::TypeExpression {
                    location: self.pop_location(),
                    kind: ast::TypeExpressionKind::Nullable(base),
                });
            } else if self.consume(Token::Exclamation)? {
                self.push_location(&base.location);
                base = Rc::new(ast::TypeExpression {
                    location: self.pop_location(),
                    kind: ast::TypeExpressionKind::NonNullable(base),
                });
            } else if self.peek(Token::BitwiseOr) && !between_union {
                self.push_location(&base.location);
                let mut members = vec![base];
                while self.consume(Token::BitwiseOr)? {
                    members.push(self.parse_type_expression_with_context(true)?);
                }
                base = Rc::new(ast::TypeExpression {
                    location: self.pop_location(),
                    kind: ast::TypeExpressionKind::Union(members),
                });
            } else if self.consume(Token::BitwiseAnd)? {
                self.push_location(&base.location);
                let right = self.parse_type_expression_with_context(false)?;
                base = Rc::new(ast::TypeExpression {
                    location: self.pop_location(),
                    kind: ast::TypeExpressionKind::Complement(base, right),
                });
            } else {
                break;
            }
        }

        if wrap_nullable {
            self.push_location(&start);
            base = Rc::new(ast::TypeExpression {
                location: self.pop_location(),
                kind: ast::TypeExpressionKind::Nullable(base),
            });
        }

        Ok(base)
    }

    fn parse_type_expression_start(&mut self) -> Result<(Rc<ast::TypeExpression>, bool), ParserFailure> {
        // Allow a `?` prefix to wrap a type into nullable.
        let wrap_nullable = self.consume(Token::Question)?;

        // Parenthesized
        if self.peek(Token::LeftParen) {
            Ok((self.parse_paren_type_expression()?, wrap_nullable))
        // `void`
        } else if self.peek(Token::Void) {
            self.mark_location();
            self.next()?;
            Ok((Rc::new(ast::TypeExpression {
                location: self.pop_location(),
                kind: ast::TypeExpressionKind::Void,
            }), wrap_nullable))
        // StringLiteral
        } else if let Token::StringLiteral(value) = &self.token.0.clone() {
            self.mark_location();
            self.next()?;
            Ok((Rc::new(ast::TypeExpression {
                location: self.pop_location(),
                kind: ast::TypeExpressionKind::StringLiteral(value.clone()),
            }), wrap_nullable))
        // NumericLiteral
        } else if let Token::NumericLiteral(value) = self.token.0.clone() {
            self.mark_location();
            self.next()?;
            Ok((Rc::new(ast::TypeExpression {
                location: self.pop_location(),
                kind: ast::TypeExpressionKind::NumericLiteral(value),
            }), wrap_nullable))
        // [T1, T2, ...Tn]
        } else if self.peek(Token::LeftBracket) {
            let mut elements = vec![];
            self.mark_location();
            self.next()?;
            elements.push(self.parse_type_expression()?);
            self.expect(Token::Comma)?;
            elements.push(self.parse_type_expression()?);
            while self.consume(Token::Comma)? {
                if self.peek(Token::RightBracket) {
                    break;
                }
                elements.push(self.parse_type_expression()?);
            }
            self.expect(Token::RightBracket)?;
            Ok((Rc::new(ast::TypeExpression {
                location: self.pop_location(),
                kind: ast::TypeExpressionKind::Tuple(elements),
            }), wrap_nullable))
        // {...}
        } else if self.peek(Token::LeftBrace) {
            Ok((self.parse_record_type_expression()?, wrap_nullable))
        // NonAttributeQualifiedIdentifier
        } else {
            self.mark_location();
            let id = self.parse_non_attribute_qualified_identifier()?;
            if let Some(id_token_or_wildcard) = id.to_identifier_or_wildcard() {
                match id_token_or_wildcard.0.as_ref() {
                    "*" => {
                        return Ok((Rc::new(ast::TypeExpression {
                            location: self.pop_location(),
                            kind: ast::TypeExpressionKind::Any,
                        }), wrap_nullable));
                    },
                    "never" => {
                        return Ok((Rc::new(ast::TypeExpression {
                            location: self.pop_location(),
                            kind: ast::TypeExpressionKind::Never,
                        }), wrap_nullable));
                    },
                    "undefined" => {
                        return Ok((Rc::new(ast::TypeExpression {
                            location: self.pop_location(),
                            kind: ast::TypeExpressionKind::Undefined,
                        }), wrap_nullable));
                    },
                    _ => {},
                }
            }
            Ok((Rc::new(ast::TypeExpression {
                location: self.pop_location(),
                kind: ast::TypeExpressionKind::Id(id),
            }), wrap_nullable))
        }
    }

    fn parse_record_type_expression(&mut self) -> Result<Rc<ast::TypeExpression>, ParserFailure> {
        self.mark_location();
        self.next()?;
        let mut fields = vec![];
        while !self.peek(Token::RightBrace) {
            fields.push(self.parse_record_type_field()?);
            if !self.consume(Token::Comma)? {
                break;
            }
        }
        self.expect(Token::RightBrace)?;
        Ok(Rc::new(ast::TypeExpression {
            location: self.pop_location(),
            kind: ast::TypeExpressionKind::Record(fields),
        }))
    }

    fn parse_record_type_field(&mut self) -> Result<Rc<ast::RecordTypeField>, ParserFailure> {
        let asdoc = self.parse_asdoc()?;
        let mut readonly = false;
        let mut key: (ast::ObjectKey, Location) = self.parse_object_key()?;
        if let ast::ObjectKey::Id(id) = &key.0 {
            if let Some(id) = id.to_identifier() {
                if self.record_type_field_readonly(id) {
                    readonly = true;
                    key = self.parse_object_key()?;
                }
            }
        }
        let nullability = if self.consume(Token::Exclamation)? {
            ast::FieldNullability::NonNullable
        } else if self.consume(Token::Question)? {
            ast::FieldNullability::Nullable
        } else {
            ast::FieldNullability::Unspecified
        };
        self.expect(Token::Colon)?;
        let type_annotation = self.parse_type_expression()?;
        Ok(Rc::new(ast::RecordTypeField {
            asdoc,
            readonly,
            key,
            nullability,
            type_annotation,
        }))
    }

    fn record_type_field_readonly(&self, id: (String, Location)) -> bool {
        id.0 == "readonly" && id.1.character_count() == "readonly".len() && !(
            self.peek(Token::Colon) ||
            self.peek(Token::Comma) ||
            self.peek(Token::Question) ||
            self.peek(Token::RightBrace) ||
            self.peek(Token::Exclamation)
        )
    }

    fn parse_object_key(&mut self) -> Result<(ast::ObjectKey, Location), ParserFailure> {
        if let Token::StringLiteral(value) = &self.token.0.clone() {
            let location = self.token_location();
            self.next()?;
            Ok((ast::ObjectKey::String(value.clone(), location.clone()), location))
        } else if let Token::NumericLiteral(value) = &self.token.0.clone() {
            let location = self.token_location();
            self.next()?;
            Ok((ast::ObjectKey::Number(value.clone(), location.clone()), location))
        } else if self.peek(Token::LeftBracket) {
            self.mark_location();
            self.next()?;
            let key_expr = self.parse_expression(ExpressionContext {
                allow_in: true,
                min_precedence: OperatorPrecedence::AssignmentAndOther,
                ..default()
            })?;
            self.expect(Token::RightBracket)?;
            let location = self.pop_location();
            Ok((ast::ObjectKey::Brackets(key_expr), location))
        } else {
            self.mark_location();
            let id = self.parse_non_attribute_qualified_identifier()?;
            let location = self.pop_location();
            Ok((ast::ObjectKey::Id(id), location))
        }
    }

    fn parse_paren_type_expression(&mut self) -> Result<Rc<ast::TypeExpression>, ParserFailure> {
        self.mark_location();
        self.next()?;

        // If `(` is followed by `)`, parse a function type
        if self.consume(Token::RightParen)? {
            self.expect(Token::FatArrow)?;
            let return_annotation = self.parse_type_expression()?;
            return Ok(Rc::new(ast::TypeExpression {
                location: self.pop_location(),
                kind: ast::TypeExpressionKind::Function { params: vec![], return_annotation },
            }));
        }

        // If `(` is followed by `...`, parse a function type
        if self.consume(Token::Ellipsis)? {
            let name = self.expect_identifier(false)?;
            let type_annotation = if self.consume(Token::Colon)? { Some(self.parse_type_expression()?) } else { None };
            let rest_param = ast::FunctionTypeParam {
                kind: ast::FunctionParamKind::Rest,
                name,
                type_annotation,
            };
            self.expect(Token::RightParen)?;
            self.expect(Token::FatArrow)?;
            let return_annotation = self.parse_type_expression()?;
            return Ok(Rc::new(ast::TypeExpression {
                location: self.pop_location(),
                kind: ast::TypeExpressionKind::Function { params: vec![rest_param], return_annotation },
            }));
        }

        let subexp = self.parse_type_expression()?;

        // If subexpression is an identifier token or an `idToken?`
        // type expression and it is followed by either `:` or `,`, parse a function type
        if let Some(mut param) = subexp.to_function_type_param() {
            let mut parse_function_type = false;
            if self.consume(Token::Colon)? {
                parse_function_type = true;
                param.type_annotation = Some(self.parse_type_expression()?);
            } else if self.peek(Token::Comma) {
                parse_function_type = true;
            }
            
            if parse_function_type {
                let mut req_params_allowed = param.kind == ast::FunctionParamKind::Required;
                let mut params = vec![param];

                while self.consume(Token::Comma)? {
                    if self.consume(Token::Ellipsis)? {
                        let name = self.expect_identifier(false)?;
                        let type_annotation = if self.consume(Token::Colon)? { Some(self.parse_type_expression()?) } else { None };
                        params.push(ast::FunctionTypeParam {
                            kind: ast::FunctionParamKind::Rest,
                            name,
                            type_annotation,
                        });
                        break;
                    } else {
                        let name = self.expect_identifier(false)?;
                        let optional = if req_params_allowed {
                            self.consume(Token::Question)?
                        } else {
                            self.expect(Token::Question)?;
                            true
                        };
                        if optional {
                            req_params_allowed = false;
                        }
                        let type_annotation = if self.consume(Token::Colon)? { Some(self.parse_type_expression()?) } else { None };
                        params.push(ast::FunctionTypeParam {
                            kind: if optional { ast::FunctionParamKind::Optional } else { ast::FunctionParamKind::Required },
                            name,
                            type_annotation,
                        });
                    }
                }

                self.expect(Token::RightParen)?;
                self.expect(Token::FatArrow)?;
                let return_annotation = self.parse_type_expression()?;
                return Ok(Rc::new(ast::TypeExpression {
                    location: self.pop_location(),
                    kind: ast::TypeExpressionKind::Function { params, return_annotation },
                }));
            }
        }

        self.expect(Token::RightParen)?;
        Ok(Rc::new(ast::TypeExpression {
            location: self.pop_location(),
            kind: ast::TypeExpressionKind::Paren(subexp),
        }))
    }

    fn parse_variable_binding(&mut self, allow_in: bool) -> Result<ast::VariableBinding, ParserFailure> {
        let pattern = self.parse_destructuring()?;
        let init = if self.consume(Token::Assign)? {
            Some(self.parse_expression(ExpressionContext {
                allow_in,
                min_precedence: OperatorPrecedence::AssignmentAndOther,
                ..default()
            })?)
        } else {
            None
        };
        Ok(ast::VariableBinding { pattern, init })
    }

    fn parse_destructuring(&mut self) -> Result<Rc<ast::Destructuring>, ParserFailure> {
        self.mark_location();
        let kind = self.parse_destructuring_kind()?;
        let non_null = self.consume(Token::Exclamation)?;
        let type_annotation = if self.consume(Token::Colon)? { Some(self.parse_type_expression()?) } else { None };
        Ok(Rc::new(ast::Destructuring {
            location: self.pop_location(),
            kind,
            non_null,
            type_annotation,
        }))
    }

    fn parse_destructuring_kind(&mut self) -> Result<ast::DestructuringKind, ParserFailure> {
        if self.consume(Token::LeftBracket)? {
            self.parse_array_destructuring()
        } else if self.consume(Token::LeftBrace)? {
            self.parse_record_destructuring()
        } else {
            Ok(ast::DestructuringKind::Binding { name: self.expect_identifier(true)? })
        }
    }

    fn parse_array_destructuring(&mut self) -> Result<ast::DestructuringKind, ParserFailure> {
        let mut items: Vec<Option<ast::ArrayDestructuringItem>> = vec![];
        while !self.peek(Token::RightBracket) {
            let mut ellipses = false;
            while self.consume(Token::Comma)? {
                items.push(None);
                ellipses = true;
            }
            if !ellipses  {
                if self.peek(Token::Ellipsis) {
                    self.mark_location();
                    self.next()?;
                    let subdestructuring = self.parse_destructuring()?;
                    items.push(Some(ast::ArrayDestructuringItem::Rest(subdestructuring, self.pop_location())));
                } else {
                    items.push(Some(ast::ArrayDestructuringItem::Pattern(self.parse_destructuring()?)));
                }
            }
            if !self.consume(Token::Comma)? {
                break;
            }
        }
        self.expect(Token::RightBracket)?;
        Ok(ast::DestructuringKind::Array(items))
    }

    fn parse_record_destructuring(&mut self) -> Result<ast::DestructuringKind, ParserFailure> {
        let mut fields: Vec<Rc<ast::RecordDestructuringField>> = vec![];
        while !self.peek(Token::RightBrace) {
            fields.push(self.parse_record_destructuring_field()?);
            if !self.consume(Token::Comma)? {
                break;
            }
        }
        self.expect(Token::RightBrace)?;
        Ok(ast::DestructuringKind::Record(fields))
    }

    fn parse_record_destructuring_field(&mut self) -> Result<Rc<ast::RecordDestructuringField>, ParserFailure> {
        self.mark_location();
        let key = self.parse_object_key()?;
        let non_null = self.consume(Token::Exclamation)?;
        let alias = if self.consume(Token::Colon)? { Some(self.parse_destructuring()?) } else { None };
        Ok(Rc::new(ast::RecordDestructuringField {
            location: self.pop_location(),
            key,
            non_null,
            alias,
        }))
    }

    fn parse_generics_where_clause(&mut self) -> Result<Option<ast::GenericsWhere>, ParserFailure> {
        if !self.peek_context_keyword("where") {
            return Ok(None);
        }
        self.next()?;
        let mut constraints: Vec<ast::GenericsWhereConstraint> = vec![];
        loop {
            let name = self.expect_identifier(false)?;
            self.expect(Token::Colon)?;
            let mut constraints_1 = vec![self.parse_type_expression()?];
            while self.consume(Token::Plus)? {
                constraints_1.push(self.parse_type_expression()?);
            }
            constraints.push(ast::GenericsWhereConstraint {
                name,
                constraints: constraints_1,
            });
            if !self.consume(Token::Comma)? {
                break;
            }
        }
        Ok(Some(ast::GenericsWhere {
            constraints,
        }))
    }

    fn parse_semicolon(&mut self) -> Result<bool, ParserFailure> {
        Ok(self.consume(Token::Semicolon)? || self.peek(Token::RightBrace) || self.previous_token.1.line_break(&self.token.1))
    }

    fn parse_substatement(&mut self, context: DirectiveContext) -> Result<(Rc<ast::Statement>, bool), ParserFailure> {
        if self.peek(Token::Var) || self.peek(Token::Const) {
            self.mark_location();
            let declaration = self.parse_simple_variable_declaration(true)?;
            let semicolon_inserted = self.parse_semicolon()?;
            let node = Rc::new(ast::Statement {
                location: self.pop_location(),
                kind: ast::StatementKind::SimpleVariableDeclaration(declaration),
            });
            Ok((node, semicolon_inserted))
        } else {
            self.parse_statement(context)
        }
    }

    fn parse_statement(&mut self, context: DirectiveContext) -> Result<(Rc<ast::Statement>, bool), ParserFailure> {
        // ExpressionStatement or LabeledStatement
        if matches!(self.token.0, Token::Identifier(_)) {
            self.parse_expression_statement_or_labeled_statement(context)
        // SuperStatement or ExpressionStatement with `super`
        } else if self.peek(Token::Super) {
            self.mark_location();
            self.next()?;
            let arguments = if self.peek(Token::LeftParen) { Some(self.parse_arguments()?) } else { None };
            let mut semicolon_inserted = false;
            if arguments.is_some() {
                semicolon_inserted = self.parse_semicolon()?;
            }
            if arguments.is_none() || (!semicolon_inserted && (self.peek(Token::Dot) || self.peek(Token::LeftBracket))) {
                if !(self.peek(Token::Dot) || self.peek(Token::LeftBracket)) {
                    self.expect(Token::Dot)?;
                }
                self.duplicate_location();
                // ExpressionStatement (`super`...)
                let mut expr = Rc::new(ast::Expression {
                    location: self.pop_location(),
                    kind: ast::ExpressionKind::Super(arguments),
                });
                expr = self.parse_subexpressions(expr, ExpressionContext {
                    allow_in: true,
                    min_precedence: OperatorPrecedence::List,
                    ..default()
                })?;
                let semicolon_inserted = self.parse_semicolon()?;
                Ok((Rc::new(ast::Statement {
                    location: self.pop_location(),
                    kind: ast::StatementKind::Expression {
                        asdoc: None,
                        expression: expr,
                    },
                }), semicolon_inserted))
            } else {
                // SuperStatement
                let node = Rc::new(ast::Statement {
                    location: self.pop_location(),
                    kind: ast::StatementKind::Super(arguments.unwrap()),
                });

                // Check if super statement is allowed here
                let allowed_here;
                if let DirectiveContext::ConstructorBlock { super_statement_found } = &context {
                    allowed_here = !super_statement_found.get();
                    super_statement_found.set(true);
                } else {
                    allowed_here = false;
                }

                if !allowed_here {
                    self.add_syntax_error(&node.location.clone(), DiagnosticKind::NotAllowedHere, diagnostic_arguments![Token(Token::Super)]);
                }

                Ok((node, semicolon_inserted))
            }
        // EmptyStatement
        } else if self.peek(Token::Semicolon) {
            self.mark_location();
            self.next()?;
            Ok((Rc::new(ast::Statement {
                location: self.pop_location(),
                kind: ast::StatementKind::Empty,
            }), true))
        // Block
        } else if self.peek(Token::LeftBrace) {
            self.mark_location();
            let context = context.override_control_context(true, ControlContext {
                breakable: true,
                iteration: false,
            });
            let block = self.parse_block(context)?;
            Ok((Rc::new(ast::Statement {
                location: self.pop_location(),
                kind: ast::StatementKind::Block(block),
            }), true))
        // IfStatement
        } else if self.peek(Token::If) {
            self.parse_if_statement(context)
        // SwitchStatement
        // `switch type`
        } else if self.peek(Token::Switch) {
            self.parse_switch_statement(context)
        // DoStatement
        } else if self.peek(Token::Do) {
            self.parse_do_statement(context)
        // WhileStatement
        } else if self.peek(Token::While) {
            self.parse_while_statement(context)
        // ForStatement
        // `for..in`
        // `for each`
        } else if self.peek(Token::For) {
            self.parse_for_statement(context)
        // WithStatement
        } else if self.peek(Token::With) {
            self.parse_with_statement(context)
        // BreakStatement
        } else if self.peek(Token::Break) {
            self.parse_break_statement(context)
        // ContinueStatement
        } else if self.peek(Token::Continue) {
            self.parse_continue_statement(context)
        // ReturnStatement
        } else if self.peek(Token::Return) {
            self.parse_return_statement(context)
        // ThrowStatement
        } else if self.peek(Token::Return) {
            self.parse_throw_statement(context)
        // TryStatement
        } else if self.peek(Token::Try) {
            self.parse_try_statement(context)
        // `default xml namespace = expression`
        } else if self.peek(Token::Default) {
            self.parse_default_xml_namespace_statement()
        // ExpressionStatement
        } else {
            self.mark_location();
            let asdoc = self.parse_asdoc()?;
            let exp = self.parse_expression(ExpressionContext {
                allow_in: true, min_precedence: OperatorPrecedence::List, ..default()
            })?;
            let semicolon_inserted = self.parse_semicolon()?;
            Ok((Rc::new(ast::Statement {
                location: self.pop_location(),
                kind: ast::StatementKind::Expression { asdoc, expression: exp, },
            }), semicolon_inserted))
        }
    }

    fn parse_expression_statement_or_labeled_statement(&mut self, context: DirectiveContext) -> Result<(Rc<ast::Statement>, bool), ParserFailure> {
        self.mark_location();
        let asdoc = self.parse_asdoc()?;
        let id = self.expect_identifier(false)?;

        // LabeledStatement
        if self.consume(Token::Colon)? {
            let (statement, semicolon_inserted) = self.parse_substatement(context.put_label(id.0.clone()))?;
            let labeled = Rc::new(ast::Statement {
                location: self.pop_location(),
                kind: ast::StatementKind::Labeled {
                    label: id, statement,
                },
            });
            return Ok((labeled, semicolon_inserted));
        }

        let mut exp: Option<Rc<ast::Expression>> = None;

        let mut initiated_as_embed = false;

        // EmbedExpression
        if let Token::StringLiteral(string_value) = &self.token.0.clone() {
            if id.0 == "embed" && self.previous_token.1.character_count() == "embed".len() {
                exp = Some(self.finish_embed_expression(id.1.clone(), string_value.clone())?);
                initiated_as_embed = true;
            }
        }

        // QualifiedIdentifier
        if !initiated_as_embed {
            let id = Rc::new(ast::Expression {
                location: id.1.clone(),
                kind: ast::ExpressionKind::Id(ast::QualifiedIdentifier {
                    attribute: false,
                    qualifier: None,
                    name: ast::IdentifierOrBrackets::Id(id.0.clone(), id.1.clone()),
                }),
            });
            if self.peek(Token::ColonColon) {
                self.push_location(&id.location);
                let id = self.finish_qualified_identifier(false, id)?;
                exp = Some(Rc::new(ast::Expression {
                    location: self.pop_location(),
                    kind: ast::ExpressionKind::Id(id),
                }));
            } else {
                exp = Some(id);
            }
        }

        let mut exp = exp.clone().unwrap();
        exp = self.parse_subexpressions(exp, ExpressionContext {
            allow_in: true, min_precedence: OperatorPrecedence::List, ..default()
        })?;
        let semicolon_inserted = self.parse_semicolon()?;
        Ok((Rc::new(ast::Statement {
            location: self.pop_location(),
            kind: ast::StatementKind::Expression { asdoc, expression: exp, },
        }), semicolon_inserted))
    }

    fn parse_block(&mut self, context: DirectiveContext) -> Result<Rc<ast::Block>, ParserFailure> {
        self.expect(Token::LeftBrace)?;
        let mut directives = vec![];
        let mut semicolon_inserted = false;
        while !self.peek(Token::RightBrace) {
            if !directives.is_empty() && !semicolon_inserted {
                self.expect(Token::Semicolon)?;
            }
            let (directive, semicolon_inserted_1) = self.parse_directive(context.clone())?;
            directives.push(directive);
            semicolon_inserted = semicolon_inserted_1;
        }
        self.expect(Token::RightBrace)?;
        Ok(Rc::new(ast::Block(directives)))
    }

    fn parse_if_statement(&mut self, context: DirectiveContext) -> Result<(Rc<ast::Statement>, bool), ParserFailure> {
        let context = context.override_control_context(true, ControlContext {
            breakable: true,
            iteration: false,
        });
        self.mark_location();
        self.next()?;
        self.expect(Token::LeftParen)?;
        let test = self.parse_expression(ExpressionContext { allow_in: true, min_precedence: OperatorPrecedence::List, ..default() })?;
        self.expect(Token::RightParen)?;
        let semicolon_inserted;
        let (consequent, semicolon_inserted_1) = self.parse_substatement(context.clone())?;
        let mut alternative = None;
        if self.peek(Token::Else) {
            if !semicolon_inserted_1 {
                self.expect(Token::Semicolon)?;
            }
            self.next()?;
            let (alternative_2, semicolon_inserted_2) = self.parse_substatement(context.clone())?;
            alternative = Some(alternative_2);
            semicolon_inserted = semicolon_inserted_2;
        } else {
            semicolon_inserted = semicolon_inserted_1;
        }
        Ok((Rc::new(ast::Statement {
            location: self.pop_location(),
            kind: ast::StatementKind::If { condition: test, consequent, alternative },
        }), semicolon_inserted))
    }

    fn parse_switch_statement(&mut self, context: DirectiveContext) -> Result<(Rc<ast::Statement>, bool), ParserFailure> {
        self.mark_location();
        self.next()?;
        if self.consume_context_keyword("type")? {
            return self.parse_switch_type_statement(context);
        }
        let context = context.override_control_context(false, ControlContext {
            breakable: true,
            iteration: false,
        });
        self.expect(Token::LeftParen)?;
        let discriminant = self.parse_expression(ExpressionContext { allow_in: true, min_precedence: OperatorPrecedence::List, ..default() })?;
        self.expect(Token::RightParen)?;
        let cases = self.parse_switch_block(context)?;
        Ok((Rc::new(ast::Statement {
            location: self.pop_location(),
            kind: ast::StatementKind::Switch { discriminant, cases },
        }), true))
    }

    fn parse_switch_block(&mut self, context: DirectiveContext) -> Result<Vec<ast::SwitchCase>, ParserFailure> {
        self.expect(Token::LeftBrace)?;
        let mut cases = vec![];
        let mut semicolon_inserted = false;
        while !self.peek(Token::RightBrace) {
            if !cases.is_empty() && !semicolon_inserted {
                self.expect(Token::Semicolon)?;
            }
            if self.consume(Token::Default)? {
                self.expect(Token::Colon)?;
                let mut directives = vec![];
                semicolon_inserted = false;
                while !(self.peek(Token::RightBrace) || self.peek(Token::Case) || self.peek(Token::Default)) {
                    if !directives.is_empty() && !semicolon_inserted {
                        self.expect(Token::Semicolon)?;
                    }
                    let (directive, semicolon_inserted_1) = self.parse_directive(context.clone())?;
                    directives.push(directive);
                    semicolon_inserted = semicolon_inserted_1;
                }
                cases.push(ast::SwitchCase {
                    expression: None,
                    consequent: directives,
                });
            } else {
                self.expect(Token::Case)?;
                let expression = Some(self.parse_expression(ExpressionContext {
                    allow_in: true, min_precedence: OperatorPrecedence::List, ..default()
                })?);
                self.expect(Token::Colon)?;
                let mut directives = vec![];
                semicolon_inserted = false;
                while !(self.peek(Token::RightBrace) || self.peek(Token::Case) || self.peek(Token::Default)) {
                    if !directives.is_empty() && !semicolon_inserted {
                        self.expect(Token::Semicolon)?;
                    }
                    let (directive, semicolon_inserted_1) = self.parse_directive(context.clone())?;
                    directives.push(directive);
                    semicolon_inserted = semicolon_inserted_1;
                }
                cases.push(ast::SwitchCase {
                    expression,
                    consequent: directives,
                });
            }
        }
        self.expect(Token::RightBrace)?;
        Ok(cases)
    }

    fn parse_switch_type_statement(&mut self, context: DirectiveContext) -> Result<(Rc<ast::Statement>, bool), ParserFailure> {
        let context = context.override_control_context(true, ControlContext {
            breakable: true,
            iteration: false,
        });
        self.expect(Token::LeftParen)?;
        let discriminant = self.parse_expression(ExpressionContext { allow_in: true, min_precedence: OperatorPrecedence::List, ..default() })?;
        self.expect(Token::RightParen)?;
        let cases = self.parse_switch_type_block(context)?;
        Ok((Rc::new(ast::Statement {
            location: self.pop_location(),
            kind: ast::StatementKind::SwitchType { discriminant, cases },
        }), true))
    }

    fn parse_switch_type_block(&mut self, context: DirectiveContext) -> Result<Vec<ast::SwitchTypeCase>, ParserFailure> {
        self.expect(Token::LeftBrace)?;
        let mut cases = vec![];
        while !self.peek(Token::RightBrace) {
            if self.consume(Token::Default)? {
                let block = self.parse_block(context.clone())?;
                cases.push(ast::SwitchTypeCase {
                    pattern: None,
                    block,
                });
            } else {
                self.expect(Token::Case)?;
                self.expect(Token::LeftParen)?;
                let pattern = Some(self.parse_destructuring()?);
                self.expect(Token::RightParen)?;
                let block = self.parse_block(context.clone())?;
                cases.push(ast::SwitchTypeCase {
                    pattern,
                    block,
                });
            }
        }
        self.expect(Token::RightBrace)?;
        Ok(cases)
    }

    fn parse_do_statement(&mut self, context: DirectiveContext) -> Result<(Rc<ast::Statement>, bool), ParserFailure> {
        let context = context.override_control_context(false, ControlContext {
            breakable: true,
            iteration: true,
        });
        self.mark_location();
        self.next()?;

        // Body
        let (body, semicolon_inserted_1) = self.parse_substatement(context)?;
        if !semicolon_inserted_1 {
            self.expect(Token::Semicolon)?;
        }

        self.expect(Token::While)?;

        // Test
        self.expect(Token::LeftParen)?;
        let test = self.parse_expression(ExpressionContext { allow_in: true, min_precedence: OperatorPrecedence::List, ..default() })?;
        self.expect(Token::RightParen)?;

        let semicolon_inserted = self.parse_semicolon()?;
        Ok((Rc::new(ast::Statement {
            location: self.pop_location(),
            kind: ast::StatementKind::Do { body, test },
        }), semicolon_inserted))
    }

    fn parse_while_statement(&mut self, context: DirectiveContext) -> Result<(Rc<ast::Statement>, bool), ParserFailure> {
        let context = context.override_control_context(false, ControlContext {
            breakable: true,
            iteration: true,
        });
        self.mark_location();
        self.next()?;

        // Test
        self.expect(Token::LeftParen)?;
        let test = self.parse_expression(ExpressionContext { allow_in: true, min_precedence: OperatorPrecedence::List, ..default() })?;
        self.expect(Token::RightParen)?;

        // Body
        let (body, semicolon_inserted) = self.parse_substatement(context)?;

        Ok((Rc::new(ast::Statement {
            location: self.pop_location(),
            kind: ast::StatementKind::While { test, body },
        }), semicolon_inserted))
    }

    /// Parses `for`, `for..in` or `for each`.
    fn parse_for_statement(&mut self, context: DirectiveContext) -> Result<(Rc<ast::Statement>, bool), ParserFailure> {
        let context = context.override_control_context(false, ControlContext {
            breakable: true,
            iteration: true,
        });
        self.mark_location();
        self.next()?;

        // `for each`
        if self.consume_context_keyword("each")? {
            return self.parse_for_each_statement(context);
        }

        self.expect(Token::LeftParen)?;

        let init_variable = if self.peek(Token::Var) || self.peek(Token::Const) {
            Some(self.parse_simple_variable_declaration(false)?)
        } else {
            None
        };

        if init_variable.is_some() && self.consume(Token::In)? {
            return self.parse_for_in_statement_with_left_variable(context, init_variable.unwrap());
        }

        let mut init_exp = if init_variable.is_none() && !self.peek(Token::Semicolon) {
            self.parse_opt_expression(ExpressionContext {
                allow_in: false,
                min_precedence: OperatorPrecedence::Postfix,
                ..default()
            })?
        } else {
            None
        };

        if init_exp.is_some() && self.consume(Token::In)? {
            return self.parse_for_in_statement_with_left_exp(context, init_exp.unwrap());
        }

        if init_exp.is_none() && init_variable.is_none() && !self.peek(Token::Semicolon) {
            init_exp = Some(self.parse_expression(ExpressionContext {
                allow_in: false, min_precedence: OperatorPrecedence::List, ..default()
            })?);
        } else if let Some(exp) = init_exp.as_ref() {
            init_exp = Some(self.parse_subexpressions(exp.clone(), ExpressionContext {
                allow_in: false, min_precedence: OperatorPrecedence::List, ..default()
            })?);
        }

        let init = if let Some(exp) = init_exp.as_ref() {
            Some(ast::ForInit::Expression(exp.clone()))
        } else if let Some(variable) = init_variable.as_ref() {
            Some(ast::ForInit::Variable(variable.clone()))
        } else {
            None
        };

        self.expect(Token::Semicolon)?;
        let test = if self.peek(Token::Semicolon) {
            None
        } else {
            Some(self.parse_expression(ExpressionContext {
                allow_in: true, min_precedence: OperatorPrecedence::List, ..default()
            })?)
        };
        self.expect(Token::Semicolon)?;
        let update = if self.peek(Token::RightParen) {
            None
        } else {
            Some(self.parse_expression(ExpressionContext {
                allow_in: true, min_precedence: OperatorPrecedence::List, ..default()
            })?)
        };
        self.expect(Token::RightParen)?;

        // Body
        let (body, semicolon_inserted) = self.parse_substatement(context)?;

        Ok((Rc::new(ast::Statement {
            location: self.pop_location(),
            kind: ast::StatementKind::For { init, test, update, body },
        }), semicolon_inserted))
    }

    fn parse_for_each_statement(&mut self, context: DirectiveContext) -> Result<(Rc<ast::Statement>, bool), ParserFailure> {
        self.expect(Token::LeftParen)?;
        let left = if self.peek(Token::Var) || self.peek(Token::Const) {
            let kind = if self.peek(Token::Var) { ast::VariableKind::Var } else { ast::VariableKind::Const };
            self.next()?;
            let binding = self.parse_variable_binding(false)?;
            if let Some(init) = &binding.init {
                self.add_syntax_error(&init.location.clone(), DiagnosticKind::IllegalForInInitializer, vec![]);
            }
            ast::ForInLeft::Variable(kind, binding)
        } else {
            ast::ForInLeft::Expression(self.parse_expression(ExpressionContext {
                allow_in: false, min_precedence: OperatorPrecedence::Postfix, ..default()
            })?)
        };
        self.expect(Token::In)?;
        let right = self.parse_expression(ExpressionContext {
            allow_in: true, min_precedence: OperatorPrecedence::List, ..default()
        })?;
        self.expect(Token::RightParen)?;

        // Body
        let (body, semicolon_inserted) = self.parse_substatement(context)?;

        Ok((Rc::new(ast::Statement {
            location: self.pop_location(),
            kind: ast::StatementKind::ForIn { each: true, left, right, body },
        }), semicolon_inserted))
    }

    fn parse_for_in_statement_with_left_variable(&mut self, context: DirectiveContext, left: ast::SimpleVariableDeclaration) -> Result<(Rc<ast::Statement>, bool), ParserFailure> {
        let variable_kind = left.kind.0;
        let variable_binding = left.bindings[0].clone();

        if let Some(init) = &variable_binding.init {
            self.add_syntax_error(&init.location.clone(), DiagnosticKind::IllegalForInInitializer, vec![]);
        }

        if left.bindings.len() > 1 {
            self.add_syntax_error(&left.kind.1.clone(), DiagnosticKind::MultipleForInBindings, vec![]);
        }

        let right = self.parse_expression(ExpressionContext {
            allow_in: true, min_precedence: OperatorPrecedence::List, ..default()
        })?;
        self.expect(Token::RightParen)?;

        // Body
        let (body, semicolon_inserted) = self.parse_substatement(context)?;

        Ok((Rc::new(ast::Statement {
            location: self.pop_location(),
            kind: ast::StatementKind::ForIn { each: false, left: ast::ForInLeft::Variable(variable_kind, variable_binding), right, body },
        }), semicolon_inserted))
    }

    fn parse_for_in_statement_with_left_exp(&mut self, context: DirectiveContext, left: Rc<ast::Expression>) -> Result<(Rc<ast::Statement>, bool), ParserFailure> {
        let right = self.parse_expression(ExpressionContext {
            allow_in: true, min_precedence: OperatorPrecedence::List, ..default()
        })?;
        self.expect(Token::RightParen)?;

        // Body
        let (body, semicolon_inserted) = self.parse_substatement(context)?;

        Ok((Rc::new(ast::Statement {
            location: self.pop_location(),
            kind: ast::StatementKind::ForIn { each: false, left: ast::ForInLeft::Expression(left), right, body },
        }), semicolon_inserted))
    }

    fn parse_simple_variable_declaration(&mut self, allow_in: bool) -> Result<ast::SimpleVariableDeclaration, ParserFailure> {
        let kind: ast::VariableKind;
        let kind_location = self.token_location();
        if self.consume(Token::Const)? {
            kind = ast::VariableKind::Const;
        } else {
            self.expect(Token::Var)?;
            kind = ast::VariableKind::Var;
        }
        let mut bindings = vec![self.parse_variable_binding(allow_in)?];
        while self.consume(Token::Comma)? {
            bindings.push(self.parse_variable_binding(allow_in)?);
        }
        Ok(ast::SimpleVariableDeclaration {
            kind: (kind, kind_location),
            bindings,
        })
    }

    fn parse_with_statement(&mut self, context: DirectiveContext) -> Result<(Rc<ast::Statement>, bool), ParserFailure> {
        let context = context.override_control_context(true, ControlContext {
            breakable: true,
            iteration: false,
        });
        self.mark_location();
        self.next()?;

        // Object
        self.expect(Token::LeftParen)?;
        let object = self.parse_expression(ExpressionContext { allow_in: true, min_precedence: OperatorPrecedence::List, ..default() })?;
        self.expect(Token::RightParen)?;

        // Body
        let (body, semicolon_inserted) = self.parse_substatement(context)?;

        Ok((Rc::new(ast::Statement {
            location: self.pop_location(),
            kind: ast::StatementKind::With { object, body },
        }), semicolon_inserted))
    }

    fn parse_break_statement(&mut self, context: DirectiveContext) -> Result<(Rc<ast::Statement>, bool), ParserFailure> {
        self.mark_location();
        self.next()?;

        let label = if self.previous_token.1.line_break(&self.token.1) { None } else { self.consume_identifier(false)? };
        let label_location = label.clone().map(|label| label.1.clone());
        let label = label.map(|label| label.0.clone());

        let semicolon_inserted = self.parse_semicolon()?;

        let node = Rc::new(ast::Statement {
            location: self.pop_location(),
            kind: ast::StatementKind::Break { label: label.clone() },
        });

        if label.is_some() && !context.is_label_defined(label.clone().unwrap()) {
            self.add_syntax_error(&label_location.unwrap(), DiagnosticKind::UndefinedLabel, diagnostic_arguments![String(label.clone().unwrap())]);
        } else if !context.is_break_allowed(label) {
            self.add_syntax_error(&node.location.clone(), DiagnosticKind::IllegalBreak, vec![]);
        }

        Ok((node, semicolon_inserted))
    }

    fn parse_continue_statement(&mut self, context: DirectiveContext) -> Result<(Rc<ast::Statement>, bool), ParserFailure> {
        self.mark_location();
        self.next()?;

        let label = if self.previous_token.1.line_break(&self.token.1) { None } else { self.consume_identifier(false)? };
        let label_location = label.clone().map(|label| label.1.clone());
        let label = label.map(|label| label.0.clone());

        let semicolon_inserted = self.parse_semicolon()?;

        let node = Rc::new(ast::Statement {
            location: self.pop_location(),
            kind: ast::StatementKind::Continue { label: label.clone() },
        });

        if label.is_some() && !context.is_label_defined(label.clone().unwrap()) {
            self.add_syntax_error(&label_location.unwrap(), DiagnosticKind::UndefinedLabel, diagnostic_arguments![String(label.clone().unwrap())]);
        } else if !context.is_continue_allowed(label) {
            self.add_syntax_error(&node.location.clone(), DiagnosticKind::IllegalContinue, vec![]);
        }

        Ok((node, semicolon_inserted))
    }

    fn parse_return_statement(&mut self, _context: DirectiveContext) -> Result<(Rc<ast::Statement>, bool), ParserFailure> {
        self.mark_location();
        self.next()?;

        let expression = if self.previous_token.1.line_break(&self.token.1) { None } else {
            self.parse_opt_expression(ExpressionContext {
                allow_in: true,
                min_precedence: OperatorPrecedence::List,
                ..default()
            })?
        };

        let semicolon_inserted = self.parse_semicolon()?;

        let node = Rc::new(ast::Statement {
            location: self.pop_location(),
            kind: ast::StatementKind::Return { expression },
        });

        Ok((node, semicolon_inserted))
    }

    fn parse_throw_statement(&mut self, _context: DirectiveContext) -> Result<(Rc<ast::Statement>, bool), ParserFailure> {
        self.mark_location();
        self.next()?;

        let line_break = self.previous_token.1.line_break(&self.token.1);

        let expression = self.parse_expression(ExpressionContext {
            allow_in: true,
            min_precedence: OperatorPrecedence::List,
            ..default()
        })?;

        if line_break {
            self.add_syntax_error(&expression.location.clone(), DiagnosticKind::ExpressionMustNotFollowLineBreak, vec![]);
        }

        let semicolon_inserted = self.parse_semicolon()?;

        let node = Rc::new(ast::Statement {
            location: self.pop_location(),
            kind: ast::StatementKind::Throw { expression },
        });

        Ok((node, semicolon_inserted))
    }

    fn parse_try_statement(&mut self, context: DirectiveContext) -> Result<(Rc<ast::Statement>, bool), ParserFailure> {
        self.mark_location();
        self.next()?;
        let context = context.clone_control();
        let block = self.parse_block(context.clone())?;
        let mut catch_clauses: Vec<ast::CatchClause> = vec![];
        let mut finally_clause: Option<ast::FinallyClause> = None;
        loop {
            if self.consume(Token::Catch)? {
                self.expect(Token::LeftParen)?;
                let pattern = self.parse_destructuring()?;
                self.expect(Token::RightParen)?;
                let block = self.parse_block(context.clone())?;
                catch_clauses.push(ast::CatchClause { pattern, block });
            } else if self.consume(Token::Finally)? {
                finally_clause = Some(ast::FinallyClause(self.parse_block(context.clone())?));
                break;
            } else {
                break;
            }
        }
        if catch_clauses.is_empty() && finally_clause.is_none() {
            self.expect(Token::Catch)?;
        }

        let node = Rc::new(ast::Statement {
            location: self.pop_location(),
            kind: ast::StatementKind::Try { block, catch_clauses, finally_clause },
        });

        Ok((node, true))
    }

    fn parse_default_xml_namespace_statement(&mut self) -> Result<(Rc<ast::Statement>, bool), ParserFailure> {
        self.mark_location();
        self.next()?;

        self.forbid_line_break_before_token();
        self.expect_context_keyword("xml")?;
        self.forbid_line_break_before_token();
        self.expect_context_keyword("namespace")?;
        self.expect(Token::Assign)?;

        let expression = self.parse_expression(ExpressionContext {
            allow_in: true,
            allow_assignment: false,
            min_precedence: OperatorPrecedence::AssignmentAndOther,
            ..default()
        })?;

        let semicolon_inserted = self.parse_semicolon()?;

        let node = Rc::new(ast::Statement {
            location: self.pop_location(),
            kind: ast::StatementKind::DefaultXmlNamespace(expression),
        });

        Ok((node, semicolon_inserted))
    }

    fn forbid_line_break_before_token(&mut self) {
        if self.previous_token.1.line_break(&self.token.1) {
            self.add_syntax_error(&self.token.1.clone(), DiagnosticKind::TokenMustNotFollowLineBreak, vec![]);
        }
    }

    fn parse_directive(&mut self, context: DirectiveContext) -> Result<(Rc<ast::Directive>, bool), ParserFailure> {
        if self.peek(Token::Import) {
            self.parse_import_directive()
        } else if self.peek(Token::Export) {
            self.parse_export_directive()
        } else if self.peek(Token::Use) {
            self.parse_use_directive()
        } else {
            let start = self.token_location();

            // AnnotatableDefinition (reservedWord)
            if self.peek_annotatable_definition_reserved_word() {
                let asdoc = self.parse_asdoc()?;
                return self.parse_annotatable_definition(AnnotatableContext {
                    start,
                    metadata_exp: vec![],
                    asdoc,
                    first_modifier: None,
                    previous_token_is_definition_keyword: false,
                    context,
                });
            }

            let (statement, semicolon_inserted) = self.parse_statement(context.clone())?;

            let id = statement.to_identifier();
            if let Some(id) = id {
                if id.0 == "include" && id.1.character_count() == "include".len() && matches!(self.token.0, Token::StringLiteral(_)) && !semicolon_inserted {
                    return self.parse_include_directive(context.clone(), start.clone());
                }

                // AnnotatableDefinition (modifierOrContextKeyword modifierOrNameOrKeyword)
                if !semicolon_inserted && (matches!(self.token.0, Token::Identifier(_)) || self.peek_reserved_namespace().is_some() || self.peek_annotatable_definition_reserved_word()) {
                    let previous_token_is_definition_keyword = is_annotatable_definition_context_keyword(&id);
                    return self.parse_annotatable_definition(AnnotatableContext {
                        start,
                        metadata_exp: vec![],
                        asdoc: statement.extract_asdoc(),
                        first_modifier: if previous_token_is_definition_keyword { None } else { statement.to_identifier_or_reserved_namespace() },
                        previous_token_is_definition_keyword,
                        context,
                    });
                }
            } else if let Some(first_modifier) = statement.to_identifier_or_reserved_namespace() {
                // AnnotatableDefinition (modifier modifierOrKeyword)
                if !semicolon_inserted && (matches!(self.token.0, Token::Identifier(_))
                    || self.peek_reserved_namespace().is_some())
                    || self.peek_annotatable_definition_reserved_word() {
                    return self.parse_annotatable_definition(AnnotatableContext {
                        start,
                        metadata_exp: vec![],
                        asdoc: statement.extract_asdoc(),
                        first_modifier: Some(first_modifier),
                        previous_token_is_definition_keyword: false,
                        context,
                    });
                }
            // AnnotatableDefinition (metaData)
            } else if let Some(metadata_exp) = statement.list_metadata_expressions() {
                let asdoc = self.parse_asdoc()?;
                return self.parse_annotatable_definition(AnnotatableContext {
                    start,
                    metadata_exp,
                    asdoc,
                    first_modifier: None,
                    previous_token_is_definition_keyword: false,
                    context,
                });
            }

            self.push_location(&start);

            Ok((Rc::new(ast::Directive {
                location: self.pop_location(),
                kind: ast::DirectiveKind::Statement(statement.clone()),
            }), semicolon_inserted))
        }
    }

    /// `AnnotatableDefinition`
    ///
    /// ```ignore
    /// self.parse_annotatable_definition(AnnotatableContext {
    ///     start,
    ///     metadata_exp,
    ///     asdoc,
    ///     first_modifier,
    ///     previous_token_is_definition_keyword,
    ///     context,
    /// })?
    /// ```
    fn parse_annotatable_definition(&mut self, annotatable_context: AnnotatableContext) -> Result<(Rc<ast::Directive>, bool), ParserFailure> {
        // Parse meta data
        let metadata = self.exps_to_metadata(&annotatable_context.metadata_exp);

        // If AnnotatableDefinition starts directly from a context keyword (the previous token),
        // skip parsing attributes.
        if annotatable_context.previous_token_is_definition_keyword {
            return self.parse_annotatable_definition_after_context_keyword(
                annotatable_context.start.clone(),
                ast::Annotations {
                    metadata,
                    modifiers: ast::Modifiers::empty(),
                    access_modifier: None,
                },
                annotatable_context.asdoc,
                annotatable_context.context,
            );
        }

        let mut modifiers = ast::Modifiers::empty();
        let mut access_modifier: Option<Rc<ast::Expression>> = None;
        let mut found_modifier = false;
        let mut is_a_context_keyword_definition = false;

        // First attribute
        if let Some(m) = &annotatable_context.first_modifier {
            found_modifier = true;
            if let Some(m) = m.to_modifier() {
                modifiers |= m;
            } else {
                access_modifier = Some(m.clone());
            }
        }

        // Parse attributes
        loop {
            if found_modifier {
                self.forbid_line_break_before_token();
            }

            // Check for `class/interface/function/const/var`
            if self.peek_annotatable_definition_reserved_word() {
                break;
            }

            let mut exp = None;
            let mut modifier = None;
            let location: Location;

            if let Some(rns) = self.peek_reserved_namespace() {
                // ReservedNamespace attribute
                self.mark_location();
                self.next()?;
                location = self.pop_location();
                exp = Some(Rc::new(ast::Expression {
                    location: location.clone(),
                    kind: ast::ExpressionKind::ReservedNamespace(rns),
                }));
                found_modifier = true;
            } else {
                // Identifier attribute
                let id = self.expect_identifier(false)?;
                location = id.1.clone();

                // Check for `enum/namespace/type`
                if is_annotatable_definition_context_keyword(&id) {
                    is_a_context_keyword_definition = true;
                    break;
                }

                let exp2 = Rc::new(ast::Expression {
                    location: id.1.clone(),
                    kind: ast::ExpressionKind::Id(ast::QualifiedIdentifier {
                        attribute: false,
                        qualifier: None,
                        name: ast::IdentifierOrBrackets::Id(id.0, id.1.clone()),
                    }),
                });

                if let Some(modifier2) = exp2.to_modifier() {
                    modifier = Some(modifier2);
                } else {
                    exp = Some(exp2);
                }

                found_modifier = true;
            }

            // Contribute modifier and verify that it is not duplicate
            if let Some(exp) = exp.as_ref() {
                if access_modifier.is_some() {
                    self.add_syntax_error(&exp.location.clone(), DiagnosticKind::DuplicateModifier, vec![]);
                }
                access_modifier = Some(exp.clone());
            } else {
                let modifier = modifier.unwrap();
                if modifiers.contains(modifier) {
                    self.add_syntax_error(&location, DiagnosticKind::DuplicateModifier, vec![]);
                }
                modifiers |= modifier;
            }
        }

        let annotations = ast::Annotations {
            metadata,
            modifiers,
            access_modifier: access_modifier.as_ref().map(|m| m.clone()),
        };

        // Previous token is a ContextKeyword identifying an annotatable directive
        if is_a_context_keyword_definition {
            return self.parse_annotatable_definition_after_context_keyword(
                annotatable_context.start.clone(),
                annotations,
                annotatable_context.asdoc,
                annotatable_context.context,
            );
        }

        // VariableDefinition
        if self.peek(Token::Var) || self.peek(Token::Const) {
            return self.parse_variable_definition(
                annotatable_context.start.clone(),
                annotations,
                annotatable_context.asdoc,
                annotatable_context.context,
            );
        }

        // FunctionDefinition
        if self.peek(Token::Function) {
            return self.parse_function_definition(
                annotatable_context.start.clone(),
                annotations,
                annotatable_context.asdoc,
                annotatable_context.context,
            );
        }

        // ClassDefinition
        if self.peek(Token::Class) {
            return self.parse_class_definition(
                annotatable_context.start.clone(),
                annotations,
                annotatable_context.asdoc,
                annotatable_context.context,
            );
        }

        // InterfaceDefinition
        if self.peek(Token::Interface) {
            return self.parse_interface_definition(
                annotatable_context.start.clone(),
                annotations,
                annotatable_context.asdoc,
                annotatable_context.context,
            );
        }

        // Error: expected one of { "class", "interface", "function", "var", "const" }
        self.add_syntax_error(&self.token_location(), DiagnosticKind::UnexpectedOrInvalidToken, vec![]);
        Err(ParserFailure)
    }

    fn parse_class_definition(
        &mut self,
        start: Location,
        annotations: ast::Annotations,
        asdoc: Option<Rc<ast::AsDoc>>,
        context: DirectiveContext,
    ) -> Result<(Rc<ast::Directive>, bool), ParserFailure> {
        self.push_location(&start);

        self.next()?;

        let name = self.expect_identifier(true)?;
        let mut generics = self.parse_generics()?;
        let mut extends_clause: Option<Rc<ast::TypeExpression>> = None;
        let mut implements_clause: Option<Vec<Rc<ast::TypeExpression>>> = None;

        loop {
            let first_token_location = self.token_location();
            let where_clause = self.parse_generics_where_clause()?;
            // `where`
            if where_clause.is_some() {
                if generics.where_clause.is_some() {
                    self.add_syntax_error(&first_token_location.clone(), DiagnosticKind::DuplicateClause, diagnostic_arguments![String("where".into())]);
                }
                generics.where_clause = where_clause;
            // `extends`
            } else if self.peek(Token::Extends) {
                if extends_clause.is_some() {
                    self.add_syntax_error(&first_token_location.clone(), DiagnosticKind::DuplicateClause, diagnostic_arguments![String("extends".into())]);
                }
                self.next()?;
                extends_clause = Some(self.parse_type_expression()?);
            // `implements`
            } else if self.peek(Token::Implements) {
                if implements_clause.is_some() {
                    self.add_syntax_error(&first_token_location.clone(), DiagnosticKind::DuplicateClause, diagnostic_arguments![String("implements".into())]);
                }
                self.next()?;
                let mut list = vec![self.parse_type_expression()?];
                while self.consume(Token::Comma)? {
                    list.push(self.parse_type_expression()?);
                }
                implements_clause = Some(list);
            } else {
                break;
            }
        }

        let block = self.parse_block(DirectiveContext::ClassBlock { name: name.0.clone() })?;

        let modifiers = annotations.modifiers.clone();

        let node = Rc::new(ast::Directive {
            location: self.pop_location(),
            kind: ast::DirectiveKind::ClassDefinition(Rc::new(ast::ClassDefinition {
                asdoc,
                annotations,
                name: name.clone(),
                generics,
                extends_clause,
                implements_clause,
                block,
            })),
        });

        // Do not allow nested classes
        if !(matches!(context, DirectiveContext::PackageBlock | DirectiveContext::TopLevel)) {
            self.add_syntax_error(&name.1.clone(), DiagnosticKind::UnallowedNestedClasses, diagnostic_arguments![]);
        }

        // Always allow `static`

        // Forbid `native`
        if modifiers.contains(ast::Modifiers::NATIVE) {
            self.add_syntax_error(&name.1.clone(), DiagnosticKind::UnallowedModifier, diagnostic_arguments![String("native".into())]);
        }

        // Forbid `override`
        if modifiers.contains(ast::Modifiers::OVERRIDE) {
            self.add_syntax_error(&name.1.clone(), DiagnosticKind::UnallowedModifier, diagnostic_arguments![String("override".into())]);
        }

        // Always allow `dynamic`

        // Always allow `final`

        Ok((node, true))
    }

    fn parse_interface_definition(
        &mut self,
        start: Location,
        annotations: ast::Annotations,
        asdoc: Option<Rc<ast::AsDoc>>,
        context: DirectiveContext,
    ) -> Result<(Rc<ast::Directive>, bool), ParserFailure> {
        self.push_location(&start);

        self.next()?;

        let name = self.expect_identifier(true)?;
        let mut generics = self.parse_generics()?;
        let mut extends_clause: Option<Vec<Rc<ast::TypeExpression>>> = None;

        loop {
            let first_token_location = self.token_location();
            let where_clause = self.parse_generics_where_clause()?;
            // `where`
            if where_clause.is_some() {
                if generics.where_clause.is_some() {
                    self.add_syntax_error(&first_token_location.clone(), DiagnosticKind::DuplicateClause, diagnostic_arguments![String("where".into())]);
                }
                generics.where_clause = where_clause;
            // `extends`
            } else if self.peek(Token::Extends) {
                if extends_clause.is_some() {
                    self.add_syntax_error(&first_token_location.clone(), DiagnosticKind::DuplicateClause, diagnostic_arguments![String("extends".into())]);
                }
                self.next()?;
                let mut list = vec![self.parse_type_expression()?];
                while self.consume(Token::Comma)? {
                    list.push(self.parse_type_expression()?);
                }
                extends_clause = Some(list);
            } else {
                break;
            }
        }

        let block = self.parse_block(DirectiveContext::InterfaceBlock)?;

        let modifiers = annotations.modifiers.clone();

        let node = Rc::new(ast::Directive {
            location: self.pop_location(),
            kind: ast::DirectiveKind::InterfaceDefinition(Rc::new(ast::InterfaceDefinition {
                asdoc,
                annotations,
                name: name.clone(),
                generics,
                extends_clause,
                block,
            })),
        });

        // Do not allow nested classes
        if !(matches!(context, DirectiveContext::PackageBlock | DirectiveContext::TopLevel)) {
            self.add_syntax_error(&name.1.clone(), DiagnosticKind::UnallowedNestedClasses, diagnostic_arguments![]);
        }

        // Forbid `static`
        if modifiers.contains(ast::Modifiers::STATIC) {
            self.add_syntax_error(&name.1.clone(), DiagnosticKind::UnallowedModifier, diagnostic_arguments![String("static".into())]);
        }

        // Forbid `native`
        if modifiers.contains(ast::Modifiers::NATIVE) {
            self.add_syntax_error(&name.1.clone(), DiagnosticKind::UnallowedModifier, diagnostic_arguments![String("native".into())]);
        }

        // Forbid `override`
        if modifiers.contains(ast::Modifiers::OVERRIDE) {
            self.add_syntax_error(&name.1.clone(), DiagnosticKind::UnallowedModifier, diagnostic_arguments![String("override".into())]);
        }

        // Forbid `dynamic`
        if modifiers.contains(ast::Modifiers::DYNAMIC) {
            self.add_syntax_error(&name.1.clone(), DiagnosticKind::UnallowedModifier, diagnostic_arguments![String("dynamic".into())]);
        }

        // Forbid `final`
        if modifiers.contains(ast::Modifiers::FINAL) {
            self.add_syntax_error(&name.1.clone(), DiagnosticKind::UnallowedModifier, diagnostic_arguments![String("final".into())]);
        }

        Ok((node, true))
    }

    fn parse_function_definition(
        &mut self,
        start: Location,
        annotations: ast::Annotations,
        asdoc: Option<Rc<ast::AsDoc>>,
        context: DirectiveContext,
    ) -> Result<(Rc<ast::Directive>, bool), ParserFailure> {
        self.push_location(&start);

        self.next()?;

        let modifiers = annotations.modifiers.clone();

        let name = self.expect_identifier(true)?;

        // `function get x`
        // `function set x`
        if let Some(name1) = self.peek_identifier(true)? {
            if name.0 == "get" && name.1.character_count() == "get".len() {
                self.next()?;
                return self.parse_getter_or_setter_definition(annotations, asdoc, context, name1, true);
            }
            if name.0 == "set" && name.1.character_count() == "set".len() {
                self.next()?;
                return self.parse_getter_or_setter_definition(annotations, asdoc, context, name1, false);
            }
        }

        let is_constructor = if let DirectiveContext::ClassBlock { name: name2 } = &context {
            !modifiers.contains(ast::Modifiers::STATIC) && &name.0 == name2
        } else {
            false
        };

        let mut generics = self.parse_generics()?;

        let body_context = if is_constructor {
            DirectiveContext::ConstructorBlock { super_statement_found: Cell::new(false) }
        } else {
            DirectiveContext::Default
        };

        let (common, where_clause) = self.parse_function_common(false, body_context)?;

        // Parse semicolon
        let semicolon = if let Some(body) = &common.body {
            match body {
                ast::FunctionBody::Expression(_) => self.parse_semicolon()?,
                ast::FunctionBody::Block(_) => true,
            }
        } else {
            self.parse_semicolon()?
        };

        generics.where_clause = where_clause;

        let is_interface_method = matches!(context, DirectiveContext::InterfaceBlock);
        let is_native = modifiers.contains(ast::Modifiers::NATIVE);

        // Body verification
        if common.body.is_some() {
            if is_native || is_interface_method {
                self.add_syntax_error(&name.1.clone(), DiagnosticKind::MethodMustNotHaveBody, diagnostic_arguments![]);
            }
        } else {
            if !(is_native || !is_interface_method) {
                self.add_syntax_error(&name.1.clone(), DiagnosticKind::MethodMustSpecifyBody, diagnostic_arguments![]);
            }
        }

        // Interface method must not contain annotations
        if is_interface_method && (annotations.access_modifier.is_some() || !annotations.metadata.is_empty() || !modifiers.is_empty()) {
            self.add_syntax_error(&name.1.clone(), DiagnosticKind::InterfaceMethodHasAnnotations, diagnostic_arguments![]);
        }

        // Allow `override` in type blocks and non constructors only
        if modifiers.contains(ast::Modifiers::OVERRIDE) && (!context.is_type_block() || is_constructor) {
            self.add_syntax_error(&name.1.clone(), DiagnosticKind::UnallowedModifier, diagnostic_arguments![String("override".into())]);
        }

        // Allow `final` in type blocks only
        if modifiers.contains(ast::Modifiers::FINAL) && !context.is_type_block() {
            self.add_syntax_error(&name.1.clone(), DiagnosticKind::UnallowedModifier, diagnostic_arguments![String("final".into())]);
        }

        // Forbid `dynamic`
        if modifiers.contains(ast::Modifiers::DYNAMIC) {
            self.add_syntax_error(&name.1.clone(), DiagnosticKind::UnallowedModifier, diagnostic_arguments![String("dynamic".into())]);
        }

        // Always allow `native`

        // Allow `static` in type blocks only
        if modifiers.contains(ast::Modifiers::STATIC) && !context.is_type_block() {
            self.add_syntax_error(&name.1.clone(), DiagnosticKind::UnallowedModifier, diagnostic_arguments![String("static".into())]);
        }

        // Constructor must not have generics
        if is_constructor && (generics.params.is_some() || generics.where_clause.is_some()) {
            self.add_syntax_error(&name.1.clone(), DiagnosticKind::MethodMustNotHaveGenerics, diagnostic_arguments![]);
        }

        let node = if is_constructor {
            Rc::new(ast::Directive {
                location: self.pop_location(),
                kind: ast::DirectiveKind::ConstructorDefinition(Rc::new(ast::ConstructorDefinition {
                    asdoc, annotations, name, common,
                })),
            })
        } else {
            Rc::new(ast::Directive {
                location: self.pop_location(),
                kind: ast::DirectiveKind::FunctionDefinition(Rc::new(ast::FunctionDefinition {
                    asdoc, annotations, name, generics, common,
                })),
            })
        };

        Ok((node, semicolon))
    }

    fn parse_getter_or_setter_definition(
        &mut self,
        annotations: ast::Annotations,
        asdoc: Option<Rc<ast::AsDoc>>,
        context: DirectiveContext,
        name: (String, Location),
        getter: bool,
    ) -> Result<(Rc<ast::Directive>, bool), ParserFailure> {
        let (common, where_clause) = self.parse_function_common(false, DirectiveContext::Default)?;

        // Parse semicolon
        let semicolon = if let Some(body) = &common.body {
            match body {
                ast::FunctionBody::Expression(_) => self.parse_semicolon()?,
                ast::FunctionBody::Block(_) => true,
            }
        } else {
            self.parse_semicolon()?
        };

        let modifiers = annotations.modifiers.clone();

        let is_interface_method = matches!(context, DirectiveContext::InterfaceBlock);
        let is_native = modifiers.contains(ast::Modifiers::NATIVE);

        // Body verification
        if common.body.is_some() {
            if is_native || is_interface_method {
                self.add_syntax_error(&name.1.clone(), DiagnosticKind::MethodMustNotHaveBody, diagnostic_arguments![]);
            }
        } else {
            if !(is_native || !is_interface_method) {
                self.add_syntax_error(&name.1.clone(), DiagnosticKind::MethodMustSpecifyBody, diagnostic_arguments![]);
            }
        }

        // Interface method must not contain annotations
        if is_interface_method && (annotations.access_modifier.is_some() || !annotations.metadata.is_empty() || !modifiers.is_empty()) {
            self.add_syntax_error(&name.1.clone(), DiagnosticKind::InterfaceMethodHasAnnotations, diagnostic_arguments![]);
        }

        // Allow `override` in type blocks only
        if modifiers.contains(ast::Modifiers::OVERRIDE) && !context.is_type_block() {
            self.add_syntax_error(&name.1.clone(), DiagnosticKind::UnallowedModifier, diagnostic_arguments![String("override".into())]);
        }

        // Allow `final` in type blocks only
        if modifiers.contains(ast::Modifiers::FINAL) && !context.is_type_block() {
            self.add_syntax_error(&name.1.clone(), DiagnosticKind::UnallowedModifier, diagnostic_arguments![String("final".into())]);
        }

        // Forbid `dynamic`
        if modifiers.contains(ast::Modifiers::DYNAMIC) {
            self.add_syntax_error(&name.1.clone(), DiagnosticKind::UnallowedModifier, diagnostic_arguments![String("dynamic".into())]);
        }

        // Always allow `native`

        // Allow `static` in type blocks only
        if modifiers.contains(ast::Modifiers::STATIC) && !context.is_type_block() {
            self.add_syntax_error(&name.1.clone(), DiagnosticKind::UnallowedModifier, diagnostic_arguments![String("static".into())]);
        }

        // Getters and setters must not have generics
        if where_clause.is_some() {
            self.add_syntax_error(&name.1.clone(), DiagnosticKind::MethodMustNotHaveGenerics, diagnostic_arguments![]);
        }

        let node = if getter {
            Rc::new(ast::Directive {
                location: self.pop_location(),
                kind: ast::DirectiveKind::GetterDefinition(Rc::new(ast::GetterDefinition {
                    asdoc, annotations, name, common,
                })),
            })
        } else {
            Rc::new(ast::Directive {
                location: self.pop_location(),
                kind: ast::DirectiveKind::SetterDefinition(Rc::new(ast::SetterDefinition {
                    asdoc, annotations, name, common,
                })),
            })
        };

        Ok((node, semicolon))
    }

    /// Parses optional generics declaration in the form `.<T, ...Tn>`.
    fn parse_generics(&mut self) -> Result<ast::Generics, ParserFailure> {
        if !self.peek(Token::Dot) {
            return Ok(ast::Generics {
                params: None,
                where_clause: None,
            });
        }
        self.next()?;
        self.expect(Token::Lt)?;

        let mut params = vec![self.parse_generic_param()?];
        while self.consume(Token::Comma)? {
            params.push(self.parse_generic_param()?);
        }

        self.expect_generics_gt()?;

        Ok(ast::Generics {
            params: Some(params),
            where_clause: None,
        })
    }

    fn parse_generic_param(&mut self) -> Result<Rc<ast::GenericParam>, ParserFailure> {
        self.mark_location();
        let name = self.expect_identifier(false)?;

        // C1
        // C1 + ..Cn
        let mut constraints = vec![];
        if self.consume(Token::Colon)? {
            constraints.push(self.parse_type_expression()?);
            while self.consume(Token::Plus)? {
                constraints.push(self.parse_type_expression()?);
            }
        }

        // T = D
        let default_type = if self.consume(Token::Assign)? {
            Some(self.parse_type_expression()?)
        } else {
            None
        };

        Ok(Rc::new(ast::GenericParam {
            location: self.pop_location(),
            name,
            constraints,
            default_type,
        }))
    }

    fn parse_variable_definition(
        &mut self,
        start: Location,
        annotations: ast::Annotations,
        asdoc: Option<Rc<ast::AsDoc>>,
        context: DirectiveContext,
    ) -> Result<(Rc<ast::Directive>, bool), ParserFailure> {
        self.push_location(&start);

        let kind: ast::VariableKind;
        if self.consume(Token::Const)? {
            kind = ast::VariableKind::Const;
        } else {
            self.expect(Token::Var)?;
            kind = ast::VariableKind::Var;
        }
        let mut bindings = vec![self.parse_variable_binding(true)?];
        while self.consume(Token::Comma)? {
            bindings.push(self.parse_variable_binding(true)?);
        }

        let semicolon = self.parse_semicolon()?;

        let modifiers = annotations.modifiers.clone();

        let node = Rc::new(ast::Directive {
            location: self.pop_location(),
            kind: ast::DirectiveKind::VariableDefinition(Rc::new(ast::VariableDefinition {
                asdoc,
                annotations,
                kind,
                bindings,
            })),
        });

        // Allow `static` in type blocks only
        if modifiers.contains(ast::Modifiers::STATIC) && !context.is_type_block() {
            self.add_syntax_error(&node.location.clone(), DiagnosticKind::UnallowedModifier, diagnostic_arguments![String("static".into())]);
        }

        // Forbid `native`
        if modifiers.contains(ast::Modifiers::NATIVE) {
            self.add_syntax_error(&node.location.clone(), DiagnosticKind::UnallowedModifier, diagnostic_arguments![String("native".into())]);
        }

        // Forbid `override`
        if modifiers.contains(ast::Modifiers::OVERRIDE) {
            self.add_syntax_error(&node.location.clone(), DiagnosticKind::UnallowedModifier, diagnostic_arguments![String("override".into())]);
        }

        // Forbid `dynamic`
        if modifiers.contains(ast::Modifiers::DYNAMIC) {
            self.add_syntax_error(&node.location.clone(), DiagnosticKind::UnallowedModifier, diagnostic_arguments![String("dynamic".into())]);
        }

        // Forbid `final`
        if modifiers.contains(ast::Modifiers::FINAL) {
            self.add_syntax_error(&node.location.clone(), DiagnosticKind::UnallowedModifier, diagnostic_arguments![String("final".into())]);
        }

        Ok((node, semicolon))
    }

    fn parse_type_definition(
        &mut self,
        start: Location,
        annotations: ast::Annotations,
        asdoc: Option<Rc<ast::AsDoc>>,
        context: DirectiveContext,
    ) -> Result<(Rc<ast::Directive>, bool), ParserFailure> {
        self.push_location(&start);

        let left = self.expect_identifier(false)?;
        let generics = self.parse_generics()?;
        self.expect(Token::Assign)?;
        let right = self.parse_type_expression()?;
        let semicolon = self.parse_semicolon()?;

        let modifiers = annotations.modifiers.clone();

        let node = Rc::new(ast::Directive {
            location: self.pop_location(),
            kind: ast::DirectiveKind::TypeDefinition(Rc::new(ast::TypeDefinition {
                asdoc,
                annotations,
                left: left.clone(),
                generics,
                right,
            })),
        });

        // Allow `static` in type blocks only
        if modifiers.contains(ast::Modifiers::STATIC) && !context.is_type_block() {
            self.add_syntax_error(&left.1.clone(), DiagnosticKind::UnallowedModifier, diagnostic_arguments![String("static".into())]);
        }

        // Forbid `native`
        if modifiers.contains(ast::Modifiers::NATIVE) {
            self.add_syntax_error(&left.1.clone(), DiagnosticKind::UnallowedModifier, diagnostic_arguments![String("native".into())]);
        }

        // Forbid `override`
        if modifiers.contains(ast::Modifiers::OVERRIDE) {
            self.add_syntax_error(&left.1.clone(), DiagnosticKind::UnallowedModifier, diagnostic_arguments![String("override".into())]);
        }

        // Forbid `dynamic`
        if modifiers.contains(ast::Modifiers::DYNAMIC) {
            self.add_syntax_error(&left.1.clone(), DiagnosticKind::UnallowedModifier, diagnostic_arguments![String("dynamic".into())]);
        }

        // Forbid `final`
        if modifiers.contains(ast::Modifiers::FINAL) {
            self.add_syntax_error(&left.1.clone(), DiagnosticKind::UnallowedModifier, diagnostic_arguments![String("final".into())]);
        }

        Ok((node, semicolon))
    }

    fn parse_namespace_definition(
        &mut self,
        start: Location,
        annotations: ast::Annotations,
        asdoc: Option<Rc<ast::AsDoc>>,
        context: DirectiveContext,
    ) -> Result<(Rc<ast::Directive>, bool), ParserFailure> {
        self.push_location(&start);

        let left = self.expect_identifier(false)?;
        let right = if self.consume(Token::Assign)? {
            Some(self.parse_expression(ExpressionContext {
                allow_in: true,
                min_precedence: OperatorPrecedence::AssignmentAndOther,
                ..default()
            })?)
        } else {
            None
        };
        let semicolon = self.parse_semicolon()?;

        let modifiers = annotations.modifiers.clone();

        let node = Rc::new(ast::Directive {
            location: self.pop_location(),
            kind: ast::DirectiveKind::NamespaceDefinition(Rc::new(ast::NamespaceDefinition {
                asdoc,
                annotations,
                left: left.clone(),
                right,
            })),
        });

        // Allow `static` in type blocks only
        if modifiers.contains(ast::Modifiers::STATIC) && !context.is_type_block() {
            self.add_syntax_error(&left.1.clone(), DiagnosticKind::UnallowedModifier, diagnostic_arguments![String("static".into())]);
        }

        // Forbid `native`
        if modifiers.contains(ast::Modifiers::NATIVE) {
            self.add_syntax_error(&left.1.clone(), DiagnosticKind::UnallowedModifier, diagnostic_arguments![String("native".into())]);
        }

        // Forbid `override`
        if modifiers.contains(ast::Modifiers::OVERRIDE) {
            self.add_syntax_error(&left.1.clone(), DiagnosticKind::UnallowedModifier, diagnostic_arguments![String("override".into())]);
        }

        // Forbid `dynamic`
        if modifiers.contains(ast::Modifiers::DYNAMIC) {
            self.add_syntax_error(&left.1.clone(), DiagnosticKind::UnallowedModifier, diagnostic_arguments![String("dynamic".into())]);
        }

        // Forbid `final`
        if modifiers.contains(ast::Modifiers::FINAL) {
            self.add_syntax_error(&left.1.clone(), DiagnosticKind::UnallowedModifier, diagnostic_arguments![String("final".into())]);
        }

        Ok((node, semicolon))
    }

    fn parse_enum_definition(
        &mut self,
        start: Location,
        annotations: ast::Annotations,
        asdoc: Option<Rc<ast::AsDoc>>,
        context: DirectiveContext,
    ) -> Result<(Rc<ast::Directive>, bool), ParserFailure> {
        self.push_location(&start);

        let name = self.expect_identifier(true)?;
        let block = self.parse_block(DirectiveContext::EnumBlock)?;
        let modifiers = annotations.modifiers.clone();

        let node = Rc::new(ast::Directive {
            location: self.pop_location(),
            kind: ast::DirectiveKind::EnumDefinition(Rc::new(ast::EnumDefinition {
                asdoc,
                annotations,
                name: name.clone(),
                block,
            })),
        });

        // Do not allow nested classes
        if !(matches!(context, DirectiveContext::PackageBlock | DirectiveContext::TopLevel)) {
            self.add_syntax_error(&name.1.clone(), DiagnosticKind::UnallowedNestedClasses, diagnostic_arguments![]);
        }

        // Forbid `static`
        if modifiers.contains(ast::Modifiers::STATIC) {
            self.add_syntax_error(&name.1.clone(), DiagnosticKind::UnallowedModifier, diagnostic_arguments![String("static".into())]);
        }

        // Forbid `native`
        if modifiers.contains(ast::Modifiers::NATIVE) {
            self.add_syntax_error(&name.1.clone(), DiagnosticKind::UnallowedModifier, diagnostic_arguments![String("native".into())]);
        }

        // Forbid `override`
        if modifiers.contains(ast::Modifiers::OVERRIDE) {
            self.add_syntax_error(&name.1.clone(), DiagnosticKind::UnallowedModifier, diagnostic_arguments![String("override".into())]);
        }

        // Forbid `dynamic`
        if modifiers.contains(ast::Modifiers::DYNAMIC) {
            self.add_syntax_error(&name.1.clone(), DiagnosticKind::UnallowedModifier, diagnostic_arguments![String("dynamic".into())]);
        }

        // Forbid `final`
        if modifiers.contains(ast::Modifiers::FINAL) {
            self.add_syntax_error(&name.1.clone(), DiagnosticKind::UnallowedModifier, diagnostic_arguments![String("final".into())]);
        }

        Ok((node, true))
    }

    fn parse_annotatable_definition_after_context_keyword(
        &mut self,
        start: Location,
        annotations: ast::Annotations,
        asdoc: Option<Rc<ast::AsDoc>>,
        context: DirectiveContext,
    ) -> Result<(Rc<ast::Directive>, bool), ParserFailure> {
        let Token::Identifier(previous_word) = &self.previous_token.0 else {
            panic!();
        };
        if !["namespace", "enum", "type"].contains(&previous_word.as_ref()) {
            panic!();
        }

        match previous_word.as_ref() {
            "type" => {
                self.parse_type_definition(start, annotations, asdoc, context)
            },
            "namespace" => {
                self.parse_namespace_definition(start, annotations, asdoc, context)
            },
            "enum" => {
                self.parse_enum_definition(start, annotations, asdoc, context)
            },
            _ => {
                panic!();
            },
        }
    }

    fn exps_to_metadata(&mut self, expressions: &Vec<Rc<ast::Expression>>) -> Vec<Rc<ast::Metadata>> {
        let mut result = vec![];
        for exp in expressions {
            if let Some(metadata) = self.exp_to_metadata(exp) {
                result.push(metadata);
            }
        }
        result
    }

    fn exp_to_metadata(&mut self, exp: &Rc<ast::Expression>) -> Option<Rc<ast::Metadata>> {
        match &exp.kind {
            ast::ExpressionKind::ArrayInitializer { elements, asdoc, .. } => {
                if elements.len() != 1 || elements[0].is_none() {
                    self.add_syntax_error(&exp.location.clone(), DiagnosticKind::MalformedMetadataElement, diagnostic_arguments![]);
                    None
                } else {
                    self.exp_to_metadata_1(asdoc.clone(), elements[0].as_ref().unwrap())
                }
            },
            ast::ExpressionKind::BracketsMember { key, asdoc, .. } => self.exp_to_metadata_1(asdoc.clone(), key),
            _ => None,
        }
    }

    fn exp_to_metadata_1(&mut self, asdoc: Option<Rc<ast::AsDoc>>, exp: &Rc<ast::Expression>) -> Option<Rc<ast::Metadata>> {
        match &exp.kind {
            ast::ExpressionKind::Id(id) => {
                if let Some(name) = id.to_metadata_name() {
                    Some(Rc::new(ast::Metadata { asdoc, location: exp.location.clone(), name, entries: vec![] }))
                } else {
                    self.add_syntax_error(&exp.location.clone(), DiagnosticKind::MalformedMetadataElement, diagnostic_arguments![]);
                    None
                }
            },
            ast::ExpressionKind::Call { base, arguments } => {
                if let Some(name) = self.exp_to_metadata_name(base) {
                    let mut entries = vec![];
                    for entry in arguments {
                        if let Some(entry) = self.exp_to_metadata_entry(entry) {
                            entries.push(entry);
                        }
                    }
                    Some(Rc::new(ast::Metadata { asdoc, location: exp.location.clone(), name, entries }))
                } else {
                    self.add_syntax_error(&exp.location.clone(), DiagnosticKind::MalformedMetadataElement, diagnostic_arguments![]);
                    None
                }
            },
            _ => {
                self.add_syntax_error(&exp.location.clone(), DiagnosticKind::MalformedMetadataElement, diagnostic_arguments![]);
                None
            },
        }
    }

    fn exp_to_metadata_name(&mut self, exp: &Rc<ast::Expression>) -> Option<(String, Location)> {
        match &exp.kind {
            ast::ExpressionKind::Id(id) => {
                if let Some(name) = id.to_metadata_name() {
                    Some(name)
                } else {
                    self.add_syntax_error(&exp.location.clone(), DiagnosticKind::MalformedMetadataElement, diagnostic_arguments![]);
                    None
                }
            },
            _ => {
                self.add_syntax_error(&exp.location.clone(), DiagnosticKind::MalformedMetadataElement, diagnostic_arguments![]);
                None
            },
        }
    }

    fn exp_to_metadata_entry(&mut self, exp: &Rc<ast::Expression>) -> Option<ast::MetadataEntry> {
        match &exp.kind {
            ast::ExpressionKind::Id(id) => {
                if let Some(value) = id.to_metadata_name() {
                    Some(ast::MetadataEntry {
                        key: None,
                        value,
                    })
                } else {
                    self.add_syntax_error(&exp.location.clone(), DiagnosticKind::MalformedMetadataElement, diagnostic_arguments![]);
                    None
                }
            },
            ast::ExpressionKind::Assignment { left, compound, right } => {
                // Compound assignment is not allowed in meta data
                if compound.is_some() {
                    self.add_syntax_error(&exp.location.clone(), DiagnosticKind::MalformedMetadataElement, diagnostic_arguments![]);
                    return None;
                }

                let Some(key) = left.to_metadata_key() else {
                    self.add_syntax_error(&exp.location.clone(), DiagnosticKind::MalformedMetadataElement, diagnostic_arguments![]);
                    return None;
                };

                let Some(value) = right.to_metadata_value() else {
                    self.add_syntax_error(&exp.location.clone(), DiagnosticKind::MalformedMetadataElement, diagnostic_arguments![]);
                    return None;
                };

                Some(ast::MetadataEntry { key: Some(key), value })
            },
            _ => {
                self.add_syntax_error(&exp.location.clone(), DiagnosticKind::MalformedMetadataElement, diagnostic_arguments![]);
                None
            },
        }
    }

    fn peek_annotatable_definition_reserved_word(&self) -> bool {
        self.peek(Token::Class) || self.peek(Token::Interface) || self.peek(Token::Function) ||
        self.peek(Token::Var) || self.peek(Token::Const)
    }

    fn parse_use_directive(&mut self) -> Result<(Rc<ast::Directive>, bool), ParserFailure> {
        self.mark_location();
        self.next()?;
        self.expect_context_keyword("namespace")?;
        let expression = self.parse_expression(ExpressionContext {
            allow_in: true,
            min_precedence: OperatorPrecedence::List,
            ..default()
        })?;
        let semicolon_inserted = self.parse_semicolon()?;

        let node = Rc::new(ast::Directive {
            location: self.pop_location(),
            kind: ast::DirectiveKind::UseNamespace(expression),
        });

        Ok((node, semicolon_inserted))
    }

    fn parse_import_directive(&mut self) -> Result<(Rc<ast::Directive>, bool), ParserFailure> {
        self.mark_location();
        self.next()?;
        let mut alias: Option<(String, Location)> = None;
        let mut package_name: Vec<(String, Location)> = vec![];
        let mut import_item = (ast::ImportItem::Wildcard, self.token_location());
        let id1 = self.expect_identifier(false)?;
        if self.consume(Token::Assign)? {
            alias = Some(id1.clone());
            package_name.push(self.expect_identifier(false)?);
        } else {
            package_name.push(id1);
        }

        if !self.peek(Token::Dot) {
            self.expect(Token::Dot)?;
        }

        while self.consume(Token::Dot)? {
            if self.peek(Token::Times) {
                import_item = (ast::ImportItem::Wildcard, self.token_location());
                self.next()?;
                break;
            } else if self.peek(Token::Power) {
                import_item = (ast::ImportItem::Recursive, self.token_location());
                self.next()?;
                break;
            } else {
                let id1 = self.expect_identifier(true)?;
                if !self.peek(Token::Dot) {
                    import_item = (ast::ImportItem::Name(id1.0, id1.1.clone()), id1.1.clone());
                    break;
                } else {
                    package_name.push(id1.clone());
                }
            }
        }

        let semicolon_inserted = self.parse_semicolon()?;

        let node = Rc::new(ast::Directive {
            location: self.pop_location(),
            kind: ast::DirectiveKind::Import(Rc::new(ast::ImportDirective {
                alias,
                package_name,
                import_item,
            })),
        });

        Ok((node, semicolon_inserted))
    }

    fn parse_export_directive(&mut self) -> Result<(Rc<ast::Directive>, bool), ParserFailure> {
        self.mark_location();
        self.next()?;
        let mut alias: Option<(String, Location)> = None;
        let mut package_name: Vec<(String, Location)> = vec![];
        let mut export_item = (ast::ExportItem::Wildcard, self.token_location());
        let id1 = self.expect_identifier(false)?;
        if self.consume(Token::Assign)? {
            alias = Some(id1.clone());
            package_name.push(self.expect_identifier(false)?);
        } else {
            package_name.push(id1);
        }

        if !self.peek(Token::Dot) {
            self.expect(Token::Dot)?;
        }

        while self.consume(Token::Dot)? {
            if self.peek(Token::Times) {
                export_item = (ast::ExportItem::Wildcard, self.token_location());
                self.next()?;
                break;
            } else {
                let id1 = self.expect_identifier(true)?;
                if !self.peek(Token::Dot) {
                    export_item = (ast::ExportItem::Name(id1.0, id1.1.clone()), id1.1.clone());
                    break;
                } else {
                    package_name.push(id1.clone());
                }
            }
        }

        let semicolon_inserted = self.parse_semicolon()?;

        let node = Rc::new(ast::Directive {
            location: self.pop_location(),
            kind: ast::DirectiveKind::Export(Rc::new(ast::ExportDirective {
                alias,
                package_name,
                export_item,
            })),
        });

        Ok((node, semicolon_inserted))
    }

    fn parse_include_directive(&mut self, context: DirectiveContext, start: Location) -> Result<(Rc<ast::Directive>, bool), ParserFailure> {
        self.push_location(&start);
        let source_path_location = self.token_location();
        let Token::StringLiteral(source) = &self.token.0.clone() else {
            panic!();
        };
        let source = source.clone();
        self.next()?;
        let semicolon_inserted = self.parse_semicolon()?;

        let replaced_by_source: Rc<Source>;

        // Select origin file path
        let origin_file_path = if let Some(file_path) = self.tokenizer.source.file_path.clone() {
            Some(file_path)
        } else {
            std::env::current_dir().ok().map(|d| d.to_string_lossy().into_owned())
        };

        // Resolve source
        if let Some(origin_file_path) = origin_file_path {
            let sub_file_path = file_paths::FlexPath::from_n_native([origin_file_path.as_ref(), source.as_ref()]).to_string_with_flex_separator();
            if let Ok(content) = std::fs::read_to_string(&sub_file_path) {
                replaced_by_source = Source::new(Some(sub_file_path.clone()), content, &self.tokenizer.source.compiler_options);
            } else {
                self.add_syntax_error(&source_path_location.clone(), DiagnosticKind::FailedToIncludeFile, vec![]);

                // Use a placeholder source
                replaced_by_source = Source::new(None, "".into(), &self.tokenizer.source.compiler_options);
            }
        } else {
            self.add_syntax_error(&source_path_location.clone(), DiagnosticKind::ParentSourceIsNotAFile, vec![]);

            // Use a placeholder source
            replaced_by_source = Source::new(None, "".into(), &self.tokenizer.source.compiler_options);
        }

        // Add subsource to super source
        self.tokenizer.source.subsources.borrow_mut().push(replaced_by_source.clone());

        // Parse directives from replacement source
        let replaced_by = parse_include_directive_source(replaced_by_source.clone(), context);

        // Delegate subsource errors to super source
        if replaced_by_source.invalidated() {
            self.tokenizer.source.invalidated.set(true);
        }

        let node = Rc::new(ast::Directive {
            location: self.pop_location(),
            kind: ast::DirectiveKind::Include(Rc::new(ast::IncludeDirective {
                source,
                replaced_by,
                replaced_by_source: replaced_by_source.clone(),
            })),
        });

        Ok((node, semicolon_inserted))
    }

    fn parse_directives(&mut self, context: DirectiveContext) -> Result<Vec<Rc<ast::Directive>>, ParserFailure> {
        let mut directives = vec![];
        let mut semicolon_inserted = false;
        while !self.peek(Token::Eof) {
            if !directives.is_empty() && !semicolon_inserted {
                self.expect(Token::Semicolon)?;
            }
            let (directive, semicolon_inserted_1) = self.parse_directive(context.clone())?;
            directives.push(directive);
            semicolon_inserted = semicolon_inserted_1;
        }
        Ok(directives)
    }

    fn parse_asdoc(&mut self) -> Result<Option<Rc<ast::AsDoc>>, ParserFailure> {
        let comments = self.source().comments.borrow();
        let last_comment = comments.last().map(|last_comment| last_comment.clone());
        drop(comments);
        Ok(last_comment.and_then(|comment| {
            if comment.is_asdoc(&self.token.1) {
                self.source().comments_mut().pop();
                let content = &comment.content.borrow()[1..];
                let lines: Vec<&str> = regex!(r"\n|\r\n?").split(content).collect();
                let lines: Vec<String> = lines.iter().map(|line| regex_replace!(r"^\s*(\*\s?)?", line, |_, _| "".to_owned()).into_owned()).collect();
                Some(self.parse_asdoc_lines(comment.location(), lines))
            } else {
                None
            }
        }))
    }

    fn parse_asdoc_lines(&self, comment_location: Location, lines: Vec<String>) -> Rc<ast::AsDoc> {
        let mut main_body = String::new();
        let mut tags: Vec<ast::AsDocTag> = vec![];
        let mut i = 0;
        let line_count = lines.len();

        let mut building_content_tag_name: Option<String> = None;
        let mut building_content: Vec<String> = vec![];
        let mut inside_code_block = false;

        while i < line_count {
            let line = &lines[i];
            let tag = if inside_code_block { None } else {
                regex_captures!(r"^[\s\t]*\@([^\s\t]+)(.*)", &line)
            };
            if let Some((_, tag_name, tag_content)) = tag {
                self.parse_asdoc_tag_or_main_content(
                    comment_location.clone(),
                    &mut building_content_tag_name,
                    &mut building_content,
                    &mut main_body,
                    &mut tags,
                );
                if regex_is_match!(r"^[\s\t]*```([^`]|$)", &tag_content) {
                    inside_code_block = true;
                }
                building_content_tag_name = Some(tag_name.into());
                building_content.push(tag_content.into());
            } else {
                if regex_is_match!(r"^[\s\t]*```([^`]|$)", &line) {
                    inside_code_block = !inside_code_block;
                }
                building_content.push(line.to_owned());
            }
            i += 1;
        }

        self.parse_asdoc_tag_or_main_content(
            comment_location.clone(),
            &mut building_content_tag_name,
            &mut building_content,
            &mut main_body,
            &mut tags,
        );

        Rc::new(ast::AsDoc { main_body, tags })
    }

    fn parse_asdoc_tag_or_main_content(
        &self,
        comment_location: Location,
        building_content_tag_name: &mut Option<String>,
        building_content: &mut Vec<String>,
        main_body: &mut String,
        tags: &mut Vec<ast::AsDocTag>
    ) {
        if let Some(tag_name) = building_content_tag_name.as_ref() {
            match tag_name.as_ref() {
                // @copy reference
                "copy" => {
                    let reference = building_content.join("\n").trim().to_owned();
                    tags.push(ast::AsDocTag::Copy(reference));
                },

                // @default value
                "default" => {
                    let reference = building_content.join("\n").trim().to_owned();
                    tags.push(ast::AsDocTag::Default(reference));
                },

                // @eventType typeOrConstant
                "eventType" => {
                    let type_or_constant = building_content.join("\n").trim().to_owned();
                    let source = Source::new(None, type_or_constant, &self.tokenizer.source.compiler_options);
                    if let Some(exp) = parser_facade::parse_expression(&source) {
                        tags.push(ast::AsDocTag::EventType(exp));
                    } else {
                        self.add_syntax_error(&comment_location.clone(), DiagnosticKind::FailedParsingAsDocTag, diagnostic_arguments![String(tag_name.clone())]);
                    }
                },

                // @example text
                "example" => {
                    let text = building_content.join("\n").trim().into();
                    tags.push(ast::AsDocTag::Example(text));
                },

                // @exampleText text
                "exampleText" => {
                    let text = building_content.join("\n").trim().into();
                    tags.push(ast::AsDocTag::ExampleText(text));
                },

                // @inheritDoc
                "inheritDoc" => {
                    let text = building_content.join("\n");

                    // Content must be empty
                    if !regex_is_match!(r"^\s*$", &text) {
                        self.add_syntax_error(&comment_location.clone(), DiagnosticKind::FailedParsingAsDocTag, diagnostic_arguments![String(tag_name.clone())]);
                    }

                    tags.push(ast::AsDocTag::InheritDoc);
                },

                // @internal text
                "internal" => {
                    let text = building_content.join("\n").trim().to_owned();

                    // Content must be non empty
                    if regex_is_match!(r"^\s*$", &text) {
                        self.add_syntax_error(&comment_location.clone(), DiagnosticKind::FailedParsingAsDocTag, diagnostic_arguments![String(tag_name.clone())]);
                    }

                    tags.push(ast::AsDocTag::Internal(text));
                },

                // @param paramName description
                "param" => {
                    let content = building_content.join("\n").trim().to_owned();
                    if let Some((_, name, description)) = regex_captures!(r"(?x) ([^\s]+) (.*)", &content) {
                        tags.push(ast::AsDocTag::Param { name: name.into(), description: description.trim_start().into() });
                    } else {
                        tags.push(ast::AsDocTag::Param { name: content, description: "".into() });
                    }
                },

                // @private
                "private" => {
                    let text = building_content.join("\n");

                    // Content must be empty
                    if !regex_is_match!(r"^\s*$", &text) {
                        self.add_syntax_error(&comment_location.clone(), DiagnosticKind::FailedParsingAsDocTag, diagnostic_arguments![String(tag_name.clone())]);
                    }

                    tags.push(ast::AsDocTag::Private);
                },

                // @return text
                "return" => {
                    let text = building_content.join("\n").trim().into();
                    tags.push(ast::AsDocTag::Return(text));
                },

                // @see reference [displayText]
                "see" => {
                    let content = building_content.join("\n").trim().to_owned();
                    if let Some((_, reference, display_text)) = regex_captures!(r"(?x) ([^\s]+) (.*)", &content) {
                        tags.push(ast::AsDocTag::See { reference: reference.to_owned(), display_text: Some(display_text.to_owned()) });
                    } else {
                        tags.push(ast::AsDocTag::See { reference: content, display_text: None });
                    }
                },

                // @throws className description
                "throws" => {
                    let class_name_and_description = building_content.join("\n").trim().to_owned();
                    let class_name_and_description = regex_captures!(r"^([^\s]+)(\s.*)?", &class_name_and_description);
                    if let Some((_, class_name, description)) = class_name_and_description {
                        let description = description.trim().to_owned();
                        let description = if description.is_empty() {
                            None
                        } else {
                            Some(description)
                        };
                        let source = Source::new(None, class_name.into(), &self.tokenizer.source.compiler_options);
                        if let Some(exp) = parser_facade::parse_type_expression(&source) {
                            tags.push(ast::AsDocTag::Throws { class_reference: exp, description });
                        } else {
                            self.add_syntax_error(&comment_location.clone(), DiagnosticKind::FailedParsingAsDocTag, diagnostic_arguments![String(tag_name.clone())]);
                        }
                    } else {
                        self.add_syntax_error(&comment_location.clone(), DiagnosticKind::FailedParsingAsDocTag, diagnostic_arguments![String(tag_name.clone())]);
                    }
                },

                // Unrecognized tag
                _ => {
                    self.add_syntax_error(&comment_location.clone(), DiagnosticKind::UnrecognizedAsDocTag, diagnostic_arguments![String(tag_name.clone())]);
                },
            }
        } else {
            *main_body = building_content.join("\n").trim().into();
        }

        *building_content_tag_name = None;
        building_content.clear();
    }

    pub fn parse_program(&mut self) -> Result<Rc<ast::Program>, ParserFailure> {
        self.mark_location();
        let mut packages = vec![];
        while self.peek(Token::Package) {
            self.mark_location();
            let asdoc = self.parse_asdoc()?;
            self.next()?;
            let mut id = vec![];
            if let Some(id1) = self.consume_identifier(false)? {
                id.push(id1.clone());
                while self.consume(Token::Dot)? {
                    id.push(self.expect_identifier(true)?);
                }
            }
            let block = self.parse_block(DirectiveContext::PackageBlock)?;
            packages.push(Rc::new(ast::PackageDefinition {
                location: self.pop_location(),
                asdoc,
                id,
                block,
            }));
        }
        let directives = self.parse_directives(DirectiveContext::TopLevel)?;
        Ok(Rc::new(ast::Program {
            location: self.pop_location(),
            packages,
            directives,
        }))
    }
}

fn parse_include_directive_source(replaced_by_source: Rc<Source>, context: DirectiveContext) -> Vec<Rc<ast::Directive>> {
    let mut parser = Parser::new(&replaced_by_source);
    if parser.next().is_ok() {
        parser.parse_directives(context).unwrap_or(vec![])
    } else {
        vec![]
    }
}

fn is_annotatable_definition_context_keyword(id: &(String, Location)) -> bool {
    ["namespace", "type", "enum"].contains(&id.0.as_ref()) && id.1.character_count() == id.0.len()
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

#[derive(Clone)]
struct ArrowFunctionContext {
    left: Option<Rc<ast::Expression>>,
    right_context: ExpressionContext,
}

impl Default for ArrowFunctionContext {
    fn default() -> Self {
        Self {
            left: None,
            right_context: default(),
        }
    }
}

#[derive(Clone)]
struct Activation {
    uses_yield: bool,
    uses_await: bool,
}

impl Activation {
    pub fn new() -> Self {
        Self {
            uses_yield: false,
            uses_await: false,
        }
    }
}

#[derive(Clone)]
pub enum DirectiveContext {
    Default,
    TopLevel,
    PackageBlock,
    ClassBlock {
        name: String,
    },
    InterfaceBlock,
    EnumBlock,
    ConstructorBlock {
        super_statement_found: Cell<bool>,
    },
    WithControl {
        to_be_labeled: Option<String>,
        control_context: ControlContext,
        labels: HashMap<String, ControlContext>,
    },
}

impl DirectiveContext {
    fn is_type_block(&self) -> bool {
        match self {
            Self::ClassBlock { .. } |
            Self::InterfaceBlock |
            Self::EnumBlock => true,
            _ => false,
        }
    }

    fn clone_control(&self) -> Self {
        match self {
            Self::WithControl { .. } => self.clone(),
            _ => Self::Default,
        }
    }

    fn override_control_context(&self, label_only: bool, mut context: ControlContext) -> Self {
        let mut prev_context = None;
        let mut label = None;
        let mut labels = match self {
            Self::WithControl { control_context, labels, to_be_labeled: label1 } => {
                prev_context = Some(control_context.clone());
                label = label1.clone();
                labels.clone()
            },
            _ => HashMap::new(),
        };
        if let Some(label) = label.clone() {
            labels.insert(label, context.clone());
        }
        if label_only {
            context = prev_context.unwrap_or(ControlContext {
                breakable: false,
                iteration: false,
            });
        }
        Self::WithControl { control_context: context, labels, to_be_labeled: None }
    }

    fn put_label(&self, label: String) -> Self {
        match self {
            Self::WithControl { control_context, labels, to_be_labeled: _ } => Self::WithControl {
                to_be_labeled: Some(label),
                control_context: control_context.clone(),
                labels: labels.clone(),
            },
            _ => Self::WithControl {
                to_be_labeled: Some(label),
                control_context: ControlContext {
                    breakable: false,
                    iteration: false,
                },
                labels: HashMap::new(),
            },
        }
    }

    fn is_label_defined(&self, label: String) -> bool {
        self.resolve_label(label).is_some()
    }

    fn resolve_label(&self, label: String) -> Option<ControlContext> {
        if let Self::WithControl { labels, .. } = &self { labels.get(&label).map(|c| c.clone()) } else { None }
    }

    fn is_break_allowed(&self, label: Option<String>) -> bool {
        if let Some(label) = label {
            let context = self.resolve_label(label);
            if let Some(context) = context { context.breakable } else { false }
        } else {
            if let Self::WithControl { control_context, .. } = &self { control_context.breakable } else { false }
        }
    }

    fn is_continue_allowed(&self, label: Option<String>) -> bool {
        if let Some(label) = label {
            let context = self.resolve_label(label);
            if let Some(context) = context { context.iteration } else { false }
        } else {
            if let Self::WithControl { control_context, .. } = &self { control_context.iteration } else { false }
        }
    }
}

#[derive(Clone)]
pub struct ControlContext {
    breakable: bool,
    iteration: bool,
}

#[derive(Clone)]
struct AnnotatableContext {
    start: Location,
    metadata_exp: Vec<Rc<ast::Expression>>,
    asdoc: Option<Rc<ast::AsDoc>>,
    first_modifier: Option<Rc<ast::Expression>>,
    previous_token_is_definition_keyword: bool,
    context: DirectiveContext,
}

/// Parser facade.
pub mod parser_facade {
    use crate::*;
    use crate::util::default;
    use std::rc::Rc;

    /// Parses `Program` until end-of-file.
    pub fn parse_program(source: &Rc<Source>) -> Option<Rc<ast::Program>> {
        let mut parser = Parser::new(source);
        if parser.next().is_ok() {
            let program = parser.parse_program().ok();
            if source.invalidated() { None } else { program }
        } else {
            None
        }
    }

    /// Parses `ListExpression^allowIn` and expects end-of-file.
    pub fn parse_expression(source: &Rc<Source>) -> Option<Rc<ast::Expression>> {
        let mut parser = Parser::new(source);
        if parser.next().is_ok() {
            let exp = parser.parse_expression(ExpressionContext {
                ..default()
            }).ok();
            if exp.is_some() {
                let _ = parser.expect_eof();
            }
            if source.invalidated() { None } else { exp }
        } else {
            None
        }
    }

    /// Parses `TypeExpression` and expects end-of-file.
    pub fn parse_type_expression(source: &Rc<Source>) -> Option<Rc<ast::TypeExpression>> {
        let mut parser = Parser::new(source);
        if parser.next().is_ok() {
            let exp = parser.parse_type_expression().ok();
            if exp.is_some() {
                let _ = parser.expect_eof();
            }
            if source.invalidated() { None } else { exp }
        } else {
            None
        }
    }

    /// Parses `Directives` until end-of-file.
    pub fn parse_directives(source: &Rc<Source>, context: DirectiveContext) -> Option<Vec<Rc<ast::Directive>>> {
        let mut parser = Parser::new(source);
        if parser.next().is_ok() {
            let directives = parser.parse_directives(context).ok();
            if source.invalidated() { None } else { directives }
        } else {
            None
        }
    }
}