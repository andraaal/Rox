use crate::expr::Expr;
use crate::value::Value;

pub struct Interpreter {}

impl Interpreter {
    fn runtime_error(&self, msg: &str) -> ! {
        eprintln!("[interpreter] {}", msg);
        std::process::exit(1);
    }
    pub fn interpret(&mut self, tree: Expr) {
        let res = self.expression(tree);
        println!("{}", res);
    }

    fn expression(&mut self, tree: Expr) -> Value {
        match tree {
            Expr::Null => Value::Nil,
            Expr::Bool(b) => Value::Bool(b),
            Expr::Number(f) => Value::Number(f),
            Expr::String(s) => Value::String(s),
            Expr::Negate(e) => self.expression(*e),
            Expr::Add(a, b) => self.numeric_op(*a, *b, |a, b| Value::Number(a + b)),
            Expr::Sub(a, b) => self.numeric_op(*a, *b, |a, b| Value::Number(a - b)),
            Expr::Mul(a, b) => self.numeric_op(*a, *b, |a, b| Value::Number(a * b)),
            Expr::Div(a, b) => self.numeric_op(*a, *b, |a, b| Value::Number(a / b)),
            Expr::Mod(a, b) => self.numeric_op(*a, *b, |a, b| Value::Number(a % b)),
            Expr::Eq(a, b) => self.comparison(*a, *b, |a, b| a == b),
            Expr::Neq(a, b) => self.comparison(*a, *b, |a, b| a != b),
            Expr::Greater(a, b) => self.numeric_op(*a, *b, |a, b| Value::Bool(a > b)),
            Expr::Less(a, b) => self.numeric_op(*a, *b, |a, b| Value::Bool(a < b)),
            Expr::GreaterEqual(a, b) => self.numeric_op(*a, *b, |a, b| Value::Bool(a >= b)),
            Expr::LessEqual(a, b) => self.numeric_op(*a, *b, |a, b| Value::Bool(a <= b)),
        }
    }

    fn numeric_op(&mut self, right: Expr, left: Expr, func: fn(f64, f64) -> Value) -> Value {
        let a = self.expression(right);
        let b = self.expression(left);

        if let (Value::Number(a), Value::Number(b)) = (a, b) {
            return
                func(a, b);
        }
        self.runtime_error("Type mismatch at ??? - Error locations not yet implemented");
    }

    fn comparison(&mut self, left: Expr, right: Expr, func: fn(Value, Value) -> bool) -> Value {
        let a = self.expression(left);
        let b = self.expression(right);

        Value::Bool(func(a, b))
    }
}