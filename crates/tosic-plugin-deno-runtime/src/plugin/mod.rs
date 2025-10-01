#[cfg(not(feature = "async"))]
mod sync_plugin;
#[cfg(feature = "async")]
mod async_plugin;

#[cfg(not(feature = "async"))]
pub use sync_plugin::DenoPlugin;
#[cfg(feature = "async")]
pub use async_plugin::DenoPlugin;