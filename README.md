# ftracker-identifiers

Validated, `no_std`-first identifier types for Rust.

This crate provides small, `Copy`, allocation-free value types that can only ever hold a valid
identifier. Parsing runs full validation up front, so once you hold one of these types there is no
partially validated state to guard against downstream.

## Identifiers

- `Cnpj`: Brazil's Cadastro Nacional da Pessoa Jurídica, the national registry identifier for
  legal entities. Supports the punctuated `AA.AAA.AAA/AAAA-DD` form, the compact 14-character form,
  the legacy numeric layout, and the 2026 alphanumeric layout, all validated with the Módulo 11
  checksum.
- `Isin`: the ISO 6166 International Securities Identification Number, a 12-character securities
  identifier validated with the ISO 6166 Luhn check digit.
- `Cfi`: the ISO 10962 Classification of Financial Instruments code, a six-letter taxonomy code
  validated against an embedded copy of the standard's code table.
- `CountryCode`: the ISO 3166-1 alpha-2 country code, two letters validated against the officially
  assigned set.

## Design

- **No invalid state is representable.** Every constructor runs the full validation rules and
  returns a typed error on failure. There is no unchecked public constructor.
- **`no_std` first.** The crate is `#![no_std]` by default and relies only on `core` and `alloc`.
  The `std` feature is additive.
- **Zero allocation and `Copy`.** Each type wraps a fixed-size byte array. Parsing, validation, and
  every accessor operate on the stack.
- **Consistent ordering and hashing.** Ordering and hashing operate over the raw ASCII bytes and
  match `str` ordering on the string accessor, so every type works as a map or set key.

## Feature flags

- `std` (default): enables the standard library and the `std` support of any enabled optional
  dependency.
- `serde`: (de)serializes each type as its canonical string. Deserialization re-runs full
  validation.
- `schemars`: implements `JsonSchema` for each type. Implies `serde`.
- `arbitrary`: implements `Arbitrary` for each type, generating valid values for fuzz targets.
- `proptest`: exposes reusable `proptest` strategies for generating valid values.

## Example

```rust
use ftracker_identifiers::{Cnpj, Isin, Cfi, CountryCode};

let cnpj = Cnpj::parse("00.000.000/0001-91").unwrap();
assert_eq!(cnpj.as_str(), "00000000000191");

let isin = Isin::parse("US0378331005").unwrap();
assert_eq!(isin.country_code(), "US");

let cfi = Cfi::parse("ESVUFR").unwrap();
assert_eq!(cfi.category(), 'E');

let country = CountryCode::parse("br").unwrap();
assert_eq!(country.as_str(), "BR");
```

## License

Licensed under the MIT license.
