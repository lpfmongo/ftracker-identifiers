# Feature Flags

All of `Cnpj`'s optional integrations are off by default and purely additive — enabling one never
changes what counts as a valid CNPJ, only what you can *do* with a `Cnpj` once you have one.

```toml
[dependencies]
ftracker-identifiers = { version = "0.0.1", features = ["serde", "schemars", "arbitrary", "proptest"] }
```

## `serde`

(De)serializes `Cnpj` as its compact 14-character string (e.g. `"12ABC34501DE35"`) — never the
punctuated form, so it round-trips as a plain identifier in JSON, YAML, or config files.

Deserialization always re-runs full validation: an untrusted payload (a malformed API request, a
hand-edited config file) can never produce an invalid `Cnpj`. A bad value fails to deserialize with
a descriptive error instead of silently producing garbage.

```rust,ignore
use ftracker_identifiers::Cnpj;

let cnpj = Cnpj::parse("00.000.000/0001-91").unwrap();
let json = serde_json::to_string(&cnpj).unwrap();
assert_eq!(json, "\"00000000000191\"");

let back: Cnpj = serde_json::from_str(&json).unwrap();
assert_eq!(cnpj, back);

// Invalid input is rejected at deserialization time, not silently accepted.
assert!(serde_json::from_str::<Cnpj>("\"not-a-cnpj\"").is_err());
```

## `schemars`

Implements `JsonSchema` for `Cnpj`, so it can appear in a generated OpenAPI/JSON Schema document as
a pattern-constrained string rather than an opaque `string` type. This feature implies `serde`.

The generated schema:

```json
{
  "type": "string",
  "format": "cnpj",
  "minLength": 14,
  "maxLength": 14,
  "pattern": "^[A-Z0-9]{12}[0-9]{2}$",
  "description": "Brazilian CNPJ (Cadastro Nacional da Pessoa Jurídica), unformatted, Módulo 11 checksum-valid."
}
```

Note that the `pattern` alone doesn't capture the Módulo 11 checksum constraint — schema
validators outside this crate (API gateways, other-language clients) can reject malformed shapes,
but true checksum validation still requires this crate (or a reimplementation of the same
algorithm; see [Structure & Formats](./format.md)).

## `arbitrary`

Implements `Arbitrary` for `Cnpj`, so fuzz targets (e.g. via `cargo fuzz`) can generate
structurally valid, checksum-correct `Cnpj` values directly, instead of generating raw strings that
mostly fail validation before reaching the code under test.

## `proptest`

Exposes reusable [`proptest`](https://docs.rs/proptest) strategies at `ftracker_identifiers::proptest`:

- `proptest::valid_cnpj()` — a `Strategy<Value = Cnpj>` producing checksum-correct values spanning
  both the numeric and alphanumeric formats.
- `proptest::valid_cnpj_formatted_string()` — the same, rendered as a punctuated `String`, useful
  for round-trip-through-formatting property tests.

This is the recommended way to property-test *your own* code that accepts a `Cnpj`, without
hand-rolling a checksum-valid generator:

```rust,ignore
use ftracker_identifiers::{proptest::valid_cnpj, Cnpj};
use proptest::proptest;

proptest! {
    #[test]
    fn my_function_accepts_any_valid_cnpj(cnpj in valid_cnpj()) {
        // exercise your own code with `cnpj` here
    }
}
```