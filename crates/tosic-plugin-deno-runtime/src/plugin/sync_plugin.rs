use std::any::Any;
use rustyscript::Runtime;
use tosic_plugin_core::traits::Plugin;

pub struct DenoPlugin {
    runtime: Runtime,
}

impl DenoPlugin {
    pub fn new(runtime: Runtime) -> Self {
        Self { runtime }
    }
    
    pub fn runtime(&self) -> &Runtime {
        &self.runtime
    }
    
    pub fn runtime_mut(&mut self) -> &mut Runtime {
        &mut self.runtime
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