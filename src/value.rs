use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Value {
    value: u64,
}

// If all of these bits are set it is a quiet NaN (this doesn't get produced as an error)
// We use these QNAN values to store other data types
const QNAN: u64 = 0x7ffc000000000000;
const NIL_VAL: u64 = QNAN | 0x1;
const FALSE_VAL: u64 = QNAN | 0x2;
const TRUE_VAL: u64 = QNAN | 0x3;

#[derive(PartialEq)]
pub enum ValueType {
    Number,
    Boolean,
    Nil,
}

impl Value {
    pub fn is(&self, value_type: ValueType) -> bool { // Redundant, but I like this method :)))
        match value_type {
            ValueType::Number => self.value & QNAN != QNAN,
            ValueType::Boolean => self.value == TRUE_VAL || self.value == FALSE_VAL,
            ValueType::Nil => self.value == NIL_VAL,
        }
    }

    pub fn get_type(&self) -> ValueType {
        if self.value & QNAN != QNAN {
            ValueType::Number
        } else if self.value == TRUE_VAL || self.value == FALSE_VAL {
            ValueType::Boolean
        } else if self.value == NIL_VAL {
            ValueType::Nil
        } else {
            unreachable!("Invalid value type")
        }
    }

    pub fn create_nil() -> Value {
        Value {
            value: NIL_VAL,
        }
    }

    pub fn as_float(&self) -> f64 {
        f64::from_bits(self.value)
    }

    pub fn as_bool(&self) -> bool {
        self.value == TRUE_VAL
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Value {
        Value {
            value: QNAN | (value as u64 + 2),
        }
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Value {
        Value {value: value.to_bits() }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.get_type() {
            ValueType::Number => write!(f, "{}", self.as_float()),
            ValueType::Boolean => write!(f, "{}", self.as_bool()),
            ValueType::Nil => write!(f, "nil"),
        }
    }
}