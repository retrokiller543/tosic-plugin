# Security Tools and Best Practices

Comprehensive guide to security analysis, vulnerability scanning, and secure development practices for Tosic Plugin.

## Table of Contents

- [Overview](#overview)
- [Security Tools](#security-tools)
- [Quick Security Check](#quick-security-check)
- [Vulnerability Scanning](#vulnerability-scanning)
- [Dependency Management](#dependency-management)
- [Code Security Analysis](#code-security-analysis)
- [Best Practices](#best-practices)
- [CI/CD Security](#cicd-security)

## Overview

Security is a critical aspect of plugin systems. Tosic Plugin includes comprehensive security tooling to help identify vulnerabilities, manage dependencies securely, and maintain secure coding practices.

### Security Philosophy

- **Zero-Trust Plugin Environment**: All plugins run in isolated environments
- **Secure by Default**: Safe defaults with explicit opt-in for dangerous operations
- **Continuous Security**: Automated security scanning in development and CI
- **Dependency Hygiene**: Regular audits and minimal dependency surface

## Security Tools

### Integrated Security Tools

The project includes several security-focused tools:

| Tool | Purpose | Command |
|------|---------|---------|
| **cargo-audit** | Vulnerability scanning | `just security-audit` |
| **cargo-deny** | Dependency policy enforcement | `just security-deny` |
| **cargo-machete** | Unused dependency detection | `just deps-unused` |
| **cargo-about** | License compliance | `just license-check` |
| **clippy** | Security-focused lints | `just lint-security` |

### Tool Installation

Tools are automatically available in the Nix development environment:

```bash
# Enter development environment (all tools included)
nix develop

# Manual installation (if not using Nix)
cargo install cargo-audit cargo-deny cargo-machete cargo-about
```

## Quick Security Check

### One-Command Security Scan

```bash
# Run all security checks
just security-all   # Audit + Deny + Unused deps + Security lints

# Or run individually
just security-audit  # Vulnerability scan
just security-deny   # Policy enforcement
just deps-unused     # Find unused dependencies
```

### Daily Security Workflow

```bash
# Before committing
just lint-security   # Security-focused clippy lints
just security-audit  # Check for vulnerabilities

# Weekly/monthly
just security-deny   # Update and check policies
just deps-unused     # Clean up unused dependencies
```

## Vulnerability Scanning

### cargo-audit

Scans dependencies for known security vulnerabilities.

#### Basic Usage

```bash
# Scan for vulnerabilities
just security-audit

# Update advisory database and scan
cargo audit --update
```

#### Advanced Usage

```bash
# Ignore specific vulnerabilities (use carefully)
cargo audit --ignore RUSTSEC-2023-0001

# Format as JSON
cargo audit --format json

# Check specific package
cargo audit --package tokio
```

#### Configuration

Create `audit.toml` for configuration:

```toml
[advisories]
# Ignore unmaintained packages warnings
ignore_unmaintained = true

# Ignore specific advisories (use with caution)
ignore = [
    # "RUSTSEC-2023-0001",  # Example: only if you've verified it's not applicable
]

[bans]
# Deny specific packages
deny = [
    "openssl",  # Prefer rustls
]
```

### Vulnerability Response

When vulnerabilities are found:

1. **Assess Impact**: Understand if the vulnerability affects your usage
2. **Update Dependencies**: `just update` to get latest versions
3. **Find Alternatives**: If no fix is available, consider alternatives
4. **Document Decisions**: If ignoring, document why in `audit.toml`

## Dependency Management

### cargo-deny

Enforces policies around dependencies, licenses, and security.

#### Configuration

Create `deny.toml`:

```toml
[graph]
# Enable all features for analysis
all-features = true

[advisories]
# Vulnerability database
db-path = "~/.cargo/advisory-db"
db-urls = ["https://github.com/rustsec/advisory-db"]

# Deny unmaintained packages
ignore-unmaintained = false

[licenses]
# Allow only specific licenses
allow = [
    "MIT",
    "Apache-2.0",
    "Apache-2.0 WITH LLVM-exception",
    "BSD-2-Clause",
    "BSD-3-Clause",
    "ISC",
    "Unicode-DFS-2016",
]

# Deny problematic licenses
deny = [
    "GPL-2.0",
    "GPL-3.0",
    "AGPL-3.0",
]

[bans]
# Deny specific crates
deny = [
    { name = "openssl" },  # Prefer rustls
    { name = "ring", version = "<0.16" },  # Old versions have issues
]

# Limit multiple versions of same crate
multiple-versions = "deny"

# Skip certain crates from multiple version check
skip = [
    { name = "windows-sys" },  # Often has multiple versions
]

[sources]
# Ensure dependencies come from trusted sources
unknown-registry = "deny"
unknown-git = "deny"

# Allow only crates.io
allow-registry = ["https://github.com/rust-lang/crates.io-index"]
```

#### Commands

```bash
# Check all policies
just security-deny

# Check specific aspects
cargo deny check advisories  # Just vulnerabilities
cargo deny check licenses    # Just license compliance
cargo deny check bans        # Just banned dependencies
cargo deny check sources     # Just source verification
```

### Unused Dependencies

#### cargo-machete

Finds unused dependencies that increase attack surface.

```bash
# Find unused dependencies
just deps-unused

# Remove unused dependencies automatically (be careful)
cargo machete --fix

# Check specific package
cargo machete --package tosic-plugin-core
```

#### Cleaning Dependencies

```bash
# Review unused dependencies
just deps-unused

# Remove manually (safer)
just remove-dep <unused-dependency>

# Verify builds still work
just test
```

## Code Security Analysis

### Security-Focused Clippy Lints

#### Configuration

In `.clippy.toml`:

```toml
# Security-focused configuration
avoid-breaking-exported-api = false
allow-expect-in-tests = true
allow-unwrap-in-tests = true
```

#### Security Lints

```bash
# Run security-focused lints
just lint-security

# Specific security lints included:
# - Potential panics and unwraps
# - Unsafe code patterns
# - Crypto-related issues
# - Input validation problems
```

### Manual Security Review

#### Code Patterns to Avoid

```rust
// ❌ Avoid unwrap() in production code
let value = might_fail().unwrap();

// ✅ Use proper error handling
let value = might_fail()?;

// ❌ Avoid unsafe code without documentation
unsafe { /* undocumented unsafe operation */ }

// ✅ Document safety requirements
/// # Safety
/// This is safe because...
unsafe { /* documented unsafe operation */ }

// ❌ Avoid hardcoded secrets
const API_KEY: &str = "secret123";

// ✅ Use environment variables
let api_key = env::var("API_KEY")?;
```

#### Security Review Checklist

- [ ] No hardcoded secrets or credentials
- [ ] Input validation on all external inputs
- [ ] Proper error handling (no `unwrap()` in production)
- [ ] Documented safety requirements for `unsafe` code
- [ ] No SQL injection vulnerabilities (if using databases)
- [ ] Secure random number generation
- [ ] Proper cryptographic library usage

## Best Practices

### Secure Development Workflow

#### 1. Development Phase

```bash
# Start with security check
just security-audit

# Develop with security lints enabled
just lint-security

# Test security scenarios
just test  # Include security-focused tests
```

#### 2. Pre-Commit Checks

```bash
# Complete security validation
just lint-security
just security-audit
just security-deny
just deps-unused
```

#### 3. Regular Maintenance

```bash
# Weekly/monthly security maintenance
just update          # Update dependencies
just security-audit  # Check for new vulnerabilities
just deps-unused     # Clean unused dependencies
```

### Plugin Security Considerations

Since Tosic Plugin is a plugin system, special security considerations apply:

#### Host Application Security

```rust
// ✅ Validate all plugin inputs
fn call_plugin_function(&mut self, name: &str, args: &[Value]) -> Result<Value> {
    // Validate function name
    if !is_valid_function_name(name) {
        return Err(SecurityError::InvalidFunctionName);
    }
    
    // Validate arguments
    for arg in args {
        validate_argument(arg)?;
    }
    
    // Call with validated inputs
    self.runtime.call(name, args)
}

// ✅ Implement resource limits
fn set_limits(&mut self) {
    self.runtime.set_memory_limit(16 * 1024 * 1024); // 16MB
    self.runtime.set_execution_timeout(Duration::from_secs(30));
    self.runtime.set_cpu_limit(CpuLimit::Percentage(50));
}
```

#### Plugin Isolation

```rust
// ✅ Sandbox plugin execution
struct PluginSandbox {
    allowed_functions: HashSet<String>,
    memory_limit: usize,
    timeout: Duration,
}

impl PluginSandbox {
    fn execute_plugin(&self, plugin: &Plugin) -> Result<Value> {
        // Set up isolated environment
        let mut runtime = IsolatedRuntime::new();
        runtime.set_allowed_functions(&self.allowed_functions);
        runtime.set_memory_limit(self.memory_limit);
        runtime.set_timeout(self.timeout);
        
        // Execute in sandbox
        runtime.execute(plugin)
    }
}
```

### Dependency Security

#### Choosing Dependencies

1. **Prefer well-maintained crates**: Check last update, issue response time
2. **Minimize dependencies**: Fewer dependencies = smaller attack surface
3. **Audit large dependencies**: Review code of critical dependencies
4. **Use official crates**: Prefer crates from known, trusted authors

```bash
# Check dependency freshness
cargo outdated

# Check dependency tree
just deps-tree

# Analyze dependency impact
cargo tree --duplicates
```

#### Dependency Pinning

In `Cargo.toml`, consider pinning critical dependencies:

```toml
[dependencies]
# Pin critical security dependencies
ring = "=0.16.20"          # Cryptography - pin exact version
rustls = "=0.21.0"         # TLS - pin exact version

# Allow patch updates for others
serde = "1.0"              # Allow 1.0.x updates
tokio = "1.0"              # Allow 1.0.x updates
```

## CI/CD Security

### GitHub Actions Security

#### Secrets Management

```yaml
# .github/workflows/security.yml
name: Security Checks

on: [push, pull_request]

jobs:
  security:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Security Audit
        run: |
          cargo install cargo-audit
          cargo audit
      
      - name: Dependency Policy Check
        run: |
          cargo install cargo-deny
          cargo deny check
      
      - name: License Check
        run: |
          cargo install cargo-about
          cargo about check
```

#### Automated Security Updates

Consider using Dependabot for automated security updates:

```yaml
# .github/dependabot.yml
version: 2
updates:
  - package-ecosystem: "cargo"
    directory: "/"
    schedule:
      interval: "weekly"
    reviewers:
      - "security-team"
    assignees:
      - "security-team"
```

### Security Monitoring

#### Advisory Notifications

Subscribe to Rust security advisories:

- [RustSec Advisory Database](https://rustsec.org/)
- [Rust Security Response WG](https://github.com/rust-lang/wg-security-response)

#### Automated Monitoring

Set up automated security scanning:

```bash
# Daily security check script
#!/bin/bash
cd /path/to/tosic-plugin

# Update advisory database
cargo audit --update

# Run security checks
just security-audit
just security-deny

# Send notification if issues found
if [ $? -ne 0 ]; then
    # Send alert notification
    echo "Security issues detected" | mail security-team@company.com
fi
```

## Incident Response

### Security Vulnerability Response

1. **Immediate Assessment**
   ```bash
   # Assess vulnerability impact
   cargo audit --json > vulnerability-report.json
   
   # Check if vulnerability affects running code
   grep -r "vulnerable_function" src/
   ```

2. **Containment**
   ```bash
   # Update immediately if fix available
   just update
   just security-audit
   
   # Temporarily pin if needed
   cargo update --package vulnerable-crate --precise fixed-version
   ```

3. **Communication**
   - Document vulnerability impact
   - Notify stakeholders
   - Plan update rollout

4. **Prevention**
   ```bash
   # Add to audit.toml to prevent regression
   echo "RUSTSEC-YYYY-NNNN" >> audit-ignore-list
   
   # Update security policies
   vim deny.toml
   ```

### Security Incident Checklist

- [ ] Identify affected systems
- [ ] Assess vulnerability severity
- [ ] Apply immediate fixes/workarounds
- [ ] Update dependencies
- [ ] Run full security scan
- [ ] Document incident and response
- [ ] Update security policies
- [ ] Review and improve processes

---

*For more security-focused workflows, see [WORKFLOWS.md](WORKFLOWS.md)*
*For build system security commands, see [BUILD_SYSTEM.md](BUILD_SYSTEM.md)*