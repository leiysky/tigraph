use std::collections::HashMap;

use mysql::{
    prelude::{ConvIr, FromValue},
    FromValueError, Value as V,
};

pub enum Type {
    Integer,
    Double,
    // Decimal(u32, u32), // Decimal(Scale, Precision)
    String,
    // Datetime,
    // Array,
    // Map,
    // Blob,
    Null,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Int(i64),
    Double(f64),
    String(String),
    Boolean(bool),
    Null,

    Object(Object),
    Array(Array),
}

impl FromValue for Value {
    type Intermediate = Value;

    fn from_value(v: V) -> Value {
        Value::from(v)
    }

    fn from_value_opt(v: V) -> Result<Value, FromValueError> {
        Ok(Value::from(v))
    }

    fn get_intermediate(v: V) -> Result<Self::Intermediate, FromValueError> {
        Ok(Value::from(v))
    }
}

impl ConvIr<Value> for Value {
    fn new(v: V) -> Result<Self, FromValueError> {
        Ok(Value::from(v))
    }

    fn commit(self) -> Value {
        Value::from(self)
    }

    fn rollback(self) -> V {
        V::NULL
    }
}

impl From<V> for Value {
    fn from(value: V) -> Value {
        match value {
            V::Int(v) => Value::Int(v),
            V::Double(v) => Value::Double(v),
            V::Bytes(v) => Value::String(String::from_utf8(v).unwrap()),
            V::NULL => Value::Null,
            _ => Value::Null,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Object {
    pub props: HashMap<String, Value>,
}

impl Object {
    pub fn new() -> Object {
        Object {
            props: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.props.clear();
    }

    pub fn get(&self, prop_name: &str) -> Option<&Value> {
        self.props.get(prop_name)
    }

    pub fn set(&mut self, prop_name: &str, value: &Value) {
        self.props.insert(String::from(prop_name), value.to_owned());
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Array {
    pub elements: Vec<Value>,
}

impl Array {
    pub fn new() -> Array {
        Array {
            elements: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.elements.clear();
    }

    pub fn get(&self, index: usize) -> Option<&Value> {
        self.elements.get(index)
    }

    pub fn set(&mut self, index: usize, value: &Value) {
        self.elements.insert(index, value.to_owned());
    }

    pub fn push_back(&mut self, value: &Value) {
        self.elements.push(value.to_owned());
    }

    pub fn pop_back(&mut self) {
        self.elements.pop();
    }
}
