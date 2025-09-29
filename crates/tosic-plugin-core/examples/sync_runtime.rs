//! Example demonstrating a synchronous plugin runtime implementation.
//!
//! This example shows how to:
//! - Implement the Runtime and Plugin traits for a mock runtime
//! - Register host functions with automatic type conversion
//! - Load and execute plugin functions
//! 
//! Run with: `cargo run --example sync_runtime`

#[cfg(feature = "async")]
compile_error!("This example requires the 'async' feature to be disabled.");

use std::collections::HashMap;
use tosic_plugin_core::*;

/// Mock plugin implementation that simulates a simple plugin with predefined functions.
struct MockPlugin {
    name: String,
    functions: HashMap<String, Box<dyn Fn(&[Value]) -> PluginResult<Value> + Send + Sync>>,
}

impl Plugin for MockPlugin {
    fn name(&self) -> Option<&str> {
        Some(&self.name)
    }
}

/// Mock runtime implementation that simulates plugin loading and execution.
#[derive(Default)]
struct MockRuntime {}

impl MockRuntime {
    fn new() -> Self {
        Self::default()
    }
}

impl Runtime for MockRuntime {
    type Plugin = MockPlugin;

    fn load(&self, bytes: &[u8], _context: &HostContext) -> PluginResult<Self::Plugin> {
        // Simulate plugin loading from bytes
        let plugin_code = String::from_utf8_lossy(bytes);
        println!("Loading plugin from {} bytes: {}", bytes.len(), plugin_code);
        
        // Create a mock plugin with some predefined functions
        let mut functions: HashMap<String, Box<dyn Fn(&[Value]) -> PluginResult<Value> + Send + Sync>> = HashMap::new();
        
        // Add a simple "add" function
        functions.insert("add".to_string(), Box::new(|args: &[Value]| -> PluginResult<Value> {
            if args.len() != 2 {
                return Err(PluginError::InvalidArgumentType);
            }
            
            let a = args[0].as_int().ok_or(PluginError::InvalidArgumentType)?;
            let b = args[1].as_int().ok_or(PluginError::InvalidArgumentType)?;
            
            Ok(Value::Int(a + b))
        }));
        
        // Add a "greet" function
        functions.insert("greet".to_string(), Box::new(|args: &[Value]| -> PluginResult<Value> {
            if args.len() != 1 {
                return Err(PluginError::InvalidArgumentType);
            }
            
            let name = args[0].as_string().ok_or(PluginError::InvalidArgumentType)?;
            
            // Simulate plugin logging (in a real implementation, this would call host functions)
            println!("[PLUGIN LOG] Plugin is greeting: {}", name);
            
            Ok(Value::String(format!("Hello from plugin, {}!", name)))
        }));
        
        Ok(MockPlugin {
            name: "mock-plugin".to_string(),
            functions,
        })
    }

    fn call(
        &self,
        plugin: &Self::Plugin,
        function_name: &str,
        args: &[Value],
    ) -> PluginResult<Value> {
        println!("Calling function '{}' with {} arguments", function_name, args.len());
        
        match plugin.functions.get(function_name) {
            Some(func) => func(args),
            None => Err(PluginError::FunctionNotFound(function_name.to_string())),
        }
    }
}

fn main() -> PluginResult<()> {
    println!("=== Synchronous Plugin Runtime Example ===\n");
    
    // Create a host context and register some host functions
    let mut host_context = HostContext::new();
    
    // Register a logging function that the plugin can call
    host_context.register("log", |message: String| {
        println!("[HOST LOG] {}", message);
    });
    
    // Register a math utility function
    host_context.register("multiply", |a: i64, b: i64| -> i64 {
        println!("[HOST] Multiplying {} * {}", a, b);
        a * b
    });
    
    // Register a function that returns no value
    host_context.register("ping", || {
        println!("[HOST] Ping received!");
        // Returns () which converts to Value::Null
    });
    
    println!("Registered host functions: {:?}\n", 
             host_context.function_names().collect::<Vec<_>>());
    
    // Create the runtime
    let runtime = MockRuntime::new();
    
    // Simulate plugin code (in a real implementation, this would be WASM, JS, etc.)
    let plugin_code = b"mock plugin code with add and greet functions";
    
    // Load the plugin
    println!("Loading plugin...");
    let plugin = runtime.load(plugin_code, &host_context)?;
    println!("Loaded plugin: {:?}\n", plugin.name());
    
    // Test calling plugin functions
    println!("=== Testing Plugin Function Calls ===\n");
    
    // Test the "add" function
    println!("1. Calling add(5, 3):");
    let result = runtime.call(&plugin, "add", &[Value::Int(5), Value::Int(3)])?;
    println!("   Result: {:?}\n", result);
    
    // Test the "greet" function (which calls host functions)
    println!("2. Calling greet('World'):");
    let result = runtime.call(&plugin, "greet", &[Value::String("World".to_string())])?;
    println!("   Result: {:?}\n", result);
    
    // Test calling host functions directly
    println!("=== Testing Host Function Calls ===\n");
    
    println!("3. Calling host function multiply(7, 6):");
    let result = host_context.call_function("multiply", &[Value::Int(7), Value::Int(6)])?;
    println!("   Result: {:?}\n", result);
    
    println!("4. Calling host function ping():");
    let result = host_context.call_function("ping", &[])?;
    println!("   Result: {:?}\n", result);
    
    // Test error cases
    println!("=== Testing Error Cases ===\n");
    
    println!("5. Calling non-existent function:");
    match runtime.call(&plugin, "nonexistent", &[]) {
        Ok(_) => println!("   Unexpected success!"),
        Err(e) => println!("   Expected error: {}", e),
    }
    
    println!("\n6. Calling add with wrong number of arguments:");
    match runtime.call(&plugin, "add", &[Value::Int(1)]) {
        Ok(_) => println!("   Unexpected success!"),
        Err(e) => println!("   Expected error: {}", e),
    }
    
    println!("\n=== Example completed successfully! ===");
    Ok(())
}