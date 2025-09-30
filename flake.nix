{
  description = "Tosic Plugin - A plugin system to abstract over different plugin runtimes";

  inputs = {
    flake-schemas.url = "https://flakehub.com/f/DeterminateSystems/flake-schemas/*";
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/*";
    rust-overlay = {
      url = "https://flakehub.com/f/oxalica/rust-overlay/0.1.*";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, flake-schemas, nixpkgs, rust-overlay }:
    let
      # Nixpkgs overlays
      overlays = [
        rust-overlay.overlays.default
        (final: prev: {
          rustToolchain = final.rust-bin.stable.latest.default.override { 
            extensions = [ "rust-src" "clippy" "rustfmt" ]; 
          };
        })
      ];

      # Helpers for producing system-specific outputs
      supportedSystems = [ "x86_64-linux" "aarch64-darwin" "x86_64-darwin" "aarch64-linux" ];
      forEachSupportedSystem = f: nixpkgs.lib.genAttrs supportedSystems (system: f {
        pkgs = import nixpkgs { inherit overlays system; };
      });
    in {
      # Schemas tell Nix about the structure of your flake's outputs
      schemas = flake-schemas.schemas;

      # Development environments
      devShells = forEachSupportedSystem ({ pkgs }:
        let
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
        in {
          default = pkgs.mkShell {
            # Pinned packages available in the environment
            packages = with pkgs; [
              # Rust toolchain and core tools
              rustToolchain
              rust-analyzer
              
              # Essential cargo extensions for justfile recipes
              cargo-edit        # add/remove dependencies (just add-dep, remove-dep)
              cargo-watch       # file watching (just watch, test-watch)
              cargo-audit       # security auditing (just security-audit)
              cargo-deny        # dependency policy checking (just security-deny)
              cargo-about       # license compliance (just license-check)  
              cargo-machete     # unused dependency detection (just deps-unused)
              cargo-tarpaulin   # test coverage (just test-coverage)
              cargo-deadlinks   # documentation link checking (just docs-check)
              
              # Additional cargo extensions from fh template
              cargo-bloat       # binary size analysis
              cargo-outdated    # dependency updates
              cargo-udeps       # unused dependencies
              
              # WASM tooling (required for WASM builds)
              wasmtime         # WASM runtime (just build-wasm, cross-wasm-*)
              wasm-pack        # WASM packaging (just build-wasm-pack)
              wasm-tools       # WASM utilities
              
              # Build and development tools
              just             # justfile command runner
              git              # version control
              setupGit         # custom git setup script
              
              # Cross-compilation tools
              # Note: 'cross' cargo extension installed via cargo when needed
              
              # Documentation tools
              miniserve        # HTTP server (just docs-serve)
              python3          # HTTP server fallback (just docs-serve)
              
              # Code analysis and utilities
              tokei            # lines of code counting (just lines)
              tree             # directory tree display (just project-status)
              ripgrep          # fast search (just search)
              fd               # fast find alternative
              
              # Core system tools
              curl             # HTTP requests
              nixpkgs-fmt      # Nix formatting
              nil              # Nix LSP
              
              # Optional tools that justfiles check for
              # Note: cloc, markdown-link-check are optional and installed when needed
            ];

            # Environment variables
            env = {
              RUST_BACKTRACE = "1";
              RUST_LOG = "debug";
              RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
            };

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
          };
        });
    };
}