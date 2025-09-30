# =============================================================================
# Cross-Compilation Commands
# =============================================================================

# Cross-compilation aliases
alias cross-a := cross-all
alias cross-m := cross-macos
alias cross-l := cross-linux
alias cross-w := cross-windows
alias cross-wasm := cross-wasm-all

# Platform-specific aliases
alias macos := cross-macos
alias linux := cross-linux
alias windows := cross-windows
alias wasm := cross-wasm-all

# =============================================================================
# Cross-Compilation Setup
# =============================================================================

# 🔧 Install cross-compilation targets
[group('cross')]
cross-setup:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{BLUE}}🔧 Setting up cross-compilation targets...{{NORMAL}}"
    
    echo "{{CYAN}}Installing Rust targets...{{NORMAL}}"
    
    # macOS targets
    rustup target add x86_64-apple-darwin
    rustup target add aarch64-apple-darwin
    
    # Linux targets
    rustup target add x86_64-unknown-linux-gnu
    rustup target add aarch64-unknown-linux-gnu
    rustup target add x86_64-unknown-linux-musl
    rustup target add aarch64-unknown-linux-musl
    
    # Windows targets
    rustup target add x86_64-pc-windows-gnu
    rustup target add x86_64-pc-windows-msvc
    
    # WebAssembly targets
    rustup target add wasm32-unknown-unknown
    rustup target add wasm32-wasi
    
    echo "{{BOLD}}{{GREEN}}✅ Cross-compilation targets installed{{NORMAL}}"

# 🔧 Install cross tool
[group('cross')]
cross-install:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{BLUE}}🔧 Installing cross tool...{{NORMAL}}"
    
    # Check if cross is already installed
    if command -v cross &> /dev/null; then
        echo "{{YELLOW}}cross tool already installed{{NORMAL}}"
        cross --version
    else
        echo "{{CYAN}}Installing cross...{{NORMAL}}"
        cargo install cross
        echo "{{BOLD}}{{GREEN}}✅ cross tool installed{{NORMAL}}"
    fi

# =============================================================================
# All Platforms
# =============================================================================

# 🌍 Cross-compile for all platforms
[group('cross')]
cross-all:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{BLUE}}🌍 Cross-compiling for all platforms...{{NORMAL}}"
    
    just cross-macos
    just cross-linux  
    just cross-windows
    
    echo "{{BOLD}}{{GREEN}}✅ Cross-compilation for all platforms completed{{NORMAL}}"

# 🌍 Cross-compile release for all platforms
[group('cross')]
cross-all-release:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{BLUE}}🌍 Cross-compiling release for all platforms...{{NORMAL}}"
    
    just cross-macos-release
    just cross-linux-release
    just cross-windows-release
    
    echo "{{BOLD}}{{GREEN}}✅ Release cross-compilation for all platforms completed{{NORMAL}}"

# =============================================================================
# macOS Cross-Compilation
# =============================================================================

# 🍎 Cross-compile for macOS (both architectures)
[group('cross')]
[group('macos')]
cross-macos:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{BLUE}}🍎 Cross-compiling for macOS...{{NORMAL}}"
    
    just cross-macos-intel
    just cross-macos-arm
    
    echo "{{BOLD}}{{GREEN}}✅ macOS cross-compilation completed{{NORMAL}}"

# 🍎 Cross-compile for macOS Intel
[group('cross')]
[group('macos')]
cross-macos-intel *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{BLUE}}🍎 Cross-compiling for macOS Intel (x86_64)...{{NORMAL}}"
    
    cmd="cargo build --workspace --target x86_64-apple-darwin"
    
    # Add profile based on environment
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
        echo "{{BOLD}}{{GREEN}}✅ macOS Intel build completed{{NORMAL}}"
        echo "{{CYAN}}📁 Binaries in target/x86_64-apple-darwin/{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ macOS Intel build failed{{NORMAL}}"
        exit 1
    fi

# 🍎 Cross-compile for macOS ARM
[group('cross')]
[group('macos')]
cross-macos-arm *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{BLUE}}🍎 Cross-compiling for macOS ARM (aarch64)...{{NORMAL}}"
    
    cmd="cargo build --workspace --target aarch64-apple-darwin"
    
    # Add profile based on environment
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
        echo "{{BOLD}}{{GREEN}}✅ macOS ARM build completed{{NORMAL}}"
        echo "{{CYAN}}📁 Binaries in target/aarch64-apple-darwin/{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ macOS ARM build failed{{NORMAL}}"
        exit 1
    fi

# 🍎 Cross-compile release for macOS
[group('cross')]
[group('macos')]
cross-macos-release:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{BLUE}}🍎 Cross-compiling release for macOS...{{NORMAL}}"
    
    PROFILE=release just cross-macos
    
    echo "{{BOLD}}{{GREEN}}✅ macOS release cross-compilation completed{{NORMAL}}"

# =============================================================================
# Linux Cross-Compilation
# =============================================================================

# 🐧 Cross-compile for Linux (all variants)
[group('cross')]
[group('linux')]
cross-linux:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{BLUE}}🐧 Cross-compiling for Linux...{{NORMAL}}"
    
    just cross-linux-glibc
    just cross-linux-musl
    
    echo "{{BOLD}}{{GREEN}}✅ Linux cross-compilation completed{{NORMAL}}"

# 🐧 Cross-compile for Linux glibc (both architectures)
[group('cross')]
[group('linux')]
cross-linux-glibc:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{BLUE}}🐧 Cross-compiling for Linux glibc...{{NORMAL}}"
    
    just cross-linux-x64-glibc
    just cross-linux-arm-glibc
    
    echo "{{BOLD}}{{GREEN}}✅ Linux glibc cross-compilation completed{{NORMAL}}"

# 🐧 Cross-compile for Linux musl (both architectures)
[group('cross')]
[group('linux')]
cross-linux-musl:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{BLUE}}🐧 Cross-compiling for Linux musl...{{NORMAL}}"
    
    just cross-linux-x64-musl
    just cross-linux-arm-musl
    
    echo "{{BOLD}}{{GREEN}}✅ Linux musl cross-compilation completed{{NORMAL}}"

# 🐧 Cross-compile for Linux x64 glibc
[group('cross')]
[group('linux')]
cross-linux-x64-glibc *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{BLUE}}🐧 Cross-compiling for Linux x64 glibc...{{NORMAL}}"
    
    # Use cross tool for better Linux compatibility
    if command -v cross &> /dev/null; then
        cmd="cross build --workspace --target x86_64-unknown-linux-gnu"
    else
        cmd="cargo build --workspace --target x86_64-unknown-linux-gnu"
    fi
    
    # Add profile based on environment
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
        echo "{{BOLD}}{{GREEN}}✅ Linux x64 glibc build completed{{NORMAL}}"
        echo "{{CYAN}}📁 Binaries in target/x86_64-unknown-linux-gnu/{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ Linux x64 glibc build failed{{NORMAL}}"
        exit 1
    fi

# 🐧 Cross-compile for Linux ARM glibc
[group('cross')]
[group('linux')]
cross-linux-arm-glibc *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{BLUE}}🐧 Cross-compiling for Linux ARM glibc...{{NORMAL}}"
    
    # Use cross tool for better Linux compatibility
    if command -v cross &> /dev/null; then
        cmd="cross build --workspace --target aarch64-unknown-linux-gnu"
    else
        cmd="cargo build --workspace --target aarch64-unknown-linux-gnu"
    fi
    
    # Add profile based on environment
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
        echo "{{BOLD}}{{GREEN}}✅ Linux ARM glibc build completed{{NORMAL}}"
        echo "{{CYAN}}📁 Binaries in target/aarch64-unknown-linux-gnu/{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ Linux ARM glibc build failed{{NORMAL}}"
        exit 1
    fi

# 🐧 Cross-compile for Linux x64 musl (static)
[group('cross')]
[group('linux')]
cross-linux-x64-musl *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{BLUE}}🐧 Cross-compiling for Linux x64 musl (static)...{{NORMAL}}"
    
    # Use cross tool for better Linux compatibility
    if command -v cross &> /dev/null; then
        cmd="cross build --workspace --target x86_64-unknown-linux-musl"
    else
        cmd="cargo build --workspace --target x86_64-unknown-linux-musl"
    fi
    
    # Add profile based on environment
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
        echo "{{BOLD}}{{GREEN}}✅ Linux x64 musl build completed{{NORMAL}}"
        echo "{{CYAN}}📁 Static binaries in target/x86_64-unknown-linux-musl/{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ Linux x64 musl build failed{{NORMAL}}"
        exit 1
    fi

# 🐧 Cross-compile for Linux ARM musl (static)
[group('cross')]
[group('linux')]
cross-linux-arm-musl *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{BLUE}}🐧 Cross-compiling for Linux ARM musl (static)...{{NORMAL}}"
    
    # Use cross tool for better Linux compatibility
    if command -v cross &> /dev/null; then
        cmd="cross build --workspace --target aarch64-unknown-linux-musl"
    else
        cmd="cargo build --workspace --target aarch64-unknown-linux-musl"
    fi
    
    # Add profile based on environment
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
        echo "{{BOLD}}{{GREEN}}✅ Linux ARM musl build completed{{NORMAL}}"
        echo "{{CYAN}}📁 Static binaries in target/aarch64-unknown-linux-musl/{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ Linux ARM musl build failed{{NORMAL}}"
        exit 1
    fi

# 🐧 Cross-compile release for Linux
[group('cross')]
[group('linux')]
cross-linux-release:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{BLUE}}🐧 Cross-compiling release for Linux...{{NORMAL}}"
    
    PROFILE=release just cross-linux
    
    echo "{{BOLD}}{{GREEN}}✅ Linux release cross-compilation completed{{NORMAL}}"

# =============================================================================
# Windows Cross-Compilation
# =============================================================================

# 🪟 Cross-compile for Windows
[group('cross')]
[group('windows')]
cross-windows:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{BLUE}}🪟 Cross-compiling for Windows...{{NORMAL}}"
    
    just cross-windows-gnu
    just cross-windows-msvc
    
    echo "{{BOLD}}{{GREEN}}✅ Windows cross-compilation completed{{NORMAL}}"

# 🪟 Cross-compile for Windows GNU
[group('cross')]
[group('windows')]
cross-windows-gnu *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{BLUE}}🪟 Cross-compiling for Windows GNU...{{NORMAL}}"
    
    # Use cross tool for better Windows compatibility
    if command -v cross &> /dev/null; then
        cmd="cross build --workspace --target x86_64-pc-windows-gnu"
    else
        cmd="cargo build --workspace --target x86_64-pc-windows-gnu"
    fi
    
    # Add profile based on environment
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
        echo "{{BOLD}}{{GREEN}}✅ Windows GNU build completed{{NORMAL}}"
        echo "{{CYAN}}📁 Binaries in target/x86_64-pc-windows-gnu/{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ Windows GNU build failed{{NORMAL}}"
        exit 1
    fi

# 🪟 Cross-compile for Windows MSVC
[group('cross')]
[group('windows')]
cross-windows-msvc *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{BLUE}}🪟 Cross-compiling for Windows MSVC...{{NORMAL}}"
    
    cmd="cargo build --workspace --target x86_64-pc-windows-msvc"
    
    # Add profile based on environment
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
        echo "{{BOLD}}{{GREEN}}✅ Windows MSVC build completed{{NORMAL}}"
        echo "{{CYAN}}📁 Binaries in target/x86_64-pc-windows-msvc/{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ Windows MSVC build failed{{NORMAL}}"
        echo "{{YELLOW}}Note: MSVC target may require Visual Studio tools{{NORMAL}}"
        exit 1
    fi

# 🪟 Cross-compile release for Windows
[group('cross')]
[group('windows')]
cross-windows-release:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{BLUE}}🪟 Cross-compiling release for Windows...{{NORMAL}}"
    
    PROFILE=release just cross-windows
    
    echo "{{BOLD}}{{GREEN}}✅ Windows release cross-compilation completed{{NORMAL}}"

# =============================================================================
# WebAssembly Cross-Compilation
# =============================================================================

# 🕸️ Cross-compile for WebAssembly (all targets)
[group('cross')]
[group('wasm')]
cross-wasm-all:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{BLUE}}🕸️ Cross-compiling for WebAssembly...{{NORMAL}}"
    
    just cross-wasm-unknown
    just cross-wasm-wasi
    
    echo "{{BOLD}}{{GREEN}}✅ WebAssembly cross-compilation completed{{NORMAL}}"

# 🕸️ Cross-compile for WASM unknown
[group('cross')]
[group('wasm')]
cross-wasm-unknown *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{BLUE}}🕸️ Cross-compiling for WASM unknown...{{NORMAL}}"
    
    # Check if wasm32 target is installed
    if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
        echo "{{YELLOW}}📦 Installing wasm32-unknown-unknown target...{{NORMAL}}"
        rustup target add wasm32-unknown-unknown
    fi
    
    cmd="cargo build --workspace --target wasm32-unknown-unknown"
    
    # Add profile based on environment
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
        echo "{{BOLD}}{{GREEN}}✅ WASM unknown build completed{{NORMAL}}"
        echo "{{CYAN}}📁 WASM files in target/wasm32-unknown-unknown/{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ WASM unknown build failed{{NORMAL}}"
        exit 1
    fi

# 🕸️ Cross-compile for WASM WASI
[group('cross')]
[group('wasm')]
cross-wasm-wasi *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{BLUE}}🕸️ Cross-compiling for WASM WASI...{{NORMAL}}"
    
    # Check if wasm32-wasi target is installed
    if ! rustup target list --installed | grep -q "wasm32-wasi"; then
        echo "{{YELLOW}}📦 Installing wasm32-wasi target...{{NORMAL}}"
        rustup target add wasm32-wasi
    fi
    
    cmd="cargo build --workspace --target wasm32-wasi"
    
    # Add profile based on environment
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
        echo "{{BOLD}}{{GREEN}}✅ WASM WASI build completed{{NORMAL}}"
        echo "{{CYAN}}📁 WASM files in target/wasm32-wasi/{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ WASM WASI build failed{{NORMAL}}"
        exit 1
    fi

# =============================================================================
# Cross-Compilation Utilities
# =============================================================================

# 📋 List available targets
[group('cross')]
cross-targets:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{CYAN}}📋 Available Cross-Compilation Targets{{NORMAL}}"
    echo "{{BOLD}}{{CYAN}}====================================={{NORMAL}}"
    echo ""
    
    echo "{{YELLOW}}Installed Targets:{{NORMAL}}"
    rustup target list --installed
    echo ""
    
    echo "{{YELLOW}}Available Targets:{{NORMAL}}"
    rustup target list | grep -v "(installed)" | head -20
    echo ""
    echo "{{CYAN}}Use 'rustup target add <target>' to install more targets{{NORMAL}}"

# 🧹 Clean cross-compilation artifacts
[group('cross')]
cross-clean:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{YELLOW}}🧹 Cleaning cross-compilation artifacts...{{NORMAL}}"
    
    # Clean specific target directories
    targets=(
        "x86_64-apple-darwin"
        "aarch64-apple-darwin"
        "x86_64-unknown-linux-gnu"
        "aarch64-unknown-linux-gnu"
        "x86_64-unknown-linux-musl"
        "aarch64-unknown-linux-musl"
        "x86_64-pc-windows-gnu"
        "x86_64-pc-windows-msvc"
        "wasm32-unknown-unknown"
        "wasm32-wasi"
    )
    
    for target in "${targets[@]}"; do
        if [ -d "target/$target" ]; then
            echo "{{CYAN}}Cleaning $target...{{NORMAL}}"
            rm -rf "target/$target"
        fi
    done
    
    echo "{{BOLD}}{{GREEN}}✅ Cross-compilation artifacts cleaned{{NORMAL}}"

# 📦 Package cross-compiled binaries
[group('cross')]
cross-package:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{CYAN}}📦 Packaging cross-compiled binaries...{{NORMAL}}"
    
    # Create dist directory
    mkdir -p dist
    
    # Package each target
    targets=(
        "x86_64-apple-darwin:macos-intel"
        "aarch64-apple-darwin:macos-arm"
        "x86_64-unknown-linux-gnu:linux-x64-glibc"
        "aarch64-unknown-linux-gnu:linux-arm-glibc"
        "x86_64-unknown-linux-musl:linux-x64-musl"
        "aarch64-unknown-linux-musl:linux-arm-musl"
        "x86_64-pc-windows-gnu:windows-gnu"
        "x86_64-pc-windows-msvc:windows-msvc"
    )
    
    for target_info in "${targets[@]}"; do
        target="${target_info%%:*}"
        name="${target_info##*:}"
        
        target_dir="target/$target/release"
        if [ -d "$target_dir" ]; then
            echo "{{CYAN}}Packaging $name...{{NORMAL}}"
            
            # Create package directory
            package_dir="dist/tosic-plugin-$name"
            mkdir -p "$package_dir"
            
            # Copy binaries (adjust extension for Windows)
            if [[ "$target" == *"windows"* ]]; then
                cp "$target_dir"/*.exe "$package_dir/" 2>/dev/null || true
            else
                # Copy all executable files
                find "$target_dir" -maxdepth 1 -type f -executable -not -name "*.d" -exec cp {} "$package_dir/" \;
            fi
            
            # Create archive
            cd dist
            if command -v tar &> /dev/null; then
                tar -czf "tosic-plugin-$name.tar.gz" "tosic-plugin-$name"
            fi
            cd ..
        fi
    done
    
    echo "{{BOLD}}{{GREEN}}✅ Cross-compiled binaries packaged in dist/{{NORMAL}}"