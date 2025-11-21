use crate::chunk::{Chunk, OpCode};

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

    let Ok(op) = OpCode::try_from(val) else {
        panic!("Unknown opcode {}", val);
    };

    match op {
        OpCode::Return => simple_instruction("RETURN", offset),
        OpCode::Constant => constant_instruction("CONSTANT", chunk, offset),
        OpCode::Negate => simple_instruction("NEGATE", offset),
        OpCode::Add => simple_instruction("ADD", offset),
        OpCode::Subtract => simple_instruction("SUBTRACT", offset),
        OpCode::Multiply => simple_instruction("MULTIPLY", offset),
        OpCode::Divide => simple_instruction("DIVIDE", offset),
        OpCode::Modulo => simple_instruction("MODULO", offset),
        OpCode::Greater => simple_instruction("GREATER", offset),
        OpCode::GreaterEqual => simple_instruction("GREATER_EQUAL", offset),
        OpCode::Less => simple_instruction("LESS", offset),
        OpCode::LessEqual => simple_instruction("LESS_EQUAL", offset),
        OpCode::Equal => simple_instruction("EQUAL", offset),
        OpCode::NotEqual => simple_instruction("NOT_EQUAL", offset),
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
