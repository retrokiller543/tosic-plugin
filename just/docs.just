# =============================================================================
# Documentation Commands
# =============================================================================

# Documentation aliases
alias docs-o := docs-open
alias docs-s := docs-serve
alias docs-c := docs-clean

# =============================================================================
# Core Documentation Commands
# =============================================================================

# 📚 Generate documentation
[group('docs')]
docs *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{CYAN}}📚 Generating documentation...{{NORMAL}}"
    
    cmd="cargo doc --workspace --no-deps"
    
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
        echo "{{BOLD}}{{GREEN}}✅ Documentation generated{{NORMAL}}"
        echo "{{CYAN}}📁 Documentation available in target/doc/{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ Documentation generation failed{{NORMAL}}"
        exit 1
    fi

# 📚 Generate and open documentation
[group('docs')]
docs-open *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{CYAN}}📚 Generating and opening documentation...{{NORMAL}}"
    
    cmd="cargo doc --workspace --no-deps --open"
    
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
        echo "{{BOLD}}{{GREEN}}✅ Documentation generated and opened{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ Documentation generation failed{{NORMAL}}"
        exit 1
    fi

# 📚 Generate documentation with dependencies
[group('docs')]
docs-all *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{CYAN}}📚 Generating documentation with dependencies...{{NORMAL}}"
    
    cmd="cargo doc --workspace"
    
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
        echo "{{BOLD}}{{GREEN}}✅ Full documentation generated{{NORMAL}}"
        echo "{{CYAN}}📁 Documentation available in target/doc/{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ Documentation generation failed{{NORMAL}}"
        exit 1
    fi

# 📚 Generate documentation for specific package
[group('docs')]
docs-package package *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{CYAN}}📚 Generating documentation for package: {{package}}...{{NORMAL}}"
    
    cmd="cargo doc --package {{package}} --no-deps"
    
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
        echo "{{BOLD}}{{GREEN}}✅ Package {{package}} documentation generated{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ Package {{package}} documentation generation failed{{NORMAL}}"
        exit 1
    fi

# =============================================================================
# Documentation Server
# =============================================================================

# 🌐 Serve documentation locally
[group('docs')]
docs-serve port="8000":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{CYAN}}🌐 Serving documentation on port {{port}}...{{NORMAL}}"
    
    # Check if documentation exists
    if [ ! -d "target/doc" ]; then
        echo "{{YELLOW}}📚 Documentation not found, generating first...{{NORMAL}}"
        just docs
    fi
    
    # Check if miniserve is available
    if command -v miniserve &> /dev/null; then
        echo "{{CYAN}}Using miniserve to serve documentation...{{NORMAL}}"
        miniserve target/doc --port {{port}} --index index.html
    elif command -v python3 &> /dev/null; then
        echo "{{CYAN}}Using Python to serve documentation...{{NORMAL}}"
        cd target/doc && python3 -m http.server {{port}}
    elif command -v python &> /dev/null; then
        echo "{{CYAN}}Using Python to serve documentation...{{NORMAL}}"
        cd target/doc && python -m SimpleHTTPServer {{port}}
    else
        echo "{{BOLD}}{{RED}}❌ No web server available{{NORMAL}}"
        echo "{{YELLOW}}Install miniserve: cargo install miniserve{{NORMAL}}"
        echo "{{YELLOW}}Or use Python: python3 -m http.server {{port}}{{NORMAL}}"
        exit 1
    fi

# 🌐 Watch and serve documentation
[group('docs')]
docs-watch port="8000":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{CYAN}}🌐 Watching and serving documentation...{{NORMAL}}"
    
    # Check if cargo-watch is available
    if ! command -v cargo-watch &> /dev/null; then
        echo "{{YELLOW}}📦 Installing cargo-watch...{{NORMAL}}"
        cargo install cargo-watch
    fi
    
    echo "{{CYAN}}Starting documentation watch mode on port {{port}}...{{NORMAL}}"
    echo "{{YELLOW}}Open http://localhost:{{port}} in your browser{{NORMAL}}"
    
    # Start background server and watch for changes
    cargo watch -x "doc --workspace --no-deps" &
    WATCH_PID=$!
    
    # Serve documentation
    just docs-serve {{port}}
    
    # Cleanup on exit
    kill $WATCH_PID 2>/dev/null || true

# =============================================================================
# Documentation Validation
# =============================================================================

# 🔍 Check documentation links
[group('docs')]
docs-check:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{YELLOW}}🔍 Checking documentation links...{{NORMAL}}"
    
    # Check if cargo-deadlinks is available
    if ! command -v cargo-deadlinks &> /dev/null; then
        echo "{{YELLOW}}📦 Installing cargo-deadlinks...{{NORMAL}}"
        cargo install cargo-deadlinks
    fi
    
    echo "{{CYAN}}Checking for broken links...{{NORMAL}}"
    cargo deadlinks
    
    if [ $? -eq 0 ]; then
        echo "{{BOLD}}{{GREEN}}✅ No broken links found{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ Broken links detected{{NORMAL}}"
        exit 1
    fi

# 🔍 Validate documentation coverage
[group('docs')]
docs-coverage:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{YELLOW}}🔍 Checking documentation coverage...{{NORMAL}}"
    
    # Use cargo to check for missing docs
    export RUSTDOCFLAGS="-D missing_docs"
    
    cmd="cargo doc --workspace --no-deps"
    
    # Add features if specified
    if [ -n "{{features}}" ]; then
        cmd="$cmd --features {{features}}"
    fi
    
    echo "{{CYAN}}Running: $cmd{{NORMAL}}"
    eval $cmd
    
    if [ $? -eq 0 ]; then
        echo "{{BOLD}}{{GREEN}}✅ Documentation coverage check passed{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ Missing documentation detected{{NORMAL}}"
        exit 1
    fi

# =============================================================================
# API Documentation
# =============================================================================

# 📖 Generate private documentation
[group('docs')]
docs-private:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{CYAN}}📖 Generating private documentation...{{NORMAL}}"
    
    cmd="cargo doc --workspace --no-deps --document-private-items"
    
    # Add features if specified
    if [ -n "{{features}}" ]; then
        cmd="$cmd --features {{features}}"
    fi
    
    echo "{{CYAN}}Running: $cmd{{NORMAL}}"
    eval $cmd
    
    if [ $? -eq 0 ]; then
        echo "{{BOLD}}{{GREEN}}✅ Private documentation generated{{NORMAL}}"
        echo "{{CYAN}}📁 Documentation available in target/doc/{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ Private documentation generation failed{{NORMAL}}"
        exit 1
    fi

# 📖 Generate examples documentation
[group('docs')]
docs-examples:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{CYAN}}📖 Generating examples documentation...{{NORMAL}}"
    
    if [ -d "examples" ]; then
        echo "{{CYAN}}Found examples directory{{NORMAL}}"
        cargo doc --workspace --no-deps --examples
        
        if [ $? -eq 0 ]; then
            echo "{{BOLD}}{{GREEN}}✅ Examples documentation generated{{NORMAL}}"
        else
            echo "{{BOLD}}{{RED}}❌ Examples documentation generation failed{{NORMAL}}"
            exit 1
        fi
    else
        echo "{{YELLOW}}No examples directory found{{NORMAL}}"
    fi

# =============================================================================
# Documentation Cleanup
# =============================================================================

# 🧹 Clean documentation
[group('docs')]
docs-clean:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{YELLOW}}🧹 Cleaning documentation...{{NORMAL}}"
    
    cargo clean --doc
    
    echo "{{BOLD}}{{GREEN}}✅ Documentation cleaned{{NORMAL}}"

# =============================================================================
# README and Markdown
# =============================================================================

# 📝 Check README links
[group('docs')]
readme-check:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{YELLOW}}📝 Checking README links...{{NORMAL}}"
    
    if [ -f "README.md" ]; then
        # Check if markdown-link-check is available
        if command -v markdown-link-check &> /dev/null; then
            markdown-link-check README.md
            
            if [ $? -eq 0 ]; then
                echo "{{BOLD}}{{GREEN}}✅ README links are valid{{NORMAL}}"
            else
                echo "{{BOLD}}{{RED}}❌ Broken links found in README{{NORMAL}}"
                exit 1
            fi
        else
            echo "{{YELLOW}}markdown-link-check not found{{NORMAL}}"
            echo "{{YELLOW}}Install with: npm install -g markdown-link-check{{NORMAL}}"
        fi
    else
        echo "{{YELLOW}}No README.md found{{NORMAL}}"
    fi

# 📝 Generate API summary
[group('docs')]
api-summary:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{CYAN}}📝 Generating API summary...{{NORMAL}}"
    
    echo "# API Summary" > API_SUMMARY.md
    echo "" >> API_SUMMARY.md
    echo "Generated on: $(date)" >> API_SUMMARY.md
    echo "" >> API_SUMMARY.md
    
    # List public modules and functions
    echo "## Public API" >> API_SUMMARY.md
    echo "" >> API_SUMMARY.md
    
    for crate in crates/*/; do
        if [ -d "$crate" ]; then
            crate_name=$(basename "$crate")
            echo "### $crate_name" >> API_SUMMARY.md
            echo "" >> API_SUMMARY.md
            
            # Extract public items (simplified)
            if [ -f "$crate/src/lib.rs" ]; then
                grep -n "^pub " "$crate/src/lib.rs" | head -10 >> API_SUMMARY.md
            fi
            echo "" >> API_SUMMARY.md
        fi
    done
    
    echo "{{BOLD}}{{GREEN}}✅ API summary generated: API_SUMMARY.md{{NORMAL}}"

# =============================================================================
# Documentation Utilities
# =============================================================================

# 📊 Documentation statistics
[group('docs')]
docs-stats:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{CYAN}}📊 Documentation Statistics{{NORMAL}}"
    echo "{{BOLD}}{{CYAN}}========================={{NORMAL}}"
    echo ""
    
    if [ -d "target/doc" ]; then
        echo "{{YELLOW}}Generated Documentation:{{NORMAL}}"
        echo "  HTML files: $(find target/doc -name "*.html" | wc -l)"
        echo "  Total size: $(du -sh target/doc | cut -f1)"
        echo ""
    else
        echo "{{YELLOW}}No documentation found{{NORMAL}}"
        echo "  Run 'just docs' to generate documentation"
        echo ""
    fi
    
    echo "{{YELLOW}}Source Documentation:{{NORMAL}}"
    echo "  Rust files with doc comments: $(find crates -name "*.rs" -exec grep -l "///" {} \; | wc -l)"
    echo "  Total doc comments: $(find crates -name "*.rs" -exec grep -c "///" {} \; | awk '{sum += $1} END {print sum}')"
    echo ""
    
    echo "{{YELLOW}}README Files:{{NORMAL}}"
    echo "  README files: $(find . -name "README*" -not -path "./target/*" | wc -l)"
    echo "  Markdown files: $(find . -name "*.md" -not -path "./target/*" | wc -l)"