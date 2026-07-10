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
    cargo tarpaulin --fail-under 65

# Regenerate the committed CFI taxonomy table from data/cfi.json
cfi-generate:
    cargo run --bin generate_cfi_table --features codegen
    cargo fmt

# Check that the committed CFI taxonomy table is up to date with data/cfi.json
cfi-check:
    cargo run --bin generate_cfi_table --features codegen
    cargo fmt
    git diff --exit-code -- src/cfi/table.rs

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

# Build every fuzz target (requires `cargo install cargo-fuzz` and a nightly toolchain)
fuzz-build:
    cargo +nightly fuzz build

# Run a fuzz target for a bounded time (requires cargo-fuzz + nightly).
# Example: `just fuzz country_code` or `just fuzz cfi 120`. Targets: country_code, cfi, isin, cnpj.
fuzz target time="60":
    mkdir -p fuzz/corpus/{{target}}
    cargo +nightly fuzz run {{target}} fuzz/corpus/{{target}} fuzz/seeds/{{target}} -- -dict=fuzz/dict/{{target}}.dict -max_total_time={{time}}

# Show line coverage of a fuzz target over its corpus, then print a summary report.
# Requires cargo-fuzz + nightly + llvm-tools.
fuzz-coverage target:
    #!/usr/bin/env bash
    set -euo pipefail
    # The corpus directory is gitignored, so recreate it if a fresh checkout lacks it.
    mkdir -p fuzz/corpus/{{target}}
    cargo +nightly fuzz coverage {{target}} fuzz/corpus/{{target}} fuzz/seeds/{{target}}
    triple="$(rustc +nightly -vV | sed -n 's/host: //p')"
    llvm_cov="$(rustc +nightly --print sysroot)/lib/rustlib/${triple}/bin/llvm-cov"
    # Map the fuzz target name to its source module (they differ for country_code -> country).
    case "{{target}}" in
        country_code) module="country" ;;
        *) module="{{target}}" ;;
    esac
    "${llvm_cov}" report \
        "target/${triple}/coverage/${triple}/release/{{target}}" \
        -instr-profile="fuzz/coverage/{{target}}/coverage.profdata" \
        -sources "src/${module}.rs" "src/${module}"

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
