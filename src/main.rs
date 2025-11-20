use crate::compiler::Parser;

mod chunk;
mod compiler;
mod debug;
mod scanner;
mod scanner_new;
mod vm;

fn main() {
    let mut args = std::env::args();
    args.next();
    if let Some(filename) = args.next() {
        if args.next().is_some() {
            println!("Usage: rox <filename>");
            return;
        }
        let file_content = std::fs::read_to_string(filename).expect("Couldn't read file");
        let parser = Parser::new(&file_content);
        parser.compile();
    } else {
        println!("Rox v0.1");
        loop {
            // println!("> ");
            // REPL
        }
    }
}
