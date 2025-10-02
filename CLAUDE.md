# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build System Commands

This project uses **Just** as its build system with 115+ organized commands. Key commands:

```bash
# Development workflow
just dev                 # Build + test + watch for changes
just quick               # Fast build + basic tests
just ci                  # Full CI pipeline (clean + lint + test + release)

# Building
just build               # Development build
just build-release       # Optimized release build
just build-wasm          # WebAssembly build

# Testing
just test                # Run all tests
just test-unit           # Unit tests only
just test-integration    # Integration tests only
just test-coverage       # Generate coverage reports

# Code quality
just lint                # Format + clippy + security checks
just format              # Format code only
just fix                 # Auto-fix clippy suggestions

# Cross-compilation
just cross-macos         # Build for macOS (Intel + ARM)
just cross-linux         # Build for Linux (x64 + ARM, glibc + musl)
just cross-windows       # Build for Windows (MSVC + GNU)
just cross-wasm          # Build for WebAssembly (WASM32 + WASI)
just cross-all           # Build for all platforms

# Security and dependencies
just security-audit      # Security vulnerability scan
just security-deny       # Check dependency policies
just deps-unused         # Find unused dependencies

# Documentation
just docs-open           # Generate and open documentation
just docs-serve          # Serve docs locally with hot reload

# Environment
just config              # Show build configuration and env vars
just env                 # Show Rust/Cargo environment variables
```

Use `just help` to see all 115+ available commands.

## Project Architecture

**Tosic Plugin** is a runtime-agnostic plugin system for Rust applications that provides unified abstractions for multiple plugin runtimes (WebAssembly, JavaScript, Lua, etc.).

### Core Architecture Components

1. **Runtime Abstraction** (`tosic-plugin-core/src/traits/runtime.rs`):
   - `Runtime` trait: Core interface for plugin runtime implementations
   - `Plugin` trait: Opaque handle to loaded plugin instances
   - `RuntimeExt` trait: Ergonomic argument passing extensions

2. **Type System** (`tosic-plugin-core/src/types/`):
   - `Value`: Boundary type for data exchange between host and plugins
   - `HostContext`: Container for host functions that plugins can call
   - `PluginSource`: Abstraction for plugin loading sources

3. **Plugin Managers** (`tosic-plugin-core/src/managers/`):
   - `SingleRuntimeManager`: Optimized for single runtime scenarios
   - `MultiRuntimeManager`: Flexible support for multiple runtimes

4. **Error Handling** (`tosic-plugin-core/src/error.rs`):
   - Uses `thiserror` for structured error types
   - `PluginError` and `PluginResult` for consistent error handling

### Workspace Structure

```
crates/
├── tosic-plugin-core/        # Core abstractions and traits
│   ├── src/traits/          # Runtime, manager, and host function traits
│   ├── src/types/           # Value types, context, and source abstractions
│   ├── src/managers/        # Single and multi-runtime managers
│   └── src/error.rs         # Structured error handling
├── tosic-plugin-deno-runtime/ # Deno/JavaScript runtime implementation
└── tosic-plugin/            # Main library crate (re-exports)
```

### Key Design Patterns

- **Feature Flags**: The `async` feature enables async/await support throughout the system
- **Global Registry**: Optional `global-registry` feature for inventory-based function registration
- **Runtime Detection**: Runtimes implement `supports_plugin()` for automatic selection
- **Type Safety**: Strongly typed interfaces with `Value` enum for runtime boundaries
- **Ergonomic APIs**: Extension traits provide convenient argument passing

## Development Environment

**Preferred setup**: Use Nix development environment with `nix develop` for consistent tooling.

**Alternative**: Manual installation requires Rust 1.75+, Just, and cargo extensions (cargo-watch, cargo-audit, etc.).

Environment variables:
- `PROFILE=dev|release` - Build profile
- `TARGET=<triple>` - Cross-compilation target
- `FEATURES=<features>` - Cargo features to enable
- `VERBOSE=0|1|2` - Build verbosity

## Code Standards

- **Error Handling**: Use `thiserror` for custom error types with meaningful messages
- **Documentation**: All public APIs must have documentation with examples
- **Formatting**: Use `just format` for consistent Rust formatting
- **Dependencies**: Avoid `rsa` or `openssl` in dependency tree (causes CI/security issues)
- **Async Support**: Use `#[cfg(feature = "async")]` for async variants of traits
- **Comments**: Always avoid comments inside the code, only comments used should be doc comments to document public items. Also follow the clean code principals when making new code, reviewing and cleaning old code

## Testing Strategy

- Unit tests: Fast, isolated tests in `src/` files
- Integration tests: End-to-end tests in `tests/` directory
- Coverage: Use `just test-coverage` for HTML reports
- Cross-platform: Test with `just cross-all` for platform compatibility

## Key Files to Understand

- `tosic-plugin-core/src/traits/runtime.rs:54-103` - Core Runtime trait definition
- `tosic-plugin-core/src/types/context/mod.rs` - Host function registration system
- `tosic-plugin-core/src/managers/single.rs` - Single runtime manager implementation
- `crates/tosic-plugin-deno-runtime/` - Example runtime implementation using Deno

The project follows a trait-based architecture where runtime implementations provide concrete behavior for the abstract `Runtime` trait, enabling the same host code to work with multiple plugin runtime environments.