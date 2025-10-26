use crate::chunk::Chunk;
use crate::chunk::OpCode::*;
use crate::vm::Vm;

mod chunk;
mod debug;
mod vm;
mod compiler;
mod scanner;
mod scanner_new;

fn main() {
    let mut args = std::env::args();
    args.next();
    if let Some(filename) = args.next() {
        if args.next().is_some() {
            println!("Usage: {} <filename>", filename);
            return;
        }
        let file_content = std::fs::read_to_string(filename).expect("Couldn't read file");
        // interpret file here.
    } else {
        println!("Rox v0.1");
        loop {
            println!("> ");
            // REPL
        }
    }

    // Test code
    let mut chunk: Chunk = Chunk::new(5, 4);
    chunk.push_code(OpConstant as u8, 1);
    chunk.push_code(0x00, 1);
    chunk.push_code(OpConstant as u8, 1);
    chunk.push_code(0x01, 1);
    chunk.push_code(OpAdd as u8, 1);
    chunk.push_code(OpConstant as u8, 1);
    chunk.push_code(0x02, 1);
    chunk.push_code(OpDivide as u8, 1);
    chunk.push_code(OpNegate as u8, 1);
    chunk.push_code(OpReturn as u8, 1);

    chunk.push_constant(1.2);
    chunk.push_constant(3.4);
    chunk.push_constant(5.6);

    let mut vm = Vm::new(chunk);
    vm.run();
}
