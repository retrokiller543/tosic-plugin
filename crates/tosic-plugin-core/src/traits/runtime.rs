//! Runtime abstraction traits for plugin loading and execution.

use crate::types::{HostContext, Value};
use crate::prelude::{PluginResult, PluginSource};

/// Opaque handle to a loaded plugin instance.
/// This trait represents a loaded piece of plugin code that can be executed.
pub trait Plugin: Send + Sync {
    /// Returns metadata about the plugin (optional).
    fn name(&self) -> Option<&str> {
        None
    }
}

/// Runtime abstraction for loading and executing plugins.
/// This trait provides a dyn-compatible interface for different plugin runtimes.
#[cfg(not(feature = "async"))]
pub trait Runtime: Send + Sync {
    /// The specific plugin type this runtime creates.
    type Plugin: Plugin;

    /// Loads plugin code from bytes with the provided host context.
    /// Returns a plugin instance that can be used to call functions.
    fn load(&mut self, source: &PluginSource, context: &HostContext) -> PluginResult<()>;

    /// Calls a function in the loaded plugin with the given arguments.
    /// Returns the result value from the plugin function.
    fn call(
        &self,
        plugin: &Self::Plugin,
        function_name: &str,
        args: &[Value],
    ) -> PluginResult<Value>;
}

/// Async runtime abstraction for loading and executing plugins.
/// This trait provides a dyn-compatible interface for different plugin runtimes.
#[cfg(feature = "async")]
#[async_trait::async_trait]
pub trait Runtime: Send + Sync {
    /// The specific plugin type this runtime creates.
    type Plugin: Plugin;

    /// Loads plugin code from bytes with the provided host context.
    /// Returns a plugin instance that can be used to call functions.
    async fn load(&mut self, source: &PluginSource, context: &HostContext) -> PluginResult<()>;

    /// Calls a function in the loaded plugin with the given arguments.
    /// Returns the result value from the plugin function.
    async fn call(
        &self,
        plugin: &Self::Plugin,
        function_name: &str,
        args: &[Value],
    ) -> PluginResult<Value>;
}