//! Core abstractions and traits for the tosic-plugin system.
//!
//! This crate provides the foundational types and traits for building a generic,
//! runtime-agnostic plugin system. It supports multiple plugin runtimes (WASM, JS, Lua, etc.)
//! through a common interface.
//!
//! # Features
//!
//! - **async**: Enable async/await support for plugin operations (recommended)
//!
//! # Core Concepts
//!
//! - [`Runtime`]: Trait for plugin runtime implementations
//! - [`Plugin`]: Opaque handle to loaded plugin instances  
//! - [`Value`]: Boundary type for data exchange between host and plugins
//! - [`HostContext`]: Container for host functions that plugins can call
//! - [`HostFunction`]: Trait for type-safe host function registration
//!
//! # Example
//!
//! ```rust
//! use tosic_plugin_core::prelude::*;
//!
//! // Create a host context and register functions
//! let mut context = HostContext::new();
//! context.register("add", |a: i64, b: i64| a + b);
//! context.register("greet", |name: String| format!("Hello, {}!", name));
//!
//! // Runtime implementations would use this context to provide host functions to plugins
//! ```

// Strict linting for release builds
#![cfg_attr(not(debug_assertions), deny(
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::missing_safety_doc,
    clippy::missing_panics_doc,
    unused,
))]
// More relaxed linting for debug builds to aid development but still warn about issues that will break release builds
#![cfg_attr(debug_assertions, warn(
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::missing_safety_doc,
    clippy::missing_panics_doc,
    unused,
))]

#[cfg(feature = "inventory")]
pub extern crate inventory;

pub mod traits;
pub mod types;
pub mod error;
pub mod managers;
pub mod prelude;

pub use crate::error::PluginError;
pub use crate::error::PluginResult;