//! Example demonstrating the new plugin manager API.
//! 
//! This example works with both sync and async features.
//! Run with: cargo run --example example
//! Run async: cargo run --example example --features async

use tosic_plugin_core::prelude::*;
use tosic_plugin_core::managers::SingleRuntimeManager;
use tosic_plugin_deno_runtime::DenoRuntime;
use serde_json::json;

#[cfg(not(feature = "async"))]
fn main() -> PluginResult<()> {
    println!("=== Sync Plugin Manager Example ===\n");
    run_sync_example()
}

#[cfg(feature = "async")]
#[tokio::main]
async fn main() -> PluginResult<()> {
    println!("=== Async Plugin Manager Example ===\n");
    run_async_example().await
}

#[cfg(not(feature = "async"))]
fn run_sync_example() -> PluginResult<()> {
    // Create a plugin manager with Deno runtime
    let mut manager = SingleRuntimeManager::new(DenoRuntime::new());
    
    // Create a host context and register some host functions
    let mut context = HostContext::new();
    context.register("hostAdd", |a: i64, b: i64| a + b);
    context.register("hostGreet", |name: String| format!("Hello from Rust, {}!", name));
    context.register("hostGetTime", || {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    });
    
    // Load plugin using the new API
    let source = PluginSource::FilePath("/Users/emil/RustroverProjects/tosic-plugin/crates/tosic-plugin-deno-runtime/js-example".to_string());
    let plugin_id = manager.load_plugin(source, &context)?;
    
    println!("Loaded plugin with ID: {:?}", plugin_id);
    if let Some(name) = manager.plugin_name(plugin_id) {
        println!("Plugin name: {}\n", name);
    }
    
    // Run tests
    run_plugin_tests(&manager, plugin_id)?;
    
    // Cleanup
    manager.unload_plugin(plugin_id)?;
    println!("Plugin unloaded successfully!");
    
    Ok(())
}

#[cfg(feature = "async")]
async fn run_async_example() -> PluginResult<()> {
    // Create a plugin manager with Deno runtime
    let mut manager = SingleRuntimeManager::new(DenoRuntime::new());
    
    // Create a host context and register some host functions
    let mut context = HostContext::new();
    context.register("hostAdd", |a: i64, b: i64| a + b);
    context.register("hostGreet", |name: String| format!("Hello from Rust, {}!", name));
    context.register("hostGetTime", || {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    });
    
    // Load plugin using the new API
    let source = PluginSource::FilePath("/Users/emil/RustroverProjects/tosic-plugin/crates/tosic-plugin-deno-runtime/js-example".to_string());
    let plugin_id = manager.load_plugin(source, &context).await?;
    
    println!("Loaded plugin with ID: {:?}", plugin_id);
    if let Some(name) = manager.plugin_name(plugin_id) {
        println!("Plugin name: {}\n", name);
    }
    
    // Run tests
    run_plugin_tests_async(&manager, plugin_id).await?;
    
    // Cleanup
    manager.unload_plugin(plugin_id).await?;
    println!("Plugin unloaded successfully!");
    
    Ok(())
}

#[cfg(not(feature = "async"))]
fn run_plugin_tests(manager: &SingleRuntimeManager<DenoRuntime>, plugin_id: PluginId) -> PluginResult<()> {
    println!("=== Testing JavaScript Plugin Functions ===\n");
    
    // Test 1: Function without arguments
    println!("1. Testing greet() - no arguments:");
    let result = manager.call_plugin(plugin_id, "greet", &[])?;
    println!("   Result: {:?}\n", result);
    
    // Test 2: Function with string argument
    println!("2. Testing greetPerson(name) - string argument:");
    let name_arg = Value::String("Rust Developer".to_string());
    let result = manager.call_plugin(plugin_id, "greetPerson", &[name_arg])?;
    println!("   Result: {:?}\n", result);
    
    // Test 3: Function with number arguments
    println!("3. Testing add(a, b) - number arguments:");
    let num1 = Value::Float(42.0);
    let num2 = Value::Float(58.0);
    let result = manager.call_plugin(plugin_id, "add", &[num1, num2])?;
    println!("   Result: {:?}\n", result);
    
    // Test 4: Function with mixed types
    println!("4. Testing processData(name, age, isActive) - mixed types:");
    let name = Value::String("Alice".to_string());
    let age = Value::Int(30);
    let is_active = Value::Bool(true);
    let result = manager.call_plugin(plugin_id, "processData", &[name, age, is_active])?;
    println!("   Result: {:?}\n", result);
    
    // Test 5: Function with object argument
    println!("5. Testing analyzeObject(obj) - object argument:");
    let obj_data = json!({
        "name": "Test Object",
        "type": "example",
        "count": 42,
        "enabled": true
    });
    let obj_arg = Value::from(obj_data);
    let result = manager.call_plugin(plugin_id, "analyzeObject", &[obj_arg])?;
    println!("   Result: {:?}\n", result);
    
    // Test 6: Function with array argument
    println!("6. Testing sumArray(numbers) - array argument:");
    let array_data = json!([1, 2, 3, 4, 5]);
    let array_arg = Value::from(array_data);
    let result = manager.call_plugin(plugin_id, "sumArray", &[array_arg])?;
    println!("   Result: {:?}\n", result);
    
    // Test 7: Host function integration
    println!("7. Testing host function integration from JavaScript:");
    let result = manager.call_plugin(plugin_id, "testHostFunctions", &[])?;
    println!("   Result: {:?}\n", result);
    
    println!("=== All sync tests completed successfully! ===");
    Ok(())
}

#[cfg(feature = "async")]
async fn run_plugin_tests_async(manager: &SingleRuntimeManager<DenoRuntime>, plugin_id: PluginId) -> PluginResult<()> {
    println!("=== Testing JavaScript Plugin Functions (Async) ===\n");
    
    // Test 1: Function without arguments
    println!("1. Testing greet() - no arguments:");
    let result = manager.call_plugin(plugin_id, "greet", &[]).await?;
    println!("   Result: {:?}\n", result);
    
    // Test 2: Function with string argument
    println!("2. Testing greetPerson(name) - string argument:");
    let name_arg = Value::String("Rust Developer".to_string());
    let result = manager.call_plugin(plugin_id, "greetPerson", &[name_arg]).await?;
    println!("   Result: {:?}\n", result);
    
    // Test 3: Function with number arguments
    println!("3. Testing add(a, b) - number arguments:");
    let num1 = Value::Float(42.0);
    let num2 = Value::Float(58.0);
    let result = manager.call_plugin(plugin_id, "add", &[num1, num2]).await?;
    println!("   Result: {:?}\n", result);
    
    // Test 4: Function with mixed types
    println!("4. Testing processData(name, age, isActive) - mixed types:");
    let name = Value::String("Alice".to_string());
    let age = Value::Int(30);
    let is_active = Value::Bool(true);
    let result = manager.call_plugin(plugin_id, "processData", &[name, age, is_active]).await?;
    println!("   Result: {:?}\n", result);
    
    // Test 5: Function with object argument
    println!("5. Testing analyzeObject(obj) - object argument:");
    let obj_data = json!({
        "name": "Test Object",
        "type": "example",
        "count": 42,
        "enabled": true
    });
    let obj_arg = Value::from(obj_data);
    let result = manager.call_plugin(plugin_id, "analyzeObject", &[obj_arg]).await?;
    println!("   Result: {:?}\n", result);
    
    // Test 6: Function with array argument
    println!("6. Testing sumArray(numbers) - array argument:");
    let array_data = json!([1, 2, 3, 4, 5]);
    let array_arg = Value::from(array_data);
    let result = manager.call_plugin(plugin_id, "sumArray", &[array_arg]).await?;
    println!("   Result: {:?}\n", result);
    
    // Test 7: Host function integration
    println!("7. Testing host function integration from JavaScript:");
    let result = manager.call_plugin(plugin_id, "testHostFunctions", &[]).await?;
    println!("   Result: {:?}\n", result);
    
    println!("=== All async tests completed successfully! ===");
    Ok(())
}