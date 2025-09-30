# =============================================================================
# Build Commands
# =============================================================================

# Main build aliases
alias build-a := build-all
alias build-d := build-dev
alias build-r := build-release
alias br := build-release
alias bd := build-dev

# Package-specific aliases
alias bp := build-package
alias bc := build-core

# =============================================================================
# Core Build Commands
# =============================================================================

# 🏗️ Build for development (default)
[group('build')]
build *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{BLUE}}🏗️ Building for development...{{NORMAL}}"
    
    # Build command with configurable options
    cmd="cargo build --workspace"
    
    # Add target if specified
    if [ -n "{{target}}" ]; then
        cmd="$cmd --target {{target}}"
    fi
    
    # Add features if specified
    if [ -n "{{features}}" ]; then
        cmd="$cmd --features {{features}}"
    fi
    
    # Add verbose flag if enabled
    if [ "{{verbose}}" != "0" ]; then
        cmd="$cmd --verbose"
    fi
    
    # Add jobs if specified
    if [ -n "{{jobs}}" ]; then
        cmd="$cmd --jobs {{jobs}}"
    fi
    
    # Add any additional args
    if [ -n "{{args}}" ]; then
        cmd="$cmd {{args}}"
    fi
    
    echo "{{CYAN}}Running: $cmd{{NORMAL}}"
    eval $cmd
    
    if [ $? -eq 0 ]; then
        echo "{{BOLD}}{{GREEN}}✅ Development build completed{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ Development build failed{{NORMAL}}"
        exit 1
    fi

# 🚀 Build optimized release
[group('build')]
build-release *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{BLUE}}🚀 Building optimized release...{{NORMAL}}"
    
    cmd="cargo build --workspace --release"
    
    # Add target if specified
    if [ -n "{{target}}" ]; then
        cmd="$cmd --target {{target}}"
    fi
    
    # Add features if specified
    if [ -n "{{features}}" ]; then
        cmd="$cmd --features {{features}}"
    fi
    
    # Add jobs if specified
    if [ -n "{{jobs}}" ]; then
        cmd="$cmd --jobs {{jobs}}"
    fi
    
    # Add any additional args
    if [ -n "{{args}}" ]; then
        cmd="$cmd {{args}}"
    fi
    
    echo "{{CYAN}}Running: $cmd{{NORMAL}}"
    eval $cmd
    
    if [ $? -eq 0 ]; then
        echo "{{BOLD}}{{GREEN}}✅ Release build completed{{NORMAL}}"
        echo "{{CYAN}}📁 Binaries available in target/release/{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ Release build failed{{NORMAL}}"
        exit 1
    fi

# 🏗️ Build development version (alias for build)
[group('build')]
build-dev *args="": 
    @just build {{args}}

# 📦 Build specific package
[group('build')]
build-package package *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{BLUE}}📦 Building package: {{package}}...{{NORMAL}}"
    
    cmd="cargo build --package {{package}}"
    
    # Add profile based on environment
    if [ "{{profile}}" = "release" ]; then
        cmd="$cmd --release"
    fi
    
    # Add target if specified
    if [ -n "{{target}}" ]; then
        cmd="$cmd --target {{target}}"
    fi
    
    # Add features if specified
    if [ -n "{{features}}" ]; then
        cmd="$cmd --features {{features}}"
    fi
    
    # Add any additional args
    if [ -n "{{args}}" ]; then
        cmd="$cmd {{args}}"
    fi
    
    echo "{{CYAN}}Running: $cmd{{NORMAL}}"
    eval $cmd
    
    if [ $? -eq 0 ]; then
        echo "{{BOLD}}{{GREEN}}✅ Package {{package}} built successfully{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ Package {{package}} build failed{{NORMAL}}"
        exit 1
    fi

# 🧩 Build tosic-plugin-core
[group('build')]
build-core *args="":
    @just build-package tosic-plugin-core {{args}}

# 🔌 Build main tosic-plugin
[group('build')]
build-main *args="":
    @just build-package tosic-plugin {{args}}

# 🏗️ Build all packages
[group('build')]
build-all *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{BLUE}}🏗️ Building all packages...{{NORMAL}}"
    
    just build-core {{args}}
    just build-main {{args}}
    
    echo "{{BOLD}}{{GREEN}}✅ All packages built successfully{{NORMAL}}"

# =============================================================================
# WebAssembly Builds
# =============================================================================

# 🕸️ Build for WebAssembly
[group('build')]
[group('wasm')]
build-wasm *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{BLUE}}🕸️ Building for WebAssembly...{{NORMAL}}"
    
    # Check if wasm32 target is installed
    if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
        echo "{{YELLOW}}📦 Installing wasm32-unknown-unknown target...{{NORMAL}}"
        rustup target add wasm32-unknown-unknown
    fi
    
    cmd="cargo build --workspace --target wasm32-unknown-unknown"
    
    # Add release flag if profile is release
    if [ "{{profile}}" = "release" ]; then
        cmd="$cmd --release"
    fi
    
    # Add features if specified
    if [ -n "{{features}}" ]; then
        cmd="$cmd --features {{features}}"
    fi
    
    # Add any additional args
    if [ -n "{{args}}" ]; then
        cmd="$cmd {{args}}"
    fi
    
    echo "{{CYAN}}Running: $cmd{{NORMAL}}"
    eval $cmd
    
    if [ $? -eq 0 ]; then
        echo "{{BOLD}}{{GREEN}}✅ WebAssembly build completed{{NORMAL}}"
        echo "{{CYAN}}📁 WASM files available in target/wasm32-unknown-unknown/{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ WebAssembly build failed{{NORMAL}}"
        exit 1
    fi

# 📦 Build WASM package using wasm-pack
[group('build')]
[group('wasm')]
build-wasm-pack package="tosic-plugin" *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{BLUE}}📦 Building WASM package with wasm-pack: {{package}}...{{NORMAL}}"
    
    # Check if wasm-pack is available
    if ! command -v wasm-pack &> /dev/null; then
        echo "{{BOLD}}{{RED}}❌ wasm-pack not found. Install it with: cargo install wasm-pack{{NORMAL}}"
        exit 1
    fi
    
    cd "crates/{{package}}"
    
    cmd="wasm-pack build"
    
    # Add release flag if profile is release
    if [ "{{profile}}" = "release" ]; then
        cmd="$cmd --release"
    else
        cmd="$cmd --dev"
    fi
    
    # Add any additional args
    if [ -n "{{args}}" ]; then
        cmd="$cmd {{args}}"
    fi
    
    echo "{{CYAN}}Running: $cmd{{NORMAL}}"
    eval $cmd
    
    if [ $? -eq 0 ]; then
        echo "{{BOLD}}{{GREEN}}✅ WASM package {{package}} built successfully{{NORMAL}}"
        echo "{{CYAN}}📁 Package available in crates/{{package}}/pkg/{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ WASM package {{package}} build failed{{NORMAL}}"
        exit 1
    fi

# =============================================================================
# Check & Validation
# =============================================================================

# ✅ Quick check without building
[group('build')]
check *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{YELLOW}}✅ Running quick check...{{NORMAL}}"
    
    cmd="cargo check --workspace"
    
    # Add target if specified
    if [ -n "{{target}}" ]; then
        cmd="$cmd --target {{target}}"
    fi
    
    # Add features if specified
    if [ -n "{{features}}" ]; then
        cmd="$cmd --features {{features}}"
    fi
    
    # Add any additional args
    if [ -n "{{args}}" ]; then
        cmd="$cmd {{args}}"
    fi
    
    echo "{{CYAN}}Running: $cmd{{NORMAL}}"
    eval $cmd
    
    if [ $? -eq 0 ]; then
        echo "{{BOLD}}{{GREEN}}✅ Check completed successfully{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ Check failed{{NORMAL}}"
        exit 1
    fi

# 🔍 Check specific package
[group('build')]
check-package package *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{YELLOW}}🔍 Checking package: {{package}}...{{NORMAL}}"
    
    cmd="cargo check --package {{package}}"
    
    # Add target if specified
    if [ -n "{{target}}" ]; then
        cmd="$cmd --target {{target}}"
    fi
    
    # Add features if specified
    if [ -n "{{features}}" ]; then
        cmd="$cmd --features {{features}}"
    fi
    
    # Add any additional args
    if [ -n "{{args}}" ]; then
        cmd="$cmd {{args}}"
    fi
    
    echo "{{CYAN}}Running: $cmd{{NORMAL}}"
    eval $cmd
    
    if [ $? -eq 0 ]; then
        echo "{{BOLD}}{{GREEN}}✅ Package {{package}} check completed{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ Package {{package}} check failed{{NORMAL}}"
        exit 1
    fi