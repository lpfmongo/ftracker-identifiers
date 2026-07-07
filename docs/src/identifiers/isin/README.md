# ISIN

**ISIN** (International Securities Identification Number) is the ISO 6166 identifier for a fungible
financial security (a stock, bond, or fund share). This crate's `Isin` type is a validated,
allocation-free representation of it.

```rust,ignore
use ftracker_identifiers::Isin;

let isin = Isin::parse("US0378331005").unwrap();
assert_eq!(isin.country_code(), "US");
assert_eq!(isin.nsin(), "037833100");
assert_eq!(isin.check_digit(), 5);
assert_eq!(isin.as_str(), "US0378331005");
```

If you hold an `Isin`, it is guaranteed to satisfy every structural rule and the ISO 6166 Luhn
check digit described in this chapter. There is no partially validated or "trust me" state.

## In this chapter

- [**Structure & Formats**](./format.md) — the three segments of an ISIN and how the check digit is
  derived.
- [**Parsing & Validation**](./parsing-and-validation.md) — what `Isin::parse` accepts, and the
  rules every constructor enforces.
- [**Formatting & Display**](./formatting-and-display.md) — rendering the canonical form without
  allocating.
- [**Error Handling**](./error-handling.md) — the `IsinError` variants and how to match on them.
- [**Feature Flags**](./feature-flags.md) — optional `serde`, `schemars`, `arbitrary`, and
  `proptest` integrations.
- [**Examples**](./examples.md) — end-to-end usage, including sorting, deduplication, and use as a
  map/set key.

## API reference

This book explains *how* and *why* to use `Isin`. For the full, generated API reference run:

```sh
cargo doc --open --all-features
```
