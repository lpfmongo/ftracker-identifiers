# CNPJ

**CNPJ** (Cadastro Nacional da Pessoa Jurídica) is Brazil's national registry identifier for legal
entities, issued by the Receita Federal (Brazil's federal revenue service). This crate's `Cnpj`
type is a validated, allocation-free representation of it.

```rust,ignore
use ftracker_identifiers::Cnpj;

let cnpj = Cnpj::parse("00.000.000/0001-91").unwrap();
assert!(cnpj.is_root());
assert_eq!(cnpj.as_str(), "00000000000191");
assert_eq!(cnpj.formatted().as_str(), "00.000.000/0001-91");
```

If you hold a `Cnpj`, it is guaranteed to satisfy every structural rule and the Módulo 11 checksum
described in this chapter — there is no partially validated or "trust me" state.

## In this chapter

- [**Structure & Formats**](./format.md) — the three segments of a CNPJ, and the 2026 move to
  alphanumeric identifiers.
- [**Parsing & Validation**](./parsing-and-validation.md) — what `Cnpj::parse` accepts, and the
  rules every constructor enforces.
- [**Formatting & Display**](./formatting-and-display.md) — rendering the compact and punctuated
  forms without allocating.
- [**Error Handling**](./error-handling.md) — the `CnpjError` variants and how to match on them.
- [**Feature Flags**](./feature-flags.md) — optional `serde`, `schemars`, `arbitrary`, and
  `proptest` integrations.
- [**Examples**](./examples.md) — end-to-end usage, including sorting, deduplication, and use as a
  map/set key.

## API reference

This book explains *how* and *why* to use `Cnpj`. For the full, generated API reference — every
method signature, trait implementation, and doc-tested example — run:

```sh
cargo doc --open --all-features
```