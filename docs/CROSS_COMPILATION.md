# Cross-Platform Compilation Guide

Complete guide to cross-compiling Tosic Plugin for all supported platforms.

## Table of Contents

- [Overview](#overview)
- [Supported Platforms](#supported-platforms)
- [Quick Start](#quick-start)
- [Platform-Specific Guides](#platform-specific-guides)
- [WebAssembly](#webassembly)
- [Configuration](#configuration)
- [Troubleshooting](#troubleshooting)

## Overview

Tosic Plugin supports cross-compilation to multiple platforms and architectures using Rust's native cross-compilation capabilities and the `cross` tool.

### Supported Cross-Compilation

- **macOS**: Intel x64 + Apple Silicon ARM64
- **Linux**: x64 + ARM64 (glibc and musl)
- **Windows**: x64 (MSVC + GNU toolchains)
- **WebAssembly**: WASM32 + WASI

## Supported Platforms

### Host Platforms (Development)

| Platform | Architecture | Status | Notes |
|----------|--------------|--------|-------|
| macOS | Intel x64 | ✅ Full | Primary development platform |
| macOS | Apple Silicon ARM64 | ✅ Full | Native and cross-compilation |
| Linux | x64 | ✅ Full | Via GitHub Actions |
| Linux | ARM64 | ⚠️ Limited | Basic support |
| Windows | x64 | ⚠️ Limited | Via GitHub Actions |

### Target Platforms (Cross-Compilation)

| Platform | Target Triple | Status | Method |
|----------|---------------|--------|---------|
| **macOS Intel** | `x86_64-apple-darwin` | ✅ Full | Native Rust |
| **macOS ARM** | `aarch64-apple-darwin` | ✅ Full | Native Rust |
| **Linux x64 glibc** | `x86_64-unknown-linux-gnu` | ✅ Full | Cross tool |
| **Linux x64 musl** | `x86_64-unknown-linux-musl` | ✅ Full | Cross tool |
| **Linux ARM glibc** | `aarch64-unknown-linux-gnu` | ✅ Full | Cross tool |
| **Linux ARM musl** | `aarch64-unknown-linux-musl` | ✅ Full | Cross tool |
| **Windows MSVC** | `x86_64-pc-windows-msvc` | ⚠️ Limited | Native Rust |
| **Windows GNU** | `x86_64-pc-windows-gnu` | ✅ Full | Cross tool |
| **WASM32** | `wasm32-unknown-unknown` | ✅ Full | Native Rust |
| **WASM WASI** | `wasm32-wasi` | ✅ Full | Native Rust |

## Quick Start

### Build for All Platforms

```bash
# Setup cross-compilation targets
just cross-setup

# Build for all platforms
just cross-all

# Build release versions for all platforms
just cross-all-release

# Package binaries
just cross-package
```

### Platform-Specific Builds

```bash
# macOS (both architectures)
just cross-macos

# Linux (all variants)
just cross-linux

# Windows (both toolchains)
just cross-windows

# WebAssembly (all targets)
just cross-wasm
```

## Platform-Specific Guides

### macOS Cross-Compilation

#### Prerequisites

```bash
# Install targets (automatic via just cross-setup)
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
```

#### Commands

```bash
# Both architectures
just cross-macos

# Specific architectures
just cross-macos-intel      # x86_64
just cross-macos-arm        # aarch64 (Apple Silicon)

# Release builds
just cross-macos-release
```

#### Configuration

```bash
# Build for specific target
TARGET=aarch64-apple-darwin just build

# With custom features
TARGET=x86_64-apple-darwin FEATURES=serde just build
```

### Linux Cross-Compilation

#### Prerequisites

```bash
# Install cross tool (if not using Nix)
cargo install cross

# Install targets
just cross-setup
```

#### Commands

```bash
# All Linux variants
just cross-linux

# glibc variants (x64 + ARM)
just cross-linux-glibc
just cross-linux-x64-glibc
just cross-linux-arm-glibc

# musl variants (static linking)
just cross-linux-musl
just cross-linux-x64-musl
just cross-linux-arm-musl

# Release builds
just cross-linux-release
```

#### Static Binaries

musl targets produce statically linked binaries:

```bash
# Static x64 binary
just cross-linux-x64-musl

# Static ARM binary  
just cross-linux-arm-musl

# Verify static linking
file target/x86_64-unknown-linux-musl/release/tosic-plugin
# Should show: statically linked
```

### Windows Cross-Compilation

#### Prerequisites

```bash
# Install targets
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-pc-windows-msvc
```

#### Commands

```bash
# Both toolchains
just cross-windows

# GNU toolchain (recommended for cross-compilation)
just cross-windows-gnu

# MSVC toolchain (requires Visual Studio on Windows)
just cross-windows-msvc

# Release builds
just cross-windows-release
```

#### Notes

- **GNU toolchain**: Better cross-compilation support
- **MSVC toolchain**: May require Windows host or Visual Studio tools
- **File extensions**: Windows binaries have `.exe` extension

## WebAssembly

### Prerequisites

```bash
# Install WASM targets
rustup target add wasm32-unknown-unknown
rustup target add wasm32-wasi

# Install wasm-pack (for npm packaging)
cargo install wasm-pack
```

### Commands

```bash
# All WASM targets
just cross-wasm-all

# Specific targets
just cross-wasm-unknown     # Pure WASM
just cross-wasm-wasi        # WASM with system interface

# wasm-pack build (for web/npm)
just build-wasm-pack
```

### WASM Variants

#### WASM32 Unknown (`wasm32-unknown-unknown`)

```bash
just cross-wasm-unknown

# Use case: Web browsers, pure WASM runtimes
# Characteristics: No system calls, minimal runtime
```

#### WASM32 WASI (`wasm32-wasi`)

```bash
just cross-wasm-wasi

# Use case: Server-side WASM, system integration
# Characteristics: System interface, file/network access
```

#### wasm-pack Integration

```bash
# Build for npm/web
just build-wasm-pack

# Custom package name
just build-wasm-pack tosic-plugin-core

# Generates: pkg/ directory with npm package
```

## Configuration

### Environment Variables

```bash
# Target platform
TARGET=aarch64-apple-darwin just build

# Build profile
PROFILE=release just cross-all

# Custom features
FEATURES="serde,async" just cross-linux

# Parallel jobs
JOBS=8 just cross-all
```

### Cross Tool Configuration

Create `.cross/Cross.toml` for advanced configuration:

```toml
[target.x86_64-unknown-linux-gnu]
image = "ghcr.io/cross-rs/x86_64-unknown-linux-gnu:main"

[target.aarch64-unknown-linux-gnu]
image = "ghcr.io/cross-rs/aarch64-unknown-linux-gnu:main"
```

### Cargo Configuration

Add to `.cargo/config.toml`:

```toml
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"

[target.aarch64-unknown-linux-musl]
linker = "aarch64-linux-musl-gcc"
```

## Advanced Cross-Compilation

### Custom Targets

```bash
# List available targets
just cross-targets
rustup target list

# Add custom target
rustup target add <target-triple>

# Build for custom target
TARGET=<target-triple> just build
```

### Cross-Compilation with Features

```bash
# Platform-specific features
TARGET=wasm32-unknown-unknown FEATURES=web just build
TARGET=x86_64-pc-windows-gnu FEATURES=windows just build

# No default features
TARGET=x86_64-unknown-linux-musl FEATURES="" just build
```

### Docker-Based Cross-Compilation

The `cross` tool uses Docker containers:

```bash
# Check cross installation
cross --version

# Build with cross (automatic when using just cross-* commands)
cross build --target x86_64-unknown-linux-gnu

# Custom cross images
cross build --target aarch64-unknown-linux-gnu
```

### Performance Optimization

```bash
# Target-specific optimizations
RUSTFLAGS="-C target-cpu=native" just build-release

# Link-time optimization
RUSTFLAGS="-C lto=fat" just cross-all-release

# Size optimization
RUSTFLAGS="-C opt-level=z" just cross-all-release
```

## Testing Cross-Compiled Binaries

### Native Testing

```bash
# Test on host platform
just test

# Cross-compile then test locally (for compatible targets)
TARGET=x86_64-apple-darwin just build
TARGET=x86_64-apple-darwin just test
```

### WASM Testing

```bash
# Test WASM builds
just test-wasm

# Run with wasmtime
wasmtime target/wasm32-wasi/debug/tosic-plugin.wasm
```

### Emulation Testing

```bash
# Test ARM on x64 (with qemu)
cross test --target aarch64-unknown-linux-gnu

# Windows testing on Unix (with wine)
cross test --target x86_64-pc-windows-gnu
```

## Binary Packaging

### Automatic Packaging

```bash
# Package all cross-compiled binaries
just cross-package

# Creates dist/ directory with:
# - tosic-plugin-macos-intel.tar.gz
# - tosic-plugin-macos-arm.tar.gz
# - tosic-plugin-linux-x64-glibc.tar.gz
# - tosic-plugin-linux-x64-musl.tar.gz
# - etc.
```

### Manual Packaging

```bash
# Find compiled binaries
find target/ -name "tosic-plugin*" -type f -executable

# Package manually
tar -czf tosic-plugin-macos-arm.tar.gz \
  -C target/aarch64-apple-darwin/release \
  tosic-plugin
```

## Troubleshooting

### Common Issues

#### Missing Targets

```bash
# Error: target not installed
# Solution:
just cross-setup
# Or manually:
rustup target add <target-triple>
```

#### Cross Tool Issues

```bash
# Error: cross command not found
# Solution:
cargo install cross
# Or via Nix:
nix develop
```

#### Docker/Cross Issues

```bash
# Error: Docker not running
# Solution: Start Docker service

# Error: Permission denied
# Solution: Add user to docker group
sudo usermod -aG docker $USER
```

#### Link Errors

```bash
# Error: linker not found
# Solution: Install appropriate toolchain
# For musl:
apt-get install musl-tools
# For Windows:
apt-get install gcc-mingw-w64
```

### Debugging Cross-Compilation

```bash
# Verbose cross compilation
VERBOSE=2 just cross-macos

# Check target installation
rustup target list --installed

# Test simple cross-compilation
cross build --target x86_64-unknown-linux-gnu --verbose
```

### Platform-Specific Issues

#### macOS

```bash
# Error: Xcode command line tools required
xcode-select --install

# Error: SDK not found
# Usually resolved by Xcode installation
```

#### Linux (musl)

```bash
# Error: musl-gcc not found
# Ubuntu/Debian:
sudo apt-get install musl-tools

# Alpine:
apk add musl-dev
```

#### Windows

```bash
# Error: link.exe not found (MSVC)
# Solution: Use GNU target instead
TARGET=x86_64-pc-windows-gnu just build

# Or install Visual Studio Build Tools
```

#### WebAssembly

```bash
# Error: wasm-pack not found
cargo install wasm-pack

# Error: wasm target not installed
rustup target add wasm32-unknown-unknown
```

### Performance Issues

```bash
# Slow cross-compilation
# Solution: Use parallel builds
JOBS=8 just cross-all

# Large binary sizes
# Solution: Enable optimizations
PROFILE=release just cross-all

# Solution: Use musl for smaller static binaries
just cross-linux-musl
```

---

*For more development workflows, see [WORKFLOWS.md](WORKFLOWS.md)*
*For build system details, see [BUILD_SYSTEM.md](BUILD_SYSTEM.md)*