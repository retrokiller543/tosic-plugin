//! Example demonstrating the MultiRuntimeManager.
//! 
//! This example shows how to use multiple runtimes with automatic selection.
//! Run with: cargo run --example multi_runtime
//! Run async: cargo run --example multi_runtime --features async

use tosic_plugin_core::prelude::*;
use tosic_plugin_core::managers::MultiRuntimeManager;
use tosic_plugin_deno_runtime::DenoRuntime;
use serde_json::json;

#[cfg(not(feature = "async"))]
fn main() -> PluginResult<()> {
    println!("=== Multi-Runtime Manager Example (Sync) ===\n");
    run_sync_example()
}

#[cfg(feature = "async")]
#[tokio::main]
async fn main() -> PluginResult<()> {
    println!("=== Multi-Runtime Manager Example (Async) ===\n");
    run_async_example().await
}

#[cfg(not(feature = "async"))]
fn run_sync_example() -> PluginResult<()> {
    // Create a multi-runtime manager
    let mut manager = MultiRuntimeManager::new();
    
    // Register different runtimes (currently only Deno, but you could add more)
    manager.register_runtime(Box::new(DenoRuntime::new()));
    
    println!("Registered runtimes: {:?}", manager.runtime_names());
    println!("Runtime count: {}\n", manager.runtime_count());
    
    // Create a host context (automatically includes global registry functions)
    let context = HostContext::new();
    
    // Load a JavaScript plugin - the manager will automatically select Deno runtime
    let js_source = PluginSource::FilePath("js-example".to_string());
    let js_plugin_id = manager.load_plugin(js_source, &context)?;
    
    println!("Loaded JS plugin with ID: {:?}", js_plugin_id);
    if let Some(name) = manager.plugin_name(js_plugin_id) {
        println!("JS Plugin name: {}\n", name);
    }
    
    // Test the JavaScript plugin
    println!("=== Testing JavaScript Plugin ===");
    let result = manager.call_plugin(js_plugin_id, "greet", &[])?;
    println!("JS greet() result: {:?}", result);
    
    let result = manager.call_plugin(js_plugin_id, "add", &[json!(10), json!(20)])?;
    println!("JS add(10, 20) result: {:?}\n", result);
    
    // Load an inline JavaScript plugin
    let inline_js = PluginSource::Code(r#"
        function calculate(x, y) {
            return {
                sum: x + y,
                product: x * y,
                message: `Calculated ${x} and ${y}`
            };
        }
        
        function getInfo() {
            return {
                runtime: "inline-js",
                version: "1.0",
                features: ["math", "messaging"]
            };
        }
    "#.to_string());
    
    let inline_plugin_id = manager.load_plugin(inline_js, &context)?;
    println!("Loaded inline JS plugin with ID: {:?}", inline_plugin_id);
    
    // Test the inline plugin
    println!("=== Testing Inline JavaScript Plugin ===");
    let result = manager.call_plugin(inline_plugin_id, "calculate", &[Value::Number(7.into()), Value::Number(3.into())])?;
    println!("calculate(7, 3) result: {:?}", result);
    
    let result = manager.call_plugin(inline_plugin_id, "getInfo", &[])?;
    println!("getInfo() result: {:?}\n", result);
    
    // Show manager statistics
    println!("=== Manager Statistics ===");
    println!("Total plugins loaded: {}", manager.plugin_count());
    println!("Plugin IDs: {:?}", manager.plugin_ids().collect::<Vec<_>>());
    
    // Cleanup
    manager.unload_plugin(js_plugin_id)?;
    manager.unload_plugin(inline_plugin_id)?;
    
    println!("\nAll plugins unloaded successfully!");
    println!("Final plugin count: {}", manager.plugin_count());
    
    Ok(())
}

#[cfg(feature = "async")]
async fn run_async_example() -> PluginResult<()> {
    // Create a multi-runtime manager
    let mut manager = MultiRuntimeManager::new();
    
    // Register different runtimes (currently only Deno, but you could add more)
    manager.register_runtime(Box::new(DenoRuntime::new()));
    
    println!("Registered runtimes: {:?}", manager.runtime_names());
    println!("Runtime count: {}\n", manager.runtime_count());
    
    // Create a host context (automatically includes global registry functions)
    let context = HostContext::new();
    
    // Load a JavaScript plugin - the manager will automatically select Deno runtime
    let js_source = PluginSource::FilePath("js-example".to_string());
    let js_plugin_id = manager.load_plugin(js_source, &context).await?;
    
    println!("Loaded JS plugin with ID: {:?}", js_plugin_id);
    if let Some(name) = manager.plugin_name(js_plugin_id) {
        println!("JS Plugin name: {}\n", name);
    }
    
    // Test the JavaScript plugin
    println!("=== Testing JavaScript Plugin (Async) ===");
    let result = manager.call_plugin(js_plugin_id, "greet", &[]).await?;
    println!("JS greet() result: {:?}", result);
    
    let result = manager.call_plugin(js_plugin_id, "add", &[Value::Number(10.into()), Value::Number(20.into())]).await?;
    println!("JS add(10, 20) result: {:?}\n", result);
    
    // Load an inline JavaScript plugin
    let inline_js = PluginSource::Code(r#"
        function calculate(x, y) {
            return {
                sum: x + y,
                product: x * y,
                message: `Calculated ${x} and ${y}`
            };
        }
        
        function getInfo() {
            return {
                runtime: "inline-js",
                version: "1.0",
                features: ["math", "messaging"]
            };
        }
    "#.to_string());
    
    let inline_plugin_id = manager.load_plugin(inline_js, &context).await?;
    println!("Loaded inline JS plugin with ID: {:?}", inline_plugin_id);
    
    // Test the inline plugin
    println!("=== Testing Inline JavaScript Plugin (Async) ===");
    let result = manager.call_plugin(inline_plugin_id, "calculate", &[Value::Number(7.into()), Value::Number(3.into())]).await?;
    println!("calculate(7, 3) result: {:?}", result);
    
    let result = manager.call_plugin(inline_plugin_id, "getInfo", &[]).await?;
    println!("getInfo() result: {:?}\n", result);
    
    // Show manager statistics
    println!("=== Manager Statistics ===");
    println!("Total plugins loaded: {}", manager.plugin_count());
    println!("Plugin IDs: {:?}", manager.plugin_ids().collect::<Vec<_>>());
    
    // Cleanup
    manager.unload_plugin(js_plugin_id).await?;
    manager.unload_plugin(inline_plugin_id).await?;
    
    println!("\nAll plugins unloaded successfully!");
    println!("Final plugin count: {}", manager.plugin_count());
    
    Ok(())
}