use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Value(u64);

// If all of these bits are set it is a quiet NaN (this type of NaN doesn't get produced by the CPU as an error)
// We use these QNAN values to store other data types
const QNAN: u64 = 0x7ffc000000000000;
const SIGN_BIT: u64 = 0x8000000000000000;
const NIL_VAL: u64 = QNAN | 0x1;
const FALSE_VAL: u64 = QNAN | 0x2;
const TRUE_VAL: u64 = QNAN | 0x3;
const OBJ_MASK: u64 = QNAN | SIGN_BIT;
#[derive(PartialEq)]
pub enum ValueType {
    Number,
    Boolean,
    Nil,
    Object,
}

impl Value {
    // Kind of redundant, but I like this method :)))
    pub fn is(&self, value_type: ValueType) -> bool {
        match value_type {
            ValueType::Number => self.0 & QNAN != QNAN,
            ValueType::Boolean => self.0 == TRUE_VAL || self.0 == FALSE_VAL,
            ValueType::Nil => self.0 == NIL_VAL,
            ValueType::Object => self.0 & OBJ_MASK == OBJ_MASK,
        }
    }

    fn compare_with_obj(&self, value_type: ValueType) -> bool {
        todo!()
    }

    pub fn get_type(&self) -> ValueType {
        if self.0 & QNAN != QNAN {
            ValueType::Number
        } else if self.0 == TRUE_VAL || self.0 == FALSE_VAL {
            ValueType::Boolean
        } else if self.0 == NIL_VAL {
            ValueType::Nil
        } else if self.0 & OBJ_MASK == OBJ_MASK {
            ValueType::Object
        } else {
            unreachable!("Invalid value type")
        }
    }

    pub fn create_nil() -> Value {
        Value(NIL_VAL)
    }

    pub fn as_float(&self) -> f64 {
        f64::from_bits(self.0)
    }

    pub fn as_bool(&self) -> bool {
        self.0 == TRUE_VAL
    }

    pub fn as_object(&self) -> u64 {
        self.0 & !OBJ_MASK
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value(QNAN | (value as u64 + 2))
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value(value.to_bits())
    }
}

impl<T> From<*mut T> for Value {
    fn from(value: *mut T) -> Self {
        Value(value as u64)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        match self.get_type() {
            ValueType::Number => write!(f, "{}", self.as_float()),
            ValueType::Boolean => write!(f, "{}", self.as_bool()),
            ValueType::Nil => write!(f, "nil"),
            ValueType::Object => todo!(),
        }
    }
}
