# Feature Flags

All of `Isin`'s optional integrations are off by default and purely additive. Enabling one never changes what counts as
a valid ISIN, only what you can *do* with an `Isin` once you have one.

```toml
[dependencies]
ftracker-identifiers = { version = "0.0.1", features = ["serde", "schemars", "arbitrary", "proptest"] }
```

## `serde`

(De)serializes `Isin` as its canonical 12-character string (e.g. `"US0378331005"`), so it round-trips as a plain
identifier in JSON, YAML, or config files.

Deserialization always re-runs full validation: an untrusted payload (a malformed API request, a hand-edited config
file) can never produce an invalid `Isin`. A bad value fails to deserialize with a descriptive error instead of silently
producing garbage.

```rust,ignore
use ftracker_identifiers::Isin;

let isin = Isin::parse("US0378331005").unwrap();
let json = serde_json::to_string(&isin).unwrap();
assert_eq!(json, "\"US0378331005\"");

let back: Isin = serde_json::from_str(&json).unwrap();
assert_eq!(isin, back);

// Invalid input is rejected at deserialization time, not silently accepted.
assert!(serde_json::from_str::<Isin>("\"not-an-isin\"").is_err());
```

## `schemars`

Implements `JsonSchema` for `Isin`, so it can appear in a generated OpenAPI/JSON Schema document as a
pattern-constrained string rather than an opaque `string` type. This feature implies `serde`.

The generated schema:

```json
{
  "type": "string",
  "format": "isin",
  "minLength": 12,
  "maxLength": 12,
  "pattern": "^[A-Z]{2}[A-Z0-9]{9}[0-9]$",
  "description": "ISIN (International Securities Identification Number, ISO 6166), Luhn checksum-valid."
}
```

Note that the `pattern` alone doesn't capture the Luhn checksum constraint. Schema validators outside this crate (API
gateways, other-language clients) can reject malformed shapes, but true checksum validation still requires this crate
(or a reimplementation of the same algorithm; see [Structure & Formats](./format.md)).

## `arbitrary`

Implements `Arbitrary` for `Isin`, so fuzz targets (e.g. via `cargo fuzz`) can generate structurally valid,
checksum-correct `Isin` values directly, instead of generating raw strings that mostly fail validation before reaching
the code under test.

## `proptest`

Exposes reusable [`proptest`](https://docs.rs/proptest) strategies at `ftracker_identifiers::isin::proptest`:

- `isin::proptest::valid_isin()` — a `Strategy<Value = Isin>` producing checksum-correct values with a two-letter
  country code and an alphanumeric NSIN.
- `isin::proptest::valid_isin_string()` — the same, rendered as a canonical `String`, useful for
  round-trip-through-parsing property tests.

This is the recommended way to property-test *your own* code that accepts an `Isin`, without hand-rolling a
checksum-valid generator:

```rust,ignore
use ftracker_identifiers::{isin::proptest::valid_isin, Isin};
use proptest::proptest;

proptest! {
    #[test]
    fn my_function_accepts_any_valid_isin(isin in valid_isin()) {
        // exercise your own code with `isin` here
    }
}
```
