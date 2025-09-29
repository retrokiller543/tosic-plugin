use std::collections::HashMap;
use crate::{PluginResult, PluginError};

/// Boundary type for passing values between the host and plugin runtime.
/// This enum represents all possible values that can cross the plugin boundary.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Bytes(Vec<u8>),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

impl Value {
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(f) => Some(*f),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            Value::Bytes(b) => Some(b),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&[Value]> {
        match self {
            Value::Array(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&HashMap<String, Value>> {
        match self {
            Value::Object(o) => Some(o),
            _ => None,
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Int(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Int(value as i64)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Float(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::Float(value as f64)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_string())
    }
}

impl From<Vec<u8>> for Value {
    fn from(value: Vec<u8>) -> Self {
        Value::Bytes(value)
    }
}

impl From<&[u8]> for Value {
    fn from(value: &[u8]) -> Self {
        Value::Bytes(value.to_vec())
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Value::Array(value)
    }
}

impl From<HashMap<String, Value>> for Value {
    fn from(value: HashMap<String, Value>) -> Self {
        Value::Object(value)
    }
}

// FromValue trait implementations for extracting Rust types from plugin Values
use crate::traits::host_function::{FromValue, IntoValue};

impl FromValue for bool {
    fn from_value(value: &Value) -> PluginResult<Self> {
        match value {
            Value::Bool(b) => Ok(*b),
            _ => Err(PluginError::InvalidArgumentType),
        }
    }
}

impl FromValue for i64 {
    fn from_value(value: &Value) -> PluginResult<Self> {
        match value {
            Value::Int(i) => Ok(*i),
            _ => Err(PluginError::InvalidArgumentType),
        }
    }
}

impl FromValue for i32 {
    fn from_value(value: &Value) -> PluginResult<Self> {
        match value {
            Value::Int(i) => Ok(*i as i32),
            _ => Err(PluginError::InvalidArgumentType),
        }
    }
}

impl FromValue for f64 {
    fn from_value(value: &Value) -> PluginResult<Self> {
        match value {
            Value::Float(f) => Ok(*f),
            _ => Err(PluginError::InvalidArgumentType),
        }
    }
}

impl FromValue for f32 {
    fn from_value(value: &Value) -> PluginResult<Self> {
        match value {
            Value::Float(f) => Ok(*f as f32),
            _ => Err(PluginError::InvalidArgumentType),
        }
    }
}

impl FromValue for String {
    fn from_value(value: &Value) -> PluginResult<Self> {
        match value {
            Value::String(s) => Ok(s.clone()),
            _ => Err(PluginError::InvalidArgumentType),
        }
    }
}

impl FromValue for Vec<u8> {
    fn from_value(value: &Value) -> PluginResult<Self> {
        match value {
            Value::Bytes(b) => Ok(b.clone()),
            _ => Err(PluginError::InvalidArgumentType),
        }
    }
}

impl FromValue for Vec<Value> {
    fn from_value(value: &Value) -> PluginResult<Self> {
        match value {
            Value::Array(a) => Ok(a.clone()),
            _ => Err(PluginError::InvalidArgumentType),
        }
    }
}

impl FromValue for HashMap<String, Value> {
    fn from_value(value: &Value) -> PluginResult<Self> {
        match value {
            Value::Object(o) => Ok(o.clone()),
            _ => Err(PluginError::InvalidArgumentType),
        }
    }
}

// IntoValue trait implementations for converting Rust types to plugin Values
impl IntoValue for bool {
    fn into_value(self) -> Value {
        Value::Bool(self)
    }
}

impl IntoValue for i64 {
    fn into_value(self) -> Value {
        Value::Int(self)
    }
}

impl IntoValue for i32 {
    fn into_value(self) -> Value {
        Value::Int(self as i64)
    }
}

impl IntoValue for f64 {
    fn into_value(self) -> Value {
        Value::Float(self)
    }
}

impl IntoValue for f32 {
    fn into_value(self) -> Value {
        Value::Float(self as f64)
    }
}

impl IntoValue for String {
    fn into_value(self) -> Value {
        Value::String(self)
    }
}

impl IntoValue for &str {
    fn into_value(self) -> Value {
        Value::String(self.to_string())
    }
}

impl IntoValue for Vec<u8> {
    fn into_value(self) -> Value {
        Value::Bytes(self)
    }
}

impl IntoValue for &[u8] {
    fn into_value(self) -> Value {
        Value::Bytes(self.to_vec())
    }
}

impl IntoValue for Vec<Value> {
    fn into_value(self) -> Value {
        Value::Array(self)
    }
}

impl IntoValue for HashMap<String, Value> {
    fn into_value(self) -> Value {
        Value::Object(self)
    }
}

impl IntoValue for Value {
    fn into_value(self) -> Value {
        self
    }
}

impl IntoValue for () {
    fn into_value(self) -> Value {
        Value::Null
    }
}