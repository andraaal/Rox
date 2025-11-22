use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct Scanner<'a> {
    source: std::iter::Peekable<std::str::Chars<'a>>,
    pub tokens: Vec<Token>,
    start: Location,
    cur: Location,
}


#[derive(Debug, Copy, Clone)]
pub struct Location {
    pub line: usize,
    pub col: usize,
    pub index: usize,
}

impl<'a> Scanner<'a> {
    pub(crate) fn new(source: &'a str) -> Self {
        Scanner {
            source: source.chars().peekable(),
            tokens: Vec::new(),
            start: Location {
                line: 1,
                col: 1,
                index: 0,
            },
            cur: Location {
                line: 1,
                col: 1,
                index: 0,
            },
        }
    }
    
    fn next(&mut self) -> Option<char> {
        let c = self.source.next();
        match c {
            Some('\n') => {
                self.cur.line += 1;
                self.cur.col = 1;
                self.cur.index += 1;
            }
            Some(_) => {
                self.cur.col += 1;
                self.cur.index += 1;
            }
            None => {}
        }
        c
    }

    fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }

    fn emit(&mut self, tkt: TokenType) {
        let token = Token {
            token_type: tkt,
            start: self.start,
            end: self.cur,
        };
        self.tokens.push(token);
    }

    pub fn lex(&mut self) {
        while let Some(c) = self.next() {
            match c {
                '%' => self.emit(TokenType::Modulo),
                '(' => self.emit(TokenType::LeftParenthesis),
                ')' => self.emit(TokenType::RightParenthesis),
                '{' => self.emit(TokenType::LeftBrace),
                '}' => self.emit(TokenType::RightBrace),
                '[' => self.emit(TokenType::LeftBracket),
                ']' => self.emit(TokenType::RightBracket),
                '.' => self.emit(TokenType::Dot),
                ';' => self.emit(TokenType::Semicolon),
                ',' => self.emit(TokenType::Comma),
                '+' => self.emit(TokenType::Plus),
                '-' => self.emit(TokenType::Minus),
                '*' => self.emit(TokenType::Star),
                '/' => self.emit(TokenType::Slash),
                '<' => {
                    if self.peek() == Some(&'=') {
                        self.next();
                        self.emit(TokenType::LessEqual);
                    } else {
                        self.emit(TokenType::Less);
                    }
                }
                '>' => {
                    if self.peek() == Some(&'=') {
                        self.next();
                        self.emit(TokenType::GreaterEqual);
                    } else {
                        self.emit(TokenType::Greater);
                    }
                }
                '!' => {
                    if self.peek() == Some(&'=') {
                        self.next();
                        self.emit(TokenType::BangEqual);
                    } else {
                        self.emit(TokenType::Bang);
                    }
                }
                '=' => {
                    if self.peek() == Some(&'=') {
                        self.next();
                        self.emit(TokenType::EqualEqual);
                    } else {
                        self.emit(TokenType::Equal);
                    }
                }
                '"' => {
                    let mut accumulator = String::new();
                    while self.peek() != Some(&'"') {
                        match self.next() {
                            None => {
                                self.emit(TokenType::Invalid(format!(
                                    "[lexer] unterminated string literal at eof"
                                )));
                            }
                            Some(cc) => accumulator.push(cc),
                        }
                    }
                    self.next();
                    self.emit(TokenType::StringLiteral(Box::new(accumulator)));
                }
                c => {
                    if c.is_digit(10) {
                        let mut accumulator = c.to_string();
                        while let Some(cc) = self.peek().filter(|c| c.is_digit(10)) {
                            accumulator.push(*cc);
                            self.next();
                        }
                        if self.peek() == Some(&'.') {
                            accumulator.push('.');
                            while let Some(cc) = self.peek().filter(|c| c.is_digit(10)) {
                                accumulator.push(*cc);
                                self.next();
                            }
                        }
                        match accumulator.parse::<f64>() {
                            Ok(f) => self.emit(TokenType::NumberLiteral(f)),
                            Err(e) => self.emit(TokenType::Invalid(format!(
                                "[lexer] invalid number format: {}",
                                e
                            ))),
                        }
                    } else if c.is_alphabetic() || c == '_' {
                        let mut accumulator = c.to_string();
                        while let Some(cc) = self
                            .peek()
                            .filter(|c| c.is_alphabetic() || c == &&'_' || c.is_digit(10))
                        {
                            accumulator.push(*cc);
                            self.next();
                        }
                        match accumulator.as_str() {
                            "and" => self.emit(TokenType::And),
                            "class" => self.emit(TokenType::Class),
                            "else" => self.emit(TokenType::Else),
                            "false" => self.emit(TokenType::False),
                            "for" => self.emit(TokenType::For),
                            "fun" => self.emit(TokenType::Fun),
                            "if" => self.emit(TokenType::If),
                            "nil" => self.emit(TokenType::Nil),
                            "or" => self.emit(TokenType::Or),
                            "print" => self.emit(TokenType::Print),
                            "return" => self.emit(TokenType::Return),
                            "super" => self.emit(TokenType::Super),
                            "this" => self.emit(TokenType::This),
                            "true" => self.emit(TokenType::True),
                            "var" => self.emit(TokenType::Var),
                            "while" => self.emit(TokenType::While),
                            _ => self.emit(TokenType::Identifier(accumulator)),
                        }
                    } else if c == '#' {
                        let mut accumulator = String::new();
                        if self.peek() == Some(&'(') {
                            let mut depth = 1;
                            loop {
                                match self.next() {
                                    None => {
                                        self.emit(TokenType::Invalid(format!(
                                            "[lexer] unterminated block comment at eof"
                                        )));
                                        return;
                                    }
                                    Some(c) => {
                                        accumulator.push(c);
                                        if c == '(' {
                                            depth += 1;
                                        } else if c == ')' {
                                            depth -= 1;
                                            if depth == 0 {
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            loop {
                                match self.next() {
                                    None | Some('\n') => break,
                                    Some(cc) => accumulator.push(cc),
                                }
                            }
                        }
                        // Maybe I will do something with the comments in the future,
                        // for now we will just let them go...
                    } else {
                        self.emit(TokenType::Invalid(format!(
                            "[lexer] unrecognized char: {}",
                            c
                        )));
                    }
                }
            }
        }
    }
}
