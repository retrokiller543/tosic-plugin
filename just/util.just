# =============================================================================
# Utility Commands - Project Management & Information
# =============================================================================

# Utility aliases
alias clean-a := clean-all
alias status := project-status
alias deps := deps-tree
alias ver := version

# =============================================================================
# Cleanup Commands
# =============================================================================

# 🧹 Clean all build artifacts
[group('util')]
clean:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{YELLOW}}🧹 Cleaning build artifacts...{{NORMAL}}"
    
    cargo clean
    
    echo "{{BOLD}}{{GREEN}}✅ Build artifacts cleaned{{NORMAL}}"

# 🧹 Deep clean (everything including registry cache)
[group('util')]
clean-all:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{YELLOW}}🧹 Deep cleaning all artifacts...{{NORMAL}}"
    
    # Clean cargo build artifacts
    cargo clean
    
    # Clean test and coverage artifacts
    just test-clean
    
    # Clean documentation
    just docs-clean
    
    # Clean cross-compilation artifacts
    just cross-clean
    
    # Clean any generated files
    rm -f Cargo.lock.bak
    rm -f licenses.html
    rm -f API_SUMMARY.md
    rm -rf dist/
    
    echo "{{BOLD}}{{GREEN}}✅ Deep clean completed{{NORMAL}}"

# =============================================================================
# Project Information
# =============================================================================

# 📊 Show project status
[group('util')]
project-status:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{CYAN}}📊 Tosic Plugin Project Status{{NORMAL}}"
    echo "{{BOLD}}{{CYAN}}=============================={{NORMAL}}"
    echo ""
    
    echo "{{YELLOW}}Project Structure:{{NORMAL}}"
    if command -v tree &> /dev/null; then
        tree crates/ -I target -L 2 2>/dev/null
    else
        find crates/ -type d -not -path "*/target/*" | head -10
    fi
    echo ""
    
    echo "{{YELLOW}}Workspace Packages:{{NORMAL}}"
    cargo metadata --format-version 1 --no-deps | grep '"name"' | head -10
    echo ""
    
    echo "{{YELLOW}}Rust Toolchain:{{NORMAL}}"
    rustc --version
    cargo --version
    echo ""
    
    echo "{{YELLOW}}Git Status:{{NORMAL}}"
    git status --porcelain | head -10 || echo "Not a git repository"

# 🔍 Show dependency information
[group('util')]
deps-tree *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{CYAN}}🔍 Dependency Tree{{NORMAL}}"
    echo "{{BOLD}}{{CYAN}}================{{NORMAL}}"
    echo ""
    
    cmd="cargo tree"
    
    # Add any additional args
    if [ -n "{{args}}" ]; then
        cmd="$cmd {{args}}"
    else
        cmd="$cmd --depth 1"
    fi
    
    eval $cmd

# 📋 Show detailed dependency info
[group('util')]
deps-info:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{CYAN}}📋 Detailed Dependency Information{{NORMAL}}"
    echo "{{BOLD}}{{CYAN}}================================={{NORMAL}}"
    echo ""
    
    echo "{{YELLOW}}🌳 Full Dependency Tree:{{NORMAL}}"
    cargo tree
    echo ""
    
    echo "{{YELLOW}}🔄 Duplicate Dependencies:{{NORMAL}}"
    cargo tree --duplicates
    echo ""
    
    echo "{{YELLOW}}📊 Dependency Statistics:{{NORMAL}}"
    cargo tree --format '{p}' | sort | uniq | wc -l | xargs echo "Total unique dependencies:"

# =============================================================================
# Version Management
# =============================================================================

# 📈 Show version information
[group('util')]
version:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{CYAN}}📈 Version Information{{NORMAL}}"
    echo "{{BOLD}}{{CYAN}}===================={{NORMAL}}"
    echo ""
    
    echo "{{YELLOW}}Package Versions:{{NORMAL}}"
    grep -r "version" crates/*/Cargo.toml | grep -v "rust-version" | head -10
    echo ""
    
    echo "{{YELLOW}}Git Information:{{NORMAL}}"
    git describe --tags --always 2>/dev/null || echo "No git tags found"
    git rev-parse --short HEAD 2>/dev/null || echo "Not a git repository"

# 📈 Set version for all crates
[group('util')]
set-version new_version:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{YELLOW}}📈 Setting version to {{new_version}} for all crates...{{NORMAL}}"
    
    # Update root Cargo.toml
    sed -i.bak 's/version = "[^"]*"/version = "{{new_version}}"/' Cargo.toml
    
    # Update each crate
    find crates -name Cargo.toml -exec sed -i.bak 's/version = "[^"]*"/version = "{{new_version}}"/' {} \;
    
    # Clean up backup files
    find . -name "*.bak" -delete
    
    echo "{{BOLD}}{{GREEN}}✅ Version updated to {{new_version}}{{NORMAL}}"

# =============================================================================
# Environment Information
# =============================================================================

# 🌍 Show environment information
[group('util')]
env:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{CYAN}}🌍 Environment Information{{NORMAL}}"
    echo "{{BOLD}}{{CYAN}}========================={{NORMAL}}"
    echo ""
    
    echo "{{YELLOW}}🦀 Rust Environment:{{NORMAL}}"
    rustc --version
    cargo --version
    rustup --version
    echo ""
    
    echo "{{YELLOW}}🔧 Rustup Toolchains:{{NORMAL}}"
    rustup toolchain list
    echo ""
    
    echo "{{YELLOW}}🎯 Installed Targets:{{NORMAL}}"
    rustup target list --installed
    echo ""
    
    echo "{{YELLOW}}🌐 Environment Variables:{{NORMAL}}"
    env | grep -E "^(RUST|CARGO)" | sort

# 🎛️ Show build configuration
[group('util')]
config:
    #!/usr/bin/env bash
    printf "{{BOLD}}{{CYAN}}🎛️ Build Configuration{{NORMAL}}\n"
    printf "{{BOLD}}{{CYAN}}====================={{NORMAL}}\n"
    printf "\n"
    
    printf "{{YELLOW}}Current justfile variables:{{NORMAL}}\n"
    printf "  Profile: {{profile}}\n"
    printf "  Target: $(if [ -n '{{target}}' ]; then echo '{{target}}'; else echo 'native'; fi)\n"
    printf "  Features: $(if [ -n '{{features}}' ]; then echo '{{features}}'; else echo 'default'; fi)\n"
    printf "  Verbose: {{verbose}}\n"
    printf "  Jobs: $(if [ -n '{{jobs}}' ]; then echo '{{jobs}}'; else echo 'auto'; fi)\n"
    printf "\n"
    
    printf "{{YELLOW}}Actual environment variables (as seen by just):{{NORMAL}}\n"
    printf "  PROFILE=%s\n" "${PROFILE:-dev}"
    printf "  TARGET=%s\n" "${TARGET:-}"
    printf "  FEATURES=%s\n" "${FEATURES:-}"
    printf "  VERBOSE=%s\n" "${VERBOSE:-0}"
    printf "  JOBS=%s\n" "${JOBS:-}"
    printf "\n"
    
    printf "{{YELLOW}}Rust environment variables:{{NORMAL}}\n"
    printf "  RUST_BACKTRACE=%s\n" "${RUST_BACKTRACE:-}"
    printf "  RUST_LOG=%s\n" "${RUST_LOG:-}"
    printf "  RUSTFLAGS=%s\n" "${RUSTFLAGS:-}"
    printf "\n"
    
    printf "{{YELLOW}}Override examples:{{NORMAL}}\n"
    printf "  PROFILE=release just build\n"
    printf "  TARGET=aarch64-apple-darwin just build\n"
    printf "  FEATURES=serde just build\n"
    printf "  VERBOSE=2 just build\n"
    printf "  JOBS=4 just build\n"

# =============================================================================
# Development Utilities
# =============================================================================

# 🔄 Update dependencies
[group('util')]
update:
    @echo "🔄 Updating dependencies..."
    cargo update
    @echo "✅ Dependencies updated"

# 🔄 Update specific dependency
[group('util')]
update-dep dep:
    @echo "{{BOLD}}{{YELLOW}}🔄 Updating dependency: {{dep}}{{NORMAL}}"
    cargo update --package "{{dep}}"
    @echo "{{BOLD}}{{GREEN}}✅ Dependency {{dep}} updated{{NORMAL}}"

# 📝 Add dependency
[group('util')]
add-dep dep *features="":
    @echo "{{BOLD}}{{YELLOW}}📝 Adding dependency: {{dep}}{{NORMAL}}"
    @if [ -n "{{features}}" ]; then \
        cargo add "{{dep}}" --features "{{features}}"; \
    else \
        cargo add "{{dep}}"; \
    fi
    @echo "{{BOLD}}{{GREEN}}✅ Dependency {{dep}} added{{NORMAL}}"

# 📝 Add development dependency
[group('util')]
add-dev-dep dep *features="":
    @echo "{{BOLD}}{{YELLOW}}📝 Adding development dependency: {{dep}}{{NORMAL}}"
    @if [ -n "{{features}}" ]; then \
        cargo add "{{dep}}" --dev --features "{{features}}"; \
    else \
        cargo add "{{dep}}" --dev; \
    fi
    @echo "{{BOLD}}{{GREEN}}✅ Development dependency {{dep}} added{{NORMAL}}"



# =============================================================================
# File Operations
# =============================================================================

# 🔍 Find files by pattern
[group('util')]
find-files pattern:
    @echo "{{BOLD}}{{YELLOW}}🔍 Finding files matching: {{pattern}}{{NORMAL}}"
    find . -name "{{pattern}}" -not -path "./target/*" -not -path "./.git/*"

# 🔍 Search in source files
[group('util')]
search term:
    @echo "{{BOLD}}{{YELLOW}}🔍 Searching for: {{term}}{{NORMAL}}"
    @if command -v rg &> /dev/null; then \
        rg "{{term}}" crates/ --type rust; \
    else \
        grep -r "{{term}}" crates/ --include="*.rs" || echo "No matches found"; \
    fi

# =============================================================================
# Workspace Management
# =============================================================================

# 🏗️ Create new crate in workspace
[group('util')]
new-crate name:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{BLUE}}🏗️ Creating new crate: {{name}}{{NORMAL}}"
    
    # Create crate directory and initialize
    mkdir -p crates/{{name}}
    cd crates/{{name}} && cargo init --lib
    
    echo "{{BOLD}}{{GREEN}}✅ Crate {{name}} created in crates/{{name}}{{NORMAL}}"

# 📋 List workspace crates
[group('util')]
list-crates:
    @echo "{{BOLD}}{{CYAN}}📋 Workspace Crates{{NORMAL}}"
    @echo "{{BOLD}}{{CYAN}}=================={{NORMAL}}"
    @echo ""
    @find crates/ -name Cargo.toml -exec grep -H "^name" {} \; | \
        sed 's|crates/||; s|/Cargo.toml:name = |: |; s|"||g'

# =============================================================================
# Performance Analysis
# =============================================================================

# ⚡ Analyze build performance
[group('util')]
perf-build:
    @echo "{{BOLD}}{{BLUE}}⚡ Analyzing build performance...{{NORMAL}}"
    time cargo build --timings --workspace
    @echo "{{CYAN}}📊 Check target/cargo-timings/ for detailed timing reports{{NORMAL}}"

# 📊 Count lines of code
[group('util')]
lines:
    @echo "{{BOLD}}{{CYAN}}📊 Lines of Code{{NORMAL}}"
    @echo "{{BOLD}}{{CYAN}}================={{NORMAL}}"
    @echo ""
    @if command -v tokei &> /dev/null; then \
        tokei crates/; \
    elif command -v cloc &> /dev/null; then \
        cloc crates/; \
    else \
        echo "{{YELLOW}}Basic line count:{{NORMAL}}"; \
        find crates/ -name "*.rs" -exec wc -l {} \; | awk '{sum += $$1} END {print "Total Rust lines: " sum}'; \
    fi

# =============================================================================
# Git Utilities
# =============================================================================

# 📊 Show git statistics
[group('util')]
git-stats:
    @echo "{{BOLD}}{{CYAN}}📊 Git Statistics{{NORMAL}}"
    @echo "{{BOLD}}{{CYAN}}================={{NORMAL}}"
    @echo ""
    @echo "{{YELLOW}}📈 Commit Count:{{NORMAL}}"
    @git rev-list --count HEAD 2>/dev/null || echo "Not a git repository"
    @echo ""
    @echo "{{YELLOW}}👥 Contributors:{{NORMAL}}"
    @git shortlog -sn 2>/dev/null || echo "Not a git repository"
    @echo ""
    @echo "{{YELLOW}}📅 Recent Activity:{{NORMAL}}"
    @git log --oneline -10 2>/dev/null || echo "Not a git repository"

# =============================================================================
# Help & Information
# =============================================================================

# 📚 Show comprehensive utility help
[group('util')]
util-help:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{CYAN}}📚 Utility Commands Help{{NORMAL}}"
    echo "{{BOLD}}{{CYAN}}======================={{NORMAL}}"
    echo ""
    
    echo "{{YELLOW}}🧹 Cleanup:{{NORMAL}}"
    echo "  clean               - Clean build artifacts"
    echo "  clean-all           - Deep clean everything"
    echo ""
    
    echo "{{YELLOW}}📊 Information:{{NORMAL}}"
    echo "  project-status      - Show project status"
    echo "  deps-tree           - Show dependency tree"
    echo "  version             - Show version information"
    echo "  env                 - Show environment information"
    echo ""
    
    echo "{{YELLOW}}🔄 Dependencies:{{NORMAL}}"
    echo "  update              - Update all dependencies"
    echo "  add-dep <dep>       - Add dependency"
    echo "  remove-dep <dep>    - Remove dependency"
    echo ""
    
    echo "{{YELLOW}}🔍 Search & Files:{{NORMAL}}"
    echo "  find-files <pat>    - Find files by pattern"
    echo "  search <term>       - Search in source files"
    echo "  replace <old> <new> - Search and replace"
    echo ""
    
    echo "{{YELLOW}}🏗️ Workspace:{{NORMAL}}"
    echo "  new-crate <name>    - Create new crate"
    echo "  list-crates         - List workspace crates"