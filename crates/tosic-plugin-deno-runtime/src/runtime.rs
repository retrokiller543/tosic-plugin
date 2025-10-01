use crate::plugin::DenoPlugin;
use glob::glob;
use rustyscript::Runtime as JsRuntime;
use rustyscript::Module;
use std::path::PathBuf;
use tosic_plugin_core::managers::SingleRuntimeManager;
use tosic_plugin_core::prelude::*;

pub type DenoPluginManager = SingleRuntimeManager<DenoRuntime>;

pub struct DenoRuntime;

macro_rules! extensions {
    ($first:literal) => {
        const SUPPORTED_EXTENSIONS: &[&str] = &[$first];
        const SUPPORTED_PATTERN: &str = concat!("*.", $first);
    };

    ($first:literal $(, $rest:literal)* $(,)?) => {
        const SUPPORTED_EXTENSIONS: &[&str] = &[$first, $($rest),*];
        const SUPPORTED_PATTERN: &str = concat!("*.{", $first $(, ",", $rest)*, "}");
    };
}

impl DenoRuntime {
    extensions!("ts", "js");

    pub fn new() -> Self {
        Self
    }

    fn get_module(&self, source: &PluginSource) -> PluginResult<Module> {
        debug_assert!(self.supports_plugin(source), "Plugin source not supported by DenoRuntime");

        match source {
            PluginSource::Code(code) => Ok(Module::new("plugin.js", code)),
            PluginSource::Bytes(bytes) => {
                let code = String::from_utf8_lossy(bytes);
                Ok(Module::new("plugin.js", &code))
            }
            PluginSource::FilePath(path) => {
                let path = PathBuf::from(path);
                
                if path.is_file() {
                    Module::load(&path)
                        .map_err(|e| PluginError::LoadError(format!("Failed to load module from file: {}", e)))
                } else if path.is_dir() {
                    let pattern = Self::build_glob_pattern(&path);
                    let entry_files: Vec<_> = glob(&pattern)
                        .map_err(|e| PluginError::LoadError(format!("Failed to glob pattern: {}", e)))?
                        .filter_map(Result::ok)
                        .filter(|p| p.is_file())
                        .collect();
                    
                    if entry_files.is_empty() {
                        return Err(PluginError::LoadError("No supported files found in directory".into()));
                    }
                    
                    // For directories, try to find index file or use the first file
                    let entry_file = entry_files.iter()
                        .find(|p| {
                            p.file_stem()
                                .and_then(|stem| stem.to_str())
                                .map(|stem| stem == "index")
                                .unwrap_or(false)
                        })
                        .unwrap_or(&entry_files[0]);
                    
                    Module::load(entry_file)
                        .map_err(|e| PluginError::LoadError(format!("Failed to load module from directory: {}", e)))
                } else {
                    Err(PluginError::LoadError("Path is neither a file nor a directory".into()))
                }
            }
        }
    }

    fn validate(code: &str) -> bool {
        rustyscript::validate(code).unwrap_or(false)
    }

    fn validate_bytes(bytes: &[u8]) -> bool {
        let code = String::from_utf8_lossy(bytes);
        Self::validate(&code)
    }

    fn build_glob_pattern(dir: &PathBuf) -> String {
        dir.join(Self::SUPPORTED_PATTERN)
            .to_string_lossy()
            .to_string()
    }

    fn matches_supported_extension(path: &PathBuf) -> bool {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some(ext) => Self::SUPPORTED_EXTENSIONS.contains(&ext),
            None => false,
        }
    }

    pub(crate) fn register_host_capabilities(runtime: &mut JsRuntime, context: &HostContext) -> PluginResult<()> {
        for (name, func) in context.functions() {
            match func {
                HostFunctionType::Sync(func) => {
                    let func = func.clone();
                    
                    let js_func = move |args: &[serde_json::Value]| {
                        let result = func(args);
                        
                        result
                            .map_err(|error| rustyscript::Error::Runtime(error.to_string()))
                    };
                    
                    runtime.register_function(name, js_func)
                        .map_err(|e| PluginError::RuntimeError(format!("Failed to register function '{}': {}", name, e)))?;
                }
                #[cfg(feature = "async")]
                HostFunctionType::Async(func) => {
                    let func = func.clone();
                    
                    let js_func = move |args: Vec<serde_json::Value>| -> std::pin::Pin<Box<dyn Future<Output = Result<serde_json::Value, rustyscript::Error>> + 'static>> {
                        let func = func.clone();
                        Box::pin(async move {
                            let res = func(&args).await;
                            res.map_err(|error| rustyscript::Error::Runtime(error.to_string()))
                        })
                    };
                    
                    runtime.register_async_function(name, js_func)
                        .map_err(|e| PluginError::RuntimeError(format!("Failed to register async function '{}': {}", name, e)))?;
                }
            }
        }

        Ok(())
    }
}

#[cfg_attr(feature = "async", async_trait::async_trait)]
impl Runtime for DenoRuntime {
    fn runtime_name(&self) -> &'static str {
        "deno"
    }

    fn supports_plugin(&self, source: &PluginSource) -> bool {
        match source {
            PluginSource::Code(code) => Self::validate(code),
            PluginSource::Bytes(bytes) => Self::validate_bytes(bytes),
            PluginSource::FilePath(path) => {
                let path = PathBuf::from(path);

                if path.is_dir() {
                    let pattern = Self::build_glob_pattern(&path);
                    return glob(&pattern)
                        .map(|paths| paths.filter_map(Result::ok).any(|p| p.is_file()))
                        .unwrap_or(false);
                }

                Self::matches_supported_extension(&path)
            }
        }
    }

    #[cfg(not(feature = "async"))]
    fn load(&mut self, source: &PluginSource, context: &HostContext) -> PluginResult<Box<dyn Plugin>> {
        let module = self.get_module(source)?;
        let mut runtime = RuntimeBuilder::new()
            .build()
            .map_err(|error| PluginError::LoadError(format!("Failed to build runtime: {}", error)))?;

        Self::register_host_capabilities(&mut runtime, context)?;

        if let PluginSource::FilePath(path) = source {
            runtime.set_current_dir(path)
                .map_err(|error| PluginError::LoadError(format!("Failed to set current directory: {}", error)))?;
        }

        runtime.load_module(&module)
            .map_err(|error| PluginError::LoadError(format!("Failed to load module: {}", error)))?;

        Ok(Box::new(DenoPlugin::new(runtime)))
    }

    #[cfg(feature = "async")]
    async fn load(&mut self, source: &PluginSource, context: &HostContext) -> PluginResult<Box<dyn Plugin>> {
        let module = self.get_module(source)?;
        
        let path = if let PluginSource::FilePath(path) = source {
            Some(path.to_string())
        } else {
            None
        };

        Ok(Box::new(DenoPlugin::new(module, context.clone(), path)))
    }

    #[cfg(not(feature = "async"))]
    fn call(&self, plugin: &mut dyn Plugin, function_name: &str, args: &[Value]) -> PluginResult<Value> {
        let plugin = plugin
            .as_any_mut()
            .downcast_mut::<DenoPlugin>()
            .ok_or_else(|| PluginError::RuntimeError("Invalid plugin type for DenoRuntime".into()))?;

        let runtime = plugin.runtime_mut();

        runtime.call_function::<Value>(None, function_name, &args)
            .map_err(|error| PluginError::RuntimeError(format!("Failed to call function '{}': {}", function_name, error)))
    }

    #[cfg(feature = "async")]
    async fn call(&self, plugin: &mut dyn Plugin, function_name: &str, args: &[Value]) -> PluginResult<Value> {
        let plugin = plugin
            .as_any_mut()
            .downcast_mut::<DenoPlugin>()
            .ok_or_else(|| PluginError::RuntimeError("Invalid plugin type for DenoRuntime".into()))?;

        plugin.call_function(function_name, args).await
    }
}
