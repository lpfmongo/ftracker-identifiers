# Error Handling

Every fallible constructor returns `LeiError` on failure. It's `Clone + PartialEq + Eq`, and it implements both
`core::error::Error` and `core::fmt::Display`, so it composes cleanly with `?` and with error-aggregation crates (
`anyhow`, `thiserror`, `eyre`, and friends).

## Variants

| Variant              | When it occurs                                                                                                                                  |
|----------------------|-------------------------------------------------------------------------------------------------------------------------------------------------|
| `Empty`              | The input string was empty.                                                                                                                     |
| `InvalidLength`      | After trimming surrounding whitespace, the input didn't contain exactly 20 characters (carries the count found).                                |
| `InvalidCharacter`   | A character outside the allowed set appeared at a given position (carries the character, 1-indexed position, and the expected character class). |
| `InvalidCheckDigits` | The ISO/IEC 7064 MOD 97-10 check digits at positions 19–20 didn't match (carries the expected and found values, each `0..=99`).                 |

The expected character class is one of `Alphanumeric` (positions 1–18) or `Digit` (positions 19–20).

## Matching on specific failures

Reach for a `match` when you need to react differently to different failure modes. For example, turning a specific error
into a targeted field-level message for a form, rather than just surfacing the generic `Display` text:

```rust,ignore
use ftracker_identifiers::{Lei, LeiError};

match Lei::parse(user_input) {
    Ok(lei) => save(lei),
    Err(LeiError::Empty) => reject("LEI is required"),
    Err(LeiError::InvalidLength { found }) => {
        reject(&format!("expected 20 characters, found {found}"))
    }
    Err(LeiError::InvalidCharacter { character, position, .. }) => {
        reject(&format!("unexpected '{character}' at position {position}"))
    }
    Err(LeiError::InvalidCheckDigits { expected, found }) => {
        reject(&format!("check digits look wrong: expected {expected:02}, found {found:02}"))
    }
}
```

## Just want a message?

If you don't need to distinguish between failure modes, `LeiError`'s `Display` implementation already produces a
human-readable message, so `?` and `.to_string()` work as expected:

```rust,ignore
use ftracker_identifiers::Lei;

fn parse_lei(input: &str) -> Result<Lei, String> {
    Lei::parse(input).map_err(|e| e.to_string())
}
```

## Untrusted input is always re-validated

This matters most when the `serde` feature is enabled: deserializing a `Lei` from JSON, YAML, or any other `serde`
format re-runs the exact same validation as `Lei::parse`. There is no serialization shortcut that could let an invalid
value slip through; see [Feature Flags](./feature-flags.md).
