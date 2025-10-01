use crate::DenoRuntime;
use rustyscript::{Module, RuntimeBuilder};
use std::any::Any;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;
use tosic_plugin_core::prelude::PluginResult;
use tosic_plugin_core::traits::Plugin;
use tosic_plugin_core::types::{HostContext, Value};
use tosic_plugin_core::PluginError;

enum RuntimeCommand {
    Call {
        function_name: String,
        args: Vec<Value>,
        response: mpsc::Sender<PluginResult<Value>>,
    },
    Shutdown,
}

struct RuntimeHandle {
    sender: mpsc::Sender<RuntimeCommand>,
    handle: Option<JoinHandle<PluginResult<()>>>,
}

impl RuntimeHandle {
    fn new(module: Module, context: HostContext, path: Option<String>) -> Self {
        let (sender, receiver) = mpsc::channel();
        let handle = Self::create_thread(receiver, module, context, path);
        
        Self {
            sender,
            handle: Some(handle),
        }
    }
    
    fn create_thread(receiver: Receiver<RuntimeCommand>, module: Module, context: HostContext, path: Option<String>) -> JoinHandle<PluginResult<()>> {
        thread::spawn(move || {
            let mut runtime = RuntimeBuilder::new()
                .build()
                .map_err(|error| PluginError::LoadError(format!("Failed to build runtime: {}", error)))?;

            DenoRuntime::register_host_capabilities(&mut runtime, &context)?;

            if let Some(path) = path {
                runtime.set_current_dir(path)
                    .map_err(|error| PluginError::LoadError(format!("Failed to set current directory: {}", error)))?;
            }

            runtime.load_module(&module)
                .map_err(|error| PluginError::LoadError(format!("Failed to load module: {}", error)))?;

            while let Ok(command) = receiver.recv() {
                match command {
                    RuntimeCommand::Call { function_name, args, response } => {
                        let result = runtime.call_function::<Value>(None, &function_name, &args)
                            .map_err(|error| PluginError::RuntimeError(
                                format!("Failed to call function '{}': {}", function_name, error)
                            ));
                        let _ = response.send(result);
                    }
                    RuntimeCommand::Shutdown => break,
                }
            }
            
            Ok(())
        })
    }
}

impl Drop for RuntimeHandle {
    fn drop(&mut self) {
        let _ = self.sender.send(RuntimeCommand::Shutdown);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

pub struct DenoPlugin {
    runtime_handle: RuntimeHandle,
}

impl DenoPlugin {
    pub fn new(module: Module, context: HostContext, path: Option<String>) -> Self {
        let runtime_handle = RuntimeHandle::new(module, context, path);

        Self {
            runtime_handle,
        }
    }
    
    pub async fn call_function(&self, function_name: &str, args: &[Value]) -> PluginResult<Value> {
        let (response_sender, response_receiver) = mpsc::channel();
        
        self.runtime_handle.sender.send(RuntimeCommand::Call {
            function_name: function_name.to_string(),
            args: args.to_vec(),
            response: response_sender,
        }).map_err(|_| PluginError::RuntimeError(
            "Failed to send command to runtime thread".into()
        ))?;
        
        response_receiver.recv().map_err(|_| PluginError::RuntimeError(
            "Failed to receive response from runtime thread".into()
        ))?
    }
}

impl Plugin for DenoPlugin {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}