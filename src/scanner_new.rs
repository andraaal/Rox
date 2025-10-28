use crate::scanner::{Token, TokenType};

#[derive(Debug)]
pub struct Scanner<'a> {
    source: &'a [u8],
    current: usize,
    line: usize,
}

#[derive(Debug, thiserror::Error)]
#[error("{reason} in line {line}")]
pub struct ScannerError {
    reason: String,
    line: usize,
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Result<Token<'a>, ScannerError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_at_end() {
            return None;
        }

        if let Err(error_message) = self.skip_whitespace() {
            return Some(Err(ScannerError {
                reason: error_message,
                line: self.line,
            }));
        }
        let ch = self.source[self.current];
        self.current += 1;

        let tk = match ch {
            b'(' => self.make_token(TokenType::LeftParenthesis),
            b')' => self.make_token(TokenType::RightParenthesis),
            b'{' => self.make_token(TokenType::LeftBrace),
            b'}' => self.make_token(TokenType::RightBrace),
            b',' => self.make_token(TokenType::Comma),
            b'.' => self.make_token(TokenType::Dot),
            b'-' => self.make_token(TokenType::Minus),
            b'+' => self.make_token(TokenType::Plus),
            b';' => self.make_token(TokenType::Semicolon),
            b'*' => self.make_token(TokenType::Star),
            b'/' => self.make_token(TokenType::Slash),
            b'<' => {
                if self.match_byte(b'=') {
                    self.make_token(TokenType::LessEqual)
                } else {
                    self.make_token(TokenType::Less)
                }
            }
            b'>' => {
                if self.match_byte(b'=') {
                    self.make_token(TokenType::GreaterEqual)
                } else {
                    self.make_token(TokenType::Greater)
                }
            }
            b'!' => {
                if self.match_byte(b'=') {
                    self.make_token(TokenType::BangEqual)
                } else {
                    self.make_token(TokenType::Bang)
                }
            }
            b'=' => {
                if self.match_byte(b'=') {
                    self.make_token(TokenType::EqualEqual)
                } else {
                    self.make_token(TokenType::Equal)
                }
            }
            b'"' => {
                let mut end = self.current;
                while let Some(&c) = self.source.get(end) {
                    match c {
                        b'"' => {
                            break;
                        }
                        b'\n' => {
                            self.line += 1;
                        }
                        _ => {}
                    }
                    end += 1;
                }
                if !self.is_at_end() {
                    let begin = self.current;
                    self.current = end;
                    self.make_token(TokenType::StringLiteral(&self.source[begin - 1..end]))
                } else {
                    return Some(Err(ScannerError {
                        reason: "Unterminated string literal".to_string(),
                        line: self.line,
                    }));
                }
            }
            b'0'..=b'9' => {
                let mut level: u8 = 0;
                let mut end = self.current;
                while let Some(&c) = self.source.get(end) {
                    if c == b'.' && level == 0 {
                        end += 1;
                        level = 1;
                        continue;
                    }
                    if c == b'e' || c == b'E' && level < 2 {
                        if let Some(&next) = self.source.get(end + 1) {
                            if next == b'+' || next == b'-' {
                                end += 2;
                                level = 2;
                                continue;
                            } else {
                                break;
                            }
                        }
                    }
                    if c < b'0' || c > b'9' {
                        break;
                    }
                }

                let num = String::from_utf8(self.source[self.current - 1..end].to_vec());
                self.current = end;

                if let Err(error_message) = num {
                    return Some(Err(ScannerError {
                        reason: error_message.to_string(),
                        line: self.line,
                    }));
                } else {
                    let parsed = num.unwrap().parse::<f64>();
                    if let Err(error_message) = parsed {
                        return Some(Err(ScannerError {
                            reason: error_message.to_string(),
                            line: self.line,
                        }));
                    } else {
                        self.make_token(TokenType::NumberLiteral(parsed.unwrap()))
                    }
                }
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let token = match ch {
                    b'a' => self.match_keyword(b"nd", TokenType::And),
                    b'c' => self.match_keyword(b"lass", TokenType::Class),
                    b'e' => self.match_keyword(b"lse", TokenType::Else),

                    b'f' => {
                        if let Some(&next) = self.source.get(self.current) {
                            match next {
                                b'o' => self.match_keyword(b"or", TokenType::For),
                                b'a' => self.match_keyword(b"alse", TokenType::False),
                                b'u' => self.match_keyword(b"un", TokenType::Fun),
                                _ => self.match_identifier(self.current),
                            }
                        } else {
                            self.match_identifier(self.current)
                        }
                    }

                    b'i' => self.match_keyword(b"f", TokenType::If),
                    b'n' => self.match_keyword(b"il", TokenType::Nil),
                    b'o' => self.match_keyword(b"r", TokenType::Or),
                    b'p' => self.match_keyword(b"rint", TokenType::Print),
                    b'r' => self.match_keyword(b"eturn", TokenType::Return),
                    b's' => self.match_keyword(b"uper", TokenType::Super),

                    b't' => {
                        if let Some(&next) = self.source.get(self.current) {
                            match next {
                                b'h' => self.match_keyword(b"is", TokenType::This),
                                b'r' => self.match_keyword(b"ue", TokenType::True),
                                _ => self.match_identifier(self.current),
                            }
                        } else {
                            self.match_identifier(self.current)
                        }
                    }

                    b'v' => self.match_keyword(b"ar", TokenType::Var),
                    b'w' => self.match_keyword(b"hile", TokenType::While),

                    // no keyword
                    _ => self.match_identifier(self.current),
                };
                token
            }
            err => {
                return Some(Err(ScannerError {
                    reason: format!("Invalid character '{err}'"),
                    line: self.line,
                }));
            }
        };

        Some(Ok(tk))
    }
}
impl<'a> Scanner<'a> {
    pub fn new(s: &'a [u8]) -> Self {
        Scanner {
            source: s,
            current: 0,
            line: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn make_token(&self, ty: TokenType<'a>) -> Token<'a> {
        Token {
            token_type: ty,
            line: self.line,
        }
    }

    fn match_byte(&mut self, ch: u8) -> bool {
        if !self.is_at_end() && self.source[self.current] == ch {
            self.current += 1;
            true
        } else {
            false
        }
    }

    fn skip_whitespace(&mut self) -> Result<(), String> {
        while let Some(&c) = self.source.get(self.current) {
            match c {
                b' ' | b'\t' | b'\r' => {
                    self.current += 1;
                }
                b'\n' => {
                    self.line += 1;
                    self.current += 1;
                }
                b'/' => {
                    if self.source.get(self.current + 1).copied() == Some(b'/') {
                        self.skip_line_comment();
                    } else if self.source.get(self.current + 1).copied() == Some(b'*') {
                        if let Err(err) = self.skip_block_comment() {
                            return Err(err);
                        }
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
        Ok(())
    }

    fn skip_line_comment(&mut self) {
        self.current += 2;
        while !self.is_at_end() && self.source.get(self.current).copied() != Some(b'\n') {
            self.current += 1;
        }
    }

    fn skip_block_comment(&mut self) -> Result<(), String> {
        self.current += 2;
        let mut level = 1;
        while level > 0 {
            match self.source.get(self.current) {
                Some(b'*') if self.source.get(self.current + 1).copied() == Some(b'/') => {
                    self.current += 2;
                    level -= 1;
                }
                Some(b'/') if self.source.get(self.current + 1).copied() == Some(b'*') => {
                    self.current += 2;
                    level += 1;
                }
                Some(b'\n') => {
                    self.line += 1;
                    self.current += 1;
                }
                Some(_) => {
                    self.current += 1;
                }
                None => return Err(format!("Unterminated block comment: {}", self.line)),
            }
        }
        Ok(())
    }

    fn match_keyword(&mut self, reference: &[u8], ttp: TokenType<'a>) -> Token<'a> {
        let mut end = self.current;
        for &c in reference {
            if Some(c) != self.source.get(end).copied() {
                return self.match_identifier(end);
            }
            end += 1;
        }
        self.current = end;
        Token {
            token_type: ttp,
            line: self.line,
        }
    }

    // Maybe we already know a few characters and don't need to scan everything thus => we can specify a custom start point
    fn match_identifier(&mut self, mut end: usize) -> Token<'a> {
        while let Some(&cha) = self.source.get(end) {
            if !cha.is_ascii_alphanumeric() && cha != b'_' {
                break;
            }
            end += 1;
        }
        let token = TokenType::StringLiteral(&self.source[self.current - 1..end]);
        self.current = end;
        Token {
            token_type: token,
            line: self.line,
        }
    }
}
