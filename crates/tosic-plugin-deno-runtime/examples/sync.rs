use tosic_plugin_core::PluginResult;

#[cfg(not(feature = "async"))]
fn main() -> PluginResult<()> {
    plugin::run_plugins()
}

#[cfg(feature = "async")]
fn main() {
    eprintln!("This example requires the `async` feature");
}

#[cfg(not(feature = "async"))]
mod plugin {
    use tosic_plugin_core::prelude::*;
    use tosic_plugin_deno_runtime::prelude::*;
    
    const PLUGIN_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/js-example");
    
    pub fn run_plugins() -> PluginResult<()> {
        let runtime = DenoRuntime::new();
        let mut manager = DenoPluginManager::new(runtime);
        
        let plugin1_id = manager.load_plugin(PluginSource::FilePath(PLUGIN_PATH.to_string()), &HostContext::default())?;
        
        let result1 = manager.call_plugin(plugin1_id, "add", (2, 3))?;
        
        println!("Result of add(2, 3): {}", result1);
        
        let result2 = manager.call_plugin(plugin1_id, "greet", ("World",))?;
        println!("Result of greet('World'): {}", result2);
        
        manager.unload_plugin(plugin1_id)?;
        
        Ok(())
    }
}