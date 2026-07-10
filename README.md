# ftracker-identifiers

Validated, `no_std`-first identifier types for Rust. It is:

* **Correct**: parsing runs full validation up front, so an invalid identifier
  can never be represented. Once you hold one of these types, there is no
  partially validated state to guard against.

* **`no_std` first**: the crate builds on `core` and `alloc` only. The `std`
  feature is additive, not required.

* **Zero-cost**: every type is `Copy`, wraps a fixed-size byte array, and
  performs parsing, validation, and every accessor on the stack.

[![Build Status][actions-badge]][actions-url]
[![MIT licensed][mit-badge]][mit-url]
[![MSRV][msrv-badge]][msrv-url]
[![API Docs][docs-badge]][docs-url]

[actions-badge]: https://github.com/lnivva/ftracker-identifiers/workflows/CI/badge.svg
[actions-url]: https://github.com/lnivva/ftracker-identifiers/actions?query=workflow%3ACI+branch%3Amain
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/lnivva/ftracker-identifiers/blob/main/LICENSE
[msrv-badge]: https://img.shields.io/badge/rustc-1.93.0+-blue.svg
[msrv-url]: #supported-rust-versions
[docs-badge]: https://docs.rs/ftracker-identifiers/badge.svg
[docs-url]: https://docs.rs/ftracker-identifiers

[API Docs](https://docs.rs/ftracker-identifiers) |
[Contributing](#contributing)

## Overview

This crate provides small, allocation-free value types that can only ever hold a
valid identifier. It currently ships four:

* **`Cnpj`**: Brazil's Cadastro Nacional da Pessoa Jurídica, the national
  registry identifier for legal entities. Supports the punctuated
  `AA.AAA.AAA/AAAA-DD` form, the compact 14-character form, the legacy numeric
  layout, and the 2026 alphanumeric layout, all validated with the Módulo 11
  checksum.
* **`Isin`**: the ISO 6166 International Securities Identification Number, a
  12-character securities identifier validated with the ISO 6166 Luhn check
  digit.
* **`Cfi`**: the ISO 10962 Classification of Financial Instruments code, a
  six-letter taxonomy code validated against an embedded copy of the standard's
  code table.
* **`CountryCode`**: the ISO 3166-1 alpha-2 country code, two letters validated
  against the officially assigned set.

## Example

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
ftracker-identifiers = "0.0.1"
```

Then parse and inspect identifiers:

```rust
use ftracker_identifiers::{Cnpj, Isin, Cfi, CountryCode};

// CNPJ accepts punctuated or compact input and exposes structured accessors.
let cnpj = Cnpj::parse("00.000.000/0001-91").unwrap();
assert_eq!(cnpj.as_str(), "00000000000191");
assert_eq!(cnpj.root(), "00000000");
assert_eq!(cnpj.branch_code(), "0001");

// ISIN exposes its country code as a validated `CountryCode`.
let isin = Isin::parse("US0378331005").unwrap();
assert_eq!(isin.country(), Some(CountryCode::parse("US").unwrap()));
assert_eq!(isin.nsin(), "037833100");

// CFI decomposes into its ISO 10962 category and group letters.
let cfi = Cfi::parse("ESVUFR").unwrap();
assert_eq!(cfi.category(), 'E');
assert_eq!(cfi.group(), 'S');

// Country codes are case-folded to their canonical uppercase form.
let country = CountryCode::parse("br").unwrap();
assert_eq!(country.as_str(), "BR");
```

## Design

* **No invalid state is representable.** Every constructor runs the full
  validation rules and returns a typed error on failure. There is no unchecked
  public constructor.
* **`no_std` first.** The crate is `#![no_std]` by default and relies only on
  `core` and `alloc`. The `std` feature is additive.
* **Zero allocation and `Copy`.** Each type wraps a fixed-size byte array.
  Parsing, validation, and every accessor operate on the stack.
* **Consistent ordering and hashing.** Ordering and hashing operate over the raw
  ASCII bytes and match `str` ordering on the string accessor, so every type
  works as a map or set key.

## Feature flags

* `std` (default): enables the standard library and the `std` support of any
  enabled optional dependency.
* `serde`: (de)serializes each type as its canonical string. Deserialization
  re-runs full validation.
* `schemars`: implements `JsonSchema` for each type. Implies `serde`.
* `arbitrary`: implements `Arbitrary` for each type, generating valid values for
  fuzz targets.
* `proptest`: exposes reusable `proptest` strategies for generating valid
  values.

## Supported Rust Versions

The minimum supported Rust version is **1.93.0**. Raising the MSRV is treated as
a breaking change and only happens in a version bump.

## Contributing

Contributions are welcome. See the [contributing guide][guide] to get started,
and please note that this project follows a [Code of Conduct][coc].

[guide]: docs/src/contributing/adding-a-new-identifier.md
[coc]: CODE_OF_CONDUCT.md

## License

This project is licensed under the [MIT license][mit-url].

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you shall be licensed as MIT, without any
additional terms or conditions.
