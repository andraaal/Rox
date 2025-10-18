use itertools::Itertools;
use std::iter::Peekable;
use std::str::Chars;
use TokenType::*;

pub fn scan(src: String) {
    let mut line = 1;
    let mut tokens: Vec<Token> = Vec::with_capacity(src.len() / 3);
    let mut source = src.chars().peekable();
    loop {
        let Some(ch) = source.next() else {
            break;
        };
        let token = match ch {
            '(' => Token {
                token_type: LeftParenthesis,
                line,
            },
            ')' => Token {
                token_type: RightParenthesis,
                line,
            },
            '{' => Token {
                token_type: LeftBrace,
                line,
            },
            '}' => Token {
                token_type: RightBrace,
                line,
            },
            ';' => Token {
                token_type: Semicolon,
                line,
            },
            ',' => Token {
                token_type: Comma,
                line,
            },
            '.' => Token {
                token_type: Dot,
                line,
            },
            '-' => Token {
                token_type: Minus,
                line,
            },
            '+' => Token {
                token_type: Plus,
                line,
            },
            '*' => Token {
                token_type: Star,
                line,
            },
            '/' => {
                // Single line comment
                if match_char(&mut source, '/') {
                    let mut char = source.next();
                    while char != None && char != Some('\n') {
                        char = source.next();
                    }
                    line += 1;
                    continue;
                // Nestable block comment
                } else if match_char(&mut source, '*') {
                    let mut char = source.next();
                    let mut indent_level = 1;

                    while indent_level > 0 {
                        if char == None {
                            tokens.push(Token {
                                token_type: ErrorToken("Unclosed block comment.".to_string()),
                                line,
                            });
                        } else if char == Some('*') && source.peek() == Some(&'/') {
                            indent_level -= 1;
                        } else if char == Some('/') && source.peek() == Some(&'*') {
                            indent_level += 1;
                        }
                        char = source.next();
                    }
                    continue;
                // Division
                } else {
                    Token {
                        token_type: Slash,
                        line,
                    }
                }
            }
            '!' => {
                // Not equal
                if match_char(&mut source, '=') {
                    Token {
                        token_type: BangEqual,
                        line,
                    }
                // Not
                } else {
                    Token {
                        token_type: Bang,
                        line,
                    }
                }
            }
            '=' => {
                // Equal
                if match_char(&mut source, '=') {
                    Token {
                        token_type: EqualEqual,
                        line,
                    }
                // Assign
                } else {
                    Token {
                        token_type: Equal,
                        line,
                    }
                }
            }
            '<' => {
                // Smaller or equal
                if match_char(&mut source, '=') {
                    Token {
                        token_type: LessEqual,
                        line,
                    }
                // Strictly smaller
                } else {
                    Token {
                        token_type: Less,
                        line,
                    }
                }
            }
            '>' => {
                // Greater or equal
                if match_char(&mut source, '=') {
                    Token {
                        token_type: GreaterEqual,
                        line,
                    }
                // Strictly greater
                } else {
                    Token {
                        token_type: Greater,
                        line,
                    }
                }
            }
            // String literal
            '"' => {
                let mut literal = String::with_capacity(10);
                let mut char = source.next();
                while char != Some('"') {
                    if char == None {
                        break; //raise error here
                    }
                    literal.push(char.unwrap());
                    char = source.next();
                }
                literal.shrink_to_fit();
                Token {
                    token_type: StringLiteral(literal),
                    line,
                }
            }
            // Number literal
            '0'..='9' => {
                // TODO: trailing dots cause problems
                // FIX: Don't use iterator, iterate manually over &str in the loop
                let lit = source
                    .peeking_take_while(|ch| ch.is_ascii_digit() || ch == &'.')
                    .collect::<String>();
                Token {
                    token_type: NumberLiteral(lit.parse().unwrap()),
                    line,
                }
            }
            // Increase line count
            '\n' => {
                line += 1;
                continue;
            }
            // Skip whitespace
            ' ' | '\r' | '\t' => {
                continue;
            }
            // Keyword / Identifier
            _ => {
                if ch.is_ascii_alphabetic() || ch == '_' {
                    // start with the already-consumed char, then consume the rest
                    let mut ident = String::with_capacity(8);
                    ident.push(ch);
                    let rest = source
                        .peeking_take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                        .collect::<String>();
                    ident.push_str(&rest);

                    let token_type = match ident.as_str() {
                        "and" => And,
                        "class" => Class,
                        "else" => Else,
                        "false" => False,
                        "for" => For,
                        "if" => If,
                        "nil" => Nil,
                        "or" => Or,
                        "print" => Print,
                        "return" => Return,
                        "super" => Super,
                        "this" => This,
                        "true" => True,
                        "var" => Var,
                        "while" => While,
                        other => Identifier(other.to_string()),
                    };

                    Token {
                        token_type,
                        line,
                    }
                } else {
                    Token {
                        token_type: ErrorToken("Unknown char".to_string()),
                        line,
                    }
                }
            }
        };

        source.next();
        tokens.push(token);
    }
}

fn match_char(source: &mut Peekable<Chars>, ch: char) -> bool {
    let result = source.peek() == Some(&ch);
    if result {
        source.next();
    }
    result
}

pub enum TokenType {
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    LeftParenthesis,
    RightParenthesis,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(f64),
    And,
    Class,
    Else,
    False,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    ErrorToken(String),
}

struct Token {
    token_type: TokenType,
    line: usize,
}
