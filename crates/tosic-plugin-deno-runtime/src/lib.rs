pub mod prelude;
mod runtime;
mod plugin;

use std::any::Any;
use std::path::PathBuf;
use std::sync::Mutex;
use tosic_plugin_core::prelude::{HostContext, Plugin, PluginResult, PluginSource, Runtime, Value};
use rustyscript::{Runtime as JsRuntime, RuntimeOptions, Module};

#[cfg(feature = "async")]
use async_trait::async_trait;

/// Wrapper around a JavaScript runtime.
/// 
/// # Safety
/// The underlying rustyscript::Runtime contains non-Send/Sync types from V8/Deno.
/// This wrapper ensures single-threaded access by using a Mutex, making it safe
/// to implement Send + Sync as long as:
/// 1. The runtime is always accessed through the mutex
/// 2. No direct access to the underlying runtime is exposed
/// 3. All JavaScript execution happens on a single thread per plugin instance
pub struct JsPlugin {
    name: String,
    runtime: Mutex<JsRuntime>
}

pub type DenoManager = tosic_plugin_core::managers::SingleRuntimeManager<DenoRuntime>;

#[cfg(feature = "async")]
unsafe impl Send for JsPlugin {}
#[cfg(feature = "async")]
unsafe impl Sync for JsPlugin {}

impl Plugin for JsPlugin {
    fn name(&self) -> Option<&str> {
        Some(&self.name)
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct DenoRuntime;

impl DenoRuntime {
    pub fn new() -> Self {
        Self
    }
    
    /// Internal helper to register host functions with a specific JS runtime instance
    fn register_host_functions(&self, runtime: &mut JsRuntime, context: &HostContext) -> PluginResult<()> {
        for function_name in context.function_names() {
            let context = context.clone(); // This is efficient with Arc-based functions
            let function_name_owned = function_name.clone(); // Clone for move closure
            let function_name_for_error = function_name.clone(); // Clone for error message
            
            // Create a wrapper function that properly handles the call
            runtime.register_function(function_name, move |args: &[serde_json::Value]| {
                // Convert serde_json::Value args to tosic Value args
                let tosic_args: Vec<Value> = args.iter()
                    .map(|v| Value::from(v.clone()))
                    .collect();
                
                // Call the actual host function through the context
                match context.call_function(&function_name_owned, &tosic_args) {
                    Ok(result) => Ok(result.into()),
                    Err(e) => Err(rustyscript::Error::Runtime(format!("Host function error: {}", e))),
                }
            }).map_err(|e| tosic_plugin_core::PluginError::RuntimeError(
                format!("Failed to register host function '{}': {}", function_name_for_error, e)
            ))?;
        }
        
        Ok(())
    }
    

    fn load_from_file(&self, path: &PathBuf, context: &HostContext) -> PluginResult<JsPlugin> {
        debug_assert!(path.is_file());

        let module = Module::load(path)
            .map_err(|e| tosic_plugin_core::PluginError::LoadError(e.to_string()))?;
        
        let mut runtime = JsRuntime::new(RuntimeOptions {
            // Configure runtime options as needed
            ..Default::default()
        }).map_err(|e| tosic_plugin_core::PluginError::RuntimeError(e.to_string()))?;
        
        // Register host functions from context
        self.register_host_functions(&mut runtime, context)?;
        
        runtime.load_module(&module).map_err(|e| tosic_plugin_core::PluginError::RuntimeError(e.to_string()))?;

        let name = path.file_name().unwrap().to_string_lossy().to_string();
        
        let plugin = JsPlugin {
            name,
            runtime: Mutex::new(runtime)
        };
        
        Ok(plugin)
    }

    fn load_from_directory(&self, path: &PathBuf, context: &HostContext) -> PluginResult<JsPlugin> {
        debug_assert!(path.is_dir());

        let modules = Module::load_dir(path)
            .map_err(|e| tosic_plugin_core::PluginError::LoadError(e.to_string()))?;
        
        let entry_point = modules.iter().find(|module| module.filename().file_name().unwrap().to_str().unwrap() == "index.js")
            .ok_or_else(|| tosic_plugin_core::PluginError::LoadError("No index.js entry point found".to_string()))?; 

        let mut runtime = JsRuntime::new(RuntimeOptions {
            // Configure runtime options as needed
            ..Default::default()
        }).map_err(|e| tosic_plugin_core::PluginError::RuntimeError(e.to_string()))?;

        // Register host functions from context
        self.register_host_functions(&mut runtime, context)?;

        let all_modules: Vec<&Module> = modules.iter().collect();
        runtime.load_modules(entry_point, all_modules).map_err(|e| tosic_plugin_core::PluginError::RuntimeError(e.to_string()))?;

        let name = path.file_name().unwrap().to_string_lossy().to_string();
        
        let plugin = JsPlugin {
            name,
            runtime: Mutex::new(runtime)
        };

        Ok(plugin)
    }

    fn load_from_code(&self, code: &str, context: &HostContext) -> PluginResult<JsPlugin> {
        let module = Module::new("plugin.js", code);
        
        let mut runtime = JsRuntime::new(RuntimeOptions {
            // Configure runtime options as needed
            ..Default::default()
        }).map_err(|e| tosic_plugin_core::PluginError::RuntimeError(e.to_string()))?;
        
        // Register host functions from context
        self.register_host_functions(&mut runtime, context)?;
        
        runtime.load_module(&module).map_err(|e| tosic_plugin_core::PluginError::RuntimeError(e.to_string()))?;

        let plugin = JsPlugin {
            name: "inline-plugin".to_string(),
            runtime: Mutex::new(runtime)
        };
        
        Ok(plugin)
    }
}

#[cfg(not(feature = "async"))]
impl Runtime for DenoRuntime {
    fn runtime_name(&self) -> &'static str {
        "deno"
    }

    fn supports_plugin(&self, source: &PluginSource) -> bool {
        match source {
            PluginSource::FilePath(path) => {
                let path = PathBuf::from(path);
                if path.is_dir() {
                    true
                } else if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
                    matches!(extension, "js" | "ts" | "mjs" | "mts")
                } else {
                    false
                }
            },
            PluginSource::Code(_) => true, // Can handle any code string as JS
            PluginSource::Bytes(_) => false, // Cannot handle raw bytes
        }
    }

    fn load(&mut self, source: &PluginSource, context: &HostContext) -> PluginResult<Box<dyn Plugin>> {
        let plugin = match source {
            PluginSource::FilePath(path) => {
                let path = PathBuf::from(path);

                if !path.exists() {
                    return Err(tosic_plugin_core::PluginError::FileNotFound);
                }

                if path.is_file() {
                    self.load_from_file(&path, context)?
                } else if path.is_dir() {
                    self.load_from_directory(&path, context)?
                } else {
                    return Err(tosic_plugin_core::PluginError::InvalidArgumentType);
                }
            },
            PluginSource::Code(code) => {
                self.load_from_code(code, context)?
            },
            _ => return Err(tosic_plugin_core::PluginError::InvalidArgumentType),
        };

        Ok(Box::new(plugin))
    }

    fn call(&self, plugin: &dyn Plugin, function_name: &str, args: &[Value]) -> PluginResult<Value> {
        let plugin = plugin.as_any().downcast_ref::<JsPlugin>()
            .ok_or(tosic_plugin_core::PluginError::InvalidPluginState)?;
        
        let mut runtime = plugin.runtime.lock()
            .map_err(|e| tosic_plugin_core::PluginError::RuntimeError(format!("Failed to acquire runtime lock: {}", e)))?;
        
        // Convert Value to serde_json::Value properly to avoid enum serialization
        let json_args: Vec<serde_json::Value> = args.iter()
            .map(|v| v.clone().into())
            .collect();
        
        let res: serde_json::Value = runtime.call_function(None, function_name, &json_args)
            .map_err(|e| tosic_plugin_core::PluginError::RuntimeError(e.to_string()))?;
        
        Ok(res.into())
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait]
impl Runtime for DenoRuntime {
    fn runtime_name(&self) -> &'static str {
        "deno"
    }

    fn supports_plugin(&self, source: &PluginSource) -> bool {
        match source {
            PluginSource::FilePath(path) => {
                let path = PathBuf::from(path);
                if path.is_dir() {
                    true
                } else if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
                    matches!(extension, "js" | "ts" | "mjs" | "mts")
                } else {
                    false
                }
            },
            PluginSource::Code(_) => true, // Can handle any code string as JS
            PluginSource::Bytes(_) => false, // Cannot handle raw bytes
        }
    }

    async fn load(&mut self, source: &PluginSource, context: &HostContext) -> PluginResult<Box<dyn Plugin>> {
        let plugin = match source {
            PluginSource::FilePath(path) => {
                let path = PathBuf::from(path);

                if !path.exists() {
                    return Err(tosic_plugin_core::PluginError::FileNotFound);
                }

                if path.is_file() {
                    self.load_from_file(&path, context)?
                } else if path.is_dir() {
                    self.load_from_directory(&path, context)?
                } else {
                    return Err(tosic_plugin_core::PluginError::InvalidArgumentType);
                }
            },
            PluginSource::Code(code) => {
                self.load_from_code(code, context)?
            },
            _ => return Err(tosic_plugin_core::PluginError::InvalidArgumentType),
        };

        Ok(Box::new(plugin))
    }

    async fn call(&self, plugin: &dyn Plugin, function_name: &str, args: &[Value]) -> PluginResult<Value> {
        let plugin = plugin.as_any().downcast_ref::<JsPlugin>()
            .ok_or(tosic_plugin_core::PluginError::InvalidPluginState)?;
        
        let mut runtime = plugin.runtime.lock()
            .map_err(|e| tosic_plugin_core::PluginError::RuntimeError(format!("Failed to acquire runtime lock: {}", e)))?;
        
        // Convert Value to serde_json::Value properly to avoid enum serialization
        let json_args: Vec<serde_json::Value> = args.iter()
            .map(|v| v.clone().into())
            .collect();
        
        let res: serde_json::Value = runtime.call_function(None, function_name, &json_args)
            .map_err(|e| tosic_plugin_core::PluginError::RuntimeError(e.to_string()))?;
        
        Ok(res.into())
    }
}