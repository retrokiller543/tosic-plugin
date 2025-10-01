//! Concrete implementations of the PluginManager trait.
//! 
//! This module provides optimized manager implementations for different use cases:
//! - SingleRuntimeManager: Optimized for single runtime scenarios
//! - MultiRuntimeManager: Flexible support for multiple runtimes

pub mod single;
pub mod multi;

pub use single::SingleRuntimeManager;
pub use multi::MultiRuntimeManager;