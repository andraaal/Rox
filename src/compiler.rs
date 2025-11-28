use crate::expr::Expr;
use crate::scanner::{Location, Scanner};
use crate::token::{Token, TokenType};
use std::cmp::PartialEq;
use std::iter::Peekable;
use std::vec::IntoIter;

pub struct Parser {
    had_error: bool,
    tokens: Peekable<IntoIter<Token>>,
    pub tree: Expr,
}

impl Parser {
    pub fn new(s: &str) -> Self {
        let mut scanner = Scanner::new(s);
        scanner.lex();
        Parser {
            had_error: false,
            tokens: scanner.tokens.into_iter().peekable(),
            tree: Expr::Null,
        }
    }

    fn report_error_at(&mut self, loc: &Location, message: &str) {
        eprintln!("[parser] {} in line {}, at {}", message, loc.line, loc.col);
        self.had_error = true;
    }

    fn report_scanner_error(&mut self, loc: &Location, message: &str) {
        eprintln!("[scanner] {} in line {}, at {}", loc.line, loc.col, message);
        self.had_error = true;
    }

    fn report_error_at_end(&mut self, message: &str) {
        eprintln!("[parser] {}", message);
        self.had_error = true;
    }

    pub fn compile(&mut self) -> bool {
        self.tree = self.expression();
        println!("{:?}", self.tree);
        !self.had_error
    }

    fn expression(&mut self) -> Expr {
        self.parse_precedence(Precedence::Assignment)
    }

    fn parse_precedence(&mut self, precedence: Precedence) -> Expr {
        let mut expr = Expr::Null;
        if let Some(token) = self.next_token() {
            if let Some(ex) = self.parse_prefix(token.token_type) {
                expr = ex;
            } else {
                self.report_error_at(
                    &token.start,
                    "Expected Constant or Infix operator, found smth else. Good luck finding it :)",
                );
            }
        } else {
            self.report_error_at_end("Expected expression, but found EOF.");
        }
        while let Some(token) =
            self.next_token_if(|tk| get_rule(&tk.token_type).infix >= precedence)
        {
            if let Some(infix) = self.parse_infix(token.token_type, expr) {
                expr = infix;
            } else {
                self.report_error_at(&token.start, "Unimplemented token");
                expr = Expr::Null;
            }
        }
        expr
    }
    // 53550127

    fn next_token_if(&mut self, func: impl Fn(&Token) -> bool) -> Option<Token> {
        self.tokens.next_if(func)
    }

    fn next_token(&mut self) -> Option<Token> {
        loop {
            match self.tokens.next() {
                Some(Token {
                    start,
                    end: _,
                    token_type: TokenType::Invalid(msg),
                }) => self.report_scanner_error(&start, msg.as_str()),
                Some(tk) => return Some(tk),
                None => return None,
            }
        }
    }

    fn expect_token_type(&mut self, typ: TokenType, msg: &str) {
        if let Some(tok) = self.next_token() {
            if tok.token_type != typ {
                self.had_error = true;
                self.report_error_at(&tok.start, msg);
            }
        } else {
            self.had_error = true;
        }
    }

    // prefix operators and constants: anything that doesn't need the expr that came before
    fn parse_prefix(&mut self, tkt: TokenType) -> Option<Expr> {
        let expr = match tkt {
            TokenType::LeftParenthesis => {
                let ex = self.expression();
                self.expect_token_type(TokenType::RightParenthesis, "Expected ')'");
                ex
            }
            TokenType::NumberLiteral(f) => Expr::Number(f),
            TokenType::StringLiteral(s) => Expr::String(s),
            TokenType::Nil => Expr::Null,
            TokenType::True => Expr::Bool(true),
            TokenType::False => Expr::Bool(false),
            TokenType::Minus => Expr::Negate(Box::new(self.parse_precedence(Precedence::Unary))),
            _ => return None,
        };

        Some(expr)
    }

    // infix, mixfix and postfix operators: They need access to the expr before
    fn parse_infix(&mut self, tkt: TokenType, lhs: Expr) -> Option<Expr> {
        let expr = match tkt {
            TokenType::Slash => {
                let rhs = self.parse_precedence(Precedence::Unary);
                Expr::Div(Box::new(lhs), Box::new(rhs))
            }
            TokenType::Plus => {
                let rhs = self.parse_precedence(Precedence::Factor);
                Expr::Add(Box::new(lhs), Box::new(rhs))
            }
            TokenType::Minus => {
                let rhs = self.parse_precedence(Precedence::Factor);
                Expr::Sub(Box::new(lhs), Box::new(rhs))
            }
            TokenType::Star => {
                let rhs = self.parse_precedence(Precedence::Unary);
                Expr::Mul(Box::new(lhs), Box::new(rhs))
            }
            TokenType::Modulo => {
                let rhs = self.parse_precedence(Precedence::Unary);
                Expr::Mod(Box::new(lhs), Box::new(rhs))
            }
            TokenType::BangEqual => {
                let rhs = self.parse_precedence(Precedence::Term);
                Expr::Neq(Box::new(lhs), Box::new(rhs))
            }
            TokenType::EqualEqual => {
                let rhs = self.parse_precedence(Precedence::Term);
                Expr::Eq(Box::new(lhs), Box::new(rhs))
            }
            TokenType::Greater => {
                let rhs = self.parse_precedence(Precedence::Term);
                Expr::Greater(Box::new(lhs), Box::new(rhs))
            }
            TokenType::GreaterEqual => {
                let rhs = self.parse_precedence(Precedence::Term);
                Expr::GreaterEqual(Box::new(lhs), Box::new(rhs))
            }
            TokenType::Less => {
                let rhs = self.parse_precedence(Precedence::Term);
                Expr::Less(Box::new(lhs), Box::new(rhs))
            }
            TokenType::LessEqual => {
                let rhs = self.parse_precedence(Precedence::Term);
                Expr::LessEqual(Box::new(lhs), Box::new(rhs))
            }
            _ => return None,
        };
        Some(expr)
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
    prefix: Precedence,
    infix: Precedence,
}

const fn get_rule(tkt: &TokenType) -> Rule {
    match tkt {
        TokenType::LeftBracket => Rule {
            prefix: Precedence::None,
            infix: Precedence::None,
        },
        TokenType::RightBracket => Rule {
            prefix: Precedence::None,
            infix: Precedence::None,
        },
        TokenType::LeftBrace => Rule {
            prefix: Precedence::None,
            infix: Precedence::None,
        },
        TokenType::RightBrace => Rule {
            prefix: Precedence::None,
            infix: Precedence::None,
        },
        TokenType::LeftParenthesis => Rule {
            prefix: Precedence::Primary,
            infix: Precedence::None,
        },
        TokenType::RightParenthesis => Rule {
            prefix: Precedence::None,
            infix: Precedence::None,
        },
        TokenType::Comma => Rule {
            prefix: Precedence::None,
            infix: Precedence::None,
        },
        TokenType::Dot => Rule {
            prefix: Precedence::None,
            infix: Precedence::None,
        },
        TokenType::Minus => Rule {
            prefix: Precedence::Unary,
            infix: Precedence::Term,
        },
        TokenType::Plus => Rule {
            prefix: Precedence::None,
            infix: Precedence::Term,
        },
        TokenType::Semicolon => Rule {
            prefix: Precedence::None,
            infix: Precedence::None,
        },
        TokenType::Slash => Rule {
            prefix: Precedence::None,
            infix: Precedence::Factor,
        },
        TokenType::Star => Rule {
            prefix: Precedence::None,
            infix: Precedence::Factor,
        },
        TokenType::Bang => Rule {
            prefix: Precedence::None,
            infix: Precedence::None,
        },
        TokenType::BangEqual => Rule {
            prefix: Precedence::None,
            infix: Precedence::Comparison,
        },
        TokenType::Equal => Rule {
            prefix: Precedence::None,
            infix: Precedence::None,
        },
        TokenType::EqualEqual => Rule {
            prefix: Precedence::None,
            infix: Precedence::Comparison,
        },
        TokenType::Greater => Rule {
            prefix: Precedence::None,
            infix: Precedence::Comparison,
        },
        TokenType::GreaterEqual => Rule {
            prefix: Precedence::None,
            infix: Precedence::Comparison,
        },
        TokenType::Less => Rule {
            prefix: Precedence::None,
            infix: Precedence::Comparison,
        },
        TokenType::LessEqual => Rule {
            prefix: Precedence::None,
            infix: Precedence::Comparison,
        },
        TokenType::Identifier(_) => Rule {
            prefix: Precedence::None,
            infix: Precedence::None,
        },
        TokenType::StringLiteral(_) => Rule {
            prefix: Precedence::None,
            infix: Precedence::None,
        },
        TokenType::NumberLiteral(_) => Rule {
            prefix: Precedence::None,
            infix: Precedence::None,
        },
        TokenType::And => Rule {
            prefix: Precedence::None,
            infix: Precedence::None,
        },
        TokenType::Class => Rule {
            prefix: Precedence::None,
            infix: Precedence::None,
        },
        TokenType::Else => Rule {
            prefix: Precedence::None,
            infix: Precedence::None,
        },
        TokenType::False => Rule {
            prefix: Precedence::None,
            infix: Precedence::None,
        },
        TokenType::For => Rule {
            prefix: Precedence::None,
            infix: Precedence::None,
        },
        TokenType::Fun => Rule {
            prefix: Precedence::None,
            infix: Precedence::None,
        },
        TokenType::If => Rule {
            prefix: Precedence::None,
            infix: Precedence::None,
        },
        TokenType::Nil => Rule {
            prefix: Precedence::None,
            infix: Precedence::None,
        },
        TokenType::Or => Rule {
            prefix: Precedence::None,
            infix: Precedence::None,
        },
        TokenType::Print => Rule {
            prefix: Precedence::None,
            infix: Precedence::None,
        },
        TokenType::Return => Rule {
            prefix: Precedence::None,
            infix: Precedence::None,
        },
        TokenType::Super => Rule {
            prefix: Precedence::None,
            infix: Precedence::None,
        },
        TokenType::This => Rule {
            prefix: Precedence::None,
            infix: Precedence::None,
        },
        TokenType::True => Rule {
            prefix: Precedence::None,
            infix: Precedence::None,
        },
        TokenType::Var => Rule {
            prefix: Precedence::None,
            infix: Precedence::None,
        },
        TokenType::While => Rule {
            prefix: Precedence::None,
            infix: Precedence::None,
        },
        TokenType::Modulo => Rule {
            prefix: Precedence::None,
            infix: Precedence::Factor,
        },
        TokenType::Invalid(_) => {
            panic!("Invalid token")
        }
    }
}
