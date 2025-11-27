use crate::chunk::{Chunk};
use crate::chunk::opcode::*;

pub fn print_chunk(chunk: &Chunk, name: &str) {
    println!("=== {} ===", name);

    let mut offset = 0;
    while offset < chunk.code().len() {
        offset = print_instruction(chunk, offset);
    }
}

pub fn print_instruction(chunk: &Chunk, offset: usize) -> usize {
    let val = chunk.code()[offset];
    let line = chunk.lines()[offset];

    if offset > 0 && line == chunk.lines()[offset - 1] {
        print!("{:04}    | ", offset);
    } else {
        print!("{:04} {:>4} ", offset, line);
    }


    match val {
        RETURN => simple_instruction("RETURN", offset),
        CONSTANT => constant_instruction("CONSTANT", chunk, offset),
        NEGATE => simple_instruction("NEGATE", offset),
        ADD => simple_instruction("ADD", offset),
        SUBTRACT => simple_instruction("SUBTRACT", offset),
        MULTIPLY => simple_instruction("MULTIPLY", offset),
        DIVIDE => simple_instruction("DIVIDE", offset),
        MODULO => simple_instruction("MODULO", offset),
        GREATER => simple_instruction("GREATER", offset),
        GREATER_EQUAL => simple_instruction("GREATER_EQUAL", offset),
        LESS => simple_instruction("LESS", offset),
        LESS_EQUAL => simple_instruction("LESS_EQUAL", offset),
        EQUAL => simple_instruction("EQUAL", offset),
        NOT_EQUAL => simple_instruction("NOT_EQUAL", offset),
        _ => simple_instruction("UNRECOGNIZED INSTRUCTION", offset),
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let code = chunk.code();
    let index = code[offset + 1];
    println!(
        "{:<16} {}: {}",
        name,
        index,
        chunk.constants()[index as usize]
    );
    offset + 2
}
