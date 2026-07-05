# Error Handling

Every fallible constructor returns `CnpjError` on failure. It's `Clone + PartialEq + Eq`, and it
implements both `core::error::Error` and `core::fmt::Display`, so it composes cleanly with `?` and
with error-aggregation crates (`anyhow`, `thiserror`, `eyre`, and friends).

## Variants

| Variant              | When it occurs                                                                                                                                  |
|----------------------|-------------------------------------------------------------------------------------------------------------------------------------------------|
| `Empty`              | The input string was empty.                                                                                                                     |
| `InvalidLength`      | After stripping punctuation (`.`, `/`, `-`, whitespace), the input didn't contain exactly 14 characters.                                        |
| `InvalidCharacter`   | A character outside the allowed set appeared at a given position (carries the character, 1-indexed position, and the expected character class). |
| `InvalidCheckDigits` | The Módulo 11 checksum didn't match one of the two verification digits (carries the position, expected digit, and found digit).                 |
| `RepeatedDigits`     | All 14 characters were identical (e.g. `"00000000000000"`) — structurally valid but never actually issued.                                      |

## Matching on specific failures

Reach for a `match` when you need to react differently to different failure modes — for example,
turning a specific error into a targeted, field-level message for a form, rather than just
surfacing the generic `Display` text:

```rust,ignore
use ftracker_identifiers::{Cnpj, CnpjError};

match Cnpj::parse(user_input) {
    Ok(cnpj) => save(cnpj),
    Err(CnpjError::Empty) => reject("CNPJ is required"),
    Err(CnpjError::InvalidLength { found }) => {
        reject(&format!("expected 14 characters, found {found}"))
    }
    Err(CnpjError::InvalidCharacter { character, position, .. }) => {
        reject(&format!("unexpected '{character}' at position {position}"))
    }
    Err(CnpjError::InvalidCheckDigits { .. }) => {
        reject("that CNPJ doesn't look right — check for typos")
    }
    Err(CnpjError::RepeatedDigits) => reject("that doesn't look like a real CNPJ"),
}
```

## Just want a message?

If you don't need to distinguish between failure modes, `CnpjError`'s `Display` implementation
already produces a human-readable message, so `?` and `.to_string()` work as expected:

```rust,ignore
use ftracker_identifiers::Cnpj;

fn parse_cnpj(input: &str) -> Result<Cnpj, String> {
    Cnpj::parse(input).map_err(|e| e.to_string())
}
```

## Untrusted input is always re-validated

This matters most when the `serde` feature is enabled: deserializing a `Cnpj` from JSON, YAML, or
any other `serde` format re-runs the exact same validation as `Cnpj::parse`. There is no
serialization shortcut that could let an invalid value slip through — see
[Feature Flags](./feature-flags.md).