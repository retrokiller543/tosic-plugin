use tosic_plugin_core::{PluginResult, PluginSource, Runtime, Value};
use tosic_plugin_deno_runtime::DenoRuntime;
use serde_json::json;

fn main() -> PluginResult<()> {
    let mut runtime = DenoRuntime::new();
    
    let source = PluginSource::FilePath("/Users/emil/RustroverProjects/tosic-plugin/crates/tosic-plugin-deno-runtime/js-example".to_string());
    
    // Create a host context and register some host functions
    let mut context = tosic_plugin_core::HostContext::new();
    context.register("hostAdd", |a: i64, b: i64| a + b);
    context.register("hostGreet", |name: String| format!("Hello from Rust, {}!", name));
    context.register("hostGetTime", || {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    });
    
    runtime.load(&source, &context)?;
    
    println!("=== Testing JavaScript Plugin Functions ===\n");
    
    // Test 1: Function without arguments
    println!("1. Testing greet() - no arguments:");
    let result = runtime.call_plugin("js-example", "greet", &[])?;
    println!("   Result: {:?}\n", result);
    
    // Test 2: Function with string argument
    println!("2. Testing greetPerson(name) - string argument:");
    let name_arg = Value::String("Rust Developer".to_string());
    let result = runtime.call_plugin("js-example", "greetPerson", &[name_arg])?;
    println!("   Result: {:?}\n", result);
    
    // Test 3: Function with number arguments
    println!("3. Testing add(a, b) - number arguments:");
    let num1 = Value::Float(42.0);
    let num2 = Value::Float(58.0);
    let result = runtime.call_plugin("js-example", "add", &[num1, num2])?;
    println!("   Result: {:?}\n", result);
    
    // Test 4: Function with mixed types
    println!("4. Testing processData(name, age, isActive) - mixed types:");
    let name = Value::String("Alice".to_string());
    let age = Value::Int(30);
    let is_active = Value::Bool(true);
    let result = runtime.call_plugin("js-example", "processData", &[name, age, is_active])?;
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
    let result = runtime.call_plugin("js-example", "analyzeObject", &[obj_arg])?;
    println!("   Result: {:?}\n", result);
    
    // Test 6: Function with array argument
    println!("6. Testing sumArray(numbers) - array argument:");
    let array_data = json!([1, 2, 3, 4, 5]);
    let array_arg = Value::from(array_data);
    let result = runtime.call_plugin("js-example", "sumArray", &[array_arg])?;
    println!("   Result: {:?}\n", result);
    
    // Test 7: Legacy load function
    println!("7. Testing load() - legacy function:");
    let result = runtime.call_plugin("js-example", "load", &[])?;
    println!("   Result: {:?}\n", result);
    
    // Test 8: Host function integration
    println!("8. Testing host function integration from JavaScript:");
    let result = runtime.call_plugin("js-example", "testHostFunctions", &[])?;
    println!("   Result: {:?}\n", result);
    
    println!("=== All tests completed successfully! ===");
    
    Ok(())
}