use std::borrow::Cow;
use crate::chunk::Chunk;
use crate::chunk::OpCode::{OpAdd, OpConstant, OpDivide, OpModulo, OpMultiply, OpNegate, OpReturn, OpSubtract};
use crate::scanner::TokenType::RightParenthesis;
use crate::scanner::{Token, TokenType, TokenValue};
use crate::scanner_new::Scanner;
use std::cmp::PartialOrd;
use crate::debug::print_chunk;
use crate::vm::Vm;

pub struct Parser<'a> {
    had_error: bool,
    panicking: bool,
    scanner: Scanner<'a>,
    chunk: Chunk,
    lookahead: Option<Token<'a>>
}

impl<'a> Parser<'a> {
    pub fn new(s: &'a str) -> Self {
        Parser {
            had_error: false,
            panicking: false,
            scanner: Scanner::new(s.as_bytes()),
            chunk: Chunk::new(s.len() / 30, s.len() / 50),
            lookahead: None,
        }
    }

    pub fn compile(mut self) -> bool {
        self.expression();
        self.emit_byte(OpReturn as u8, 1);
        if !self.had_error {
            print_chunk(&self.chunk, "Main");
            self.chunk.shrink("Main");
            let mut vm = Vm::new(self.chunk);
            vm.run();
        }
        !self.had_error
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        if let Some(token) = self.next_token() {
            let rule = get_rule(token.token_type);
            if let Some(prefix) = rule.prefix {
                prefix(self, token);
            } else {
                self.report_error_at(token, "Can't start expression with non prefix token");
                self.panicking = true;
            }
        } else {
            self.report_error_at_end("Expected expression, but found EOF.");
        }
        
        while let Some(token) = self.peek_token() {
            let next_rule = get_rule(token.token_type);
            if next_rule.precedence >= precedence {
                if let Some(infix) = next_rule.infix {
                    let tok = self.next_token().unwrap(); // This is the one we peeked at, it can't be invalid
                    infix(self, tok);
                }
            } else {
                break;
            }
        }
    }

    fn number(&mut self, number_token: Token) {
        if let Some(TokenValue::NumberLiteral(Cow::Owned(num))) = number_token.value {
            self.emit_constant(num, number_token);
        }
    }

    fn grouping(&mut self, _: Token) {
        // Maybe use that one day for error reporting
        self.expression();
        self.expect_token_type(RightParenthesis, "Expected ')'");
    }

    fn unary(&mut self, operator: Token<'_>) {
        self.parse_precedence(Precedence::Unary);

        match operator.token_type {
            TokenType::Minus => self.emit_byte(OpNegate as u8, operator.line),
            TokenType::Bang => todo!(),
            _ => panic!("This is not an unary operator {:?}", operator),
        }
    }

    fn binary(&mut self, operator: Token<'_>, next_precedence: Precedence) {
        self.parse_precedence(next_precedence);

        match operator.token_type {
            TokenType::Plus => self.emit_byte(OpAdd as u8, operator.line),
            TokenType::Minus => self.emit_byte(OpSubtract as u8, operator.line),
            TokenType::Star => self.emit_byte(OpMultiply as u8, operator.line),
            TokenType::Slash => self.emit_byte(OpDivide as u8, operator.line),
            TokenType::Percent => self.emit_byte(OpModulo as u8, operator.line),
            _ => panic!("This is not a binary operator: {}", operator),
        }
    }

    fn peek_token(&mut self) -> &Option<Token<'a>>{
        if self.lookahead.is_none() {
            self.lookahead = self.next_token();
        }
        &self.lookahead
    }

    fn next_token(&mut self) -> Option<Token<'a>> {
        if self.lookahead.is_some() {
            return self.lookahead.take();
        }
        loop {
            match self.scanner.next() {
                Some(Ok(tok)) => return Some(tok),
                Some(Err(err)) => self.report_error(err.to_string()),
                None => return None,
            }
        }
    }

    fn expect_token_type(&mut self, typ: TokenType, msg: &str) {
        if let Some(tok) = self.next_token() {
            if tok.token_type != typ {
                self.had_error = true;
                self.report_error_at(tok, msg);
            }
        } else {
            self.had_error = true;
        }
    }

    fn report_error_at(&mut self, tk: Token<'_>, msg: &str) {
        self.report_error(format!("[line {}] at '{}': {}", tk.line, tk, msg));
    }

    fn report_error_at_end(&mut self, msg: &str) {
        self.report_error(format!("[EOF] at end: {}", msg))
    }

    fn report_error(&mut self, msg: String) {
        self.had_error = true;
        println!("{}", msg);
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        &mut self.chunk
    }

    fn emit_byte(&mut self, byte: u8, line: usize) {
        self.current_chunk().push_code(byte, line)
    }

    fn emit_constant(&mut self, value: f64, token: Token<'_>) {
        let index = self.current_chunk().push_constant(value);
        if index > u8::MAX as usize {
            self.report_error_at(token, "Too many constants");
        } else {
            self.emit_byte(OpConstant as u8, token.line);
            self.emit_byte(index as u8, token.line);
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

#[derive(Debug)]
pub struct PrecedenceError(u8);

impl TryFrom<u8> for Precedence {
    // Not used right now, but maybe someday I will
    type Error = PrecedenceError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const VARIANTS: [Precedence; 11] = [
            Precedence::None,
            Precedence::Assignment,
            Precedence::Or,
            Precedence::And,
            Precedence::Equality,
            Precedence::Comparison,
            Precedence::Term,
            Precedence::Factor,
            Precedence::Unary,
            Precedence::Call,
            Precedence::Primary,
        ];

        VARIANTS
            .get(value as usize)
            .copied()
            .ok_or(PrecedenceError(value))
    }
}

struct Rule {
    precedence: Precedence,
    prefix: Option<fn(&mut Parser<'_>, Token<'_>)>,
    infix: Option<fn(&mut Parser<'_>, Token<'_>)>,
}

const RULES_LENGTH: usize = TokenType::_TokenCount as usize; // TODO: use std::mem::variant_count when it becomes stable

const fn get_rule(tkt: TokenType) -> &'static Rule {
    &RULES[tkt as usize]
}

const RULES: [Rule; RULES_LENGTH] = [
    Rule {
        // Left Bracket
        precedence: Precedence::None,
        prefix: None,
        infix: None,
    },
    Rule {
        // Right Bracket
        precedence: Precedence::None,
        prefix: None,
        infix: None,
    },
    Rule {
        // Left Brace
        precedence: Precedence::None,
        prefix: None,
        infix: None,
    },
    Rule {
        // Right Brace
        precedence: Precedence::None,
        prefix: None,
        infix: None,
    },
    Rule {
        // Left Parenthesis
        precedence: Precedence::None,
        prefix: Some(|p, t| p.grouping(t)),
        infix: None,
    },
    Rule {
        // Right Parenthesis
        precedence: Precedence::None,
        prefix: None,
        infix: None,
    },
    Rule {
        // Comma
        precedence: Precedence::None,
        prefix: None,
        infix: None,
    },
    Rule {
        // Dot
        precedence: Precedence::None,
        prefix: None,
        infix: None,
    },
    Rule {
        // Minus
        precedence: Precedence::Term,
        prefix: Some(|p, t| p.unary(t)),
        infix: Some(|p, t| p.binary(t, Precedence::Factor)),
    },
    Rule {
        // Plus
        precedence: Precedence::Term,
        prefix: None,
        infix: Some(|p, t| p.binary(t, Precedence::Factor)),
    },
    Rule {
        // Semicolon
        precedence: Precedence::None,
        prefix: None,
        infix: None,
    },
    Rule {
        // Slash
        precedence: Precedence::Factor,
        prefix: None,
        infix: Some(|p, t| p.binary(t, Precedence::Unary)),
    },
    Rule {
        // Star
        precedence: Precedence::Factor,
        prefix: None,
        infix: Some(|p, t| p.binary(t, Precedence::Unary)),
    },
    Rule {
        // Bang
        precedence: Precedence::Unary,
        prefix: None,
        infix: None,
    },
    Rule {
        // BangEqual
        precedence: Precedence::None,
        prefix: None,
        infix: None,
    },
    Rule {
        // Equal
        precedence: Precedence::None,
        prefix: None,
        infix: None,
    },
    Rule {
        // EqualEqual
        precedence: Precedence::None,
        prefix: None,
        infix: None,
    },
    Rule {
        // Greater
        precedence: Precedence::None,
        prefix: None,
        infix: None,
    },
    Rule {
        // GreaterEqual
        precedence: Precedence::None,
        prefix: None,
        infix: None,
    },
    Rule {
        // Less
        precedence: Precedence::None,
        prefix: None,
        infix: None,
    },
    Rule {
        // LessEqual
        precedence: Precedence::None,
        prefix: None,
        infix: None,
    },
    Rule {
        // Identifier
        precedence: Precedence::None,
        prefix: None,
        infix: None,
    },
    Rule {
        // StringLiteral
        precedence: Precedence::None,
        prefix: None,
        infix: None,
    },
    Rule {
        // NumberLiteral
        precedence: Precedence::None,
        prefix: Some(|p, t| p.number(t)),
        infix: None,
    },
    Rule {
        // And
        precedence: Precedence::None,
        prefix: None,
        infix: None,
    },
    Rule {
        // Class
        precedence: Precedence::None,
        prefix: None,
        infix: None,
    },
    Rule {
        // Else
        precedence: Precedence::None,
        prefix: None,
        infix: None,
    },
    Rule {
        // False
        precedence: Precedence::None,
        prefix: None,
        infix: None,
    },
    Rule {
        // For
        precedence: Precedence::None,
        prefix: None,
        infix: None,
    },
    Rule {
        // Fun
        precedence: Precedence::None,
        prefix: None,
        infix: None,
    },
    Rule {
        // If
        precedence: Precedence::None,
        prefix: None,
        infix: None,
    },
    Rule {
        // Nil
        precedence: Precedence::None,
        prefix: None,
        infix: None,
    },
    Rule {
        // Or
        precedence: Precedence::None,
        prefix: None,
        infix: None,
    },
    Rule {
        // Print
        precedence: Precedence::None,
        prefix: None,
        infix: None,
    },
    Rule {
        // Return
        precedence: Precedence::None,
        prefix: None,
        infix: None,
    },
    Rule {
        // Super
        precedence: Precedence::None,
        prefix: None,
        infix: None,
    },
    Rule {
        // This
        precedence: Precedence::None,
        prefix: None,
        infix: None,
    },
    Rule {
        // True
        precedence: Precedence::None,
        prefix: None,
        infix: None,
    },
    Rule {
        // Var
        precedence: Precedence::None,
        prefix: None,
        infix: None,
    },
    Rule {
        // While
        precedence: Precedence::None,
        prefix: None,
        infix: None,
    },
    Rule {
        // Percent
        precedence: Precedence::Factor,
        prefix: None,
        infix: Some(|p, t| p.binary(t, Precedence::Unary)),
    }
];
