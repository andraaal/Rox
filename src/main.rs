use crate::compiler::Parser;
use crate::interpreter::Interpreter;

mod chunk;
mod compiler;
mod debug;
mod scanner;
mod token;
mod value;
mod vm;
mod object;
mod expr;
mod interpreter;

fn main() {
    let mut args = std::env::args();
    args.next();
    if let Some(filename) = args.next() {
        if args.next().is_some() {
            println!("Usage: rox <filename>");
            return;
        }
        let file_content = std::fs::read_to_string(filename).expect("Couldn't read file");
        let mut parser = Parser::new(&file_content);
        parser.compile();
        let mut interpreter = Interpreter{};
        interpreter.interpret(parser.tree);
    } else {
        println!("Rox v0.1");
        // REPL
    }
}
