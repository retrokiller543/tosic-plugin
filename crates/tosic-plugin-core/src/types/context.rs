//! Host context for plugin function registration.

use std::collections::HashMap;
use std::sync::Arc;
use crate::PluginResult;
use crate::types::Value;

use crate::traits::host_function::HostFunction;

/// Type-erased host function that can be stored in the context.
type BoxedHostFunction = Arc<dyn Fn(&[Value]) -> PluginResult<Value> + Send + Sync>;

/// Iterator that takes ownership of HostContext and yields its functions.
pub struct HostContextIntoIter {
    inner: std::collections::hash_map::IntoIter<String, BoxedHostFunction>,
}

/// Context containing host functions that can be injected into plugin runtimes.
/// Functions are identified by their string names and can be called from plugins.
#[derive(Default, Clone)]
pub struct HostContext {
    functions: HashMap<String, BoxedHostFunction>,
}


impl IntoIterator for HostContext {
    type Item = (String, BoxedHostFunction);
    type IntoIter = HostContextIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        HostContextIntoIter {
            inner: self.functions.into_iter()
        }
    }
}

impl Iterator for HostContextIntoIter {
    type Item = (String, BoxedHostFunction);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl HostContext {
    /// Creates a new empty host context.
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }

    /// Registers a host function with the given name.
    /// The function can have any signature that implements HostFunction.
    pub fn register<Args, F>(&mut self, name: impl Into<String>, func: F)
    where
        F: HostFunction<Args> + 'static,
        Args: ExtractArgs,
    {
        let boxed_func = Arc::new(move |args: &[Value]| -> PluginResult<Value> {
            let extracted_args = Args::extract_args(args)?;
            func.call(extracted_args)
        });
        
        self.functions.insert(name.into(), boxed_func);
    }

    /// Gets a host function by name and calls it with the provided arguments.
    pub fn call_function(&self, name: &str, args: &[Value]) -> PluginResult<Value> {
        match self.functions.get(name) {
            Some(func) => func(args),
            None => Err(crate::PluginError::HostFunctionNotFound(name.to_string())),
        }
    }

    /// Returns all registered function names.
    pub fn function_names(&self) -> impl Iterator<Item = &String> {
        self.functions.keys()
    }

    /// Returns an iterator over all registered functions as (name, function) pairs.
    /// This is more efficient than iterating over names when you need both.
    pub fn functions(&self) -> impl Iterator<Item = (&String, &BoxedHostFunction)> {
        self.functions.iter()
    }

    /// Returns true if a function with the given name is registered.
    pub fn has_function(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }
}

/// Trait for extracting arguments from a Value array into the appropriate tuple type.
/// 
/// # Errors
/// Returns `PluginError::InvalidArgumentType` if argument extraction fails.
pub trait ExtractArgs: Sized {
    /// Extracts typed arguments from a Value slice.
    fn extract_args(args: &[Value]) -> PluginResult<Self>;
}

/// Macro to implement ExtractArgs for different tuple sizes.
macro_rules! impl_extract_args {
    () => {
        impl ExtractArgs for () {
            fn extract_args(args: &[Value]) -> PluginResult<Self> {
                if args.is_empty() {
                    Ok(())
                } else {
                    Err(crate::PluginError::InvalidArgumentType)
                }
            }
        }
    };
    
    ($($arg:ident),+) => {
        impl<$($arg,)+> ExtractArgs for ($($arg,)+)
        where
            $($arg: crate::traits::host_function::FromValue,)+
        {
            fn extract_args(args: &[Value]) -> PluginResult<Self> {
                #[allow(unused)]
                const ARG_COUNT: usize = {
                    let mut count = 0;
                    $( let _ = stringify!($arg); count += 1; )+
                    count
                };
                if args.len() != ARG_COUNT {
                    return Err(crate::PluginError::InvalidArgumentType);
                }
                
                let mut iter = args.iter();
                Ok((
                    $($arg::from_value(iter.next().unwrap())?,)+
                ))
            }
        }
    };
}

// Generate implementations for 0 to 16 arguments
impl_extract_args!();
impl_extract_args!(A1);
impl_extract_args!(A1, A2);
impl_extract_args!(A1, A2, A3);
impl_extract_args!(A1, A2, A3, A4);
impl_extract_args!(A1, A2, A3, A4, A5);
impl_extract_args!(A1, A2, A3, A4, A5, A6);
impl_extract_args!(A1, A2, A3, A4, A5, A6, A7);
impl_extract_args!(A1, A2, A3, A4, A5, A6, A7, A8);
impl_extract_args!(A1, A2, A3, A4, A5, A6, A7, A8, A9);
impl_extract_args!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);
impl_extract_args!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11);
impl_extract_args!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12);
impl_extract_args!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13);
impl_extract_args!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14);
impl_extract_args!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15);
impl_extract_args!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16);