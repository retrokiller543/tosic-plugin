//! Error types for plugin operations.

use thiserror::Error;

/// Errors that can occur during plugin operations.
#[derive(Error, Debug)]
pub enum PluginError {
    /// Failed to load plugin from bytes.
    #[error("Failed to load plugin: {0}")]
    LoadError(String),
    
    /// Failed to call a specific function in the plugin.
    #[error("Failed to call function '{function}': {message}")]
    CallError { 
        /// The name of the function that failed to call.
        function: String, 
        /// The error message describing why the call failed.
        message: String 
    },
    
    /// Function was not found in the loaded plugin.
    #[error("Function '{0}' not found in plugin")]
    FunctionNotFound(String),
    
    /// Invalid argument type provided to a function call.
    #[error("Invalid argument type for function call")]
    InvalidArgumentType,
    
    /// General runtime error during plugin execution.
    #[error("Runtime error: {0}")]
    RuntimeError(String),
    
    /// Host function was not found in the context.
    #[error("Host function '{0}' not found")]
    HostFunctionNotFound(String),
    
    /// Plugin is in an invalid state for the requested operation.
    #[error("Invalid plugin state")]
    InvalidPluginState,
}

/// Result type for plugin operations that may fail.
pub type PluginResult<T, E = PluginError> = Result<T, E>;