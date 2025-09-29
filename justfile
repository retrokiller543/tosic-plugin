# =============================================================================
# Tosic Plugin Build System
# =============================================================================
# A runtime-agnostic plugin system for Rust applications

# Project configuration
PROJECT_DIR := justfile_directory()
PROJECT_NAME := "tosic-plugin"

# Color variables for output formatting
BOLD := '\033[1m'
NORMAL := '\033[0m'
RED := '\033[31m'
GREEN := '\033[32m'
YELLOW := '\033[33m'
BLUE := '\033[34m'
CYAN := '\033[36m'

# Build configuration with defaults
profile := env_var_or_default("PROFILE", "dev")
target := env_var_or_default("TARGET", "")
features := env_var_or_default("FEATURES", "")
verbose := env_var_or_default("VERBOSE", "0")
jobs := env_var_or_default("JOBS", "")

# =============================================================================
# Module Imports
# =============================================================================

import 'just/build.just'
import 'just/test.just'
import 'just/lint.just'
import 'just/docs.just'
import 'just/cross.just'
import 'just/util.just'

# =============================================================================
# Main Commands & Aliases
# =============================================================================

# Default recipe - show help
[private]
default: help

# Main command aliases
alias b := build
alias t := test
alias r := build-release
alias c := clean
alias l := lint
alias d := docs

# Development workflow aliases
alias dev := develop
alias ci := pipeline
alias w := watch

# Cross-compilation aliases
alias cross := cross-all
alias wasm := build-wasm

# Testing aliases
alias cov := test-coverage
alias bench := test-bench

# =============================================================================
# Main Workflow Commands
# =============================================================================

# 🚀 Start development workflow (build + test + watch)
[group('workflow')]
develop *args="":
    @echo -e "{{BOLD}}{{BLUE}}🚀 Starting development workflow...{{NORMAL}}"
    just build {{args}}
    just test

# 🏗️ Complete CI pipeline
[group('workflow')]
pipeline: clean lint test build-release
    @echo -e "{{BOLD}}{{GREEN}}✅ CI pipeline completed successfully{{NORMAL}}"

# 🎯 Quick development iteration
[group('workflow')]
quick *args="": 
    @echo -e "{{BOLD}}{{CYAN}}🎯 Quick development iteration...{{NORMAL}}"
    just build {{args}}
    just test

# 🔄 Watch for changes and rebuild
[group('workflow')]
watch:
    @echo -e "{{BOLD}}{{YELLOW}}👀 Watching for changes...{{NORMAL}}"
    cargo watch -x "check --workspace" -x "test --workspace --lib"

# =============================================================================
# Help & Information
# =============================================================================

# Show available commands
help:
    @just --list --unsorted

# Show project configuration
[group('info')]
info:
    #!/usr/bin/env bash
    echo -e "{{BOLD}}{{CYAN}}Tosic Plugin Build System{{NORMAL}}"
    echo -e "{{BOLD}}{{CYAN}}==========================={{NORMAL}}"
    echo ""
    echo -e "{{BOLD}}{{YELLOW}}Project:{{NORMAL}} {{GREEN}}{{PROJECT_NAME}}{{NORMAL}}"
    echo -e "{{BOLD}}{{YELLOW}}Directory:{{NORMAL}} {{BLUE}}{{PROJECT_DIR}}{{NORMAL}}"
    echo ""
    echo -e "{{BOLD}}{{CYAN}}Configuration:{{NORMAL}}"
    echo -e "  {{YELLOW}}Profile:{{NORMAL}} {{profile}}"
    echo -e "  {{YELLOW}}Target:{{NORMAL}} ${TARGET:-native}"
    echo -e "  {{YELLOW}}Features:{{NORMAL}} ${FEATURES:-default}"
    echo -e "  {{YELLOW}}Verbose:{{NORMAL}} {{verbose}}"
    echo -e "  {{YELLOW}}Jobs:{{NORMAL}} ${JOBS:-auto}"
    echo ""
    echo -e "{{BOLD}}{{CYAN}}Quick Start:{{NORMAL}}"
    echo -e "  {{CYAN}}just dev{{NORMAL}}     - Start development workflow"
    echo -e "  {{CYAN}}just test{{NORMAL}}    - Run all tests"
    echo -e "  {{CYAN}}just release{{NORMAL}} - Build optimized release"
    echo -e "  {{CYAN}}just help{{NORMAL}}    - Show all commands"

# Show brief help (alias)
alias h := help