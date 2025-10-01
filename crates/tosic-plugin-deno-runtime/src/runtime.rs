use std::path::PathBuf;
use glob::glob;
use rustyscript::{Module, RuntimeBuilder};
use tosic_plugin_core::managers::SingleRuntimeManager;
use tosic_plugin_core::prelude::*;
use rustyscript::Runtime as JsRuntime;
use crate::plugin::DenoPlugin;

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

        Err(PluginError::RuntimeError("Deno plugin loading not implemented".into()))
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

    fn register_host_capabilities(&self, runtime: &mut JsRuntime, context: &HostContext) -> PluginResult<()> {
        for (name, func) in context.functions() {
            let func = func.clone();

            runtime.register_function(name, move |args| {
                let tosic_args: Vec<Value> = args.iter()
                    .map(|v| Value::from(v.clone()))
                    .collect();

                let res = (func)(&tosic_args).unwrap();

                Ok(res.into())
            }).map_err(|error| PluginError::LoadError(format!("Failed to register host function '{}': {}", name, error)))?;
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

        self.register_host_capabilities(&mut runtime, context)?;

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
        todo!()
    }

    #[cfg(not(feature = "async"))]
    fn call(&self, plugin: &dyn Plugin, function_name: &str, args: &[Value]) -> PluginResult<Value> {
        let plugin = plugin
            .as_any_mut()
            .downcast_mut::<DenoPlugin>()
            .ok_or_else(|| PluginError::RuntimeError("Invalid plugin type for DenoRuntime".into()))?;

        let runtime = plugin.runtime_mut();

        runtime.call_function::<serde_json::Value>(None, function_name, &args)
            .map(Into::into)
            .map_err(|error| PluginError::RuntimeError(format!("Failed to call function '{}': {}", function_name, error)))
    }

    #[cfg(feature = "async")]
    async fn call(&self, plugin: &dyn Plugin, function_name: &str, args: &[Value]) -> PluginResult<Value> {
        todo!()
    }
}
