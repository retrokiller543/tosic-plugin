//! Plugin manager trait for managing multiple plugins.

/// Trait for managing multiple plugin instances.
/// 
/// This trait provides a high-level interface for plugin lifecycle management.
/// Implementations can handle plugin discovery, loading, unloading, and coordination.
pub trait PluginManager: Send + Sync {}