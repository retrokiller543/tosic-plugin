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
//! use tosic_plugin_core::*;
//!
//! // Create a host context and register functions
//! let mut context = HostContext::new();
//! context.register("add", |a: i64, b: i64| a + b);
//! context.register("greet", |name: String| format!("Hello, {}!", name));
//!
//! // Runtime implementations would use this context to provide host functions to plugins
//! ```

// Strict linting for release builds
#![cfg_attr(not(debug_assertions), deny(missing_docs))]
#![cfg_attr(not(debug_assertions), deny(clippy::all))]
#![cfg_attr(not(debug_assertions), deny(clippy::pedantic))]
#![cfg_attr(not(debug_assertions), deny(unsafe_code))]
#![cfg_attr(not(debug_assertions), deny(unused))]

pub mod traits;
pub mod types;
mod error;

// Re-export core types and traits
pub use error::*;
pub use traits::{host_function::*, runtime::*};
pub use types::*;
