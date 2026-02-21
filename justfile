# smc-paths — development task runner
# Install just: https://github.com/casey/just
# Usage: just <recipe>

# List available recipes
default:
    @just --list

# ── Code quality ──────────────────────────────────────────────────────────────

# Check formatting (matches CI)
fmt-check:
    cargo fmt --all -- --check

# Apply formatting
fmt:
    cargo fmt --all

# Run Clippy (matches CI — warnings are errors)
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# ── Tests ─────────────────────────────────────────────────────────────────────

# Run all tests
test:
    cargo test --all-features

# Run all tests with output shown
test-verbose:
    cargo test --all-features -- --nocapture

# Run doctests only
test-doc:
    cargo test --doc --all-features

# ── Combined checks ───────────────────────────────────────────────────────────

# Run everything CI runs (fmt check → clippy → test)
ci: fmt-check lint test

# Run pre-commit hooks on all files
pre-commit:
    pre-commit run --all-files

# ── Housekeeping ──────────────────────────────────────────────────────────────

# Install pre-commit hooks into the local git repo
install-hooks:
    pre-commit install
    @echo "✅ pre-commit hooks installed"

# Remove build artefacts
clean:
    cargo clean
