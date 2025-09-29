# Cross-compilation Commands
# =========================
# Build for multiple platforms and architectures

# === Platform Targets ===

# macOS targets
_target_macos_intel := "x86_64-apple-darwin"
_target_macos_arm := "aarch64-apple-darwin"

# Linux targets  
_target_linux_x64 := "x86_64-unknown-linux-gnu"
_target_linux_arm64 := "aarch64-unknown-linux-gnu"
_target_linux_musl_x64 := "x86_64-unknown-linux-musl"
_target_linux_musl_arm64 := "aarch64-unknown-linux-musl"

# Windows targets
_target_windows_x64 := "x86_64-pc-windows-msvc"
_target_windows_x86 := "i686-pc-windows-msvc"
_target_windows_gnu_x64 := "x86_64-pc-windows-gnu"

# WASM targets
_target_wasm32 := "wasm32-unknown-unknown"
_target_wasm32_wasi := "wasm32-wasi"

# === Helper Functions ===

# Cross-compile build command
_cross_build := "cargo build " + _cargo_flags + " " + _jobs_flag + " " + _features_flag + " --workspace"

# Cross-compile for specific target
_build_target target_triple:
    @echo "🌍 Building for {{target_triple}}..."
    {{_cross_build}} --target {{target_triple}}

# Release build for specific target
_build_target_release target_triple:
    @echo "🚀 Building release for {{target_triple}}..."
    {{_cross_build}} --target {{target_triple}} --release

# === macOS Cross-compilation ===

# 🍎 Build for macOS (both Intel and ARM)
cross-macos: cross-macos-intel cross-macos-arm

# 🍎 Build for macOS Intel
cross-macos-intel:
    just _build_target {{_target_macos_intel}}

# 🍎 Build for macOS ARM (Apple Silicon)
cross-macos-arm:
    just _build_target {{_target_macos_arm}}

# 🍎 Build release for macOS (both)
cross-macos-release: cross-macos-intel-release cross-macos-arm-release

# 🍎 Build release for macOS Intel
cross-macos-intel-release:
    just _build_target_release {{_target_macos_intel}}

# 🍎 Build release for macOS ARM
cross-macos-arm-release:
    just _build_target_release {{_target_macos_arm}}

# === Linux Cross-compilation ===

# 🐧 Build for Linux (x64 and ARM64, both glibc and musl)
cross-linux: cross-linux-glibc cross-linux-musl

# 🐧 Build for Linux with glibc
cross-linux-glibc: cross-linux-x64 cross-linux-arm64

# 🐧 Build for Linux with musl (static)
cross-linux-musl: cross-linux-musl-x64 cross-linux-musl-arm64

# 🐧 Build for Linux x64
cross-linux-x64:
    just _build_target {{_target_linux_x64}}

# 🐧 Build for Linux ARM64
cross-linux-arm64:
    just _build_target {{_target_linux_arm64}}

# 🐧 Build for Linux x64 (musl/static)
cross-linux-musl-x64:
    just _build_target {{_target_linux_musl_x64}}

# 🐧 Build for Linux ARM64 (musl/static)
cross-linux-musl-arm64:
    just _build_target {{_target_linux_musl_arm64}}

# 🐧 Build release for Linux (all targets)
cross-linux-release: cross-linux-x64-release cross-linux-arm64-release cross-linux-musl-x64-release cross-linux-musl-arm64-release

# 🐧 Build release for Linux x64
cross-linux-x64-release:
    just _build_target_release {{_target_linux_x64}}

# 🐧 Build release for Linux ARM64
cross-linux-arm64-release:
    just _build_target_release {{_target_linux_arm64}}

# 🐧 Build release for Linux x64 (musl)
cross-linux-musl-x64-release:
    just _build_target_release {{_target_linux_musl_x64}}

# 🐧 Build release for Linux ARM64 (musl)
cross-linux-musl-arm64-release:
    just _build_target_release {{_target_linux_musl_arm64}}

# === Windows Cross-compilation ===

# 🪟 Build for Windows (MSVC and GNU)
cross-windows: cross-windows-msvc cross-windows-gnu

# 🪟 Build for Windows MSVC
cross-windows-msvc: cross-windows-x64 cross-windows-x86

# 🪟 Build for Windows GNU
cross-windows-gnu: cross-windows-gnu-x64

# 🪟 Build for Windows x64 (MSVC)
cross-windows-x64:
    just _build_target {{_target_windows_x64}}

# 🪟 Build for Windows x86 (MSVC)
cross-windows-x86:
    just _build_target {{_target_windows_x86}}

# 🪟 Build for Windows x64 (GNU)
cross-windows-gnu-x64:
    just _build_target {{_target_windows_gnu_x64}}

# 🪟 Build release for Windows (all targets)
cross-windows-release: cross-windows-x64-release cross-windows-x86-release cross-windows-gnu-x64-release

# 🪟 Build release for Windows x64 (MSVC)
cross-windows-x64-release:
    just _build_target_release {{_target_windows_x64}}

# 🪟 Build release for Windows x86 (MSVC)
cross-windows-x86-release:
    just _build_target_release {{_target_windows_x86}}

# 🪟 Build release for Windows x64 (GNU)
cross-windows-gnu-x64-release:
    just _build_target_release {{_target_windows_gnu_x64}}

# === WebAssembly Cross-compilation ===

# 🌐 Build for WebAssembly (all targets)
cross-wasm: cross-wasm32 cross-wasm32-wasi

# 🌐 Build for WebAssembly (unknown-unknown)
cross-wasm32:
    just _build_target {{_target_wasm32}}

# 🌐 Build for WebAssembly (WASI)
cross-wasm32-wasi:
    just _build_target {{_target_wasm32_wasi}}

# 🌐 Build release for WebAssembly (all targets)
cross-wasm-release: cross-wasm32-release cross-wasm32-wasi-release

# 🌐 Build release for WebAssembly (unknown-unknown)
cross-wasm32-release:
    just _build_target_release {{_target_wasm32}}

# 🌐 Build release for WebAssembly (WASI)
cross-wasm32-wasi-release:
    just _build_target_release {{_target_wasm32_wasi}}

# === All Platform Builds ===

# 🌍 Build for all major platforms (development)
cross-all: cross-macos cross-linux cross-windows

# 🌍 Build release for all major platforms
cross-all-release: cross-macos-release cross-linux-release cross-windows-release

# 🌍 Build for all platforms including WASM
cross-everything: cross-all cross-wasm

# 🌍 Build release for everything
cross-everything-release: cross-all-release cross-wasm-release

# === Target Management ===

# 📋 List all installed targets
cross-list-targets:
    @echo "📋 Installed Rust targets:"
    rustup target list --installed

# 📋 List all available targets
cross-list-available:
    @echo "📋 Available Rust targets:"
    rustup target list

# ⚡ Install common cross-compilation targets
cross-install-targets:
    @echo "⚡ Installing common cross-compilation targets..."
    rustup target add {{_target_macos_intel}} {{_target_macos_arm}}
    rustup target add {{_target_linux_x64}} {{_target_linux_arm64}}
    rustup target add {{_target_linux_musl_x64}} {{_target_linux_musl_arm64}}
    rustup target add {{_target_windows_x64}} {{_target_windows_x86}} {{_target_windows_gnu_x64}}
    rustup target add {{_target_wasm32}} {{_target_wasm32_wasi}}

# ⚡ Install specific target
cross-install-target target_triple:
    @echo "⚡ Installing target: {{target_triple}}"
    rustup target add {{target_triple}}

# === Custom Target Builds ===

# 🎯 Build for custom target
cross-build target_triple *args="":
    @echo "🎯 Building for custom target: {{target_triple}}"
    {{_cross_build}} --target {{target_triple}} {{args}}

# 🎯 Build release for custom target
cross-build-release target_triple *args="":
    @echo "🎯 Building release for custom target: {{target_triple}}"
    {{_cross_build}} --target {{target_triple}} --release {{args}}

# === Information ===

# 📊 Show cross-compilation status
cross-status:
    @echo "📊 Cross-compilation Status"
    @echo "=========================="
    @echo ""
    @echo "🍎 macOS Targets:"
    @echo "  Intel x64:     {{_target_macos_intel}}"
    @echo "  Apple Silicon: {{_target_macos_arm}}"
    @echo ""
    @echo "🐧 Linux Targets:"
    @echo "  x64 (glibc):   {{_target_linux_x64}}"
    @echo "  ARM64 (glibc): {{_target_linux_arm64}}"
    @echo "  x64 (musl):    {{_target_linux_musl_x64}}"
    @echo "  ARM64 (musl):  {{_target_linux_musl_arm64}}"
    @echo ""
    @echo "🪟 Windows Targets:"
    @echo "  x64 (MSVC):    {{_target_windows_x64}}"
    @echo "  x86 (MSVC):    {{_target_windows_x86}}"
    @echo "  x64 (GNU):     {{_target_windows_gnu_x64}}"
    @echo ""
    @echo "🌐 WebAssembly Targets:"
    @echo "  WASM32:        {{_target_wasm32}}"
    @echo "  WASM32-WASI:   {{_target_wasm32_wasi}}"
    @echo ""
    @echo "Current configuration:"
    @echo "  Profile: {{profile}}"
    @echo "  Features: {{if features != "" { features } else { "default" }}}"
    @echo ""
    @echo "Installed targets:"
    @rustup target list --installed