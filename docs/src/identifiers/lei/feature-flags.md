# Feature Flags

All of `Lei`'s optional integrations are off by default and purely additive. Enabling one never changes what counts as a
valid LEI, only what you can *do* with a `Lei` once you have one.

```toml
[dependencies]
ftracker-identifiers = { version = "0.0.1", features = ["serde", "schemars", "arbitrary", "proptest"] }
```

## `serde`

(De)serializes `Lei` as its canonical 20-character string (e.g. `"5493000IBP32UQZ0KL24"`), so it round-trips as a plain
identifier in JSON, YAML, or config files.

Deserialization always re-runs full validation: an untrusted payload (a malformed API request, a hand-edited config
file) can never produce an invalid `Lei`. A bad value fails to deserialize with a descriptive error instead of silently
producing garbage.

```rust,ignore
use ftracker_identifiers::Lei;

let lei = Lei::parse("5493000IBP32UQZ0KL24").unwrap();
let json = serde_json::to_string(&lei).unwrap();
assert_eq!(json, "\"5493000IBP32UQZ0KL24\"");

let back: Lei = serde_json::from_str(&json).unwrap();
assert_eq!(lei, back);

// Invalid input is rejected at deserialization time, not silently accepted.
assert!(serde_json::from_str::<Lei>("\"not-an-lei\"").is_err());
```

## `schemars`

Implements `JsonSchema` for `Lei`, so it can appear in a generated OpenAPI/JSON Schema document as a pattern-constrained
string rather than an opaque `string` type. This feature implies `serde`.

The generated schema:

```json
{
  "type": "string",
  "format": "lei",
  "minLength": 20,
  "maxLength": 20,
  "pattern": "^[A-Z0-9]{18}[0-9]{2}$",
  "description": "LEI (Legal Entity Identifier, ISO 17442), ISO/IEC 7064 MOD 97-10 checksum-valid."
}
```

Note that the `pattern` alone doesn't capture the MOD 97-10 checksum constraint. Schema validators outside this crate (
API gateways, other-language clients) can reject malformed shapes, but true checksum validation still requires this
crate (or a reimplementation of the same algorithm; see [Structure & Formats](./format.md)).

## `arbitrary`

Implements `Arbitrary` for `Lei`, so fuzz targets (e.g. via `cargo fuzz`) can generate structurally valid,
checksum-correct `Lei` values directly, instead of generating raw strings that mostly fail validation before reaching
the code under test.

## `proptest`

Exposes reusable [`proptest`](https://docs.rs/proptest) strategies at
`ftracker_identifiers::lei::proptest`:

- `lei::proptest::valid_lei()` — a `Strategy<Value = Lei>` producing checksum-correct values with an alphanumeric base
  and matching MOD 97-10 check digits.
- `lei::proptest::valid_lei_string()` — the same, rendered as a canonical `String`, useful for
  round-trip-through-parsing property tests.

This is the recommended way to property-test *your own* code that accepts a `Lei`, without hand-rolling a checksum-valid
generator:

```rust,ignore
use ftracker_identifiers::{lei::proptest::valid_lei, Lei};
use proptest::proptest;

proptest! {
    #[test]
    fn my_function_accepts_any_valid_lei(lei in valid_lei()) {
        // exercise your own code with `lei` here
    }
}
```
