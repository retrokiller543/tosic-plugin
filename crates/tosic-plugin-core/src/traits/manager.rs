//! Plugin manager trait for managing multiple plugins.

use crate::types::{HostContext, Value};
use crate::prelude::{PluginResult, PluginSource};
use crate::traits::host_function::IntoArgs;

/// Unique identifier for a loaded plugin instance.
/// This allows managers to track and reference plugins after loading.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PluginId(pub u64);

/// Trait for managing plugin instances.
/// 
/// This trait provides a minimal, flexible interface for plugin lifecycle management.
/// Implementations can be optimized for single-runtime or multi-runtime scenarios.
/// The trait is intentionally minimal to allow maximum implementation flexibility.
#[cfg(not(feature = "async"))]
pub trait PluginManager {
    /// Loads a plugin from the given source with the provided host context.
    /// Returns a unique identifier that can be used to reference the plugin.
    /// 
    /// # Errors
    /// Returns an error if the plugin cannot be loaded or if no compatible runtime is available.
    fn load_plugin(&mut self, source: PluginSource, context: &HostContext) -> PluginResult<PluginId>;

    /// Calls a function in the specified plugin with the given arguments.
    /// Returns the result value from the plugin function.
    /// 
    /// # Errors
    /// Returns an error if the plugin ID is invalid, function doesn't exist, or the call fails.
    fn call_plugin(&mut self, id: PluginId, function_name: &str, args: impl IntoArgs) -> PluginResult<Value>;

    /// Unloads the specified plugin and frees its resources.
    /// After this call, the plugin ID becomes invalid.
    /// 
    /// # Errors
    /// Returns an error if the plugin ID is invalid or unloading fails.
    fn unload_plugin(&mut self, id: PluginId) -> PluginResult<()>;

    /// Returns the name of the plugin with the given ID, if available.
    fn plugin_name(&self, id: PluginId) -> Option<&str>;

    /// Returns true if a plugin with the given ID is currently loaded.
    fn is_plugin_loaded(&self, id: PluginId) -> bool;
}

/// Async trait for managing plugin instances.
/// 
/// This trait provides a minimal, flexible interface for plugin lifecycle management.
/// Implementations can be optimized for single-runtime or multi-runtime scenarios.
#[cfg(feature = "async")]
#[async_trait::async_trait]
pub trait PluginManager: Send + Sync {
    /// Loads a plugin from the given source with the provided host context.
    /// Returns a unique identifier that can be used to reference the plugin.
    /// 
    /// # Errors
    /// Returns an error if the plugin cannot be loaded or if no compatible runtime is available.
    async fn load_plugin(&mut self, source: PluginSource, context: &HostContext) -> PluginResult<PluginId>;

    /// Calls a function in the specified plugin with the given arguments.
    /// Returns the result value from the plugin function.
    /// 
    /// # Errors
    /// Returns an error if the plugin ID is invalid, function doesn't exist, or the call fails.
    async fn call_plugin(&mut self, id: PluginId, function_name: &str, args: impl IntoArgs + Send + Sync) -> PluginResult<Value>;

    /// Unloads the specified plugin and frees its resources.
    /// After this call, the plugin ID becomes invalid.
    /// 
    /// # Errors
    /// Returns an error if the plugin ID is invalid or unloading fails.
    async fn unload_plugin(&mut self, id: PluginId) -> PluginResult<()>;

    /// Returns the name of the plugin with the given ID, if available.
    fn plugin_name(&self, id: PluginId) -> Option<&str>;

    /// Returns true if a plugin with the given ID is currently loaded.
    fn is_plugin_loaded(&self, id: PluginId) -> bool;
}
