# LEI

**LEI** (Legal Entity Identifier) is the ISO 17442 identifier for a legally distinct entity that participates in
financial transactions (a company, fund, or government body). This crate's `Lei` type is a validated, allocation-free
representation of it.

```rust,ignore
use ftracker_identifiers::Lei;

let bbc = Lei::parse("5493000IBP32UQZ0KL24").unwrap();
assert_eq!(bbc.lou_prefix(), "5493");
assert_eq!(bbc.entity_id(), "000IBP32UQZ0KL");
assert_eq!(bbc.check_digits(), 24);
assert_eq!(bbc.as_str(), "5493000IBP32UQZ0KL24");
```

If you hold a `Lei`, it is guaranteed to satisfy every structural rule and the ISO/IEC 7064 MOD 97-10 check digits
described in this chapter. There is no partially validated or "trust me" state.

## In this chapter

- [**Structure & Formats**](./format.md) — the three segments of an LEI and how the check digits are derived.
- [**Parsing & Validation**](./parsing-and-validation.md) — what `Lei::parse` accepts, the rules every constructor
  enforces, and what this crate deliberately does *not* check.
- [**Formatting & Display**](./formatting-and-display.md) — rendering the canonical form without allocating.
- [**Error Handling**](./error-handling.md) — the `LeiError` variants and how to match on them.
- [**Feature Flags**](./feature-flags.md) — optional `serde`, `schemars`, `arbitrary`, and `proptest` integrations.
- [**Examples**](./examples.md) — end-to-end usage, including sorting, deduplication, and use as a map/set key.

## API reference

This book explains *how* and *why* to use `Lei`. For the full, generated API reference run:

```sh
cargo doc --open --all-features
```
