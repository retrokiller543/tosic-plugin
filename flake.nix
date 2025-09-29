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
            cargo-watch
            cargo-edit
            cargo-audit
            
            # Development tools
            git
            setupGit
            
            # Optional: useful for plugin development
            pkg-config
            openssl
            
            # Nix tools
            nixpkgs-fmt
            nil
          ];

          shellHook = ''
            echo "🦀 Tosic Plugin Development Environment"
            echo "Rust version: $(rustc --version)"
            echo "Cargo version: $(cargo --version)"
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
            echo "Available commands:"
            echo "  cargo build      - Build the project"
            echo "  cargo test       - Run tests"
            echo "  cargo watch -x check  - Watch for changes and check"
            echo "  setup-git        - Manually configure git from .env"
          '';

          # Environment variables
          RUST_BACKTRACE = "1";
          RUST_LOG = "debug";
        };
      });
}