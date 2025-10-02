//! Runtime abstraction traits for plugin loading and execution.

use std::any::Any;
use crate::types::{HostContext, Value};
use crate::prelude::{PluginResult, PluginSource};
use crate::traits::host_function::IntoArgs;

/// Opaque handle to a loaded plugin instance.
/// This trait represents a loaded piece of plugin code that can be executed.
#[cfg(not(feature = "async"))]
pub trait Plugin {
    /// Returns metadata about the plugin (optional).
    fn name(&self) -> Option<&str> {
        None
    }
    
    /// Returns a reference to the plugin as Any for downcasting.
    fn as_any(&self) -> &dyn Any;

    /// Returns a mutable reference to the plugin as Any for downcasting.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Opaque handle to a loaded plugin instance (async version).
/// This trait represents a loaded piece of plugin code that can be executed.
#[cfg(feature = "async")]
pub trait Plugin: Send + Sync {
    /// Returns metadata about the plugin (optional).
    fn name(&self) -> Option<&str> {
        None
    }
    
    /// Returns a reference to the plugin as Any for downcasting.
    fn as_any(&self) -> &dyn Any;
    
    /// Returns a mutable reference to the plugin as Any for downcasting.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl Plugin for Box<dyn Plugin> {
    fn name(&self) -> Option<&str> {
        (**self).name()
    }
    
    fn as_any(&self) -> &dyn Any {
        (**self).as_any()
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        (**self).as_any_mut()
    }
}

/// Runtime abstraction for loading and executing plugins.
/// This trait provides a dyn-compatible interface for different plugin runtimes.
#[cfg(not(feature = "async"))]
pub trait Runtime {
    /// Returns the name of this runtime for identification purposes.
    fn runtime_name(&self) -> &'static str;

    /// Checks if this runtime can handle the given plugin source.
    /// This allows managers to automatically select appropriate runtimes.
    fn supports_plugin(&self, source: &PluginSource) -> bool;

    /// Loads plugin code from source with the provided host context.
    /// Returns a plugin instance that can be used to call functions.
    fn load(&mut self, source: &PluginSource, context: &HostContext) -> PluginResult<Box<dyn Plugin>>;

    /// Calls a function in the loaded plugin with the given arguments.
    /// Returns the result value from the plugin function.
    fn call(
        &self,
        plugin: &mut dyn Plugin,
        function_name: &str,
        args: &[Value],
    ) -> PluginResult<Value>;
}

/// Async runtime abstraction for loading and executing plugins.
/// This trait provides a dyn-compatible interface for different plugin runtimes.
#[cfg(feature = "async")]
#[async_trait::async_trait]
pub trait Runtime: Send + Sync {
    /// Returns the name of this runtime for identification purposes.
    fn runtime_name(&self) -> &'static str;

    /// Checks if this runtime can handle the given plugin source.
    /// This allows managers to automatically select appropriate runtimes.
    fn supports_plugin(&self, source: &PluginSource) -> bool;

    /// Loads plugin code from source with the provided host context.
    /// Returns a plugin instance that can be used to call functions.
    async fn load(&mut self, source: &PluginSource, context: &HostContext) -> PluginResult<Box<dyn Plugin>>;

    /// Calls a function in the loaded plugin with the given arguments.
    /// Returns the result value from the plugin function.
    async fn call(
        &self,
        plugin: &mut dyn Plugin,
        function_name: &str,
        args: &[Value],
    ) -> PluginResult<Value>;
}

// ================================================================================================
// Extension traits for ergonomic argument passing
// ================================================================================================

/// Extension trait that adds ergonomic argument methods to the sync Runtime trait.
/// This provides the same functionality but with generic argument support.
#[cfg(not(feature = "async"))]
pub trait RuntimeExt: Runtime {
    /// Calls a function in the loaded plugin with ergonomic argument support.
    /// 
    /// Accepts various argument formats:
    /// - Tuples: `(arg1, arg2, ...)` where each arg implements `IntoValue`
    /// - Slices: `&[Value]` (backward compatibility)
    /// - Vectors: `Vec<Value>`
    /// - Arrays: `[Value; N]`
    fn call_with<A: IntoArgs>(
        &self,
        plugin: &mut dyn Plugin,
        function_name: &str,
        args: A,
    ) -> PluginResult<Value> {
        let args_vec = args.into_args();
        self.call(plugin, function_name, &args_vec)
    }
}

/// Extension trait that adds ergonomic argument methods to the async Runtime trait.
/// This provides the same functionality but with generic argument support.
#[cfg(feature = "async")]
pub trait RuntimeExt: Runtime {
    /// Calls a function in the loaded plugin with ergonomic argument support.
    /// 
    /// Accepts various argument formats:
    /// - Tuples: `(arg1, arg2, ...)` where each arg implements `IntoValue`
    /// - Slices: `&[Value]` (backward compatibility)
    /// - Vectors: `Vec<Value>`
    /// - Arrays: `[Value; N]`
    async fn call_with<A: IntoArgs>(
        &self,
        plugin: &mut dyn Plugin,
        function_name: &str,
        args: A,
    ) -> PluginResult<Value> {
        let args_vec = args.into_args();
        self.call(plugin, function_name, &args_vec).await
    }
}

// Blanket implementation for all Runtime types
#[cfg(not(feature = "async"))]
impl<T: Runtime> RuntimeExt for T {}

#[cfg(feature = "async")]
impl<T: Runtime> RuntimeExt for T {}