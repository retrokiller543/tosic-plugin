//! Value type for plugin data exchange.

use std::collections::HashMap;
use serde::Serialize;
use crate::{PluginResult, PluginError};

/// Boundary type for passing values between the host and plugin runtime.
/// This enum represents all possible values that can cross the plugin boundary.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Value {
    /// Represents a null/none value.
    Null,
    /// Boolean value (true or false).
    Bool(bool),
    /// 64-bit signed integer.
    Int(i64),
    /// 64-bit floating point number.
    Float(f64),
    /// UTF-8 string.
    String(String),
    /// Binary data as a vector of bytes.
    Bytes(Vec<u8>),
    /// Array of values.
    Array(Vec<Value>),
    /// Object/map with string keys and Value values.
    Object(HashMap<String, Value>),
}

impl Value {
    /// Returns true if the value is null.
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// Attempts to extract a boolean value.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Attempts to extract an integer value.
    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(i) => Some(*i),
            _ => None,
        }
    }

    /// Attempts to extract a floating point value.
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(f) => Some(*f),
            _ => None,
        }
    }

    /// Attempts to extract a string slice.
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    /// Attempts to extract a byte slice.
    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            Value::Bytes(b) => Some(b),
            _ => None,
        }
    }

    /// Attempts to extract an array slice.
    pub fn as_array(&self) -> Option<&[Value]> {
        match self {
            Value::Array(a) => Some(a),
            _ => None,
        }
    }

    /// Attempts to extract an object map.
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

impl From<serde_json::Value> for Value {
    fn from(value: serde_json::Value) -> Self {
        match value {
            serde_json::Value::Null => Value::Null,
            serde_json::Value::Bool(b) => Value::Bool(b),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Value::Int(i)
                } else if let Some(f) = n.as_f64() {
                    Value::Float(f)
                } else {
                    Value::Null // Fallback for unsupported number types
                }
            }
            serde_json::Value::String(s) => Value::String(s),
            serde_json::Value::Array(arr) => {
                Value::Array(arr.into_iter().map(Value::from).collect())
            }
            serde_json::Value::Object(obj) => {
                let map = obj.into_iter()
                    .map(|(k, v)| (k, Value::from(v)))
                    .collect();
                Value::Object(map)
            }
        }
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

impl Into<serde_json::Value> for Value {
    fn into(self) -> serde_json::Value {
        match self {
            Value::Null => serde_json::Value::Null,
            Value::Bool(b) => serde_json::Value::Bool(b),
            Value::Int(i) => serde_json::Value::Number(serde_json::Number::from(i)),
            Value::Float(f) => {
                serde_json::Number::from_f64(f)
                    .map(serde_json::Value::Number)
                    .unwrap_or(serde_json::Value::Null)
            }
            Value::String(s) => serde_json::Value::String(s),
            Value::Bytes(b) => {
                serde_json::Value::Array(b.into_iter().map(|byte| serde_json::Value::Number(serde_json::Number::from(byte))).collect())
            }
            Value::Array(arr) => {
                let json_arr: Vec<serde_json::Value> = arr.into_iter().map(|v| v.into()).collect();
                serde_json::Value::Array(json_arr)
            }
            Value::Object(obj) => {
                let json_obj: serde_json::Map<String, serde_json::Value> = obj
                    .into_iter()
                    .map(|(k, v)| (k, v.into()))
                    .collect();
                serde_json::Value::Object(json_obj)
            }
        }
    }
}