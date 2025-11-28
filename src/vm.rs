use crate::chunk::{opcode, Chunk};
use crate::compiler;
use crate::debug::print_instruction;
use crate::value::Value;

pub struct Vm {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
}

pub enum InterpretResult {
    InterpretOK,
    InterpretError(&'static str),
    InterpretRuntimeError(&'static str),
}

impl Vm {
    pub fn new(chunk: Chunk) -> Vm {
        Vm {
            chunk,
            ip: 0,
            stack: Vec::with_capacity(256),
        }
    }

    pub fn change_chunk(&mut self, chunk: Chunk) {
        self.chunk = chunk;
        self.ip = 0;
        self.stack.clear();
    }

    pub fn interpret(file: String) -> InterpretResult {
        let ch = Chunk::new(8, 4);
        let mut parser = compiler::Parser::new(&file);
        let success = parser.compile();
        if !success {
            return InterpretResult::InterpretError("Error");
        }
        let mut vm = Vm::new(ch);
        vm.run()
    }
    pub fn run(&mut self) -> InterpretResult {
        println!();
        loop {
            // Disable for efficient interpretation
            // DEBUG begin
            print_instruction(&self.chunk, self.ip);
            // DEBUG end

            let Ok(op) = self.read_byte().try_into() else {
                return InterpretResult::InterpretError("Invalid OpCode");
            };

            match op {
                opcode::RETURN => {
                    if self.stack.len() > 0 {
                        println!("{}", self.stack.pop().unwrap());
                    } else {
                        println!("No return value: stack is empty");
                    }
                    return InterpretResult::InterpretOK;
                }
                opcode::CONSTANT => {
                    let index = self.read_byte() as usize;
                    let constant = Value::Nil; // self.chunk.constants()[index]; FIXME
                    self.stack.push(constant);
                }
                opcode::NEGATE => {
                    let Some(x) = self.stack.last_mut() else {
                        return InterpretResult::InterpretRuntimeError(
                            "No value to perform operation on.",
                        );
                    };
                    *x = Value::from(-1.0 * 1.0); //FIXME
                    // TODO Could optimize here, just need to change first bit
                }
                opcode::ADD => self.numeric_binary_operation(|a, b| Value::from(a + b)),
                opcode::SUBTRACT => self.numeric_binary_operation(|a, b| Value::from(a - b)),
                opcode::MULTIPLY => self.numeric_binary_operation(|a, b| Value::from(a * b)),
                opcode::DIVIDE => self.numeric_binary_operation(|a, b| Value::from(a / b)),
                opcode::MODULO => self.numeric_binary_operation(|a, b| Value::from(a % b)),
                opcode::GREATER => self.numeric_binary_operation(|a, b| Value::from(a > b)),
                opcode::GREATER_EQUAL => self.numeric_binary_operation(|a, b| Value::from(a >= b)),
                opcode::LESS => self.numeric_binary_operation(|a, b| Value::from(a < b)),
                opcode::LESS_EQUAL => self.numeric_binary_operation(|a, b| Value::from(a <= b)),
                opcode::EQUAL => self.binary_operation(|a, b| Value::from(a == b)),
                opcode::NOT_EQUAL => self.binary_operation(|a, b| Value::from(a != b)),
                _ => todo!(),
            }

            // DEBUG begin
            print!("Stack: ");
            for val in self.stack.iter() {
                print!("[ {} ]", val)
            }
            println!();
            println!();
            // DEBUG end
        }
    }

    fn read_byte(&mut self) -> u8 {
        let result = self.chunk.code()[self.ip];
        self.ip += 1;
        result
    }

    fn binary_operation<F>(&mut self, callback: F)
    where
        F: Fn(Value, Value) -> Value,
    {
        let b = self.stack.pop().expect("No value to perform operation on.");
        let a = self.stack.pop().expect("No value to perform operation on.");
        self.stack.push(callback(a, b));
    }

    fn numeric_binary_operation<F>(&mut self, callback: F)
    where
        F: Fn(f64, f64) -> Value,
    {
        let b = self.stack.pop().expect("No value to perform operation on.");
        let a = self.stack.pop().expect("No value to perform operation on.");

        match (a, b) {
            (Value::Number(c), Value::Number(d)) => {
                self.stack.push(callback(c, d));
            }
            _ => todo!(),
        }
    }

    fn runtime_error(&mut self, message: &'static str) {
        println!("[ERROR] in line {}: {}", self.ip, message);
    }
}
