//! Example demonstrating an asynchronous plugin runtime implementation.
//!
//! This example shows how to:
//! - Implement the async Runtime and Plugin traits for a mock runtime
//! - Register host functions with automatic type conversion
//! - Load and execute plugin functions asynchronously
//! - Handle concurrent plugin operations
//! 
//! Run with: `cargo run --example async_runtime --features async`

#[cfg(feature = "async")]
use crate::plugin::*;

#[cfg(feature = "async")]
mod plugin {
    use std::collections::HashMap;
    use std::time::Duration;
    
    pub use std::sync::Arc;
    pub use tosic_plugin_core::prelude::*;

    /// Mock plugin implementation that simulates an async plugin with predefined functions.
    struct AsyncMockPlugin {
        name: String,
        functions: HashMap<String, Box<dyn Fn(&[Value]) -> PluginResult<Value> + Send + Sync>>,
    }

    impl Plugin for AsyncMockPlugin {
        fn name(&self) -> Option<&str> {
            Some(&self.name)
        }
    }

    /// Mock async runtime implementation that simulates plugin loading and execution.
    #[derive(Default)]
    struct AsyncMockRuntime {
        name: String,
    }

    impl AsyncMockRuntime {
        fn new(name: impl Into<String>) -> Self {
            Self {
                name: name.into(),
            }
        }
    }

    #[async_trait::async_trait]
    impl Runtime for AsyncMockRuntime {
        type Plugin = AsyncMockPlugin;

        async fn load(&self, bytes: &[u8], _context: &HostContext) -> PluginResult<Self::Plugin> {
            // Simulate async plugin loading (e.g., network fetch, compilation, etc.)
            println!("[{}] Starting async plugin load from {} bytes...", self.name, bytes.len());

            // Simulate some async work
            tokio::time::sleep(Duration::from_millis(100)).await;

            let plugin_code = String::from_utf8_lossy(bytes);
            println!("[{}] Loaded plugin code: {}", self.name, plugin_code);

            // Create a mock plugin with some async-aware functions
            let mut functions: HashMap<String, Box<dyn Fn(&[Value]) -> PluginResult<Value> + Send + Sync>> = HashMap::new();

            // Add an async "add" function (simulated)
            functions.insert("add".to_string(), Box::new(|args: &[Value]| -> PluginResult<Value> {
                if args.len() != 2 {
                    return Err(PluginError::InvalidArgumentType);
                }

                let a = args[0].as_int().ok_or(PluginError::InvalidArgumentType)?;
                let b = args[1].as_int().ok_or(PluginError::InvalidArgumentType)?;

                // Simulate some computation
                println!("[PLUGIN] Async computing {} + {}", a, b);
                Ok(Value::Int(a + b))
            }));

            // Add a "fetch_data" function that simulates async I/O
            functions.insert("fetch_data".to_string(), Box::new(|args: &[Value]| -> PluginResult<Value> {
                if args.len() != 1 {
                    return Err(PluginError::InvalidArgumentType);
                }

                let url = args[0].as_string().ok_or(PluginError::InvalidArgumentType)?;

                // Simulate async data fetching
                println!("[PLUGIN] Simulating async fetch from: {}", url);
                Ok(Value::String(format!("Data from {}", url)))
            }));

            // Add a "process_batch" function that works with arrays
            functions.insert("process_batch".to_string(), Box::new(|args: &[Value]| -> PluginResult<Value> {
                if args.len() != 1 {
                    return Err(PluginError::InvalidArgumentType);
                }

                let array = args[0].as_array().ok_or(PluginError::InvalidArgumentType)?;

                // Process each item (simulate async work per item)
                let mut results = Vec::new();
                for (i, item) in array.iter().enumerate() {
                    if let Some(num) = item.as_int() {
                        println!("[PLUGIN] Processing item {}: {}", i, num);
                        results.push(Value::Int(num * 2));
                    } else {
                        results.push(item.clone());
                    }
                }

                Ok(Value::Array(results))
            }));

            println!("[{}] Plugin loaded successfully with {} functions", self.name, functions.len());

            Ok(AsyncMockPlugin {
                name: format!("async-mock-plugin-{}", self.name),
                functions,
            })
        }

        async fn call(
            &self,
            plugin: &Self::Plugin,
            function_name: &str,
            args: &[Value],
        ) -> PluginResult<Value> {
            println!("[{}] Async calling function '{}' with {} arguments",
                     self.name, function_name, args.len());

            // Simulate async function call overhead
            tokio::time::sleep(Duration::from_millis(10)).await;

            match plugin.functions.get(function_name) {
                Some(func) => {
                    let result = func(args)?;
                    println!("[{}] Function '{}' completed", self.name, function_name);
                    Ok(result)
                },
                None => Err(PluginError::FunctionNotFound(function_name.to_string())),
            }
        }
    }
}

#[cfg(feature = "async")]
#[tokio::main]
async fn main() -> PluginResult<()> {
    println!("=== Asynchronous Plugin Runtime Example ===\n");
    
    // Create a host context and register some host functions
    let mut host_context = HostContext::new();
    
    // Register async-friendly host functions
    host_context.register("log_async", |level: String, message: String| {
        println!("[HOST LOG:{}] {}", level, message);
        format!("logged: {}", message)
    });
    
    // Register a computation function
    host_context.register("compute_hash", |data: String| -> i64 {
        // Simulate hash computation
        let hash = data.len() as i64 * 31;
        println!("[HOST] Computing hash for '{}': {}", data, hash);
        hash
    });
    
    // Register a batch processing function
    host_context.register("validate_batch", |items: Vec<Value>| -> bool {
        println!("[HOST] Validating batch of {} items", items.len());
        // All items must be non-null for validation to pass
        items.iter().all(|item| !item.is_null())
    });
    
    println!("Registered host functions: {:?}\n", 
             host_context.function_names().collect::<Vec<_>>());
    
    // Create multiple runtimes to demonstrate concurrent execution
    let runtime1 = Arc::new(AsyncMockRuntime::new("Runtime-1"));
    let runtime2 = Arc::new(AsyncMockRuntime::new("Runtime-2"));
    
    // Simulate different plugin codes
    let plugin_code1 = b"async plugin v1.0 - math operations";
    let plugin_code2 = b"async plugin v2.0 - data processing";
    
    println!("=== Loading Plugins Concurrently ===\n");
    
    // Load plugins concurrently
    let (plugin1, plugin2) = tokio::try_join!(
        runtime1.load(plugin_code1, &host_context),
        runtime2.load(plugin_code2, &host_context)
    )?;
    
    let plugin1 = Arc::new(plugin1);
    let plugin2 = Arc::new(plugin2);
    
    println!("Loaded plugins: '{}' and '{}'\n", 
             plugin1.name().unwrap_or("unknown"), 
             plugin2.name().unwrap_or("unknown"));
    
    // Test concurrent plugin function calls
    println!("=== Testing Concurrent Plugin Function Calls ===\n");
    
    // Execute multiple operations concurrently
    let operations = vec![
        tokio::spawn({
            let runtime = Arc::clone(&runtime1);
            let plugin = Arc::clone(&plugin1);
            async move {
                println!("Task 1: Starting add operation");
                let result = runtime.call(&*plugin, "add", &[Value::Int(10), Value::Int(20)]).await;
                println!("Task 1: Add completed");
                result
            }
        }),
        
        tokio::spawn({
            let runtime = Arc::clone(&runtime2);
            let plugin = Arc::clone(&plugin2);
            async move {
                println!("Task 2: Starting fetch_data operation");
                let result = runtime.call(&*plugin, "fetch_data", &[Value::String("https://api.example.com/data".to_string())]).await;
                println!("Task 2: Fetch completed");
                result
            }
        }),
        
        tokio::spawn({
            let runtime = Arc::clone(&runtime1);
            let plugin = Arc::clone(&plugin1);
            async move {
                println!("Task 3: Starting batch processing");
                let batch = vec![Value::Int(1), Value::Int(2), Value::Int(3), Value::Int(4)];
                let result = runtime.call(&*plugin, "process_batch", &[Value::Array(batch)]).await;
                println!("Task 3: Batch processing completed");
                result
            }
        }),
    ];
    
    // Wait for all operations to complete
    let results = futures::future::try_join_all(operations).await;
    
    match results {
        Ok(plugin_results) => {
            for (i, result) in plugin_results.into_iter().enumerate() {
                match result {
                    Ok(value) => println!("Operation {}: Success - {:?}", i + 1, value),
                    Err(e) => println!("Operation {}: Error - {}", i + 1, e),
                }
            }
        }
        Err(e) => println!("Task execution error: {}", e),
    }
    
    // Test host function calls
    println!("\n=== Testing Host Function Calls ===\n");
    
    let host_results = vec![
        host_context.call_function("log_async", &[Value::String("INFO".to_string()), Value::String("System ready".to_string())]),
        host_context.call_function("compute_hash", &[Value::String("test_data".to_string())]),
        host_context.call_function("validate_batch", &[Value::Array(vec![Value::Int(1), Value::String("test".to_string()), Value::Bool(true)])]),
    ];
    
    for (i, result) in host_results.into_iter().enumerate() {
        match result {
            Ok(value) => println!("Host function {}: {:?}", i + 1, value),
            Err(e) => println!("Host function {}: Error - {}", i + 1, e),
        }
    }
    
    // Test error handling in async context
    println!("\n=== Testing Error Handling ===\n");
    
    println!("Testing invalid function call...");
    match runtime1.call(&plugin1, "nonexistent", &[]).await {
        Ok(_) => println!("Unexpected success!"),
        Err(e) => println!("Expected error: {}", e),
    }
    
    println!("\nTesting invalid arguments...");
    match runtime2.call(&plugin2, "add", &[Value::String("not a number".to_string())]).await {
        Ok(_) => println!("Unexpected success!"),
        Err(e) => println!("Expected error: {}", e),
    }
    
    println!("\n=== Async Example completed successfully! ===");
    Ok(())
}

#[cfg(not(feature = "async"))]
fn main() {
    panic!("This example requires the 'async' feature to be enabled. Please run with `--features async`.");
}