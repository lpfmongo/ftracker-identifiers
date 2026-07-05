# Show available commands
default:
    @just --list

setup:
    cargo

# Check Rust code formatting without modifying files
format-check:
    cargo fmt --all -- --check

# Format all Rust code
format:
    cargo fmt --all

# Run Clippy lints for the workspace
lint:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

# build the workspace (CI environment)
build-ci: clean
    cargo build --workspace

# Build the workspace (developer environment)
build: clean
    cargo build --workspace --verbose
    cargo tarpaulin --fail-under 30

# Generate the third-party license report
licenses-generate:
    cargo about generate about.hbs > third-party-licenses.html

# Check that the third-party license report is up to date
licenses-check:
    cargo about generate about.hbs > /tmp/third-party-licenses.html
    diff -u third-party-licenses.html /tmp/third-party-licenses.html

# Run dependency policy checks
deny-check:
    cargo deny check all

# Run the full dependency and license policy suite
policy-check: licenses-check deny-check

# Run all workspace tests
test: clean
    cargo test --workspace --verbose

# Build the workspace in release mode
build-release: clean
    cargo build --workspace --release --verbose

# Remove build artifacts
clean:
    cargo clean

# Build the documentation book (requires mdbook: cargo install mdbook)
docs-build:
    mdbook build docs

# Serve the documentation book locally with live-reload
docs-serve:
    mdbook serve docs --open
