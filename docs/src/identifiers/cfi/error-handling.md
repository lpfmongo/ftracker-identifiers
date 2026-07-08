# Error Handling

Every fallible constructor returns `CfiError` on failure. It's `Clone + PartialEq + Eq`, and it implements both
`core::error::Error` and `core::fmt::Display`, so it composes cleanly with `?` and with error-aggregation crates
(`anyhow`, `thiserror`, `eyre`, and friends).

## Variants

| Variant            | When it occurs                                                                                                            |
|--------------------|---------------------------------------------------------------------------------------------------------------------------|
| `Empty`            | The input string was empty.                                                                                               |
| `InvalidLength`    | After trimming surrounding whitespace, the input didn't contain exactly 6 characters (carries the count found).           |
| `InvalidCharacter` | A non-letter appeared at a given position (carries the character and 1-indexed position).                                 |
| `UnknownCategory`  | Position 1 is not a category defined by ISO 10962 (carries the offending code).                                           |
| `UnknownGroup`     | Position 2 is not a group of the resolved category (carries the category and offending group code).                       |
| `InvalidAttribute` | An attribute (positions 3–6) is not permitted for the resolved category and group (carries category, group, index, code). |

The three taxonomic variants — `UnknownCategory`, `UnknownGroup`, `InvalidAttribute` — are what set CFI apart from the
checksum-based identifiers: they tell you *which level* of the ISO 10962 hierarchy the code fell off, and at
`InvalidAttribute` the `index` field (1–4) says which of the four attributes was wrong.

## Matching on specific failures

Reach for a `match` when you need to react differently to different failure modes — for example, turning a specific
error into a targeted field-level message for a form:

```rust,ignore
use ftracker_identifiers::{Cfi, CfiError};

match Cfi::parse(user_input) {
    Ok(cfi) => save(cfi),
    Err(CfiError::Empty) => reject("CFI is required"),
    Err(CfiError::InvalidLength { found }) => {
        reject(&format!("expected 6 characters, found {found}"))
    }
    Err(CfiError::UnknownCategory { code }) => {
        reject(&format!("'{code}' is not a known CFI category"))
    }
    Err(CfiError::UnknownGroup { category, code }) => {
        reject(&format!("'{code}' is not a group of category '{category}'"))
    }
    Err(CfiError::InvalidAttribute { index, code, .. }) => {
        reject(&format!("attribute {index} ('{code}') is not valid here"))
    }
    Err(CfiError::InvalidCharacter { character, position }) => {
        reject(&format!("unexpected '{character}' at position {position}"))
    }
}
```

## Just want a message?

If you don't need to distinguish between failure modes, `CfiError`'s `Display` implementation already produces a
human-readable message, so `?` and `.to_string()` work as expected:

```rust,ignore
use ftracker_identifiers::Cfi;

fn parse_cfi(input: &str) -> Result<Cfi, String> {
    Cfi::parse(input).map_err(|e| e.to_string())
}
```

## Untrusted input is always re-validated

This matters most when the `serde` feature is enabled: deserializing a `Cfi` from JSON, YAML, or any other `serde`
format re-runs the exact same validation — including the taxonomy checks — as `Cfi::parse`. There is no serialization
shortcut that could let an invalid value slip through; see [Feature Flags](./feature-flags.md).
