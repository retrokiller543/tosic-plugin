//! Value type for plugin data exchange.

use crate::{PluginResult, PluginError};

/// Boundary type for passing values between the host and plugin runtime.
/// This is an alias to serde_json::Value for maximum compatibility and stability.
pub type Value = serde_json::Value;

// Blanket implementations using serde for maximum compatibility
use crate::traits::host_function::{FromValue, IntoValue};
use serde::{Deserialize, Serialize};

impl<T> FromValue for T
where
    T: for<'de> Deserialize<'de>,
{
    fn from_value(value: &Value) -> PluginResult<Self> {
        serde_json::from_value(value.clone())
            .map_err(|_| PluginError::InvalidArgumentType)
    }
}

impl<T> IntoValue for T
where
    T: Serialize,
{
    fn into_value(self) -> Value {
        serde_json::to_value(self)
            .unwrap_or(Value::Null)
    }
}