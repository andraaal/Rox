use crate::chunk::{Chunk, OpCode, Value};
use crate::debug::print_instruction;
use crate::compiler;

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
        let parser = compiler::Parser::new(&file);
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
                OpCode::OpReturn => {
                    if self.stack.len() > 0 {
                        println!("{}", self.stack.pop().unwrap());
                    } else {
                        println!("No return value: stack is empty");
                    }
                    return InterpretResult::InterpretOK;
                }
                OpCode::OpConstant => {
                    let index = self.read_byte() as usize;
                    let constant = self.chunk.constants()[index];
                    self.stack.push(constant);
                }
                OpCode::OpNegate => {
                    let Some(x) = self.stack.last_mut() else {
                        return InterpretResult::InterpretRuntimeError(
                            "No value to perform operation on.",
                        );
                    };
                    *x = (-1 as Value) * *x;
                }
                OpCode::OpAdd => self.binary_operation(|a, b| a + b),
                OpCode::OpSubtract => self.binary_operation(|a, b| a - b),
                OpCode::OpMultiply => self.binary_operation(|a, b| a * b),
                OpCode::OpDivide => self.binary_operation(|a, b| a / b),
                OpCode::OpModulo => self.binary_operation(|a, b| a % b),
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
}
