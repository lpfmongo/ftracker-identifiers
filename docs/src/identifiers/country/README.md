# Country Code

**Country Code** is the ISO 3166-1 alpha-2 two letter code that identifies a country, dependent territory, or special
area of geographical interest. This crate's `CountryCode` type is a validated, allocation free representation of it.

```rust,ignore
use ftracker_identifiers::CountryCode;

let code = CountryCode::parse("US").unwrap();
assert_eq!(code.as_str(), "US");
assert_eq!(code.as_bytes(), b"US");
```

If you hold a `CountryCode`, it is guaranteed to be a code that ISO 3166-1 officially assigns. There is no partially
validated or "trust me" state.

## Membership, not checksum

A country code has **no check digit**. It is valid exactly when its two letters name a code that the standard officially
assigns, so this crate embeds that set as a compile time bitmap and tests membership against it. Only the codes are
embedded. The country name, the alpha-3 code, and the numeric code are all out of scope, so `CountryCode` tells you
whether a code is assigned, not what country it names.

## In this chapter

* [**Structure & Formats**](./format.md): the two letters of a country code and what is (and is not) modeled.
* [**Parsing & Validation**](./parsing-and-validation.md): what `CountryCode::parse` accepts, and the rules every
  constructor enforces.
* [**Formatting & Display**](./formatting-and-display.md): rendering the canonical form without allocating.
* [**Error Handling**](./error-handling.md): the `CountryCodeError` variants and how to match on them.
* [**Feature Flags**](./feature-flags.md): optional `serde`, `schemars`, `arbitrary`, and `proptest` integrations.
* [**Examples**](./examples.md): end to end usage, including sorting, deduplication, and use as a map or set key.

## API reference

This book explains how and why to use `CountryCode`. For the full, generated API reference run:

```sh
cargo doc --open --all-features
```
