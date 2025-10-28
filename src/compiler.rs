use crate::chunk::Chunk;
use crate::scanner::Token;
use crate::scanner_new::{Scanner, ScannerError};

pub fn compile(input: String) -> Chunk {
    let sc = Scanner::new(input.as_bytes());
    loop {}
}

fn expression() {}

fn next_token<'a>(sc: &mut Scanner<'a>) -> Option<Token<'a>> {
    while let Some(next) = sc.next() {
        if let Ok(ok) = next {
            return Some(ok);
        } else {
            report_error(next.unwrap_err());
        }
    }
    None
}

fn report_error(error: ScannerError) {
    println!("{:?}", error);
}
