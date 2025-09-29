use thiserror::Error;

/// Errors that can occur during plugin operations.
#[derive(Error, Debug)]
pub enum PluginError {
    #[error("Failed to load plugin: {0}")]
    LoadError(String),
    
    #[error("Failed to call function '{function}': {message}")]
    CallError { function: String, message: String },
    
    #[error("Function '{0}' not found in plugin")]
    FunctionNotFound(String),
    
    #[error("Invalid argument type for function call")]
    InvalidArgumentType,
    
    #[error("Runtime error: {0}")]
    RuntimeError(String),
    
    #[error("Host function '{0}' not found")]
    HostFunctionNotFound(String),
    
    #[error("Invalid plugin state")]
    InvalidPluginState,
}

pub type PluginResult<T, E = PluginError> = Result<T, E>;