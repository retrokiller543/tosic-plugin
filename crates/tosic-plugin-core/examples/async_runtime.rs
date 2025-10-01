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
use std::sync::Arc;

#[cfg(feature = "async")]
mod plugin {
    use std::collections::HashMap;
    use std::time::Duration;
    
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

        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    /// Mock async runtime implementation that simulates plugin loading and execution.
    #[derive(Default)]
    pub struct AsyncMockRuntime {
        name: String,
    }

    impl AsyncMockRuntime {
        pub fn new(name: impl Into<String>) -> Self {
            Self {
                name: name.into(),
            }
        }
    }

    #[async_trait::async_trait]
    impl Runtime for AsyncMockRuntime {
        fn runtime_name(&self) -> &'static str {
            "async-mock"
        }

        fn supports_plugin(&self, source: &PluginSource) -> bool {
            match source {
                PluginSource::Code(_) => true,
                PluginSource::Bytes(_) => true,
                PluginSource::FilePath(path) => path.ends_with(".async"),
            }
        }

        async fn load(&mut self, source: &PluginSource, _context: &HostContext) -> PluginResult<Box<dyn Plugin>> {
            // Simulate async plugin loading (e.g., network fetch, compilation, etc.)
            let plugin_code = match source {
                PluginSource::Code(code) => code.clone(),
                PluginSource::Bytes(bytes) => String::from_utf8_lossy(bytes).to_string(),
                PluginSource::FilePath(path) => format!("async plugin from {}", path),
            };
            println!("[{}] Starting async plugin load: {}", self.name, plugin_code);

            // Simulate some async work
            tokio::time::sleep(Duration::from_millis(100)).await;
            println!("[{}] Loaded plugin code", self.name);

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

            Ok(Box::new(AsyncMockPlugin {
                name: format!("async-mock-plugin-{}", self.name),
                functions,
            }))
        }

        async fn call(
            &self,
            plugin: &dyn Plugin,
            function_name: &str,
            args: &[Value],
        ) -> PluginResult<Value> {
            println!("[{}] Async calling function '{}' with {} arguments",
                     self.name, function_name, args.len());

            // Simulate async function call overhead
            tokio::time::sleep(Duration::from_millis(10)).await;

            let plugin = plugin.as_any().downcast_ref::<AsyncMockPlugin>()
                .ok_or(PluginError::InvalidPluginState)?;

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
    run_async_example().await
}

#[cfg(feature = "async")]
async fn run_async_example() -> PluginResult<()> {
    // Create a multi-runtime manager to demonstrate async usage
    let mut manager = tosic_plugin_core::managers::MultiRuntimeManager::new();
    
    // Register multiple async mock runtimes
    manager.register_runtime(Box::new(plugin::AsyncMockRuntime::new("Runtime-1")));
    manager.register_runtime(Box::new(plugin::AsyncMockRuntime::new("Runtime-2")));
    
    println!("Registered runtimes: {:?}", manager.runtime_names());
    println!("Runtime count: {}\n", manager.runtime_count());
    
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
    
    // Load plugins using the manager
    println!("=== Loading Plugins with Manager ===\n");
    
    let plugin1_source = PluginSource::Bytes(b"async plugin v1.0 - math operations".to_vec());
    let plugin2_source = PluginSource::Code("async plugin v2.0 - data processing".to_string());
    
    let plugin1_id = manager.load_plugin(plugin1_source, &host_context).await?;
    let plugin2_id = manager.load_plugin(plugin2_source, &host_context).await?;
    
    println!("Loaded plugins with IDs: {:?} and {:?}", plugin1_id, plugin2_id);
    if let Some(name1) = manager.plugin_name(plugin1_id) {
        println!("Plugin 1 name: {}", name1);
    }
    if let Some(name2) = manager.plugin_name(plugin2_id) {
        println!("Plugin 2 name: {}\n", name2);
    }
    
    // Test concurrent plugin function calls using the manager
    println!("=== Testing Concurrent Plugin Function Calls ===\n");
    
    // Execute multiple operations concurrently
    let manager = Arc::new(manager);
    let operations = vec![
        tokio::spawn({
            let manager = Arc::clone(&manager);
            async move {
                println!("Task 1: Starting add operation");
                let result = manager.call_plugin(plugin1_id, "add", &[Value::Int(10), Value::Int(20)]).await;
                println!("Task 1: Add completed");
                result
            }
        }),
        
        tokio::spawn({
            let manager = Arc::clone(&manager);
            async move {
                println!("Task 2: Starting fetch_data operation");
                let result = manager.call_plugin(plugin2_id, "fetch_data", &[Value::String("https://api.example.com/data".to_string())]).await;
                println!("Task 2: Fetch completed");
                result
            }
        }),
        
        tokio::spawn({
            let manager = Arc::clone(&manager);
            async move {
                println!("Task 3: Starting batch processing");
                let batch = vec![Value::Int(1), Value::Int(2), Value::Int(3), Value::Int(4)];
                let result = manager.call_plugin(plugin1_id, "process_batch", &[Value::Array(batch)]).await;
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
    match manager.call_plugin(plugin1_id, "nonexistent", &[]).await {
        Ok(_) => println!("Unexpected success!"),
        Err(e) => println!("Expected error: {}", e),
    }
    
    println!("\nTesting invalid arguments...");
    match manager.call_plugin(plugin2_id, "add", &[Value::String("not a number".to_string())]).await {
        Ok(_) => println!("Unexpected success!"),
        Err(e) => println!("Expected error: {}", e),
    }
    
    // Show manager statistics
    println!("\n=== Manager Statistics ===");
    println!("Total plugins loaded: {}", manager.plugin_count());
    println!("Plugin IDs: {:?}", manager.plugin_ids().collect::<Vec<_>>());
    
    // Cleanup - extract manager from Arc for cleanup
    let mut manager = match Arc::try_unwrap(manager) {
        Ok(manager) => manager,
        Err(_) => return Err(crate::PluginError::RuntimeError("Failed to cleanup manager".to_string())),
    };
    
    manager.unload_plugin(plugin1_id).await?;
    manager.unload_plugin(plugin2_id).await?;
    
    println!("\nAll plugins unloaded successfully!");
    println!("Final plugin count: {}", manager.plugin_count());
    
    println!("\n=== Async Example completed successfully! ===");
    Ok(())
}

#[cfg(not(feature = "async"))]
fn main() {
    panic!("This example requires the 'async' feature to be enabled. Please run with `--features async`.");
}