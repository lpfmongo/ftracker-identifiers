# Parsing & Validation

## Constructors

| Constructor                 | Accepts                                        |
|-----------------------------|------------------------------------------------|
| `Cfi::parse` / `Cfi::new`   | 6-character strings, any ASCII case, trimmed   |
| `Cfi::from_bytes`           | Exactly 6 pre-normalized uppercase ASCII bytes |
| `FromStr` / `TryFrom<&str>` | Same as `parse`, for use in generic code       |

`Cfi::new` is a plain alias for `Cfi::parse`; `FromStr` and `TryFrom<&str>` both delegate to it too, so
`"...".parse::<Cfi>()` and `Cfi::try_from("...")` behave identically to calling `Cfi::parse` directly.

`Cfi::from_bytes` is the lower-level constructor: it skips whitespace-trimming and case-folding and expects an
already-normalized `[u8; 6]`. It still runs every validation rule below. Prefer`Cfi::parse` unless you're constructing
bytes programmatically (for example, in a generator or migration script).

## What `Cfi::parse` accepts

- The canonical 6-character form.
- Lowercase letters — they're folded to uppercase automatically.
- Leading and trailing whitespace — it's trimmed before validation.

Interior separators are **not** stripped: a CFI has no conventional internal punctuation, so a character in the middle
of the string that isn't a letter is reported as invalid rather than silently removed.

```rust,ignore
use ftracker_identifiers::Cfi;

assert!(Cfi::parse("ESVUFR").is_ok());
assert!(Cfi::parse("esvufr").is_ok());    // lowercase is folded
assert!(Cfi::parse("  ESVUFR ").is_ok()); // surrounding whitespace is trimmed
assert!(Cfi::parse("EZVUFR").is_err());   // 'Z' is not a group of category 'E'
```

## Validation rules

The string-based constructors run the following rules, in this order:

1. **Length** — after surrounding whitespace is trimmed, the input must contain exactly 6 characters. (`Cfi::parse`
   rejects an empty string up front.)
2. **Character class** — every position must be an uppercase ASCII letter (`A`–`Z`).
3. **Category** — position 1 must be a category defined by ISO 10962.
4. **Group** — position 2 must be a group defined for that category.
5. **Attributes** — each of positions 3–6 must be a code the standard permits for the resolved category and group at
   that attribute position.

Rules 3–5 are checked against the embedded taxonomy table with a couple of allocation-free binary searches and bitmask
tests — no heap, no runtime initialization. Each rule maps to exactly one `CfiError` variant;
see [Error Handling](./error-handling.md) for the full list and how to match on it.

## What it doesn't do

`Cfi::parse` validates *shape and taxonomy*, not *meaning* or *existence*. It cannot tell you what a code classifies in
human-readable terms (the descriptive names are not embedded; see [Structure & Formats](./format.md)), nor whether a
particular instrument has been assigned that classification by an issuer or numbering agency.
