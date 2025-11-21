use crate::value::Value;

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum OpCode {
    Return = 0,
    Constant = 1,
    Negate = 2,
    Add = 3,
    Subtract = 4,
    Multiply = 5,
    Divide = 6,
    Modulo = 7,
    Greater = 8,
    GreaterEqual = 9,
    Less = 10,
    LessEqual = 11,
    Equal = 12,
    NotEqual = 13,
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

#[derive(Debug, thiserror::Error)]
#[error("Invalid opcode: {0}")]
pub struct OpCodeError(u8);

impl TryFrom<u8> for OpCode {
    type Error = OpCodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OpCode::Return),
            1 => Ok(OpCode::Constant),
            2 => Ok(OpCode::Negate),
            3 => Ok(OpCode::Add),
            4 => Ok(OpCode::Subtract),
            5 => Ok(OpCode::Multiply),
            6 => Ok(OpCode::Divide),
            7 => Ok(OpCode::Modulo),
            8 => Ok(OpCode::Greater),
            9 => Ok(OpCode::GreaterEqual),
            10 => Ok(OpCode::Less),
            11 => Ok(OpCode::LessEqual),
            12 => Ok(OpCode::Equal),
            13 => Ok(OpCode::NotEqual),
            _ => Err(OpCodeError(value)),
        }
    }
}

impl From<OpCode> for u8 {
    fn from(value: OpCode) -> Self {
        value as u8
    }
}
