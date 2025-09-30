# Build System Reference

Complete reference for the Tosic Plugin build system with **115+ commands** organized in logical groups.

## Table of Contents

- [Overview](#overview)
- [Command Groups](#command-groups)
- [Configuration](#configuration)
- [Command Reference](#command-reference)
- [Advanced Usage](#advanced-usage)
- [Troubleshooting](#troubleshooting)

## Overview

The build system is based on [Just](https://just.systems/) and provides:

- **115+ commands** organized in logical groups
- **Cross-platform builds** for all major platforms
- **Security tooling** integration
- **Development workflow** automation
- **Comprehensive testing** support

### Quick Start

```bash
just help          # Show all commands
just config        # Show configuration
just dev           # Start development workflow
```

## Command Groups

### 🏗️ Build Commands (`[build]`)

Core building functionality:

```bash
just build          # Development build (default)
just build-release  # Optimized release build
just build-wasm     # WebAssembly build
just check          # Quick syntax check (no artifacts)
```

### 🧪 Test Commands (`[test]`)

Comprehensive testing:

```bash
just test           # Run all tests
just test-unit      # Unit tests only
just test-coverage  # Generate coverage reports
just test-watch     # Continuous testing
```

### 🔍 Lint Commands (`[lint]`)

Code quality and formatting:

```bash
just lint           # Quick lint (clippy + format check)
just lint-all       # Comprehensive linting
just format         # Format code
just fix            # Auto-fix issues
```

### 📚 Documentation (`[docs]`)

Documentation generation:

```bash
just docs           # Generate docs
just docs-open      # Generate and open docs
just docs-serve     # Serve docs locally
```

### 🌍 Cross-Compilation (`[cross]`)

Multi-platform building:

```bash
just cross-all      # Build for all platforms
just cross-macos    # macOS (Intel + ARM)
just cross-linux    # Linux (glibc + musl)
just cross-windows  # Windows (MSVC + GNU)
```

### 🛡️ Security (`[security]`)

Security analysis:

```bash
just security-audit # Vulnerability scanning
just security-deny  # Dependency policy check
just deps-unused    # Find unused dependencies
```

### 🔧 Utilities (`[util]`)

Project management:

```bash
just clean          # Clean build artifacts
just update         # Update dependencies
just project-status # Show project overview
```

## Configuration

### Environment Variables

The build system uses these environment variables:

```bash
# Core configuration
PROFILE=dev|release     # Build profile (default: dev)
TARGET=<rust-target>    # Target platform (default: native)
FEATURES=<features>     # Cargo features (default: none)
VERBOSE=0|1|2          # Build verbosity (default: 0)
JOBS=<number>          # Parallel jobs (default: auto)

# Rust environment
RUST_BACKTRACE=0|1|full # Error backtraces
RUST_LOG=<level>        # Logging level
RUSTFLAGS=<flags>       # Additional Rust flags
```

### Check Configuration

```bash
# Show current configuration
just config

# Show environment variables
just env

# Show project status
just project-status
```

### Examples

```bash
# Release build for macOS ARM
TARGET=aarch64-apple-darwin PROFILE=release just build

# Verbose build with features
VERBOSE=2 FEATURES=serde just build

# Cross-compile with specific jobs
JOBS=8 just cross-all
```

## Command Reference

### Build Commands

#### Core Building

| Command | Description | Usage |
|---------|-------------|-------|
| `build` | Development build | `just build [args]` |
| `build-release` | Release build | `just build-release [args]` |
| `build-dev` | Alias for build | `just build-dev [args]` |
| `check` | Quick check | `just check [args]` |

#### Package-Specific

| Command | Description | Usage |
|---------|-------------|-------|
| `build-core` | Build core crate | `just build-core [args]` |
| `build-main` | Build main crate | `just build-main [args]` |
| `build-package` | Build specific package | `just build-package <pkg> [args]` |

#### WebAssembly

| Command | Description | Usage |
|---------|-------------|-------|
| `build-wasm` | WASM build | `just build-wasm [args]` |
| `build-wasm-pack` | wasm-pack build | `just build-wasm-pack [pkg] [args]` |

### Test Commands

#### Core Testing

| Command | Description | Usage |
|---------|-------------|-------|
| `test` | All tests | `just test [args]` |
| `test-unit` | Unit tests | `just test-unit [args]` |
| `test-integration` | Integration tests | `just test-integration [args]` |
| `test-doc` | Documentation tests | `just test-doc [args]` |

#### Coverage

| Command | Description | Usage |
|---------|-------------|-------|
| `test-coverage` | Generate coverage | `just test-coverage [args]` |
| `coverage-html` | HTML coverage | `just coverage-html [args]` |
| `coverage-lcov` | LCOV coverage | `just coverage-lcov [args]` |

#### Continuous Testing

| Command | Description | Usage |
|---------|-------------|-------|
| `test-watch` | Watch tests | `just test-watch` |
| `test-watch-all` | Watch all tests | `just test-watch-all` |

#### Benchmarks

| Command | Description | Usage |
|---------|-------------|-------|
| `test-bench` | Run benchmarks | `just test-bench [args]` |
| `bench-all` | All benchmarks | `just bench-all [args]` |
| `bench-package` | Package benchmarks | `just bench-package <pkg> [args]` |

### Linting Commands

#### Core Linting

| Command | Description | Usage |
|---------|-------------|-------|
| `lint` | Quick lint | `just lint [args]` |
| `lint-all` | Comprehensive lint | `just lint-all` |
| `format` | Format code | `just format` |
| `format-check` | Check formatting | `just format-check` |

#### Fixes

| Command | Description | Usage |
|---------|-------------|-------|
| `fix` | Auto-fix clippy | `just fix` |
| `fix-all` | Fix and format | `just fix-all` |

#### Specialized Linting

| Command | Description | Usage |
|---------|-------------|-------|
| `lint-security` | Security lints | `just lint-security` |
| `lint-performance` | Performance lints | `just lint-performance` |
| `lint-code-quality` | Code quality lints | `just lint-code-quality` |
| `lint-tests` | Test-specific lints | `just lint-tests` |
| `lint-wasm` | WASM-specific lints | `just lint-wasm` |

### Documentation Commands

#### Generation

| Command | Description | Usage |
|---------|-------------|-------|
| `docs` | Generate docs | `just docs [args]` |
| `docs-open` | Generate and open | `just docs-open [args]` |
| `docs-all` | Docs with deps | `just docs-all [args]` |
| `docs-private` | Private docs | `just docs-private` |

#### Serving

| Command | Description | Usage |
|---------|-------------|-------|
| `docs-serve` | Serve locally | `just docs-serve [port]` |
| `docs-watch` | Watch and serve | `just docs-watch [port]` |

#### Validation

| Command | Description | Usage |
|---------|-------------|-------|
| `docs-check` | Check links | `just docs-check` |
| `docs-coverage` | Doc coverage | `just docs-coverage` |
| `readme-check` | Check README | `just readme-check` |

### Cross-Compilation Commands

#### Platform Groups

| Command | Description | Usage |
|---------|-------------|-------|
| `cross-all` | All platforms | `just cross-all` |
| `cross-macos` | macOS builds | `just cross-macos` |
| `cross-linux` | Linux builds | `just cross-linux` |
| `cross-windows` | Windows builds | `just cross-windows` |

#### Specific Targets

| Command | Description | Usage |
|---------|-------------|-------|
| `cross-macos-intel` | macOS Intel | `just cross-macos-intel [args]` |
| `cross-macos-arm` | macOS ARM | `just cross-macos-arm [args]` |
| `cross-linux-x64-glibc` | Linux x64 glibc | `just cross-linux-x64-glibc [args]` |
| `cross-linux-x64-musl` | Linux x64 musl | `just cross-linux-x64-musl [args]` |
| `cross-windows-msvc` | Windows MSVC | `just cross-windows-msvc [args]` |

#### WebAssembly

| Command | Description | Usage |
|---------|-------------|-------|
| `cross-wasm-all` | All WASM targets | `just cross-wasm-all` |
| `cross-wasm-unknown` | WASM32 unknown | `just cross-wasm-unknown [args]` |
| `cross-wasm-wasi` | WASM32 WASI | `just cross-wasm-wasi [args]` |

#### Setup

| Command | Description | Usage |
|---------|-------------|-------|
| `cross-setup` | Install targets | `just cross-setup` |
| `cross-install` | Install cross tool | `just cross-install` |
| `cross-targets` | List targets | `just cross-targets` |

### Security Commands

| Command | Description | Usage |
|---------|-------------|-------|
| `security-audit` | Vulnerability scan | `just security-audit` |
| `security-deny` | Policy check | `just security-deny` |
| `deps-unused` | Unused deps | `just deps-unused` |
| `license-check` | License compliance | `just license-check` |

### Utility Commands

#### Project Management

| Command | Description | Usage |
|---------|-------------|-------|
| `clean` | Clean artifacts | `just clean` |
| `clean-all` | Deep clean | `just clean-all` |
| `update` | Update deps | `just update` |
| `update-dep` | Update specific dep | `just update-dep <dep>` |

#### Information

| Command | Description | Usage |
|---------|-------------|-------|
| `project-status` | Project overview | `just project-status` |
| `version` | Version info | `just version` |
| `env` | Environment info | `just env` |
| `config` | Build config | `just config` |

#### Dependencies

| Command | Description | Usage |
|---------|-------------|-------|
| `add-dep` | Add dependency | `just add-dep <dep> [features]` |
| `add-dev-dep` | Add dev dependency | `just add-dev-dep <dep> [features]` |
| `deps-tree` | Dependency tree | `just deps-tree [args]` |
| `deps-info` | Detailed dep info | `just deps-info` |

#### Workspace

| Command | Description | Usage |
|---------|-------------|-------|
| `new-crate` | Create new crate | `just new-crate <name>` |
| `list-crates` | List crates | `just list-crates` |

#### Analysis

| Command | Description | Usage |
|---------|-------------|-------|
| `lines` | Count lines | `just lines` |
| `perf-build` | Build performance | `just perf-build` |
| `git-stats` | Git statistics | `just git-stats` |

#### File Operations

| Command | Description | Usage |
|---------|-------------|-------|
| `find-files` | Find files | `just find-files <pattern>` |
| `search` | Search in code | `just search <term>` |

### Workflow Commands

| Command | Description | Usage |
|---------|-------------|-------|
| `dev` | Development workflow | `just dev [args]` |
| `quick` | Quick iteration | `just quick [args]` |
| `ci` | CI pipeline | `just ci` |
| `watch` | Watch and rebuild | `just watch` |

## Advanced Usage

### Command Composition

```bash
# Multiple commands in sequence
just clean && just build && just test

# Conditional execution
just build || just clean

# With different configurations
PROFILE=release just build && just test
```

### Parallel Execution

```bash
# Parallel cross-compilation
JOBS=8 just cross-all

# Parallel testing
just test-unit & just test-integration & wait
```

### Custom Targets

```bash
# Custom target build
TARGET=aarch64-unknown-linux-musl just build

# Custom features
FEATURES="serde,tokio" just test
```

### Environment Combinations

```bash
# Release cross-compilation
PROFILE=release just cross-all

# Verbose debug build
VERBOSE=2 RUST_BACKTRACE=full just build

# Specific target with features
TARGET=wasm32-unknown-unknown FEATURES=web just build
```

## Troubleshooting

### Common Issues

#### Command Not Found

```bash
# Check if command exists
just --list | grep <command>

# Show all available commands
just help
```

#### Build Failures

```bash
# Clean and rebuild
just clean-all
just build

# Check configuration
just config

# Verbose build
VERBOSE=2 just build
```

#### Environment Issues

```bash
# Check environment
just env

# Reset configuration
unset PROFILE TARGET FEATURES VERBOSE JOBS
just config
```

### Debug Mode

```bash
# Enable just debugging
just --verbose <command>

# Show command that would be executed
just --dry-run <command>

# List all recipes
just --list --unsorted
```

### Performance Issues

```bash
# Check build timing
just perf-build

# Parallel builds
JOBS=8 just build

# Check dependency compilation
cargo build --timings
```

---

*For specific workflows, see [WORKFLOWS.md](WORKFLOWS.md)*
*For development setup, see [DEVELOPMENT.md](DEVELOPMENT.md)*