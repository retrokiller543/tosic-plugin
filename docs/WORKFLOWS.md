# Development Workflows

Complete guide to common development workflows for Tosic Plugin using the 115+ command build system.

## Table of Contents

- [Overview](#overview)
- [Daily Development](#daily-development)
- [Feature Development](#feature-development)
- [Quality Assurance](#quality-assurance)
- [Release Preparation](#release-preparation)
- [Debugging Workflows](#debugging-workflows)
- [Performance Optimization](#performance-optimization)
- [CI/CD Integration](#cicd-integration)

## Overview

This guide provides step-by-step workflows for common development scenarios using the Tosic Plugin build system. Each workflow is designed to be efficient, comprehensive, and follows best practices.

### Workflow Philosophy

- **Fast Feedback**: Quick iterations with immediate validation
- **Comprehensive Validation**: Full testing before merging
- **Security First**: Regular security checks and dependency audits
- **Cross-Platform**: Ensure compatibility across all supported platforms

## Daily Development

### Starting Your Development Session

```bash
# 1. Enter development environment
nix develop

# 2. Check project status and configuration
just config
just project-status

# 3. Update dependencies (if needed)
just update

# 4. Start development workflow
just dev
```

**What `just dev` does:**
- Builds the project in development mode
- Runs all tests
- Starts file watching for automatic rebuilds
- Provides immediate feedback on changes

### Quick Iteration Cycle

For rapid development with minimal overhead:

```bash
# Fast check (syntax + basic validation)
just check

# Quick build + basic tests
just quick

# Format code
just format

# Quick lint check
just lint
```

**Use this cycle when:**
- Making small changes
- Experimenting with code
- Need immediate feedback

### Making Changes Workflow

```bash
# 1. Create feature branch
git checkout -b feature/new-functionality

# 2. Make your changes...

# 3. Quick validation
just quick

# 4. Run specific tests if needed
just test-unit                    # Fast unit tests
just test-integration            # Slower integration tests

# 5. Check formatting and basic linting
just format
just lint

# 6. Commit changes
git add .
git commit -m "Add new functionality"
```

### End of Day Workflow

```bash
# 1. Full validation before pushing
just ci

# 2. Generate documentation
just docs

# 3. Check security
just security-audit

# 4. Push changes
git push origin feature/new-functionality
```

## Feature Development

### Starting a New Feature

```bash
# 1. Create workspace crate if needed
just new-crate feature-name

# 2. Add required dependencies
just add-dep serde features=derive
just add-dev-dep tokio-test

# 3. Set up initial structure
# ... create files and basic structure ...

# 4. Initial build and test
just build
just test
```

### Feature Development Cycle

```bash
# 1. Write tests first (TDD approach)
# ... write failing tests ...
just test-unit

# 2. Implement feature
# ... implement functionality ...

# 3. Validate implementation
just test-unit                   # Check tests pass
just lint                       # Check code quality
just docs                       # Update documentation

# 4. Cross-platform validation
just cross-macos                # Test macOS builds
just cross-linux               # Test Linux builds

# 5. Security check
just security-audit            # Check for vulnerabilities
just deps-unused               # Clean unused dependencies
```

### Adding Dependencies

```bash
# 1. Add dependency with careful consideration
just add-dep new-dependency features=required-features

# 2. Check dependency impact
just deps-tree                 # View dependency tree
just deps-info                 # Detailed dependency information

# 3. Security validation
just security-audit            # Check for vulnerabilities
just security-deny             # Check policy compliance

# 4. Build verification
just build                     # Ensure build still works
just test                      # Ensure tests still pass
```

### WebAssembly Development

```bash
# 1. Set up WASM build
just cross-setup               # Install WASM targets

# 2. Build for WASM
just cross-wasm-all            # All WASM variants
just cross-wasm-unknown        # Pure WASM
just cross-wasm-wasi           # WASM with system interface

# 3. Test WASM builds
just test-wasm                 # WASM-specific tests

# 4. Package for web (if applicable)
just build-wasm-pack           # Create npm package
```

## Quality Assurance

### Pre-Commit Workflow

```bash
# 1. Format code
just format

# 2. Fix linting issues
just fix                       # Auto-fix clippy suggestions
just lint-all                  # Comprehensive linting

# 3. Run all tests
just test                      # All test suites
just test-coverage             # Generate coverage report

# 4. Security checks
just security-audit            # Vulnerability scan
just security-deny             # Policy compliance
just deps-unused               # Unused dependencies

# 5. Documentation
just docs                      # Generate documentation
just docs-check                # Validate documentation

# 6. Cross-platform validation
just cross-all                 # Build for all platforms
```

### Comprehensive Quality Check

```bash
# Complete quality assurance pipeline
just ci

# This runs:
# - Code formatting check
# - Comprehensive linting (security, performance, code quality)
# - All test suites with coverage
# - Security auditing
# - Cross-platform builds
# - Documentation generation
```

### Security-Focused Workflow

```bash
# 1. Security audit
just security-audit            # Scan for vulnerabilities

# 2. Dependency policy check
just security-deny             # Check dependency policies

# 3. License compliance
just license-check             # Verify license compatibility

# 4. Code security analysis
just lint-security             # Security-focused lints

# 5. Dependency cleanup
just deps-unused               # Find and remove unused deps

# 6. Update security tools
cargo install cargo-audit cargo-deny --force
```

### Performance-Focused Workflow

```bash
# 1. Performance lints
just lint-performance          # Performance-focused clippy lints

# 2. Build performance analysis
just perf-build                # Analyze build times

# 3. Binary size analysis
just perf-size                 # Analyze binary sizes

# 4. Benchmarks
just bench-all                 # Run all benchmarks

# 5. Release build optimization
PROFILE=release just build     # Optimized release build

# 6. Cross-platform performance
PROFILE=release just cross-all # Release builds for all platforms
```

## Release Preparation

### Pre-Release Workflow

```bash
# 1. Version update
# ... update version in Cargo.toml files ...

# 2. Update changelog
# ... document changes ...

# 3. Full validation
just ci                        # Complete CI pipeline

# 4. Cross-platform release builds
PROFILE=release just cross-all

# 5. Package distributions
just cross-package             # Create distribution packages

# 6. Documentation update
just docs-all                  # Complete documentation
just docs-check                # Validate all links

# 7. Security final check
just security-audit
just security-deny
```

### Release Build Workflow

```bash
# 1. Clean environment
just clean-all

# 2. Release builds for all platforms
PROFILE=release just cross-all

# 3. Verify release builds
find target/ -name "*tosic-plugin*" -type f -executable

# 4. Package for distribution
just cross-package

# 5. Test packaged distributions
# ... test extracted packages work correctly ...

# 6. Tag release
git tag v1.0.0
git push --tags
```

### Documentation Release

```bash
# 1. Generate complete documentation
just docs-all                  # All documentation
just docs-private              # Include private APIs

# 2. Validate documentation
just docs-check                # Check links and references
just readme-check              # Validate README

# 3. Serve documentation locally for review
just docs-serve                # Review at localhost:8000

# 4. Generate documentation coverage
just docs-coverage             # Ensure good coverage
```

## Debugging Workflows

### Build Issues

```bash
# 1. Check configuration
just config                    # Current build config
just env                       # Environment variables

# 2. Clean and rebuild
just clean-all
just build

# 3. Verbose build for debugging
VERBOSE=2 just build

# 4. Check dependencies
just deps-tree                 # Dependency tree
just deps-info                 # Detailed dependency info

# 5. Target-specific debugging
TARGET=x86_64-unknown-linux-gnu VERBOSE=2 just build
```

### Test Failures

```bash
# 1. Run specific test suite
just test-unit                 # Unit tests only
just test-integration          # Integration tests only

# 2. Run with verbose output
RUST_BACKTRACE=full just test

# 3. Run single test
cargo test specific_test_name -- --nocapture

# 4. Test with logging
RUST_LOG=debug just test

# 5. Test minimal configuration
FEATURES="" just test          # No features enabled
```

### Cross-Compilation Issues

```bash
# 1. Check target installation
just cross-targets             # List available targets
rustup target list --installed

# 2. Install missing targets
just cross-setup

# 3. Test specific platform
TARGET=aarch64-apple-darwin just build

# 4. Debug cross tool
cross --version
docker --version              # Required for cross tool

# 5. Verbose cross-compilation
VERBOSE=2 just cross-macos
```

### Performance Issues

```bash
# 1. Build timing analysis
just perf-build

# 2. Dependency compilation time
cargo build --timings

# 3. Parallel build optimization
JOBS=8 just build

# 4. Binary size analysis
cargo install cargo-bloat
cargo bloat --release

# 5. Profile optimization
RUSTFLAGS="-C target-cpu=native" just build-release
```

## Performance Optimization

### Build Performance

```bash
# 1. Parallel builds
JOBS=$(nproc) just build       # Use all CPU cores

# 2. Incremental compilation (default in dev)
PROFILE=dev just build

# 3. Build timing analysis
just perf-build
cargo build --timings

# 4. Dependency pre-compilation
cargo build --dependencies-only
```

### Runtime Performance

```bash
# 1. Profile-guided optimization
RUSTFLAGS="-C target-cpu=native" just build-release

# 2. Link-time optimization
RUSTFLAGS="-C lto=fat" just build-release

# 3. Size optimization
RUSTFLAGS="-C opt-level=z" just build-release

# 4. Static linking (musl)
just cross-linux-x64-musl
just cross-linux-arm-musl
```

### Memory Usage Optimization

```bash
# 1. Memory profiling build
RUSTFLAGS="-C force-frame-pointers=yes" just build-release

# 2. Heap analysis
# Use tools like valgrind, heaptrack, or memory profilers

# 3. Stack usage analysis
RUSTFLAGS="-Z emit-stack-sizes" just build

# 4. Binary size analysis
cargo bloat --release --crates
```

## CI/CD Integration

### GitHub Actions Workflow

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  ci:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Nix
        uses: cachix/install-nix-action@v20
        
      - name: Enter Nix environment and run CI
        run: |
          nix develop --command just ci
      
      - name: Cross-platform builds
        run: |
          nix develop --command just cross-all
```

### Local CI Simulation

```bash
# Simulate CI environment locally
just ci                        # Full CI pipeline

# Individual CI steps
just format-check              # Check formatting
just lint-all                  # All linting
just test                      # All tests
just security-audit            # Security checks
just cross-all                 # Cross-compilation
```

### Pre-Commit Hooks

```bash
# Set up pre-commit hook
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
set -e
just lint
just test-unit
just security-audit
EOF

chmod +x .git/hooks/pre-commit
```

### Continuous Deployment

```bash
# Automated release workflow
# 1. Version bump
# 2. Full CI validation
just ci

# 3. Cross-platform release builds
PROFILE=release just cross-all

# 4. Package for distribution
just cross-package

# 5. Deploy to package registries
# cargo publish (when ready)
```

## Advanced Workflows

### Dependency Management

```bash
# 1. Regular dependency updates
just update                    # Update all dependencies
just update-dep tokio          # Update specific dependency

# 2. Security-focused updates
just security-audit            # Check for vulnerabilities
cargo audit --update           # Update vulnerability database

# 3. Dependency analysis
just deps-tree                 # View dependency tree
just deps-info                 # Detailed dependency information
cargo tree --duplicates        # Find duplicate dependencies
```

### Code Analysis

```bash
# 1. Code metrics
just lines                     # Lines of code count
tokei --output json            # Detailed code statistics

# 2. Complexity analysis
# Use tools like cargo-complexity

# 3. Dead code detection
cargo +nightly udeps           # Unused dependencies

# 4. Licensing analysis
just license-check             # License compliance
cargo about generate about.toml # Generate license report
```

### Documentation Workflows

```bash
# 1. Comprehensive documentation
just docs-all                  # All documentation
just docs-private              # Include private APIs

# 2. Documentation validation
just docs-check                # Validate links
just readme-check              # Validate README

# 3. Documentation serving
just docs-serve                # Serve locally
just docs-watch                # Watch and auto-rebuild

# 4. Documentation coverage
just docs-coverage             # Coverage analysis
```

---

*For specific command details, see [BUILD_SYSTEM.md](BUILD_SYSTEM.md)*  
*For development setup, see [DEVELOPMENT.md](DEVELOPMENT.md)*  
*For security practices, see [SECURITY.md](SECURITY.md)*  
*For cross-compilation, see [CROSS_COMPILATION.md](CROSS_COMPILATION.md)*