pub mod prelude;

use std::path::PathBuf;
use std::sync::Mutex;
use tosic_plugin_core::prelude::{HostContext, Plugin, PluginResult, PluginSource, Runtime, Value};
use rustyscript::{Runtime as JsRuntime, RuntimeOptions, Module};

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

// SAFETY: JsPlugin wraps the non-Send/Sync rustyscript::Runtime in a Mutex,
// ensuring exclusive access. The JavaScript execution will always happen
// on the thread that acquires the mutex lock.
unsafe impl Send for JsPlugin {}
unsafe impl Sync for JsPlugin {}

impl Plugin for JsPlugin {
    fn name(&self) -> Option<&str> {
        Some(&self.name)
    }
}

pub struct DenoRuntime {
    plugins: Vec<JsPlugin>,
}

impl DenoRuntime {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }
    
    /// Register host functions from the HostContext with the JavaScript runtime
    fn register_host_functions(&self, runtime: &mut JsRuntime, context: &HostContext) -> PluginResult<()> {
        for function_name in context.function_names() {
            let name = function_name.clone();
            let context = context.clone(); // This is now efficient with Arc-based functions
            let name_for_closure = name.clone();
            
            // Create a wrapper function that properly handles the call
            runtime.register_function(&name, move |args: &[serde_json::Value]| {
                // Convert serde_json::Value args to tosic Value args
                let tosic_args: Vec<Value> = args.iter()
                    .map(|v| Value::from(v.clone()))
                    .collect();
                
                // Call the actual host function through the context
                match context.call_function(&name_for_closure, &tosic_args) {
                    Ok(result) => Ok(result.into()),
                    Err(e) => Err(rustyscript::Error::Runtime(format!("Host function error: {}", e))),
                }
            }).map_err(|e| tosic_plugin_core::PluginError::RuntimeError(
                format!("Failed to register host function '{}': {}", function_name, e)
            ))?;
        }
        
        Ok(())
    }
    
    pub fn get_plugin(&mut self, name: &str) -> Option<&mut JsPlugin> {
        self.plugins.iter_mut().find(|p| p.name == name)
    }
    
    pub fn call_plugin(&self, name: &str, function_name: &str, args: &[Value]) -> PluginResult<Value> {
        let plugin = self.plugins.iter().find(|p| p.name == name)
            .ok_or_else(|| tosic_plugin_core::PluginError::LoadError(format!("Plugin '{}' not found", name)))?;
        
        self.call(plugin, function_name, args)
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
}

impl Runtime for DenoRuntime {
    type Plugin = JsPlugin;

    fn load(&mut self, source: &PluginSource, context: &HostContext) -> PluginResult<()> {
        let module = match source {
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
            _ => return Err(tosic_plugin_core::PluginError::InvalidArgumentType),
        };

        self.plugins.push(module);
        
        Ok(())
    }

    fn call(&self, plugin: &Self::Plugin, function_name: &str, args: &[Value]) -> PluginResult<Value> {
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