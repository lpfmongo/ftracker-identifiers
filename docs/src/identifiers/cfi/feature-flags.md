# Feature Flags

All of `Cfi`'s optional integrations are off by default and purely additive. Enabling one never
changes what counts as a valid CFI, only what you can *do* with a `Cfi` once you have one.

```toml
[dependencies]
ftracker-identifiers = { version = "0.0.1", features = ["serde", "schemars", "arbitrary", "proptest"] }
```

## `serde`

(De)serializes `Cfi` as its canonical 6-character string (e.g. `"ESVUFR"`), so it round-trips as a
plain identifier in JSON, YAML, or config files.

Deserialization always re-runs full validation, including the taxonomy checks: an untrusted payload
(a malformed API request, a hand-edited config file) can never produce an invalid `Cfi`. A bad value
fails to deserialize with a descriptive error instead of silently producing garbage.

```rust,ignore
use ftracker_identifiers::Cfi;

let cfi = Cfi::parse("ESVUFR").unwrap();
let json = serde_json::to_string(&cfi).unwrap();
assert_eq!(json, "\"ESVUFR\"");

let back: Cfi = serde_json::from_str(&json).unwrap();
assert_eq!(cfi, back);

// Invalid input is rejected at deserialization time, not silently accepted.
assert!(serde_json::from_str::<Cfi>("\"not-a-cfi\"").is_err());
```

## `schemars`

Implements `JsonSchema` for `Cfi`, so it can appear in a generated OpenAPI/JSON Schema document as a
pattern-constrained string rather than an opaque `string` type. This feature implies `serde`.

The generated schema:

```json
{
  "type": "string",
  "format": "cfi",
  "minLength": 6,
  "maxLength": 6,
  "pattern": "^[A-Z]{6}$",
  "description": "CFI (Classification of Financial Instruments, ISO 10962). The pattern is structural; taxonomic validity is enforced on deserialization."
}
```

The `pattern` only captures the six-uppercase-letter *shape*; a regex cannot express which
category/group/attribute *combinations* ISO 10962 defines. Schema validators outside this crate can
reject malformed shapes, but true taxonomic validation still requires this crate.

## `arbitrary`

Implements `Arbitrary` for `Cfi`, so fuzz targets (e.g. via `cargo fuzz`) can generate
*taxonomically valid* `Cfi` values directly — by walking the embedded ISO 10962 table — instead of
generating raw strings that mostly fail validation before reaching the code under test.

## `proptest`

Exposes reusable [`proptest`](https://docs.rs/proptest) strategies at
`ftracker_identifiers::cfi::proptest`:

- `cfi::proptest::valid_cfi()` — a `Strategy<Value = Cfi>` producing valid values by choosing a
  category, a group within it, and a permitted letter for each attribute.
- `cfi::proptest::valid_cfi_string()` — the same, rendered as a canonical `String`, useful for
  round-trip-through-parsing property tests.

This is the recommended way to property-test *your own* code that accepts a `Cfi`, without
hand-rolling a taxonomy-aware generator:

```rust,ignore
use ftracker_identifiers::{cfi::proptest::valid_cfi, Cfi};
use proptest::proptest;

proptest! {
    #[test]
    fn my_function_accepts_any_valid_cfi(cfi in valid_cfi()) {
        // exercise your own code with `cfi` here
    }
}
```
