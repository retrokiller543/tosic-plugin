#![cfg(feature = "global-registry")]

use std::collections::HashMap;

use super::{HostFunctionType, HostContext, BoxedHostFunction};

#[cfg(feature = "async")]
use super::BoxedAsyncHostFunction;

/// Registry capability that can create either sync or async host functions
pub struct HostCapability {
    name: &'static str,
    kind: HostCapabilityKind,
}

/// Enum to unify sync and async function creation in the registry
pub enum HostCapabilityKind {
    /// Synchronous host function creator
    Sync {
        /// Function that creates a boxed sync function
        boxer: fn() -> BoxedHostFunction,
    },
    /// Asynchronous host function creator
    #[cfg(feature = "async")]
    Async {
        /// Function that creates a boxed async function
        boxer: fn() -> BoxedAsyncHostFunction,
    },
}

impl HostCapability {
    /// Creates a new synchronous host capability that can be registered with inventory
    pub const fn new_sync(name: &'static str, boxer: fn() -> BoxedHostFunction) -> Self {
        Self {
            name,
            kind: HostCapabilityKind::Sync { boxer },
        }
    }
    
    /// Creates a new asynchronous host capability that can be registered with inventory
    #[cfg(feature = "async")]
    pub const fn new_async(name: &'static str, boxer: fn() -> BoxedAsyncHostFunction) -> Self {
        Self {
            name,
            kind: HostCapabilityKind::Async { boxer },
        }
    }
}

inventory::collect!(HostCapability);

/// Registry that provides access to all registered host capabilities
/// This is hidden from the public API - users just get the functions via HostContext::new()
pub(crate) struct HostCapabilityRegistry;

fn init_cache() -> HashMap<String, HostFunctionType> {
    let mut map = HashMap::new();
    
    for capability in inventory::iter::<HostCapability> {
        let name = capability.name;
    
        let func_type = match &capability.kind {
            HostCapabilityKind::Sync { boxer } => HostFunctionType::Sync(boxer()),
            #[cfg(feature = "async")]
            HostCapabilityKind::Async { boxer } => HostFunctionType::Async(boxer()),
        };
    
        map.insert(name.to_string(), func_type);
    }
    
    map
}

impl HostCapabilityRegistry {
    /// Loads all registered capabilities into a HostContext
    /// This is called automatically when creating a new HostContext with global-registry feature
    pub(crate) fn load_into_context(context: &mut HostContext) {
        use std::sync::OnceLock;
        
        static CACHED_FUNCTIONS: OnceLock<HashMap<String, HostFunctionType>> = OnceLock::new();
        
        for (name, func_type) in CACHED_FUNCTIONS.get_or_init(init_cache).iter() {
            context.functions.insert(name.clone(), func_type.clone());
        }
    }
}

/// Macro to register a synchronous host function in the global inventory.
/// 
/// # Example
/// ```rust
/// # use tosic_plugin_core::prelude::*;
/// fn add(a: i32, b: i32) -> i32 {
///     a + b  
/// }
/// 
/// register_sync_fn!("add", add);
/// ```
#[macro_export]
macro_rules! register_sync_fn {
    ($name:literal, $func:ident) => {
        $crate::inventory::submit! {
            $crate::prelude::HostCapability::new_sync(
                $name,
                || $crate::prelude::box_fn($func)
            )
        }
    };
}

/// Macro to register an asynchronous host function in the global inventory.
/// 
/// # Example  
/// ```rust
/// # use tosic_plugin_core::prelude::*;
/// async fn async_add(a: i32, b: i32) -> i32 {
///     a + b
/// }
/// 
/// register_async_fn!("async_add", async_add);
/// ```
#[cfg(feature = "async")]
#[macro_export]
macro_rules! register_async_fn {
    ($name:literal, $func:ident) => {
        $crate::inventory::submit! {
            $crate::prelude::HostCapability::new_async(
                $name,
                || $crate::prelude::box_async_fn($func)
            )
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::PluginResult;
    use crate::types::Value;
    use super::*;
    
    fn test_sync_fn(a: i32, b: i32) -> i32 {
        a + b
    }
    
    #[cfg(feature = "async")]
    async fn test_async_fn(a: i32, b: i32) -> i32 {
        a + b
    }
    
    register_sync_fn!("test_sync_fn", test_sync_fn);
    
    #[cfg(feature = "async")]
    register_async_fn!("test_async_fn", test_async_fn);
    
    fn run_func(context: &HostContext, name: &str, args: &[Value]) -> PluginResult<Value> {
        #[cfg(not(feature = "async"))]
        let res = context.call_function(name, args)?;
        
        #[cfg(feature = "async")]
        let res = futures::executor::block_on(context.call_function(name, args))?;
        
        Ok(res)
    }
    
    #[test]
    fn test_registry() {
        let context = HostContext::new();
        assert!(context.has_function("test_sync_fn"));
        assert!(context.has_sync_function("test_sync_fn"));
        
        let res = run_func(&context, "test_sync_fn", &[Value::Int(1), Value::Int(2)]).unwrap();
        
        assert_eq!(res.as_int().unwrap(), 3);
        
        #[cfg(feature = "async")]
        {
            assert!(context.has_function("test_async_fn"));
            assert!(context.has_async_function("test_async_fn"));
            
            let res = run_func(&context, "test_async_fn", &[Value::Int(3), Value::Int(4)]).unwrap();
            assert_eq!(res.as_int().unwrap(), 7);
        }
    }
}