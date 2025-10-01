//! Multi-runtime flexible plugin manager.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::traits::{PluginManager, PluginId, Runtime, Plugin};
use crate::types::{HostContext, Value};
use crate::prelude::{PluginResult, PluginSource};

/// Plugin entry that stores a plugin along with its associated runtime.
struct PluginEntry {
    plugin: Box<dyn Plugin>,
    runtime_index: usize,
}

/// Flexible plugin manager that supports multiple runtime types.
/// 
/// This manager can work with different runtime types simultaneously, automatically
/// selecting the appropriate runtime based on plugin source compatibility. It uses
/// dynamic dispatch for maximum flexibility at the cost of some performance overhead.
/// 
/// # Features
/// - Automatic runtime selection based on plugin compatibility
/// - Support for unlimited runtime types
/// - Runtime registration at runtime
/// - Plugin source type detection
/// 
/// # Example
/// ```ignore
/// use tosic_plugin_core::managers::MultiRuntimeManager;
/// use tosic_plugin_deno_runtime::DenoRuntime;
/// 
/// let mut manager = MultiRuntimeManager::new();
/// manager.register_runtime(Box::new(DenoRuntime::new()));
/// // Manager can now handle any plugin types supported by registered runtimes
/// ```
pub struct MultiRuntimeManager {
    runtimes: Vec<Box<dyn Runtime>>,
    plugins: HashMap<PluginId, PluginEntry>,
    next_id: AtomicU64,
}

impl MultiRuntimeManager {
    /// Creates a new multi-runtime manager with no registered runtimes.
    pub fn new() -> Self {
        Self {
            runtimes: Vec::new(),
            plugins: HashMap::new(),
            next_id: AtomicU64::new(1),
        }
    }

    /// Registers a runtime with this manager.
    /// The runtime will be used for plugins that it declares support for.
    pub fn register_runtime(&mut self, runtime: Box<dyn Runtime>) {
        self.runtimes.push(runtime);
    }

    /// Returns the names of all registered runtimes.
    pub fn runtime_names(&self) -> Vec<&str> {
        self.runtimes.iter().map(|r| r.runtime_name()).collect()
    }

    /// Returns the number of registered runtimes.
    pub fn runtime_count(&self) -> usize {
        self.runtimes.len()
    }

    /// Returns the number of currently loaded plugins.
    pub fn plugin_count(&self) -> usize {
        self.plugins.len()
    }

    /// Returns an iterator over all loaded plugin IDs.
    pub fn plugin_ids(&self) -> impl Iterator<Item = PluginId> + '_ {
        self.plugins.keys().copied()
    }

    /// Generates the next unique plugin ID.
    fn next_plugin_id(&self) -> PluginId {
        PluginId(self.next_id.fetch_add(1, Ordering::Relaxed))
    }

    /// Finds the first runtime that supports the given plugin source.
    fn find_compatible_runtime(&mut self, source: &PluginSource) -> Option<(usize, &mut Box<dyn Runtime>)> {
        self.runtimes
            .iter_mut()
            .enumerate()
            .find(|(_, runtime)| runtime.supports_plugin(source))
    }
}

impl Default for MultiRuntimeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(feature = "async", async_trait::async_trait)]
impl PluginManager for MultiRuntimeManager {
    #[cfg(not(feature = "async"))]
    fn load_plugin(&mut self, source: PluginSource, context: &HostContext) -> PluginResult<PluginId> {
        let (runtime_index, runtime) = self.find_compatible_runtime(&source)
            .ok_or_else(|| crate::PluginError::LoadError(
                "No compatible runtime found for this plugin source".to_string()
            ))?;
        
        let plugin = runtime.load(&source, context)?;

        let id = self.next_plugin_id();
        let entry = PluginEntry {
            plugin,
            runtime_index,
        };
        self.plugins.insert(id, entry);

        Ok(id)
    }
    
    #[cfg(feature = "async")]
    async fn load_plugin(&mut self, source: PluginSource, context: &HostContext) -> PluginResult<PluginId> {
        // Find a compatible runtime
        let (runtime_index, runtime) = self.find_compatible_runtime(&source)
            .ok_or_else(|| crate::PluginError::LoadError(
                "No compatible runtime found for this plugin source".to_string()
            ))?;

        // Load the plugin using the compatible runtime
        let plugin = runtime.load(&source, context).await?;
        
        // Generate ID and store the plugin with its runtime info
        let id = self.next_plugin_id();
        let entry = PluginEntry {
            plugin,
            runtime_index,
        };
        self.plugins.insert(id, entry);
        
        Ok(id)
    }

    #[cfg(not(feature = "async"))]
    fn call_plugin(&self, id: PluginId, function_name: &str, args: &[Value]) -> PluginResult<Value> {
        let entry = self.plugins.get(&id)
            .ok_or(crate::PluginError::InvalidPluginState)?;

        let runtime = &self.runtimes[entry.runtime_index];
        runtime.call(&*entry.plugin, function_name, args)
    }

    #[cfg(feature = "async")]
    async fn call_plugin(&self, id: PluginId, function_name: &str, args: &[Value]) -> PluginResult<Value> {
        let entry = self.plugins.get(&id)
            .ok_or(crate::PluginError::InvalidPluginState)?;
        
        let runtime = &self.runtimes[entry.runtime_index];
        runtime.call(&*entry.plugin, function_name, args).await
    }

    #[cfg(not(feature = "async"))]
    fn unload_plugin(&mut self, id: PluginId) -> PluginResult<()> {
        match self.plugins.remove(&id) {
            Some(_) => Ok(()),
            None => Err(crate::PluginError::InvalidPluginState),
        }
    }

    #[cfg(feature = "async")]
    async fn unload_plugin(&mut self, id: PluginId) -> PluginResult<()> {
        match self.plugins.remove(&id) {
            Some(_) => Ok(()),
            None => Err(crate::PluginError::InvalidPluginState),
        }
    }

    fn plugin_name(&self, id: PluginId) -> Option<&str> {
        self.plugins.get(&id).and_then(|entry| entry.plugin.name())
    }

    fn is_plugin_loaded(&self, id: PluginId) -> bool {
        self.plugins.contains_key(&id)
    }
}