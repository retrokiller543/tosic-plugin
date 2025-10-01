#![cfg(feature = "async")]

use std::future::Future;
use std::pin::Pin;
use crate::PluginResult;
use crate::prelude::Value;
use super::*;

/// Trait for functions that can be used as async host functions.
/// This trait is implemented for functions with different arities.
#[diagnostic::on_unimplemented(
    message = "the function `{Self}` cannot be used as a async host function",
    note = "ensure your function arguments implement `FromValue` and return type implements `IntoValue`. Functions must be `Fn(...) -> impl Future<Output = R> + Send + Sync`. Maximum 16 arguments supported."
)]
pub trait AsyncHostFunction<Args>: Send + Sync {
    /// The return type of the host function.
    type Output: IntoValue;

    /// Calls the host function with the provided arguments.
    ///
    /// # Errors
    /// Returns an error if the function call fails or if argument types are invalid.
    fn call(&self, args: Args) -> Pin<Box<dyn Future<Output = PluginResult<Value>> + Send + '_>>;
}

#[allow(missing_docs)]
macro_rules! async_host_function_impl {
    () => {
        impl<F, Fut, R> AsyncHostFunction<()> for F
        where
            F: Fn() -> Fut + Send + Sync,
            Fut: Future<Output = R> + Send + 'static,
            R: IntoValue + Send + Sync,
        {
            type Output = R;
            
            #[inline(always)]
            fn call(&self, _args: ()) -> Pin<Box<dyn Future<Output = PluginResult<Value>> + Send + '_>> {
                let fut = self();
                Box::pin(async move {
                    Ok(fut.await.into_value())
                })
            }
        }
    };
    
    // Recursive case: generate implementation for N arguments
    ($($arg:ident),+) => {
        impl<F, $($arg,)+ Fut, R> AsyncHostFunction<($($arg,)+)> for F
        where
            F: Fn($($arg,)+) -> Fut + Send + Sync,
            Fut: Future<Output = R> + Send + 'static,
            $($arg: FromValue + Send + Sync,)+
            R: IntoValue + Send + Sync,
        {
            type Output = R;
            
            #[allow(non_snake_case)]
            #[inline(always)]
            fn call(&self, args: ($($arg,)+)) -> Pin<Box<dyn Future<Output = PluginResult<Value>> + Send + '_>> {
                let ($($arg,)+) = args;
                let fut = self($($arg,)+);
                Box::pin(async move {
                    Ok(fut.await.into_value())
                })
            }
        }
    };
}

async_host_function_impl!();
async_host_function_impl!(A1);
async_host_function_impl!(A1, A2);
async_host_function_impl!(A1, A2, A3);
async_host_function_impl!(A1, A2, A3, A4);
async_host_function_impl!(A1, A2, A3, A4, A5);
async_host_function_impl!(A1, A2, A3, A4, A5, A6);
async_host_function_impl!(A1, A2, A3, A4, A5, A6, A7);
async_host_function_impl!(A1, A2, A3, A4, A5, A6, A7, A8);
async_host_function_impl!(A1, A2, A3, A4, A5, A6, A7, A8, A9);
async_host_function_impl!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);
async_host_function_impl!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11);
async_host_function_impl!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12);
async_host_function_impl!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13);
async_host_function_impl!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14);
async_host_function_impl!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15);
async_host_function_impl!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16);