use crate::chunk::{Chunk, OpCode};
use crate::compiler;
use crate::debug::print_instruction;
use crate::value::{Value, ValueType};

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
                OpCode::Return => {
                    if self.stack.len() > 0 {
                        println!("{}", self.stack.pop().unwrap());
                    } else {
                        println!("No return value: stack is empty");
                    }
                    return InterpretResult::InterpretOK;
                }
                OpCode::Constant => {
                    let index = self.read_byte() as usize;
                    let constant = self.chunk.constants()[index];
                    self.stack.push(constant);
                }
                OpCode::Negate => {
                    let Some(x) = self.stack.last_mut() else {
                        return InterpretResult::InterpretRuntimeError(
                            "No value to perform operation on.",
                        );
                    };
                    *x = Value::from(-1.0 * x.as_float());
                    // TODO Could optimize here, just need to change first bit
                }
                OpCode::Add => self.numeric_binary_operation(|a, b| Value::from(a + b)),
                OpCode::Subtract => self.numeric_binary_operation(|a, b| Value::from(a - b)),
                OpCode::Multiply => self.numeric_binary_operation(|a, b| Value::from(a * b)),
                OpCode::Divide => self.numeric_binary_operation(|a, b| Value::from(a / b)),
                OpCode::Modulo => self.numeric_binary_operation(|a, b| Value::from(a % b)),
                OpCode::Greater => self.numeric_binary_operation(|a, b| Value::from(a > b)),
                OpCode::GreaterEqual => self.numeric_binary_operation(|a, b| Value::from(a >= b)),
                OpCode::Less => self.numeric_binary_operation(|a, b| Value::from(a < b)),
                OpCode::LessEqual => self.numeric_binary_operation(|a, b| Value::from(a <= b)),
                OpCode::Equal => self.binary_operation(|a, b| Value::from(a == b)),
                OpCode::NotEqual => self.binary_operation(|a, b| Value::from(a != b)),
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
        if !a.is(ValueType::Number) || !b.is(ValueType::Number) {
            self.runtime_error("Both operands must be numbers");
        }
        self.stack.push(callback(a.as_float(), b.as_float()));
    }

    fn runtime_error(&mut self, message: &'static str) {
        println!("[ERROR] in line {}: {}", self.ip, message);
    }
}
