//! Single-runtime optimized plugin manager.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::traits::{PluginManager, PluginId, Runtime, Plugin};
use crate::types::{HostContext, Value};
use crate::prelude::{PluginResult, PluginSource};

/// High-performance plugin manager optimized for a single runtime type.
/// 
/// This manager is generic over a specific runtime type, allowing for compile-time
/// optimizations and zero-cost abstractions. It's ideal when you know you'll only
/// use one type of runtime (e.g., only WASM or only JavaScript).
/// 
/// # Features
/// - Zero-cost runtime calls (no dynamic dispatch)
/// - Compile-time optimization
/// - Type-safe plugin handling
/// - Minimal memory overhead
/// 
/// # Example
/// ```ignore
/// use tosic_plugin_core::managers::SingleRuntimeManager;
/// use tosic_plugin_deno_runtime::DenoRuntime;
/// 
/// let mut manager = SingleRuntimeManager::new(DenoRuntime::new());
/// // Manager is now optimized specifically for Deno runtime
/// ```
pub struct SingleRuntimeManager<R: Runtime> {
    runtime: R,
    plugins: HashMap<PluginId, Box<dyn Plugin>>,
    next_id: AtomicU64,
}

impl<R: Runtime> SingleRuntimeManager<R> {
    /// Creates a new single-runtime manager with the provided runtime.
    pub fn new(runtime: R) -> Self {
        Self {
            runtime,
            plugins: HashMap::new(),
            next_id: AtomicU64::new(1),
        }
    }

    /// Returns a reference to the underlying runtime.
    pub fn runtime(&self) -> &R {
        &self.runtime
    }

    /// Returns a mutable reference to the underlying runtime.
    pub fn runtime_mut(&mut self) -> &mut R {
        &mut self.runtime
    }

    /// Generates the next unique plugin ID.
    fn next_plugin_id(&self) -> PluginId {
        PluginId(self.next_id.fetch_add(1, Ordering::Relaxed))
    }

    /// Returns the number of currently loaded plugins.
    pub fn plugin_count(&self) -> usize {
        self.plugins.len()
    }

    /// Returns an iterator over all loaded plugin IDs.
    pub fn plugin_ids(&self) -> impl Iterator<Item = PluginId> + '_ {
        self.plugins.keys().copied()
    }
}

#[cfg_attr(feature = "async", async_trait::async_trait)]
impl<R: Runtime> PluginManager for SingleRuntimeManager<R> {
    #[cfg(not(feature = "async"))]
    fn load_plugin(&mut self, source: PluginSource, context: &HostContext) -> PluginResult<PluginId> {
        // Check if runtime supports this plugin type
        if !self.runtime.supports_plugin(&source) {
            return Err(crate::PluginError::LoadError(
                format!("Runtime '{}' does not support this plugin source", self.runtime.runtime_name())
            ));
        }

        // Load the plugin using the runtime
        let plugin = self.runtime.load(&source, context)?;

        // Generate ID and store the plugin
        let id = self.next_plugin_id();
        self.plugins.insert(id, plugin);

        Ok(id)
    }
    
    #[cfg(feature = "async")]
    async fn load_plugin(&mut self, source: PluginSource, context: &HostContext) -> PluginResult<PluginId> {
        // Check if runtime supports this plugin type
        if !self.runtime.supports_plugin(&source) {
            return Err(crate::PluginError::LoadError(
                format!("Runtime '{}' does not support this plugin source", self.runtime.runtime_name())
            ));
        }

        // Load the plugin using the runtime
        let plugin = self.runtime.load(&source, context).await?;
        
        // Generate ID and store the plugin
        let id = self.next_plugin_id();
        self.plugins.insert(id, plugin);
        
        Ok(id)
    }

    #[cfg(not(feature = "async"))]
    fn call_plugin(&mut self, id: PluginId, function_name: &str, args: &[Value]) -> PluginResult<Value> {
        match self.plugins.get_mut(&id) {
            Some(plugin) => self.runtime.call(plugin, function_name, args),
            None => Err(crate::PluginError::InvalidPluginState),
        }
    }

    #[cfg(feature = "async")]
    async fn call_plugin(&mut self, id: PluginId, function_name: &str, args: &[Value]) -> PluginResult<Value> {
        match self.plugins.get_mut(&id) {
            Some(plugin) => self.runtime.call(plugin, function_name, args).await,
            None => Err(crate::PluginError::InvalidPluginState),
        }
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
        self.plugins.get(&id).and_then(|plugin| plugin.name())
    }

    fn is_plugin_loaded(&self, id: PluginId) -> bool {
        self.plugins.contains_key(&id)
    }
}