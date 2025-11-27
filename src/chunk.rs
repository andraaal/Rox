use crate::value::Value;

pub(crate) mod opcode {
    pub const RETURN: u8 = 0;
    pub const CONSTANT: u8 = 1;
    pub const NEGATE: u8 = 2;
    pub const ADD: u8 = 3;
    pub const SUBTRACT: u8 = 4;
    pub const MULTIPLY: u8 = 5;
    pub const DIVIDE: u8 = 6;
    pub const MODULO: u8 = 7;
    pub const GREATER: u8 = 8;
    pub const GREATER_EQUAL: u8 = 9;
    pub const LESS: u8 = 10;
    pub const LESS_EQUAL: u8 = 11;
    pub const EQUAL: u8 = 12;
    pub const NOT_EQUAL: u8 = 13;
}

// When you add an opcode, don't forget to adjust the try_into implementation

pub struct Chunk {
    code: Vec<u8>,
    constants: Vec<Value>,
    lines: Vec<usize>,
}

impl Chunk {
    pub fn new(capacity: usize, const_capacity: usize) -> Chunk {
        Chunk {
            code: Vec::with_capacity(capacity),
            constants: Vec::with_capacity(const_capacity),
            lines: Vec::with_capacity(capacity / 6),
        }
    }
    pub fn push_code(&mut self, code: u8, line: usize) {
        self.code.push(code);
        self.lines.push(line);
    }

    pub fn push_constant(&mut self, constant: Value) -> usize {
        self.constants.push(constant);
        self.constants.len() -1
    }

    pub fn shrink(&mut self, name: &str) {
        let code_len = self.code.capacity();
        let const_len = self.constants.capacity();
        self.code.shrink_to_fit();
        self.constants.shrink_to_fit();

        // DEBUG
        println!("=== Chunk {} ===", name);
        println!("Shrank byte code vector by {} to {} entries", code_len - self.code.capacity(), self.code.capacity());
        println!("Shrank constant vector by {} to {} entries\n", const_len - self.constants.capacity(), self.constants.capacity());
    }

    pub fn code(&self) -> &[u8] {
        &self.code
    }

    pub fn constants(&self) -> &[Value] {
        &self.constants
    }

    pub fn lines(&self) -> &[usize] {
        &self.lines
    }
}
