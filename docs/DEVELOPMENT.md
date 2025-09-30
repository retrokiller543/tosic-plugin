# Development Guide

This guide covers the complete development setup and workflow for Tosic Plugin.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Environment Setup](#environment-setup)
- [Development Workflow](#development-workflow)
- [Code Organization](#code-organization)
- [Testing Strategy](#testing-strategy)
- [Debugging](#debugging)
- [IDE Setup](#ide-setup)

## Prerequisites

### Required Tools

- **Rust 1.75+** - Modern Rust toolchain
- **Nix** - Package manager and development environment (recommended)
- **Just** - Command runner for build automation
- **Git** - Version control

### Optional Tools (Installed via Nix)

- **WASM Tools** - `wasmtime`, `wasm-pack`, `wasm-tools`
- **Security Tools** - `cargo-audit`, `cargo-deny`, `cargo-machete`
- **Analysis Tools** - `tokei`, `cargo-bloat`, `cargo-tarpaulin`

## Environment Setup

### Option 1: Nix Development Environment (Recommended)

The Nix flake provides a complete, reproducible development environment:

```bash
# Clone the repository
git clone <repository-url>
cd tosic-plugin

# Enter development environment (installs all tools automatically)
nix develop

# Verify setup
just config  # Shows environment configuration
```

**Benefits of Nix:**
- All tools pinned to specific versions
- Automatic tool installation
- Cross-platform consistency
- No conflicts with system packages

### Option 2: Manual Tool Installation

If you prefer not to use Nix:

```bash
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Just
cargo install just

# Install essential cargo extensions
cargo install cargo-watch cargo-audit cargo-deny cargo-machete cargo-tarpaulin

# Install WASM tools
cargo install wasm-pack
rustup target add wasm32-unknown-unknown wasm32-wasi

# Clone and setup
git clone <repository-url>
cd tosic-plugin
```

### Environment Variables

Key environment variables for development:

```bash
# Build configuration
export PROFILE=dev          # or 'release'
export VERBOSE=1             # Build verbosity
export RUST_BACKTRACE=1      # Error backtraces
export RUST_LOG=debug        # Logging level

# Cross-compilation
export TARGET=              # Leave empty for native
export FEATURES=             # Cargo features to enable

# Check current configuration
just config
```

## Development Workflow

### Daily Development

```bash
# Start development session
nix develop              # Enter development environment
just dev                 # Build + test + start watching

# Make changes to code...

# Quick validation
just quick               # Fast build + basic tests

# Before committing
just ci                  # Full CI pipeline
```

### Common Tasks

#### Building

```bash
# Development builds
just build               # Standard dev build
just build-release       # Optimized release build
just build-wasm          # WebAssembly build

# Cross-platform builds
just cross-macos         # macOS (Intel + ARM)
just cross-linux         # Linux (glibc + musl)
just cross-windows       # Windows (MSVC + GNU)
```

#### Testing

```bash
# Run tests
just test                # All tests
just test-unit           # Unit tests only
just test-integration    # Integration tests only
just test-coverage       # Generate coverage report

# Continuous testing
just test-watch          # Watch for changes and re-run tests
```

#### Code Quality

```bash
# Formatting and linting
just format              # Format code
just lint                # Run clippy + formatting check
just fix                 # Auto-fix clippy suggestions

# Security and dependencies
just security-audit      # Security vulnerability scan
just security-deny       # Check dependency policies
just deps-unused         # Find unused dependencies
```

### File Watching

The development environment supports automatic rebuilding on file changes:

```bash
just watch               # Watch and rebuild on changes
just test-watch          # Watch and run tests on changes
just docs-watch          # Watch and rebuild docs on changes
```

## Code Organization

### Workspace Structure

```
crates/
├── tosic-plugin-core/   # Core abstractions and traits
│   ├── src/
│   │   ├── traits/      # Runtime and host function traits
│   │   ├── types/       # Value types and context
│   │   ├── error.rs     # Error handling
│   │   └── lib.rs       # Public API
│   └── examples/        # Usage examples
└── tosic-plugin/        # Main library crate
    ├── src/
    │   └── lib.rs       # Re-exports and convenience APIs
    └── Cargo.toml
```

### Code Standards

#### Rust Style

```bash
# Apply standard formatting
just format

# Check adherence to style guide
just lint
```

#### Error Handling

- Use `thiserror` for custom error types
- Provide meaningful error messages
- Include context in error chains

#### Documentation

- All public APIs must have documentation
- Include examples in doc comments
- Update documentation for breaking changes

```rust
/// Represents a plugin runtime abstraction.
///
/// # Examples
///
/// ```rust
/// use tosic_plugin_core::Runtime;
/// 
/// let runtime = MyRuntime::new()?;
/// let result = runtime.call_function("add", &[1.into(), 2.into()])?;
/// ```
pub trait Runtime {
    // ...
}
```

## Testing Strategy

### Test Organization

```
tests/
├── unit/           # Unit tests (fast, isolated)
├── integration/    # Integration tests (slower, end-to-end)
└── fixtures/       # Test data and fixtures
```

### Test Commands

```bash
# Different test suites
just test-unit           # Fast unit tests
just test-integration    # Slower integration tests
just test-all           # All tests
just test-doc           # Documentation tests

# Test configuration
just test-minimal       # Test with minimal features
just test-all-features  # Test with all features enabled

# Performance testing
just bench              # Run benchmarks
just bench-all          # Comprehensive benchmarks
```

### Coverage Reports

```bash
# Generate coverage
just test-coverage      # HTML + LCOV reports
just coverage-html      # HTML report only
just coverage-lcov      # LCOV report only

# View coverage
open target/tarpaulin/tarpaulin-report.html
```

## Debugging

### Environment Debugging

```bash
# Check configuration
just config             # Build configuration
just env                # Rust/Cargo environment
just project-status     # Project overview

# Dependency information
just deps-tree          # Dependency tree
just deps-info          # Detailed dependency info
```

### Build Debugging

```bash
# Verbose builds
VERBOSE=2 just build

# Build timing analysis
just perf-build         # Analyze build performance

# Target-specific builds
TARGET=x86_64-apple-darwin just build
```

### Runtime Debugging

```bash
# Debug builds with symbols
PROFILE=dev just build

# Enable Rust backtraces
RUST_BACKTRACE=full just test

# Enable logging
RUST_LOG=trace just test
```

## IDE Setup

### Rust Analyzer

For VS Code, IntelliJ, or other Rust Analyzer-supported editors:

1. **Enter Nix environment**: `nix develop`
2. **Start your IDE** from within the Nix shell
3. **Rust Analyzer** will automatically detect the project configuration

### VS Code Configuration

Recommended `.vscode/settings.json`:

```json
{
  "rust-analyzer.check.command": "clippy",
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.procMacro.enable": true,
  "files.watcherExclude": {
    "**/target/**": true
  }
}
```

### IntelliJ/CLion

1. Install the Rust plugin
2. Open the project
3. Configure toolchain to use the Nix-provided Rust

### Command Line Debugging

```bash
# Check rust-analyzer status
rust-analyzer --version

# Manual LSP server
rust-analyzer

# Check project compilation
cargo check --workspace --all-targets
```

## Performance Optimization

### Build Performance

```bash
# Parallel builds
JOBS=8 just build

# Build timing analysis
just perf-build

# Dependency compilation optimization
cargo build --timings
```

### Runtime Performance

```bash
# Release builds
just build-release

# Profile-guided optimization
RUSTFLAGS="-C target-cpu=native" just build-release

# Binary size analysis
just perf-size
cargo install cargo-bloat
cargo bloat --release
```

## Troubleshooting

### Common Issues

#### Build Failures

```bash
# Clean build artifacts
just clean-all

# Update dependencies
just update

# Check for conflicting tools
which cargo rust-analyzer
```

#### Test Failures

```bash
# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run single-threaded
cargo test -- --test-threads=1
```

#### Environment Issues

```bash
# Reset Nix environment
exit  # Exit nix develop
nix develop  # Re-enter

# Check tool versions
just env
rustc --version
cargo --version
```

### Getting Help

1. **Check documentation**: `just docs-open`
2. **View configuration**: `just config`
3. **Check build system**: See [BUILD_SYSTEM.md](BUILD_SYSTEM.md)
4. **Cross-compilation issues**: See [CROSS_COMPILATION.md](CROSS_COMPILATION.md)
5. **Security tools**: See [SECURITY.md](SECURITY.md)

---

*For more specific workflows, see [WORKFLOWS.md](WORKFLOWS.md)*