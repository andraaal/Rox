use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use TokenType::*;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TokenType {
    LeftBracket = 0,
    RightBracket = 1,
    LeftBrace = 2,
    RightBrace = 3,
    LeftParenthesis = 4,
    RightParenthesis = 5,
    Comma = 6,
    Dot = 7,
    Minus = 8,
    Plus = 9,
    Semicolon = 10,
    Slash = 11,
    Star = 12,
    Bang = 13,
    BangEqual = 14,
    Equal = 15,
    EqualEqual = 16,
    Greater = 17,
    GreaterEqual = 18,
    Less = 19,
    LessEqual = 20,
    Identifier = 21,
    StringLiteral = 22,
    NumberLiteral = 23,
    And = 24,
    Class = 25,
    Else = 26,
    False = 27,
    For = 28,
    Fun = 29,
    If = 30,
    Nil = 31,
    Or = 32,
    Print = 33,
    Return = 34,
    Super = 35,
    This = 36,
    True = 37,
    Var = 38,
    While = 39,
    Percent = 40,
    _TokenCount = 41,
}

#[derive(Debug)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub line: usize,
    pub value: Option<TokenValue<'a>>,
}

#[derive(Debug)]
pub enum TokenValue<'a> {
    NumberLiteral(Cow<'a, f64>),
    Identifier(&'a [u8]),
    StringLiteral(&'a [u8]),
}

impl Display for TokenValue<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenValue::NumberLiteral(cow) => write!(f, "{}", cow),
            TokenValue::Identifier(iden) => write!(f, "{}", String::from_utf8_lossy(iden)),
            TokenValue::StringLiteral(lit) => write!(f, "{}", String::from_utf8_lossy(lit)),
        }
    }
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.token_type {
            LeftBracket => write!(f, "["),
            RightBracket => write!(f, "]"),
            LeftBrace => write!(f, "{{"),
            RightBrace => write!(f, "}}"),
            LeftParenthesis => write!(f, "("),
            RightParenthesis => write!(f, ")"),
            Comma => write!(f, ","),
            Dot => write!(f, "."),
            Minus => write!(f, "-"),
            Plus => write!(f, "+"),
            Semicolon => write!(f, ";"),
            Slash => write!(f, "/"),
            Star => write!(f, "*"),
            Bang => write!(f, "!"),
            BangEqual => write!(f, "!="),
            Equal => write!(f, "="),
            EqualEqual => write!(f, "=="),
            Greater => write!(f, ">"),
            GreaterEqual => write!(f, ">="),
            Less => write!(f, "<"),
            LessEqual => write!(f, "<="),
            And => write!(f, "and"),
            Class => write!(f, "class"),
            Else => write!(f, "else"),
            False => write!(f, "false"),
            For => write!(f, "for"),
            Fun => write!(f, "fun"),
            If => write!(f, "if"),
            Nil => write!(f, "nil"),
            Or => write!(f, "or"),
            Print => write!(f, "print"),
            Return => write!(f, "return"),
            Super => write!(f, "super"),
            This => write!(f, "this"),
            True => write!(f, "true"),
            Var => write!(f, "var"),
            While => write!(f, "while"),
            Percent => write!(f, "%"),
            _TokenCount => write!(f, "<! Internal Token Count !>"),
            Identifier | StringLiteral | NumberLiteral => {
                if let Some(val) = &self.value {
                    write!(f, "{}", val)
                } else {
                    unreachable!("Found literal or identifier without value -> Error in Scanner")
                }
            }
        }
    }
}
