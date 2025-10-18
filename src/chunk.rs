#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum OpCode {
    OpReturn = 0,
    OpConstant = 1,
    OpNegate = 2,
    OpAdd = 3,
    OpSubtract = 4,
    OpMultiply = 5,
    OpDivide = 6,
    OpModulo = 7,
}

// When you add opcode, don't forget to adjust the try_into implementation

pub type Value = f64;

pub struct Chunk {
    code: Vec<u8>,
    constants: Vec<Value>,
    lines: Vec<u32>,
}

impl Chunk {
    pub fn new(capacity: usize, const_capacity: usize) -> Chunk {
        Chunk {
            code: Vec::with_capacity(capacity),
            constants: Vec::with_capacity(const_capacity),
            lines: Vec::with_capacity(capacity),
        }
    }
    pub fn push_code(&mut self, code: u8, line: u32) {
        self.code.push(code);
        self.lines.push(line);
    }

    pub fn push_constant(&mut self, constant: Value) {
        self.constants.push(constant);
    }

    pub fn code(&self) -> &[u8] {
        &self.code
    }

    pub fn constants(&self) -> &[Value] {
        &self.constants
    }

    pub fn lines(&self) -> &[u32] {
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
            0 => Ok(OpCode::OpReturn),
            1 => Ok(OpCode::OpConstant),
            2 => Ok(OpCode::OpNegate),
            3 => Ok(OpCode::OpAdd),
            4 => Ok(OpCode::OpSubtract),
            5 => Ok(OpCode::OpMultiply),
            6 => Ok(OpCode::OpDivide),
            7 => Ok(OpCode::OpModulo),
            _ => Err(OpCodeError(value)),
        }
    }
}

impl From<OpCode> for u8 {
    fn from(value: OpCode) -> Self {
        value as u8
    }
}
