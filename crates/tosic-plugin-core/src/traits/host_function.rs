use crate::PluginResult;
use crate::types::Value;

/// Trait for types that can be extracted from plugin Values.
pub trait FromValue: Sized {
    fn from_value(value: &Value) -> PluginResult<Self>;
}

/// Trait for types that can be converted into plugin Values.
pub trait IntoValue {
    fn into_value(self) -> Value;
}

/// Trait for functions that can be used as host functions.
/// This trait is implemented for functions with different arities.
pub trait HostFunction<Args>: Send + Sync {
    type Output: IntoValue;
    
    fn call(&self, args: Args) -> PluginResult<Value>;
}

/// Macro to generate HostFunction implementations for different arities.
macro_rules! impl_host_function {
    // Base case: no arguments
    () => {
        impl<F, R> HostFunction<()> for F
        where
            F: Fn() -> R + Send + Sync,
            R: IntoValue,
        {
            type Output = R;
            
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