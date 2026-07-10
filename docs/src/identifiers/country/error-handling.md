# Error Handling

Every fallible constructor returns `CountryCodeError` on failure. It is `Clone + PartialEq + Eq`, and it implements both
`core::error::Error` and `core::fmt::Display`, so it composes cleanly with `?` and with error aggregation crates
(`anyhow`, `thiserror`, `eyre`, and friends).

## Variants

* `Empty`: the input string was empty.
* `InvalidLength`: after trimming surrounding whitespace, the input did not contain exactly two characters. It carries
  the count found.
* `InvalidCharacter`: a non letter appeared at a given position. It carries the character and its 1 indexed position.
* `Unassigned`: the two letters are well formed but are not a code that ISO 3166-1 officially assigns. It carries the
  offending code as two characters.

`Unassigned` is the variant that sets a country code apart from a purely structural check. The two letters look right,
but the pair is not in the assigned set. Reserved codes such as `EU` and `UK` surface here too, because they are
reserved rather than assigned.

## Matching on specific failures

Reach for a `match` when you need to react differently to different failure modes, for example turning a specific error
into a targeted field level message for a form:

```rust,ignore
use ftracker_identifiers::{CountryCode, CountryCodeError};

match CountryCode::parse(user_input) {
    Ok(code) => save(code),
    Err(CountryCodeError::Empty) => reject("country code is required"),
    Err(CountryCodeError::InvalidLength { found }) => {
        reject(&format!("expected 2 characters, found {found}"))
    }
    Err(CountryCodeError::InvalidCharacter { character, position }) => {
        reject(&format!("unexpected '{character}' at position {position}"))
    }
    Err(CountryCodeError::Unassigned { code }) => {
        reject(&format!("'{}{}' is not an assigned country code", code[0], code[1]))
    }
    Err(_) => reject("country code is not valid"),
}
```

The final arm handles any variant added in a future release: `CountryCodeError` is marked non exhaustive, so downstream
`match` expressions include a catch all.

## Just want a message?

If you do not need to distinguish between failure modes, the `Display` implementation already produces a human readable
message, so `?` and `.to_string()` work as expected:

```rust,ignore
use ftracker_identifiers::CountryCode;

fn parse_code(input: &str) -> Result<CountryCode, String> {
    CountryCode::parse(input).map_err(|e| e.to_string())
}
```

## Untrusted input is always re-validated

This matters most when the `serde` feature is enabled: deserializing a `CountryCode` from JSON, YAML, or any other
`serde` format re-runs the exact same validation, including the membership check, as `CountryCode::parse`. There is no
serialization shortcut that could let an invalid value slip through. See [Feature Flags](./feature-flags.md).
