# Error Handling

Every fallible constructor returns `IsinError` on failure. It's `Clone + PartialEq + Eq`, and it
implements both `core::error::Error` and `core::fmt::Display`, so it composes cleanly with `?` and
with error-aggregation crates (`anyhow`, `thiserror`, `eyre`, and friends).

## Variants

| Variant             | When it occurs                                                                                                                                  |
|---------------------|-------------------------------------------------------------------------------------------------------------------------------------------------|
| `Empty`             | The input string was empty.                                                                                                                     |
| `InvalidLength`     | After trimming surrounding whitespace, the input didn't contain exactly 12 characters (carries the count found).                                |
| `InvalidCharacter`  | A character outside the allowed set appeared at a given position (carries the character, 1-indexed position, and the expected character class). |
| `InvalidCheckDigit` | The ISO 6166 Luhn check digit at position 12 didn't match (carries the expected and found digits).                                              |

The expected character class is one of `Letter` (positions 1–2), `Alphanumeric` (positions 3–11),
or `Digit` (position 12).

## Matching on specific failures

Reach for a `match` when you need to react differently to different failure modes. For example, turning a specific error
into a targeted field-level message for a form, rather than just surfacing the generic `Display` text:

```rust,ignore
use ftracker_identifiers::{Isin, IsinError};

match Isin::parse(user_input) {
    Ok(isin) => save(isin),
    Err(IsinError::Empty) => reject("ISIN is required"),
    Err(IsinError::InvalidLength { found }) => {
        reject(&format!("expected 12 characters, found {found}"))
    }
    Err(IsinError::InvalidCharacter { character, position, .. }) => {
        reject(&format!("unexpected '{character}' at position {position}"))
    }
    Err(IsinError::InvalidCheckDigit { expected, found }) => {
        reject(&format!("check digit looks wrong: expected {expected}, found {found}"))
    }
}
```

## Just want a message?

If you don't need to distinguish between failure modes, `IsinError`'s `Display` implementation already produces a
human-readable message, so `?` and `.to_string()` work as expected:

```rust,ignore
use ftracker_identifiers::Isin;

fn parse_isin(input: &str) -> Result<Isin, String> {
    Isin::parse(input).map_err(|e| e.to_string())
}
```

## Untrusted input is always re-validated

This matters most when the `serde` feature is enabled: deserializing an `Isin` from JSON, YAML, or any other `serde`
format re-runs the exact same validation as `Isin::parse`. There is no serialization shortcut that could let an invalid
value slip through; see [Feature Flags](./feature-flags.md).
