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
    cargo about generate about.hbs > third-party-licenses.html
    cargo deny check
    cargo tarpaulin --fail-under 30

# Run all workspace tests
test: clean
    cargo test --workspace --verbose

# Build the workspace in release mode
build-release: clean
    cargo build --workspace --release --verbose

# Remove build artifacts
clean:
    cargo clean
