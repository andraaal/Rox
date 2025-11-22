use crate::chunk::Chunk;
use crate::chunk::OpCode::*;
use crate::debug::print_chunk;
use crate::scanner::Scanner;
use crate::token::{Token, TokenType};
use crate::value::Value;
use crate::vm::Vm;
use std::borrow::Cow;

pub struct Parser<'a> {
    had_error: bool,
    panicking: bool,
    scanner: Scanner<'a>,
    chunk: Chunk,
    lookahead: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(s: &'a str) -> Self {
        Parser {
            had_error: false,
            panicking: false,
            scanner: Scanner::new(s),
            chunk: Chunk::new(s.len() / 30, s.len() / 50),
            lookahead: None,
        }
    }

    pub fn compile(mut self) -> bool {
        self.expression();
        self.emit_byte(Return as u8, 1);
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
    // 53550127
    fn constant(&mut self, token: Token) {
        match token.token_type {
            TokenType::True => self.emit_constant(true.into(), token),
            TokenType::False => self.emit_constant(false.into(), token),
            TokenType::Nil => self.emit_constant(Value::create_nil(), token),
            _ => panic!("This is not a constant: {}", token),
        }
    }

    fn number(&mut self, number_token: Token) {
        if let Some(TokenValue::NumberLiteral(Cow::Owned(num))) = number_token.value {
            self.emit_constant(num.into(), number_token);
        }
    }

    fn grouping(&mut self, _: Token) {
        // Maybe use that one day for error reporting
        self.expression();
        self.expect_token_type(TokenType::RightParenthesis, "Expected ')'");
    }

    fn unary(&mut self, operator: Token<'_>) {
        self.parse_precedence(Precedence::Unary);

        match operator.token_type {
            TokenType::Minus => self.emit_byte(Negate as u8, operator.line),
            TokenType::Bang => todo!(),
            _ => panic!("This is not an unary operator {:?}", operator),
        }
    }

    fn binary(&mut self, operator: Token<'_>, next_precedence: Precedence) {
        self.parse_precedence(next_precedence);

        match operator.token_type {
            TokenType::Plus => self.emit_byte(Add as u8, operator.line),
            TokenType::Minus => self.emit_byte(Subtract as u8, operator.line),
            TokenType::Star => self.emit_byte(Multiply as u8, operator.line),
            TokenType::Slash => self.emit_byte(Divide as u8, operator.line),
            TokenType::Modulo => self.emit_byte(Modulo as u8, operator.line),
            TokenType::EqualEqual => self.emit_byte(Equal as u8, operator.line),
            TokenType::BangEqual => self.emit_byte(NotEqual as u8, operator.line),
            TokenType::Greater => self.emit_byte(Greater as u8, operator.line),
            TokenType::GreaterEqual => self.emit_byte(GreaterEqual as u8, operator.line),
            TokenType::Less => self.emit_byte(Less as u8, operator.line),
            TokenType::LessEqual => self.emit_byte(LessEqual as u8, operator.line),
            _ => panic!("This is not a binary operator: {}", operator),
        }
    }

    fn peek_token(&mut self) -> &Option<Token<'a>> {
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

    fn emit_constant(&mut self, value: Value, token: Token<'_>) {
        let index = self.current_chunk().push_constant(value);
        if index > u8::MAX as usize {
            self.report_error_at(token, "Too many constants");
        } else {
            self.emit_byte(Constant as u8, token.line);
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

struct Rule {
    precedence: Precedence,
    prefix: Option<fn(&mut Parser<'_>, Token)>,
    infix: Option<fn(&mut Parser<'_>, Token)>,
}


const fn get_rule(tkt: TokenType) -> &'static Rule {
    &RULES[tkt as usize]
}

const RULES: [Rule; 41] = [
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
        precedence: Precedence::Comparison,
        prefix: None,
        infix: Some(|p, t| p.binary(t, Precedence::Term)),
    },
    Rule {
        // Equal
        precedence: Precedence::None,
        prefix: None,
        infix: None,
    },
    Rule {
        // EqualEqual
        precedence: Precedence::Comparison,
        prefix: None,
        infix: Some(|p, t| p.binary(t, Precedence::Term)),
    },
    Rule {
        // Greater
        precedence: Precedence::Comparison,
        prefix: None,
        infix: Some(|p, t| p.binary(t, Precedence::Term)),
    },
    Rule {
        // GreaterEqual
        precedence: Precedence::Comparison,
        prefix: None,
        infix: Some(|p, t| p.binary(t, Precedence::Term)),
    },
    Rule {
        // Less
        precedence: Precedence::Comparison,
        prefix: None,
        infix: Some(|p, t| p.binary(t, Precedence::Term)),
    },
    Rule {
        // LessEqual
        precedence: Precedence::Comparison,
        prefix: None,
        infix: Some(|p, t| p.binary(t, Precedence::Term)),
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
        prefix: Some(|p, t| p.constant(t)),
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
        prefix: Some(|p, t| p.constant(t)),
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
        prefix: Some(|p, t| p.constant(t)),
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
    },
];
