//! A prelude module for tosic-plugin crate to re-export commonly used items from tosic-plugin-core crate.

pub use tosic_plugin_core::prelude::*;

macro_rules! runtime {
    ($name:literal, $crate_name:ident) => {
        #[cfg(feature = $name)]
        pub use $crate_name::prelude::*;
    };
}

runtime!("deno-runtime", tosic_plugin_deno_runtime);
