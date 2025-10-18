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
        OpCode::OpReturn => simple_instruction("OP_RETURN", offset),
        OpCode::OpConstant => constant_instruction("OP_CONSTANT", chunk, offset),
        OpCode::OpNegate => simple_instruction("OP_NEGATE", offset),
        OpCode::OpAdd => simple_instruction("OP_ADD", offset),
        OpCode::OpSubtract => simple_instruction("OP_SUBTRACT", offset),
        OpCode::OpMultiply => simple_instruction("OP_MULTIPLY", offset),
        OpCode::OpDivide => simple_instruction("OP_DIVIDE", offset),
        OpCode::OpModulo => simple_instruction("OP_MODULO", offset),
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
