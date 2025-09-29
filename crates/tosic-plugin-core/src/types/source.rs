pub enum PluginSource {
    /// Plugin source code as a string.
    Code(String),
    /// Path to the plugin source file.
    FilePath(String),
    /// Raw bytes of the plugin source.
    Bytes(Vec<u8>),
}