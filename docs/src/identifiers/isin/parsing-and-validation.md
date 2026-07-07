# Parsing & Validation

## Constructors

| Constructor                 | Accepts                                         |
|-----------------------------|-------------------------------------------------|
| `Isin::parse` / `Isin::new` | 12-character strings, any ASCII case, trimmed   |
| `Isin::from_bytes`          | Exactly 12 pre-normalized uppercase ASCII bytes |
| `FromStr` / `TryFrom<&str>` | Same as `parse`, for use in generic code        |

`Isin::new` is a plain alias for `Isin::parse`; `FromStr` and `TryFrom<&str>` both delegate to it too, so
`"...".parse::<Isin>()` and `Isin::try_from("...")` behave identically to calling`Isin::parse` directly.

`Isin::from_bytes` is the lower-level constructor: it skips whitespace-trimming and case-folding and expects an
already-normalized `[u8; 12]`. It still runs every validation rule below. It just assumes the caller has already dealt
with formatting. Prefer `Isin::parse` unless you're constructing bytes programmatically (for example, in a generator or
migration script).

## What `Isin::parse` accepts

- The canonical 12-character form: `CCNNNNNNNNND`.
- Lowercase letters — they're folded to uppercase automatically.
- Leading and trailing whitespace — it's trimmed before validation.

Interior separators are **not** stripped: an ISIN has no conventional internal punctuation, so a character in the middle
of the string that isn't a letter or digit is reported as invalid rather than silently removed.

```rust,ignore
use ftracker_identifiers::Isin;

assert!(Isin::parse("US0378331005").is_ok());
assert!(Isin::parse("us0378331005").is_ok());   // lowercase is folded
assert!(Isin::parse("  US0378331005 ").is_ok()); // surrounding whitespace is trimmed
assert!(Isin::parse("US0378331006").is_err());   // wrong check digit
```

## Validation rules

Every fallible constructor runs the same rules, in this order:

1. **Length** — after surrounding whitespace is trimmed, the input must contain exactly 12 characters. (`Isin::parse`
   rejects an empty string up front.)
2. **Character class** — positions 1–2 accept an uppercase letter; positions 3–11 accept a digit or an uppercase letter;
   position 12 accepts only a digit.
3. **Check digit** — position 12 must match the ISO 6166 Luhn digit computed from the first 11 characters.

Each rule maps to exactly one `IsinError` variant; see [Error Handling](./error-handling.md) for the full list and how
to match on it.

## What it doesn't do

`Isin::parse` validates *shape and checksum*, not *existence*. It cannot tell you whether a particular ISIN has actually
been allocated, identifies a tradable instrument, or refers to the security you think it does. That requires a lookup
against an issuing agency or market-data provider, which is out of scope for this crate. 

It also does not validate the country code against the live ISO 3166-1 list (see [Structure & Formats](./format.md)).
