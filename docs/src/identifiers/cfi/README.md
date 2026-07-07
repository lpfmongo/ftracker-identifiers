# CFI

**CFI** (Classification of Financial Instruments) is the ISO 10962 six-letter code that classifies
a financial instrument by **category**, **group**, and four **attributes**. This crate's `Cfi` type
is a validated, allocation-free representation of it.

```rust,ignore
use ftracker_identifiers::Cfi;

let cfi = Cfi::parse("ESVUFR").unwrap();
assert_eq!(cfi.category(), 'E');
assert_eq!(cfi.group(), 'S');
assert_eq!(cfi.attributes(), ['V', 'U', 'F', 'R']);
assert_eq!(cfi.as_str(), "ESVUFR");
```

If you hold a `Cfi`, it is guaranteed to describe a category, group, and attribute combination that
ISO 10962 actually defines. There is no partially validated or "trust me" state.

## Taxonomy, not checksum

Unlike [CNPJ](../cnpj/README.md) (Módulo 11) and [ISIN](../isin/README.md) (Luhn), a CFI has **no
check digit**. A CFI is valid exactly when its letters name a combination the standard defines, so
this crate embeds the ISO 10962 code taxonomy as a generated, `no_std` lookup table and validates
against it. Only the classification *codes* are embedded — not ISO's descriptive text — so `Cfi`
tells you whether a code is well-formed, not what each letter *means*.

## In this chapter

- [**Structure & Formats**](./format.md) — the six positions of a CFI and how they nest.
- [**Parsing & Validation**](./parsing-and-validation.md) — what `Cfi::parse` accepts, and the rules
  every constructor enforces.
- [**Formatting & Display**](./formatting-and-display.md) — rendering the canonical form without
  allocating.
- [**Error Handling**](./error-handling.md) — the `CfiError` variants and how to match on them.
- [**Feature Flags**](./feature-flags.md) — optional `serde`, `schemars`, `arbitrary`, and
  `proptest` integrations.
- [**Examples**](./examples.md) — end-to-end usage, including sorting, deduplication, and use as a
  map/set key.

## API reference

This book explains *how* and *why* to use `Cfi`. For the full, generated API reference run:

```sh
cargo doc --open --all-features
```
