# Variables with default values
profile := env_var_or_default("PROFILE", "debug")
target := env_var_or_default("TARGET", "")
features := env_var_or_default("FEATURES", "")
verbose := env_var_or_default("VERBOSE", "0")
jobs := env_var_or_default("JOBS", "")

# Utility Commands
# ================
# Project management and utility commands

# === Cleanup Commands ===

# 🧹 Clean all build artifacts
clean-all:
    @echo "🧹 Cleaning all build artifacts..."
    cargo clean
    @echo "✅ All artifacts cleaned"

# 🧹 Clean build artifacts but keep dependencies
clean-builds:
    @echo "🧹 Cleaning build artifacts (keeping dependencies)..."
    find target -name "tosic-plugin*" -type f -delete 2>/dev/null || true
    find target -name "deps" -type d -exec find {} -name "tosic_plugin*" -delete \; 2>/dev/null || true
    @echo "✅ Build artifacts cleaned"

# 🧹 Clean documentation
clean-docs:
    @echo "🧹 Cleaning documentation..."
    cargo clean --doc
    @echo "✅ Documentation cleaned"

# 🧹 Clean test artifacts
clean-tests:
    @echo "🧹 Cleaning test artifacts..."
    find . -name "*.profraw" -delete 2>/dev/null || true
    find target -name "*test*" -type f -delete 2>/dev/null || true
    @echo "✅ Test artifacts cleaned"

# 🧹 Deep clean (everything including registry cache)
clean-deep:
    @echo "🧹 Deep cleaning (including registry cache)..."
    cargo clean
    rm -rf ~/.cargo/registry/cache
    rm -rf ~/.cargo/git/db
    @echo "✅ Deep clean completed"

# === Project Information ===

# 📊 Show project status
status:
    @echo "📊 Tosic Plugin Project Status"
    @echo "=============================="
    @echo ""
    @echo "📁 Project Structure:"
    @tree crates/ -I target -L 2 2>/dev/null || find crates/ -type d -not -path "*/target/*" | head -10
    @echo ""
    @echo "📦 Workspace Packages:"
    @cargo metadata --format-version 1 --no-deps | grep '"name"' | head -10
    @echo ""
    @echo "🔧 Rust Toolchain:"
    @rustc --version
    @cargo --version
    @echo ""
    @echo "🎯 Git Status:"
    @git status --porcelain | head -10 || echo "Not a git repository"

# 📋 Show dependency information
deps:
    @echo "📋 Dependency Information"
    @echo "========================"
    @echo ""
    @echo "🌳 Dependency Tree (top level):"
    @cargo tree --depth 1
    @echo ""
    @echo "📊 Dependency Statistics:"
    @cargo tree --format '{p}' | sort | uniq | wc -l | xargs echo "Total unique dependencies:"

# 📋 Show detailed dependency info
deps-detailed:
    @echo "📋 Detailed Dependency Information"
    @echo "=================================="
    @echo ""
    @echo "🌳 Full Dependency Tree:"
    @cargo tree
    @echo ""
    @echo "🔄 Duplicate Dependencies:"
    @cargo tree --duplicates

# 🎯 Show build targets
targets:
    @echo "🎯 Available Build Targets"
    @echo "========================="
    @echo ""
    @echo "🦀 Installed Rust Targets:"
    @rustup target list --installed
    @echo ""
    @echo "📱 Default Target:"
    @rustc -vV | grep host

# === Environment Information ===

# 🌍 Show environment information
env:
    @echo "🌍 Environment Information"
    @echo "========================="
    @echo ""
    @echo "🦀 Rust Environment:"
    @rustc --version
    @cargo --version
    @rustup --version
    @echo ""
    @echo "🔧 Rustup Toolchains:"
    @rustup toolchain list
    @echo ""
    @echo "📦 Cargo Configuration:"
    @cargo config get || echo "No custom cargo config"
    @echo ""
    @echo "🌐 Environment Variables:"
    @env | grep -E "^(RUST|CARGO)" | sort

# 🎛️ Show build configuration
config:
    @echo "🎛️ Build Configuration"
    @echo "====================="
    @echo ""
    @echo "Current settings:"
    @echo "  Profile: {{profile}}"
    @echo "  Target: {{if target != "" { target } else { "native" }}}"
    @echo "  Features: {{if features != "" { features } else { "default" }}}"
    @echo "  Verbose: {{verbose}}"
    @echo "  Jobs: {{if jobs != "" { jobs } else { "auto" }}}"
    @echo ""
    @echo "Override with environment variables:"
    @echo "  PROFILE=release just build"
    @echo "  TARGET=aarch64-apple-darwin just build"
    @echo "  FEATURES=serde just build"
    @echo "  VERBOSE=2 just build"
    @echo "  JOBS=4 just build"

# === Development Utilities ===

# 🔄 Update dependencies
update:
    @echo "🔄 Updating dependencies..."
    cargo update
    @echo "✅ Dependencies updated"

# 🔄 Update specific dependency
update-dep dep:
    @echo "🔄 Updating dependency: {{dep}}"
    cargo update --package "{{dep}}"
    @echo "✅ Dependency {{dep}} updated"

# 📝 Add dependency
add-dep dep *features="":
    #!/usr/bin/env bash
    echo "📝 Adding dependency: {{dep}}"
    if [ "{{features}}" != "" ]; then
        cargo add {{dep}} --features "{{features}}"
    else
        cargo add {{dep}}
    fi
    echo "✅ Dependency {{dep}} added"

# 📝 Add development dependency
add-dev-dep dep *features="":
    #!/usr/bin/env bash
    echo "📝 Adding development dependency: {{dep}}"
    if [ "{{features}}" != "" ]; then
        cargo add {{dep}} --dev --features "{{features}}"
    else
        cargo add {{dep}} --dev
    fi
    echo "✅ Development dependency {{dep}} added"

# 🗑️ Remove dependency
remove-dep dep:
    @echo "🗑️ Removing dependency: {{dep}}"
    cargo remove {{dep}}
    @echo "✅ Dependency {{dep}} removed"

# === Version Management ===

# 📈 Show version information
version:
    #!/usr/bin/env bash
    echo "📈 Version Information"
    echo "====================="
    echo ""
    grep -r "version" crates/*/Cargo.toml | grep -v "rust-version" | head -10
    echo ""
    echo "Git information:"
    git describe --tags --always 2>/dev/null || echo "No git tags found"
    git rev-parse --short HEAD 2>/dev/null || echo "Not a git repository"

# 📈 Set version for all crates
set-version new_version:
    #!/usr/bin/env bash
    echo "📈 Setting version to {{new_version}} for all crates..."
    sed -i.bak 's/version = "[^"]*"/version = "{{new_version}}"/' Cargo.toml
    find crates -name Cargo.toml -exec sed -i.bak 's/version = "[^"]*"/version = "{{new_version}}"/' {} \;
    find . -name "*.bak" -delete
    echo "✅ Version updated to {{new_version}}"

# === File Operations ===

# 🔍 Find files by pattern
find-files pattern:
    @echo "🔍 Finding files matching: {{pattern}}"
    @find . -name "{{pattern}}" -not -path "./target/*" -not -path "./.git/*"

# 🔍 Search in source files
search term:
    #!/usr/bin/env bash
    echo "🔍 Searching for: {{term}}"
    grep -r "{{term}}" crates/ --include="*.rs" || echo "No matches found"

# 🔍 Search and replace in source files
replace old new:
    #!/usr/bin/env bash
    echo "🔍 Replacing '{{old}}' with '{{new}}' in source files..."
    find crates/ -name "*.rs" -exec sed -i.bak 's/{{old}}/{{new}}/g' {} \;
    find . -name "*.bak" -delete
    echo "✅ Replacement completed"

# === Git Utilities ===

# 📝 Initialize git repository with basic setup
git-init:
    @echo "📝 Initializing git repository..."
    @if [ ! -d .git ]; then \
        git init; \
        echo "target/" > .gitignore; \
        echo "Cargo.lock" >> .gitignore; \
        echo "**/*.rs.bk" >> .gitignore; \
        echo ".env" >> .gitignore; \
        git add .; \
        git commit -m "Initial commit"; \
        echo "✅ Git repository initialized"; \
    else \
        echo "⚠️ Git repository already exists"; \
    fi

# 📊 Show git statistics
git-stats:
    #!/usr/bin/env bash
    echo "📊 Git Statistics"
    echo "================"
    echo ""
    echo "📈 Commit Count:"
    git rev-list --count HEAD 2>/dev/null || echo "Not a git repository"
    echo ""
    echo "👥 Contributors:"
    git shortlog -sn 2>/dev/null || echo "Not a git repository"
    echo ""
    echo "📅 Recent Activity:"
    git log --oneline -10 2>/dev/null || echo "Not a git repository"

# === Performance Analysis ===

# ⚡ Analyze build performance
perf-build:
    #!/usr/bin/env bash
    echo "⚡ Analyzing build performance..."
    time cargo build --timings
    echo "📊 Check target/cargo-timings/ for detailed timing reports"

# ⚡ Analyze binary size
perf-size:
    @echo "⚡ Analyzing binary sizes..."
    @if [ -d target/release ]; then \
        find target/release -name "tosic*" -type f -exec ls -lh {} \; 2>/dev/null || echo "No release binaries found"; \
    else \
        echo "No release build found. Run 'just release' first"; \
    fi

# === Workspace Management ===

# 🏗️ Create new crate in workspace
new-crate name:
    #!/usr/bin/env bash
    echo "🏗️ Creating new crate: {{name}}"
    mkdir -p crates/{{name}}
    cd crates/{{name}} && cargo init --lib

    # Add to workspace members in root Cargo.toml
    if grep -q "members = \[" Cargo.toml; then
        sed -i.bak '/members = \[/a\
    "crates/{{name}}",' Cargo.toml
    else
        echo "" >> Cargo.toml
        echo "[workspace]" >> Cargo.toml
        echo "members = [" >> Cargo.toml
        echo '    "crates/{{name}}",' >> Cargo.toml
        echo "]" >> Cargo.toml
    fi

    find . -name "*.bak" -delete
    echo "✅ Crate {{name}} created in crates/{{name}}"

# 📋 List workspace crates
list-crates:
    @echo "📋 Workspace Crates"
    @echo "=================="
    @echo ""
    @find crates/ -name Cargo.toml -exec grep -H "^name" {} \; | sed 's|crates/||; s|/Cargo.toml:name = |: |; s|"||g'

# === Help and Information ===

# 📚 Show comprehensive utility help
util-help:
    @echo "📚 Utility Commands Help"
    @echo "======================="
    @echo ""
    @echo "🧹 Cleanup:"
    @echo "  clean-all           - Clean all build artifacts"
    @echo "  clean-builds        - Clean builds (keep dependencies)"
    @echo "  clean-docs          - Clean documentation"
    @echo "  clean-deep          - Deep clean including caches"
    @echo ""
    @echo "📊 Information:"
    @echo "  status              - Show project status"
    @echo "  deps                - Show dependency information"
    @echo "  targets             - Show build targets"
    @echo "  env                 - Show environment information"
    @echo "  version             - Show version information"
    @echo ""
    @echo "🔄 Dependencies:"
    @echo "  update              - Update all dependencies"
    @echo "  add-dep <dep>       - Add dependency"
    @echo "  remove-dep <dep>    - Remove dependency"
    @echo ""
    @echo "🔍 Search & Files:"
    @echo "  find-files <pat>    - Find files by pattern"
    @echo "  search <term>       - Search in source files"
    @echo "  replace <old> <new> - Search and replace"
    @echo ""
    @echo "🏗️ Workspace:"
    @echo "  new-crate <name>    - Create new crate"
    @echo "  list-crates         - List workspace crates"