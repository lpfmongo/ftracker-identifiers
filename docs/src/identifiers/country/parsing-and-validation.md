# Parsing & Validation

## Constructors

* `CountryCode::parse` and `CountryCode::new` accept two character strings, in any ASCII case, trimmed of surrounding
  whitespace.
* `CountryCode::from_bytes` accepts exactly two pre normalized uppercase ASCII bytes.
* `FromStr` and `TryFrom<&str>` behave like `parse`, for use in generic code.

`CountryCode::new` is a plain alias for `CountryCode::parse`. `FromStr` and `TryFrom<&str>` both delegate to it too, so
`"...".parse::<CountryCode>()` and `CountryCode::try_from("...")` behave identically to calling `CountryCode::parse`
directly.

`CountryCode::from_bytes` is the lower level constructor. It skips whitespace trimming and case folding and expects an
already normalized `[u8; 2]`. It still runs every validation rule below. Prefer `CountryCode::parse` unless you are
constructing bytes programmatically (for example, in a generator or a migration script).

## What `CountryCode::parse` accepts

* The canonical two character form.
* Lowercase letters. They are folded to uppercase automatically.
* Leading and trailing whitespace. It is trimmed before validation.

Interior separators are **not** stripped. A country code has no internal punctuation, so a character that is not a
letter is reported as invalid rather than silently removed.

```rust,ignore
use ftracker_identifiers::CountryCode;

assert!(CountryCode::parse("US").is_ok());
assert!(CountryCode::parse("us").is_ok());    // lowercase is folded
assert!(CountryCode::parse("  BR ").is_ok()); // surrounding whitespace is trimmed
assert!(CountryCode::parse("ZZ").is_err());   // well formed but not assigned
```

## Validation rules

The string based constructors run the following rules, in this order:

1. **Length**: after surrounding whitespace is trimmed, the input must contain exactly two characters.
   (`CountryCode::parse` rejects an empty string first.)
2. **Character class**: both positions must be an uppercase ASCII letter (`A` to `Z`).
3. **Assignment**: the two letters together must be a code that ISO 3166-1 officially assigns.

Rule 3 is checked against the embedded bitmap with a single array index and a single bit test. There is no heap
allocation and no runtime initialization. Each rule maps to exactly one `CountryCodeError` variant. See
[Error Handling](./error-handling.md) for the full list and how to match on it.

## What it doesn't do

`CountryCode::parse` validates shape and membership, not meaning. It cannot tell you which country a code names (the
names are not embedded, see [Structure & Formats](./format.md)), and it does not map between the alpha-2 code and the
alpha-3 or numeric forms.
