# Tosic Plugin

> A runtime-agnostic plugin system for Rust applications

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.75+-blue.svg)](https://www.rust-lang.org)

## Overview

Tosic Plugin is a Rust library that provides a unified abstraction layer for building plugin systems that can work across multiple runtime environments. Whether your plugins need to run in WebAssembly, JavaScript engines, Lua, or other runtime environments, Tosic Plugin provides a consistent interface for plugin loading, execution, and host-plugin communication.

## Why Tosic Plugin?

Modern applications often need plugin systems to provide extensibility and customization. However, different use cases call for different runtime environments:

- **WebAssembly (WASM)** for security, performance, and language diversity
- **JavaScript** for rapid development and ecosystem compatibility  
- **Lua** for lightweight scripting and configuration
- **Native binaries** for maximum performance

Traditionally, supporting multiple plugin runtimes requires separate implementations for each, leading to code duplication and maintenance overhead. Tosic Plugin solves this by providing:

- **Runtime Abstraction**: Write your plugin host code once, support multiple runtimes
- **Type-Safe Interface**: Strongly typed communication between host and plugins
- **Unified API**: Consistent loading, execution, and lifecycle management
- **Async Support**: Full async/await support for non-blocking plugin operations
- **Flexible Host Functions**: Easy registration of host functions that plugins can call

## Core Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Your App      │    │   Tosic Plugin   │    │   Plugin        │
│                 │    │                  │    │   Runtimes      │
│  ┌───────────┐  │    │  ┌─────────────┐ │    │                 │
│  │Host Funcs │◄─┼────┼─►│HostContext  │ │    │ ┌─────────────┐ │
│  └───────────┘  │    │  └─────────────┘ │    │ │ WebAssembly │ │
│                 │    │                  │    │ └─────────────┘ │
│  ┌───────────┐  │    │  ┌─────────────┐ │    │ ┌─────────────┐ │
│  │Plugin Mgr │◄─┼────┼─►│  Runtime    │◄┼────┼─│ JavaScript  │ │
│  └───────────┘  │    │  │  Trait      │ │    │ └─────────────┘ │
└─────────────────┘    │  └─────────────┘ │    │ ┌─────────────┐ │
                       └──────────────────┘    │ │    Lua      │ │
                                               │ └─────────────┘ │
                                               └─────────────────┘
```

## Quick Start

### Prerequisites

- **Rust 1.75+** 
- **Nix** (recommended) or manual tool installation
- **Just** command runner (installed via Nix or `cargo install just`)

### Development Setup

#### Option 1: Using Nix (Recommended)

```bash
# Clone the repository
git clone <repository-url>
cd tosic-plugin

# Enter the development environment
nix develop

# Start development
just dev
```

#### Option 2: Manual Setup

```bash
# Install required tools
cargo install just cargo-watch cargo-audit

# Clone and build
git clone <repository-url>
cd tosic-plugin
just dev
```

### Basic Usage

```rust
use tosic_plugin_core::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a host context and register functions
    let mut context = HostContext::new();
    
    // Register host functions that plugins can call
    context.register("log", |message: String| {
        println!("Plugin says: {}", message);
    });
    
    context.register("add_numbers", |a: i64, b: i64| -> i64 {
        a + b
    });
    
    // Load and use plugins through runtime implementations
    // (Specific runtime implementations are provided separately)
    
    Ok(())
}
```

## Build System

Tosic Plugin uses a comprehensive build system based on **Just** (justfile) that supports:

### Quick Commands

```bash
just dev          # Start development (build + test + watch)
just test         # Run all tests  
just release      # Build optimized release
just help         # Show all available commands
```

### Cross-Platform Builds

```bash
just cross-macos      # Build for macOS (Intel + ARM)
just cross-linux     # Build for Linux (x64 + ARM + musl)
just cross-windows    # Build for Windows (MSVC + GNU)
just cross-wasm       # Build for WebAssembly
```

### Development Tools

```bash
just lint         # Run formatting and linting
just docs-open    # Generate and open documentation
just clean        # Clean build artifacts
just audit        # Security audit
just deps         # Show dependency information
```

### Configuration

The build system supports configuration via environment variables:

```bash
PROFILE=release just build    # Release build
TARGET=aarch64-apple-darwin just build  # Specific target
FEATURES=serde just test      # With features
VERBOSE=2 just build          # Verbose output
```

## Project Structure

```
tosic-plugin/
├── crates/
│   ├── tosic-plugin-core/    # Core abstractions and traits
│   │   ├── src/
│   │   │   ├── traits/       # Runtime and host function traits
│   │   │   ├── types/        # Value types and context
│   │   │   └── error.rs      # Error types
│   │   └── examples/         # Usage examples
│   └── tosic-plugin/         # Main library crate
├── just/                     # Modular build commands
│   ├── build.just           # Build commands
│   ├── test.just            # Testing commands
│   ├── cross.just           # Cross-compilation
│   ├── docs.just            # Documentation
│   ├── lint.just            # Linting and formatting
│   └── util.just            # Utilities
├── justfile                 # Main build system entry
└── flake.nix               # Nix development environment
```

## Supported Platforms

**Host Platforms:**
- macOS (Intel x64 + Apple Silicon ARM64)
- Linux (x64 + ARM64, glibc + musl)
- Windows (MSVC + GNU toolchain)

**Plugin Runtime Targets:**
- WebAssembly (WASM32 + WASI)
- Native shared libraries
- JavaScript engines (planned)
- Lua runtime (planned)

## Development Workflow

### Getting Started

```bash
# Enter development environment
nix develop  # or ensure tools are installed

# Run development workflow
just dev
```

### Making Changes

```bash
# Quick iteration
just quick               # Fast build + basic tests

# Full validation
just ci                  # Complete CI pipeline

# Specific tasks
just test-unit           # Unit tests only
just build-release       # Optimized build
just docs                # Generate documentation
```

### Quality Assurance

```bash
# Code quality
just lint                # Format + lint
just audit               # Security audit
just deps-outdated       # Check outdated dependencies

# Testing
just test-all            # All tests including integration
just test-coverage       # Test coverage report
just cross-all           # Test cross-compilation
```

## Contributing

1. **Development Environment**: Use `nix develop` for consistent tooling
2. **Code Quality**: Run `just lint` before committing
3. **Testing**: Ensure `just test` passes on all changes
4. **Documentation**: Update docs for public API changes
5. **Cross-Platform**: Test builds with `just cross-all` for platform compatibility

### Code Standards

- Follow Rust standard formatting (`just format`)
- All public APIs must have documentation
- Use `thiserror` for error handling
- Prefer strongly typed interfaces over dynamic typing
- Include examples for new features

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Status

**Current Status**: Alpha Development

Tosic Plugin is in active development. The core abstractions are stabilizing, but breaking changes may occur until version 1.0. Runtime implementations are being developed separately and will be released as companion crates.

---

*For detailed API documentation, build instructions, and examples, see the generated documentation: `just docs-open`*