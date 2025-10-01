//! A prelude module to re-export commonly used items from the crate.

use cfg_if::cfg_if;
pub use crate::traits::*;
pub use crate::types::*;
pub use crate::error::*;

pub use crate::register_sync_fn;

cfg_if! {
    if #[cfg(feature = "async")] {
        pub use crate::register_async_fn;
    }
}

#[cfg(feature = "async")]
pub extern crate async_trait;