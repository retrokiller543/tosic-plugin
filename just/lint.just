# =============================================================================
# Lint Commands - Comprehensive Security & Quality Analysis
# =============================================================================

# Lint aliases
alias lint-a := lint-all
alias lint-s := lint-security
alias lint-p := lint-performance
alias lint-q := lint

# Security aliases
alias sec := lint-security
alias audit := security-audit
alias deny := security-deny

# =============================================================================
# Core Linting Commands
# =============================================================================

# 🔍 Quick lint (clippy + fmt check)
[group('lint')]
lint *args="":
    #!/usr/bin/env bash
    echo "{{BOLD}}{{YELLOW}}🔍 Running quick lint...{{NORMAL}}"
    
    echo "{{CYAN}}Running clippy...{{NORMAL}}"
    cargo clippy --workspace --all-targets --all-features -- -D warnings {{args}}
    
    echo "{{CYAN}}Checking formatting...{{NORMAL}}"
    cargo fmt --all -- --check
    
    echo "{{BOLD}}{{GREEN}}✅ Quick lint completed{{NORMAL}}"

# 🔍 Comprehensive lint (all checks)
[group('lint')]
lint-all:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{YELLOW}}🔍 Running comprehensive lint...{{NORMAL}}"
    
    just lint-security
    just lint-performance
    just lint-code-quality
    just format-check
    
    echo "{{BOLD}}{{GREEN}}✅ Comprehensive lint completed{{NORMAL}}"

# ⚡ Performance-focused linting
[group('lint')]
lint-performance:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{YELLOW}}⚡ Running performance lints...{{NORMAL}}"
    
    cargo clippy --workspace --all-targets --all-features -- \
        -W clippy::perf \
        -W clippy::nursery \
        -W clippy::inefficient_to_string \
        -W clippy::unnecessary_to_owned \
        -W clippy::clone_on_ref_ptr \
        -W clippy::large_types_passed_by_value \
        -W clippy::too_many_arguments \
        -W clippy::trivially_copy_pass_by_ref \
        -W clippy::redundant_allocation \
        -W clippy::vec_init_then_push
    
    echo "{{BOLD}}{{GREEN}}✅ Performance lints completed{{NORMAL}}"

# 🛡️ Security-focused linting
[group('lint')]
[group('security')]
lint-security:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{YELLOW}}🛡️ Running security lints...{{NORMAL}}"
    
    # Security-focused clippy lints
    cargo clippy --workspace --all-targets --all-features -- \
        -W clippy::suspicious \
        -W clippy::cargo \
        -W clippy::undocumented_unsafe_blocks \
        -W clippy::multiple_unsafe_ops_per_block \
        -W clippy::mem_forget \
        -W clippy::mem_replace_with_uninit \
        -W clippy::panic \
        -W clippy::panic_in_result_fn \
        -W clippy::unimplemented \
        -W clippy::unreachable \
        -W clippy::todo \
        -W clippy::unwrap_used \
        -W clippy::expect_used \
        -W clippy::indexing_slicing \
        -W clippy::integer_arithmetic \
        -W clippy::float_arithmetic \
        -W clippy::as_conversions \
        -W clippy::cast_lossless \
        -W clippy::cast_possible_truncation \
        -W clippy::cast_possible_wrap \
        -W clippy::cast_precision_loss \
        -W clippy::cast_sign_loss
    
    echo "{{BOLD}}{{GREEN}}✅ Security lints completed{{NORMAL}}"

# 📝 Code quality linting
[group('lint')]
lint-code-quality:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{YELLOW}}📝 Running code quality lints...{{NORMAL}}"
    
    cargo clippy --workspace --all-targets --all-features -- \
        -W clippy::complexity \
        -W clippy::style \
        -W clippy::correctness \
        -W clippy::cognitive_complexity \
        -W clippy::too_many_lines \
        -W clippy::missing_docs_in_private_items \
        -W clippy::missing_errors_doc \
        -W clippy::missing_safety_doc \
        -W clippy::doc_markdown \
        -W clippy::redundant_else \
        -W clippy::redundant_closure \
        -W clippy::redundant_pattern_matching \
        -W clippy::single_match_else \
        -W clippy::verbose_bit_mask \
        -W clippy::module_name_repetitions \
        -W clippy::similar_names
    
    echo "{{BOLD}}{{GREEN}}✅ Code quality lints completed{{NORMAL}}"

# =============================================================================
# Security Audit Commands
# =============================================================================

# 🔒 Security audit with cargo-audit
[group('security')]
security-audit:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{YELLOW}}🔒 Running security audit...{{NORMAL}}"
    
    # Check if cargo-audit is installed
    if ! command -v cargo-audit &> /dev/null; then
        echo "{{YELLOW}}📦 Installing cargo-audit...{{NORMAL}}"
        cargo install cargo-audit
    fi
    
    echo "{{CYAN}}Updating advisory database...{{NORMAL}}"
    cargo audit --update
    
    echo "{{CYAN}}Running audit...{{NORMAL}}"
    cargo audit
    
    echo "{{BOLD}}{{GREEN}}✅ Security audit completed{{NORMAL}}"

# 🚫 Dependency policy checking with cargo-deny
[group('security')]
security-deny:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{YELLOW}}🚫 Running dependency policy check...{{NORMAL}}"
    
    # Check if cargo-deny is installed
    if ! command -v cargo-deny &> /dev/null; then
        echo "{{YELLOW}}📦 Installing cargo-deny...{{NORMAL}}"
        cargo install cargo-deny
    fi
    
    # Create deny.toml if it doesn't exist
    if [ ! -f "deny.toml" ]; then
        echo "{{YELLOW}}📝 Creating deny.toml configuration...{{NORMAL}}"
        cargo deny init
    fi
    
    echo "{{CYAN}}Running dependency policy check...{{NORMAL}}"
    cargo deny check
    
    echo "{{BOLD}}{{GREEN}}✅ Dependency policy check completed{{NORMAL}}"

# 🔍 Unused dependency detection with cargo-machete
[group('security')]
deps-unused:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{YELLOW}}🔍 Checking for unused dependencies...{{NORMAL}}"
    
    # Check if cargo-machete is installed
    if ! command -v cargo-machete &> /dev/null; then
        echo "{{YELLOW}}📦 Installing cargo-machete...{{NORMAL}}"
        cargo install cargo-machete
    fi
    
    echo "{{CYAN}}Scanning for unused dependencies...{{NORMAL}}"
    cargo machete
    
    echo "{{BOLD}}{{GREEN}}✅ Unused dependency check completed{{NORMAL}}"

# 📄 License compliance with cargo-about
[group('security')]
license-check:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{YELLOW}}📄 Checking license compliance...{{NORMAL}}"
    
    # Check if cargo-about is installed
    if ! command -v cargo-about &> /dev/null; then
        echo "{{YELLOW}}📦 Installing cargo-about...{{NORMAL}}"
        cargo install cargo-about
    fi
    
    echo "{{CYAN}}Generating license report...{{NORMAL}}"
    cargo about generate about.hbs > licenses.html
    
    echo "{{BOLD}}{{GREEN}}✅ License report generated: licenses.html{{NORMAL}}"

# =============================================================================
# Formatting Commands
# =============================================================================

# 🎨 Format code
[group('lint')]
format:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{YELLOW}}🎨 Formatting code...{{NORMAL}}"
    
    cargo fmt --all
    
    echo "{{BOLD}}{{GREEN}}✅ Code formatted{{NORMAL}}"

# 🎨 Check formatting without modifying
[group('lint')]
format-check:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{YELLOW}}🎨 Checking code formatting...{{NORMAL}}"
    
    cargo fmt --all -- --check
    
    if [ $? -eq 0 ]; then
        echo "{{BOLD}}{{GREEN}}✅ Code formatting is correct{{NORMAL}}"
    else
        echo "{{BOLD}}{{RED}}❌ Code formatting issues found{{NORMAL}}"
        echo "{{CYAN}}Run 'just format' to fix formatting{{NORMAL}}"
        exit 1
    fi

# =============================================================================
# Advanced Linting
# =============================================================================

# 🕸️ WASM-specific lints
[group('lint')]
[group('wasm')]
lint-wasm:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{YELLOW}}🕸️ Running WASM-specific lints...{{NORMAL}}"
    
    # Check if wasm32 target is available
    if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
        echo "{{YELLOW}}📦 Installing wasm32-unknown-unknown target...{{NORMAL}}"
        rustup target add wasm32-unknown-unknown
    fi
    
    # WASM-specific clippy lints
    cargo clippy --workspace --target wasm32-unknown-unknown --all-features -- \
        -W clippy::panic \
        -W clippy::panic_in_result_fn \
        -W clippy::unimplemented \
        -W clippy::todo \
        -W clippy::unwrap_used \
        -W clippy::expect_used \
        -A clippy::redundant_static_lifetimes
    
    echo "{{BOLD}}{{GREEN}}✅ WASM lints completed{{NORMAL}}"

# 🧪 Test-specific lints
[group('lint')]
lint-tests:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{YELLOW}}🧪 Running test-specific lints...{{NORMAL}}"
    
    cargo clippy --workspace --tests --all-features -- \
        -W clippy::assertions_on_constants \
        -W clippy::debug_assert_with_mut_call \
        -W clippy::panic_in_result_fn \
        -A clippy::unwrap_used \
        -A clippy::expect_used
    
    echo "{{BOLD}}{{GREEN}}✅ Test lints completed{{NORMAL}}"

# 📊 Complexity analysis
[group('lint')]
complexity:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{YELLOW}}📊 Running complexity analysis...{{NORMAL}}"
    
    cargo clippy --workspace --all-targets --all-features -- \
        -W clippy::cognitive_complexity \
        -W clippy::too_many_arguments \
        -W clippy::too_many_lines \
        -W clippy::type_complexity \
        -W clippy::cyclomatic_complexity
    
    echo "{{BOLD}}{{GREEN}}✅ Complexity analysis completed{{NORMAL}}"

# =============================================================================
# Fix Commands
# =============================================================================

# 🔧 Auto-fix clippy suggestions
[group('lint')]
fix:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{YELLOW}}🔧 Auto-fixing clippy suggestions...{{NORMAL}}"
    
    cargo clippy --workspace --all-targets --all-features --fix --allow-dirty
    
    echo "{{BOLD}}{{GREEN}}✅ Auto-fix completed{{NORMAL}}"

# 🔧 Fix and format
[group('lint')]
fix-all:
    #!/usr/bin/env bash
    echo "{{BOLD}}{{YELLOW}}🔧 Auto-fixing and formatting...{{NORMAL}}"
    
    just fix
    just format
    
    echo "{{BOLD}}{{GREEN}}✅ All fixes applied{{NORMAL}}"