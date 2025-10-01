//! Host context for plugin function registration.

mod registry;

use std::collections::HashMap;
use std::sync::Arc;
use cfg_if::cfg_if;
use crate::PluginResult;
use crate::types::Value;
use crate::traits::host_function::HostFunction;

#[cfg(feature = "global-registry")]
pub use registry::*;

cfg_if! {
    if #[cfg(feature = "async")] {
        use std::future::Future;
        use std::pin::Pin;
        
        use crate::traits::host_function::AsyncHostFunction;
        
        /// Type-erased asynchronous host function that can be stored in the context.
        pub(crate) type BoxedAsyncHostFunction = Arc<dyn Fn(&[Value]) -> Pin<Box<dyn Future<Output = PluginResult<Value>> + Send>> + Send + Sync>;
    }
}

/// Type-erased host function that can be stored in the context.
pub(crate) type BoxedHostFunction = Arc<dyn Fn(&[Value]) -> PluginResult<Value> + Send + Sync>;

/// Boxes a synchronous host function into a type-erased BoxedHostFunction.
#[inline(always)]
pub fn box_fn<F, Args>(func: F) -> BoxedHostFunction
where
    F: HostFunction<Args> + 'static,
    Args: ExtractArgs,
{
    let func = Arc::new(func);
    Arc::new(move |args: &[Value]| -> PluginResult<Value> {
        let extracted_args = Args::extract_args(args)?;
        func.call(extracted_args)
    })
}

/// Boxes a synchronous host function into a type-erased BoxedHostFunction.
#[cfg(feature = "async")]
#[inline(always)]
pub fn box_async_fn<F, Args>(func: F) -> BoxedAsyncHostFunction
where
    F: AsyncHostFunction<Args> + Send + Sync + 'static,
    Args: ExtractArgs + Send,
{
    let func = Arc::new(func);
    Arc::new(move |args: &[Value]| -> Pin<Box<dyn Future<Output=PluginResult<Value>> + Send + 'static>> {
        let func = Arc::clone(&func);
        let args = args.to_vec();
        Box::pin(async move {
            let extracted_args = match Args::extract_args(&args) {
                Ok(a) => a,
                Err(e) => return Err(e),
            };
            
            func.call(extracted_args).await
        })
    })
}

/// Unified enum for both sync and async host functions
#[derive(Clone)]
pub enum HostFunctionType {
    /// Synchronous host function
    Sync(BoxedHostFunction),
    /// Asynchronous host function
    #[cfg(feature = "async")]
    Async(BoxedAsyncHostFunction),
}

/// Iterator that takes ownership of HostContext and yields its functions.
pub struct HostContextIntoIter {
    inner: std::collections::hash_map::IntoIter<String, HostFunctionType>,
}

/// Context containing host functions that can be injected into plugin runtimes.
/// Functions are identified by their string names and can be called from plugins.
#[derive(Default, Clone)]
pub struct HostContext {
    functions: HashMap<String, HostFunctionType>,
}

impl IntoIterator for HostContext {
    type Item = (String, HostFunctionType);
    type IntoIter = HostContextIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        HostContextIntoIter {
            inner: self.functions.into_iter()
        }
    }
}

impl Iterator for HostContextIntoIter {
    type Item = (String, HostFunctionType);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl HostContext {
    /// Creates a new empty host context.
    pub fn new() -> Self {
        #[cfg(feature = "global-registry")]
        {
            let mut context = Self {
                functions: HashMap::new(),
            };
            registry::HostCapabilityRegistry::load_into_context(&mut context);
            context
        }
        #[cfg(not(feature = "global-registry"))]
        {
            Self {
                functions: HashMap::new(),
            }
        }
    }

    /// Registers a host function with the given name.
    /// The function can have any signature that implements HostFunction.
    pub fn register<Args, F>(&mut self, name: impl Into<String>, func: F)
    where
        F: HostFunction<Args> + 'static,
        Args: ExtractArgs,
    {
        self.functions.insert(name.into(), HostFunctionType::Sync(box_fn(func)));
    }
    
    /// Registers an asynchronous host function with the given name.
    /// The function can have any signature that implements AsyncHostFunction.
    #[cfg(feature = "async")]
    pub fn register_async<Args, F>(&mut self, name: impl Into<String>, func: F)
    where
        F: AsyncHostFunction<Args> + Send + Sync + 'static,
        Args: ExtractArgs + Send,
    {
        self.functions.insert(name.into(), HostFunctionType::Async(box_async_fn(func)));
    }

    /// Gets a host function by name and calls it with the provided arguments.
    #[cfg(not(feature = "async"))]
    pub fn call_function(&self, name: &str, args: &[Value]) -> PluginResult<Value> {
        match self.functions.get(name) {
            Some(HostFunctionType::Sync(func)) => func(args),
            None => Err(crate::PluginError::HostFunctionNotFound(name.to_string())),
        }
    }

    /// Gets a host function by name and calls it with the provided arguments.
    #[cfg(feature = "async")]
    pub async fn call_function(&self, name: &str, args: &[Value]) -> PluginResult<Value> {
        match self.functions.get(name) {
            Some(HostFunctionType::Sync(func)) => func(args),
            Some(HostFunctionType::Async(func)) => func(args).await,
            None => Err(crate::PluginError::HostFunctionNotFound(name.to_string())),
        }
    }

    /// Returns all registered function names.
    pub fn function_names(&self) -> impl Iterator<Item = &String> {
        self.functions.keys()
    }

    /// Returns all registered synchronous function names.
    pub fn sync_function_names(&self) -> impl Iterator<Item = &String> {
        self.functions.iter().filter_map(|(name, func_type)| {
            if matches!(func_type, HostFunctionType::Sync(_)) {
                Some(name)
            } else {
                None
            }
        })
    }

    /// Returns all registered asynchronous function names.
    #[cfg(feature = "async")]
    pub fn async_function_names(&self) -> impl Iterator<Item = &String> {
        self.functions.iter().filter_map(|(name, func_type)| {
            if matches!(func_type, HostFunctionType::Async(_)) {
                Some(name)
            } else {
                None
            }
        })
    }

    /// Returns an iterator over all registered functions as (name, function_type) pairs.
    /// This is more efficient than iterating over names when you need both.
    pub fn functions(&self) -> impl Iterator<Item = (&String, &HostFunctionType)> {
        self.functions.iter()
    }

    /// Returns true if any function with the given name is registered.
    pub fn has_function(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }

    /// Returns true if a synchronous function with the given name is registered.
    pub fn has_sync_function(&self, name: &str) -> bool {
        matches!(self.functions.get(name), Some(HostFunctionType::Sync(_)))
    }

    /// Returns true if an asynchronous function with the given name is registered.
    #[cfg(feature = "async")]
    pub fn has_async_function(&self, name: &str) -> bool {
        matches!(self.functions.get(name), Some(HostFunctionType::Async(_)))
    }

    /// Returns the type of function registered with the given name.
    pub fn function_type(&self, name: &str) -> Option<&HostFunctionType> {
        self.functions.get(name)
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