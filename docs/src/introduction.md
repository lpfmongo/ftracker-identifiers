# FTracker Identifiers

`ftracker-identifiers` is a Rust crate of small, validated value types for the identifiers used across financial and
regulatory data.

Every type in this crate follows the same philosophy:

- **Parse, don't validate.** If you're holding a value of the type, it is guaranteed correct. There is no "unchecked"
  variant floating around your codebase that might secretly be malformed.
- **Zero-cost.** Types are small, `Copy`, and allocation-free. Validating and formatting an identifier should not
  require a heap allocation.
- **`no_std`-friendly.** The crate builds without `std` by default consumers who need it, falling back to `alloc` only
  where unavoidable (see each identifier's own chapter for specifics).
- **Additive feature flags.** Integrations with `serde`, `schemars`, `arbitrary`, and `proptest` are opt-in and never
  change an identifier's validation rules. Enabling a feature only adds capabilities, it never loosens or tightens what
  counts as "valid."

## What's covered today

- [**CNPJ**](./identifiers/cnpj/README.md) — Brazil's national registry identifier for legal entities, supporting both
  the legacy numeric-only format and the 2026 alphanumeric format.
- [**ISIN**](./identifiers/isin/README.md) — the ISO 6166 identifier for a fungible financial security, validated with
  the ISO 6166 Luhn check digit.

## What's planned

The **Identifiers** section of this book is organized so each identifier gets its own chapter, following the same shape:
structure, parsing & validation, formatting, error handling, feature flags, and examples. **CFI** is next on the
roadmap; see [Adding a New Identifier](./contributing/adding-a-new-identifier.md) if you'd like to help build it out.

## Installation

Add the crate to your `Cargo.toml`, enabling only the feature flags you need:

```toml
[dependencies]
ftracker-identifiers = "0.0.1"

# Optional integrations — see each identifier's "Feature Flags" chapter for details.
# ftracker-identifiers = { version = "0.0.1", features = ["serde", "schemars"] }
```

## Minimum supported Rust version

This crate targets the Rust version pinned in `rust-toolchain.toml` at the repository root. Check that file for the
exact version this documentation was written against.