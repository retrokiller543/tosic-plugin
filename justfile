# =============================================================================
# Tosic Plugin Build System
# =============================================================================
# A runtime-agnostic plugin system for Rust applications

# Enable unstable features for advanced functionality
set unstable

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
    @echo "{{BOLD}}{{GREEN}}✅ CI pipeline completed successfully{{NORMAL}}"

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
    printf "{{BOLD}}{{CYAN}}Tosic Plugin Build System{{NORMAL}}\n"
    printf "{{BOLD}}{{CYAN}}===========================\n{{NORMAL}}"
    printf "\n"
    printf "{{BOLD}}{{YELLOW}}Project:{{NORMAL}} {{GREEN}}{{PROJECT_NAME}}{{NORMAL}}\n"
    printf "{{BOLD}}{{YELLOW}}Directory:{{NORMAL}} {{BLUE}}{{PROJECT_DIR}}{{NORMAL}}\n"
    printf "\n"
    printf "{{BOLD}}{{CYAN}}Configuration:{{NORMAL}}\n"
    printf "  {{YELLOW}}Profile:{{NORMAL}} {{profile}}\n"
    printf "  {{YELLOW}}Target:{{NORMAL}} ${TARGET:-native}\n"
    printf "  {{YELLOW}}Features:{{NORMAL}} ${FEATURES:-default}\n"
    printf "  {{YELLOW}}Verbose:{{NORMAL}} {{verbose}}\n"
    printf "  {{YELLOW}}Jobs:{{NORMAL}} ${JOBS:-auto}\n"
    printf "\n"
    printf "{{BOLD}}{{CYAN}}Quick Start:{{NORMAL}}\n"
    printf "  {{CYAN}}just dev{{NORMAL}}     - Start development workflow\n"
    printf "  {{CYAN}}just test{{NORMAL}}    - Run all tests\n"
    printf "  {{CYAN}}just release{{NORMAL}} - Build optimized release\n"
    printf "  {{CYAN}}just help{{NORMAL}}    - Show all commands\n"

# Show brief help (alias)
alias h := help