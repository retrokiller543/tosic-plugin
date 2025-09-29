{
  description = "Tosic Plugin - A plugin system to abstract over different plugin runtimes";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "clippy" "rustfmt" ];
        };

        # Helper script to configure git from environment
        setupGit = pkgs.writeScriptBin "setup-git" ''
          #!${pkgs.bash}/bin/bash
          if [ -f .env ]; then
            source .env
            if [ -n "$GIT_USER_EMAIL" ]; then
              echo "Setting git user.email to: $GIT_USER_EMAIL"
              git config user.email "$GIT_USER_EMAIL"
            fi
            if [ -n "$GIT_USER_NAME" ]; then
              echo "Setting git user.name to: $GIT_USER_NAME"
              git config user.name "$GIT_USER_NAME"
            fi
          else
            echo "No .env file found, using default git configuration"
          fi
        '';

      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust toolchain
            rustToolchain
            
            # Cargo extensions
            cargo-watch
            cargo-edit
            cargo-audit
            cargo-outdated
            cargo-license
            cargo-machete
            cargo-tarpaulin  # for test coverage
            
            # WASM tooling
            wasmtime
            wasm-pack
            wasm-tools
            
            # Build and development tools
            just  # justfile command runner
            git
            setupGit
            
            # Cross-compilation and linking
            pkg-config
            openssl
            
            # Documentation and analysis
            tokei  # code statistics
            tree   # directory tree display
            
            # JavaScript/Node.js (for potential JS runtime support)
            nodejs_20
            
            # Python (for documentation serving)
            python3
            
            # Nix tools
            nixpkgs-fmt
            nil
            
            # Optional: Additional development utilities
            fd      # fast find alternative
            ripgrep # fast grep alternative
            bat     # better cat with syntax highlighting
            
            # HTTP server tools for documentation
            miniserve
          ];

          shellHook = ''
            echo "🦀 Tosic Plugin Development Environment"
            echo "======================================="
            echo "Rust version: $(rustc --version)"
            echo "Cargo version: $(cargo --version)"
            echo "Just version: $(just --version 2>/dev/null || echo "not found")"
            echo ""
            
            # Auto-configure git if .env exists
            if [ -f .env ]; then
              echo "📧 Configuring git from .env file..."
              setup-git
            else
              echo "💡 Create a .env file with GIT_USER_EMAIL and GIT_USER_NAME to auto-configure git"
              echo "   Example: echo 'GIT_USER_EMAIL=work@company.com' > .env"
            fi
            echo ""
            echo "🚀 Quick Start Commands:"
            echo "  just dev         - Start development workflow (build + test + watch)"
            echo "  just test        - Run all tests"
            echo "  just release     - Build optimized release"
            echo "  just help        - Show all available commands"
            echo ""
            echo "🌍 Cross-compilation:"
            echo "  just cross-macos     - Build for macOS (Intel + ARM)"
            echo "  just cross-linux     - Build for Linux (x64 + ARM)"
            echo "  just cross-windows   - Build for Windows"
            echo ""
            echo "🛠️  WASM Support Ready:"
            echo "  wasmtime --version   - $(wasmtime --version 2>/dev/null || echo "not found")"
            echo "  wasm-pack --version  - $(wasm-pack --version 2>/dev/null || echo "not found")"
            echo ""
            echo "🔧 Development Tools:"
            echo "  just lint        - Run linting and formatting"
            echo "  just docs-open   - Generate and open documentation"
            echo "  just clean       - Clean build artifacts"
            echo ""
            echo "For a complete list of commands: just help"
          '';

          # Environment variables
          RUST_BACKTRACE = "1";
          RUST_LOG = "debug";
        };
      });
}