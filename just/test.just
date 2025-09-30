# =============================================================================
# Test Commands - Comprehensive Testing & Coverage
# =============================================================================

# Test aliases
alias test-a := test-all
alias test-c := test-coverage
alias test-i := test-integration
alias test-u := test-unit
alias test-d := test-doc

# Benchmark aliases  
alias bench := test-bench
alias bench-a := bench-all

# Coverage aliases
alias cov := test-coverage
alias cov-html := coverage-html
alias cov-lcov := coverage-lcov

# =============================================================================
# Core Test Commands
# =============================================================================

# 🧪 Run all tests
[group('test')]
test *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{GREEN}}🧪 Running all tests...{{NORMAL}}"
    
    cmd="cargo test --workspace"
    
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
        echo "{{BOLD}}{{GREEN}}✅ All tests passed{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ Some tests failed{{NORMAL}}"
        exit 1
    fi

# 🧪 Run unit tests only
[group('test')]
test-unit *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{GREEN}}🧪 Running unit tests...{{NORMAL}}"
    
    cmd="cargo test --workspace --lib"
    
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
        echo "{{BOLD}}{{GREEN}}✅ Unit tests passed{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ Unit tests failed{{NORMAL}}"
        exit 1
    fi

# 🧪 Run integration tests
[group('test')]
test-integration *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{GREEN}}🧪 Running integration tests...{{NORMAL}}"
    
    cmd="cargo test --workspace --test '*'"
    
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
        echo "{{BOLD}}{{GREEN}}✅ Integration tests passed{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ Integration tests failed{{NORMAL}}"
        exit 1
    fi

# 📚 Run documentation tests
[group('test')]
test-doc *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{GREEN}}📚 Running documentation tests...{{NORMAL}}"
    
    cmd="cargo test --workspace --doc"
    
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
        echo "{{BOLD}}{{GREEN}}✅ Documentation tests passed{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ Documentation tests failed{{NORMAL}}"
        exit 1
    fi

# 🧪 Run all test types
[group('test')]
test-all:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{GREEN}}🧪 Running comprehensive test suite...{{NORMAL}}"
    
    just test-unit
    just test-integration  
    just test-doc
    
    echo "{{BOLD}}{{GREEN}}✅ All test types completed{{NORMAL}}"

# 🧪 Test specific package
[group('test')]
test-package package *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{GREEN}}🧪 Testing package: {{package}}...{{NORMAL}}"
    
    cmd="cargo test --package {{package}}"
    
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
        echo "{{BOLD}}{{GREEN}}✅ Package {{package}} tests passed{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ Package {{package}} tests failed{{NORMAL}}"
        exit 1
    fi

# =============================================================================
# Test Coverage
# =============================================================================

# 📊 Generate test coverage
[group('test')]
[group('coverage')]
test-coverage *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{YELLOW}}📊 Generating test coverage...{{NORMAL}}"
    
    # Check if cargo-tarpaulin is installed
    if ! command -v cargo-tarpaulin &> /dev/null; then
        echo "{{YELLOW}}📦 Installing cargo-tarpaulin...{{NORMAL}}"
        cargo install cargo-tarpaulin
    fi
    
    cmd="cargo tarpaulin --workspace --out Html --out Lcov"
    
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
        echo "{{BOLD}}{{GREEN}}✅ Coverage report generated{{NORMAL}}"
        echo "{{CYAN}}📁 HTML report: tarpaulin-report.html{{NORMAL}}"
        echo "{{CYAN}}📁 LCOV report: lcov.info{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ Coverage generation failed{{NORMAL}}"
        exit 1
    fi

# 📊 Generate HTML coverage report
[group('coverage')]
coverage-html *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{YELLOW}}📊 Generating HTML coverage report...{{NORMAL}}"
    
    # Check if cargo-tarpaulin is installed
    if ! command -v cargo-tarpaulin &> /dev/null; then
        echo "{{YELLOW}}📦 Installing cargo-tarpaulin...{{NORMAL}}"
        cargo install cargo-tarpaulin
    fi
    
    cmd="cargo tarpaulin --workspace --out Html"
    
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
        echo "{{BOLD}}{{GREEN}}✅ HTML coverage report generated: tarpaulin-report.html{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ HTML coverage generation failed{{NORMAL}}"
        exit 1
    fi

# 📊 Generate LCOV coverage report
[group('coverage')]
coverage-lcov *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{YELLOW}}📊 Generating LCOV coverage report...{{NORMAL}}"
    
    # Check if cargo-tarpaulin is installed
    if ! command -v cargo-tarpaulin &> /dev/null; then
        echo "{{YELLOW}}📦 Installing cargo-tarpaulin...{{NORMAL}}"
        cargo install cargo-tarpaulin
    fi
    
    cmd="cargo tarpaulin --workspace --out Lcov"
    
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
        echo "{{BOLD}}{{GREEN}}✅ LCOV coverage report generated: lcov.info{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ LCOV coverage generation failed{{NORMAL}}"
        exit 1
    fi

# =============================================================================
# Benchmarks
# =============================================================================

# ⚡ Run benchmarks
[group('test')]
[group('bench')]
test-bench *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{BLUE}}⚡ Running benchmarks...{{NORMAL}}"
    
    cmd="cargo bench --workspace"
    
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
        echo "{{BOLD}}{{GREEN}}✅ Benchmarks completed{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ Benchmarks failed{{NORMAL}}"
        exit 1
    fi

# ⚡ Run all benchmarks (comprehensive)
[group('bench')]
bench-all:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{BLUE}}⚡ Running comprehensive benchmarks...{{NORMAL}}"
    
    # Check if criterion is available (for more detailed benchmarks)
    if cargo tree | grep -q "criterion"; then
        echo "{{CYAN}}Running criterion benchmarks...{{NORMAL}}"
        just test-bench --features bench
    else
        echo "{{CYAN}}Running standard benchmarks...{{NORMAL}}"
        just test-bench
    fi
    
    echo "{{BOLD}}{{GREEN}}✅ All benchmarks completed{{NORMAL}}"

# ⚡ Benchmark specific package
[group('bench')]
bench-package package *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{BLUE}}⚡ Benchmarking package: {{package}}...{{NORMAL}}"
    
    cmd="cargo bench --package {{package}}"
    
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
        echo "{{BOLD}}{{GREEN}}✅ Package {{package}} benchmarks completed{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ Package {{package}} benchmarks failed{{NORMAL}}"
        exit 1
    fi

# =============================================================================
# Advanced Testing
# =============================================================================

# 🕸️ Test WASM builds
[group('test')]
[group('wasm')]
test-wasm *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{GREEN}}🕸️ Testing WASM builds...{{NORMAL}}"
    
    # Check if wasm32 target is available
    if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
        echo "{{YELLOW}}📦 Installing wasm32-unknown-unknown target...{{NORMAL}}"
        rustup target add wasm32-unknown-unknown
    fi
    
    cmd="cargo test --workspace --target wasm32-unknown-unknown"
    
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
        echo "{{BOLD}}{{GREEN}}✅ WASM tests passed{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ WASM tests failed{{NORMAL}}"
        exit 1
    fi

# 🧪 Test with minimal features
[group('test')]
test-minimal *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{GREEN}}🧪 Testing with minimal features...{{NORMAL}}"
    
    cmd="cargo test --workspace --no-default-features"
    
    # Add target if specified
    if [ -n "{{target}}" ]; then
        cmd="$cmd --target {{target}}"
    fi
    
    # Add any additional args
    if [ -n "{{args}}" ]; then
        cmd="$cmd {{args}}"
    fi
    
    echo "{{CYAN}}Running: $cmd{{NORMAL}}"
    eval $cmd
    
    if [ $? -eq 0 ]; then
        echo "{{BOLD}}{{GREEN}}✅ Minimal feature tests passed{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ Minimal feature tests failed{{NORMAL}}"
        exit 1
    fi

# 🧪 Test with all features
[group('test')]
test-all-features *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{GREEN}}🧪 Testing with all features...{{NORMAL}}"
    
    cmd="cargo test --workspace --all-features"
    
    # Add target if specified
    if [ -n "{{target}}" ]; then
        cmd="$cmd --target {{target}}"
    fi
    
    # Add any additional args
    if [ -n "{{args}}" ]; then
        cmd="$cmd {{args}}"
    fi
    
    echo "{{CYAN}}Running: $cmd{{NORMAL}}"
    eval $cmd
    
    if [ $? -eq 0 ]; then
        echo "{{BOLD}}{{GREEN}}✅ All feature tests passed{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ All feature tests failed{{NORMAL}}"
        exit 1
    fi

# =============================================================================
# Watch Mode
# =============================================================================

# 👀 Watch for changes and run tests
[group('test')]
test-watch:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{YELLOW}}👀 Watching for test changes...{{NORMAL}}"
    
    # Check if cargo-watch is available
    if ! command -v cargo-watch &> /dev/null; then
        echo "{{YELLOW}}📦 Installing cargo-watch...{{NORMAL}}"
        cargo install cargo-watch
    fi
    
    echo "{{CYAN}}Starting test watch mode...{{NORMAL}}"
    cargo watch -x "test --workspace --lib"

# 👀 Watch for changes and run all tests
[group('test')]
test-watch-all:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{YELLOW}}👀 Watching for all test changes...{{NORMAL}}"
    
    # Check if cargo-watch is available
    if ! command -v cargo-watch &> /dev/null; then
        echo "{{YELLOW}}📦 Installing cargo-watch...{{NORMAL}}"
        cargo install cargo-watch
    fi
    
    echo "{{CYAN}}Starting comprehensive test watch mode...{{NORMAL}}"
    cargo watch -x "test --workspace"

# =============================================================================
# Test Utilities
# =============================================================================

# 🧹 Clean test artifacts
[group('test')]
test-clean:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{YELLOW}}🧹 Cleaning test artifacts...{{NORMAL}}"
    
    # Remove coverage reports
    rm -f tarpaulin-report.html lcov.info cobertura.xml
    
    # Remove criterion benchmark data
    rm -rf target/criterion
    
    # Remove any profraw files
    find . -name "*.profraw" -delete 2>/dev/null || true
    
    echo "{{BOLD}}{{GREEN}}✅ Test artifacts cleaned{{NORMAL}}"

# 📊 Show test summary
[group('test')]
test-summary:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{CYAN}}📊 Test Summary{{NORMAL}}"
    echo "{{BOLD}}{{CYAN}}============={{NORMAL}}"
    echo ""
    
    # Count test files
    echo "{{YELLOW}}Test Files:{{NORMAL}}"
    find . -name "*.rs" -path "*/tests/*" -o -name "lib.rs" -o -name "main.rs" | \
        xargs grep -l "#\[test\]" 2>/dev/null | wc -l | \
        xargs echo "  Unit test files:"
    
    find . -name "*.rs" -path "*/tests/*" | wc -l | \
        xargs echo "  Integration test files:"
    
    echo ""
    echo "{{YELLOW}}Coverage Files:{{NORMAL}}"
    [ -f "tarpaulin-report.html" ] && echo "  ✅ HTML report: tarpaulin-report.html" || echo "  ❌ No HTML coverage report"
    [ -f "lcov.info" ] && echo "  ✅ LCOV report: lcov.info" || echo "  ❌ No LCOV coverage report"
    
    echo ""
    echo "{{YELLOW}}Benchmark Data:{{NORMAL}}"
    [ -d "target/criterion" ] && echo "  ✅ Criterion data available" || echo "  ❌ No benchmark data"