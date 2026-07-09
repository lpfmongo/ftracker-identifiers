# Feature Flags

All of `CountryCode`'s optional integrations are off by default and purely additive. Enabling one never changes what
counts as a valid country code, only what you can do with a `CountryCode` once you have one.

```toml
[dependencies]
ftracker-identifiers = { version = "0.0.1", features = ["serde", "schemars", "arbitrary", "proptest"] }
```

## `serde`

(De)serializes `CountryCode` as its canonical two letter string (for example `"US"`), so it round trips as a plain
identifier in JSON, YAML, or config files.

Deserialization always re-runs full validation, including the membership check. An untrusted payload (a malformed API
request, a hand edited config file) can never produce an invalid `CountryCode`. A bad value fails to deserialize with a
descriptive error instead of silently producing garbage.

```rust,ignore
use ftracker_identifiers::CountryCode;

let code = CountryCode::parse("US").unwrap();
let json = serde_json::to_string(&code).unwrap();
assert_eq!(json, "\"US\"");

let back: CountryCode = serde_json::from_str(&json).unwrap();
assert_eq!(code, back);

// Invalid input is rejected at deserialization time, not silently accepted.
assert!(serde_json::from_str::<CountryCode>("\"ZZ\"").is_err());
```

## `schemars`

Implements `JsonSchema` for `CountryCode`, so it can appear in a generated OpenAPI or JSON Schema document as a pattern
constrained string rather than an opaque `string` type. This feature implies `serde`.

The generated schema:

```json
{
  "type": "string",
  "format": "iso3166-1-alpha2",
  "minLength": 2,
  "maxLength": 2,
  "pattern": "^[A-Z]{2}$",
  "description": "ISO 3166-1 alpha-2 country code. The pattern is structural; membership in the assigned set is enforced on deserialization."
}
```

The `pattern` only captures the two uppercase letter **shape**. A regex cannot express which two letter codes are
assigned. Schema validators outside this crate can reject malformed shapes, but true membership validation still
requires this crate.

## `arbitrary`

Implements `Arbitrary` for `CountryCode`, so fuzz targets (for example via `cargo fuzz`) can generate assigned codes
directly, by choosing from the embedded set, instead of generating raw strings that mostly fail validation before
reaching the code under test.

## `proptest`

Exposes reusable [`proptest`](https://docs.rs/proptest) strategies at `ftracker_identifiers::country::proptest`:

* `country::proptest::valid_country_code()`: a `Strategy<Value = CountryCode>` producing valid values by choosing from
  the assigned set.
* `country::proptest::valid_country_code_string()`: the same, rendered as a canonical `String`, useful for round trip
  through parsing property tests.

This is the recommended way to property test your own code that accepts a `CountryCode`, without hand rolling a
generator that only yields assigned codes:

```rust,ignore
use ftracker_identifiers::{country::proptest::valid_country_code, CountryCode};
use proptest::proptest;

proptest! {
    #[test]
    fn my_function_accepts_any_valid_country_code(code in valid_country_code()) {
        // exercise your own code with `code` here
    }
}
```
