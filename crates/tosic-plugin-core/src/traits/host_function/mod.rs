//! Host function traits for type-safe function registration and calling.

mod async_fn;

use crate::PluginResult;
use crate::types::Value;

#[cfg(feature = "async")]
pub use async_fn::*;

/// Trait for types that can be extracted from plugin Values.
#[diagnostic::on_unimplemented(
    message = "the type `{Self}` cannot be extracted from a plugin Value",
    note = "ensure your type implements `FromValue` or use one of the built-in types: bool, i32, i64, f32, f64, String, Vec<u8>, Vec<Value>, HashMap<String, Value>"
)]
pub trait FromValue: Sized {
    /// Extracts a Rust type from a plugin Value.
    /// 
    /// # Errors
    /// Returns `PluginError::InvalidArgumentType` if the value cannot be converted to the target type.
    fn from_value(value: &Value) -> PluginResult<Self>;
}

/// Trait for types that can be converted into plugin Values.
#[diagnostic::on_unimplemented(
    message = "the type `{Self}` cannot be converted into a plugin Value",
    note = "ensure your type implements `IntoValue` or use one of the built-in types: bool, i32, i64, f32, f64, String, &str, Vec<u8>, &[u8], Vec<Value>, HashMap<String, Value>, Value, ()"
)]
pub trait IntoValue {
    /// Converts a Rust type into a plugin Value.
    fn into_value(self) -> Value;
}

/// Trait for functions that can be used as host functions.
/// This trait is implemented for functions with different arities.
#[diagnostic::on_unimplemented(
    message = "the function `{Self}` cannot be used as a host function",
    note = "ensure your function arguments implement `FromValue` and return type implements `IntoValue`. Functions must be `Fn(...) -> R + Send + Sync`. Maximum 16 arguments supported."
)]
pub trait HostFunction<Args>: Send + Sync {
    /// The return type of the host function.
    type Output: IntoValue;

    /// Calls the host function with the provided arguments.
    ///
    /// # Errors
    /// Returns an error if the function call fails or if argument types are invalid.
    fn call(&self, args: Args) -> PluginResult<Value>;
}

#[allow(missing_docs)]
macro_rules! impl_host_function {
    // Base case: no arguments
    () => {
        impl<F, R> HostFunction<()> for F
        where
            F: Fn() -> R + Send + Sync,
            R: IntoValue,
        {
            type Output = R;
            
            #[inline(always)]
            fn call(&self, _args: ()) -> PluginResult<Value> {
                Ok(self().into_value())
            }
        }
    };
    
    // Recursive case: generate implementation for N arguments
    ($($arg:ident),+) => {
        impl<F, $($arg,)+ R> HostFunction<($($arg,)+)> for F
        where
            F: Fn($($arg,)+) -> R + Send + Sync,
            $($arg: FromValue,)+
            R: IntoValue,
        {
            type Output = R;
            
            #[allow(non_snake_case)]
            #[inline(always)]
            fn call(&self, ($($arg,)+): ($($arg,)+)) -> PluginResult<Value> {
                Ok(self($($arg,)+).into_value())
            }
        }
    };
}

// Generate implementations for 0 to 16 arguments
impl_host_function!();
impl_host_function!(A1);
impl_host_function!(A1, A2);
impl_host_function!(A1, A2, A3);
impl_host_function!(A1, A2, A3, A4);
impl_host_function!(A1, A2, A3, A4, A5);
impl_host_function!(A1, A2, A3, A4, A5, A6);
impl_host_function!(A1, A2, A3, A4, A5, A6, A7);
impl_host_function!(A1, A2, A3, A4, A5, A6, A7, A8);
impl_host_function!(A1, A2, A3, A4, A5, A6, A7, A8, A9);
impl_host_function!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);
impl_host_function!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11);
impl_host_function!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12);
impl_host_function!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13);
impl_host_function!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14);
impl_host_function!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15);
impl_host_function!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16);