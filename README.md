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

Tosic Plugin uses a comprehensive build system based on **Just** (justfile) with **115+ commands** organized into logical groups:

### Quick Start Commands

```bash
just dev          # Start development (build + test + watch)
just test         # Run all tests  
just release      # Build optimized release
just help         # Show all 115+ available commands
just config       # Show current configuration and env vars
```

### Cross-Platform Builds

```bash
just cross-macos     # Build for macOS (Intel + ARM)
just cross-linux    # Build for Linux (x64 + ARM, glibc + musl)
just cross-windows   # Build for Windows (MSVC + GNU)
just cross-wasm      # Build for WebAssembly (WASM32 + WASI)
just cross-all       # Build for all platforms
```

### Security & Quality Tools

```bash
just lint            # Comprehensive linting (clippy + formatting)
just security-audit  # Security vulnerability scan
just security-deny   # Dependency policy checking  
just deps-unused     # Find unused dependencies
just test-coverage   # Generate test coverage reports
```

### Documentation & Analysis

```bash
just docs-open       # Generate and open documentation
just docs-serve      # Serve docs locally with hot reload
just lines           # Count lines of code with tokei
just perf-build      # Analyze build performance
just git-stats       # Show git repository statistics
```

### Workspace Management

```bash
just new-crate name  # Create new crate in workspace
just list-crates     # List all workspace crates
just add-dep dep     # Add dependency with features support
just update-dep dep  # Update specific dependency
```

### Configuration & Debugging

The build system supports configuration via environment variables and provides debugging tools:

```bash
# Configuration
PROFILE=release just build    # Release build
TARGET=aarch64-apple-darwin just build  # Specific target
FEATURES=serde just test      # With features
VERBOSE=2 just build          # Verbose output

# Debugging
just config       # Show all environment variables and settings
just env          # Show Rust/Cargo environment variables
just project-status  # Show project overview and status
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
├── docs/                     # Development documentation
│   ├── DEVELOPMENT.md        # Detailed development guide
│   ├── BUILD_SYSTEM.md       # Complete build system reference
│   ├── CROSS_COMPILATION.md  # Cross-platform build guide
│   ├── SECURITY.md           # Security tools and practices
│   └── WORKFLOWS.md          # Development workflows
├── just/                     # Modular build commands (115+ commands)
│   ├── build.just           # Build commands (dev, release, WASM)
│   ├── test.just            # Testing commands (unit, integration, coverage)
│   ├── cross.just           # Cross-compilation (all platforms)
│   ├── docs.just            # Documentation generation and serving
│   ├── lint.just            # Linting, formatting, security auditing
│   └── util.just            # Utilities and workspace management
├── justfile                 # Main build system entry point
├── flake.nix               # Nix development environment
└── README.md               # This file
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

For detailed development workflows, see the [docs/](docs/) directory:

- **[docs/DEVELOPMENT.md](docs/DEVELOPMENT.md)** - Complete development setup and workflows
- **[docs/BUILD_SYSTEM.md](docs/BUILD_SYSTEM.md)** - Full build system reference (115+ commands)
- **[docs/CROSS_COMPILATION.md](docs/CROSS_COMPILATION.md)** - Cross-platform building guide
- **[docs/SECURITY.md](docs/SECURITY.md)** - Security tools and best practices
- **[docs/WORKFLOWS.md](docs/WORKFLOWS.md)** - Common development workflows

### Quick Reference

```bash
# Quick iteration
just quick               # Fast build + basic tests
just dev                 # Development workflow (build + test + watch)

# Full validation
just ci                  # Complete CI pipeline
just cross-all           # Test all cross-compilation targets

# Quality assurance
just lint                # Format + lint + security checks
just test-coverage       # Generate test coverage reports
just docs-open           # Generate and view documentation
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