# Parsing & Validation

## Constructors

| Constructor                 | Accepts                                         |
|-----------------------------|-------------------------------------------------|
| `Lei::parse` / `Lei::new`   | 20-character strings, any ASCII case, trimmed   |
| `Lei::from_bytes`           | Exactly 20 pre-normalized uppercase ASCII bytes |
| `FromStr` / `TryFrom<&str>` | Same as `parse`, for use in generic code        |

`Lei::new` is a plain alias for `Lei::parse`; `FromStr` and `TryFrom<&str>` both delegate to it too, so
`"...".parse::<Lei>()` and `Lei::try_from("...")` behave identically to calling `Lei::parse` directly.

`Lei::from_bytes` is the lower-level constructor: it skips whitespace-trimming and case-folding and expects an
already-normalized `[u8; 20]`. It still runs every validation rule below. It just assumes the caller has already dealt
with formatting. Prefer `Lei::parse` unless you're constructing bytes programmatically (for example, in a generator or
migration script).

## What `Lei::parse` accepts

- The canonical 20-character form: four alphanumeric LOU characters, fourteen alphanumeric entity-specific characters,
  and two numeric check digits.
- Lowercase letters — they're folded to uppercase automatically.
- Leading and trailing whitespace — it's trimmed before validation.

Interior separators are **not** stripped: an LEI has no conventional internal punctuation, so a character in the middle
of the string that isn't a letter or digit is reported as invalid rather than silently removed.

```rust,ignore
use ftracker_identifiers::Lei;

assert!(Lei::parse("5493000IBP32UQZ0KL24").is_ok());
assert!(Lei::parse("5493000ibp32uqz0kl24").is_ok());   // lowercase is folded
assert!(Lei::parse("  5493000IBP32UQZ0KL24 ").is_ok()); // surrounding whitespace is trimmed
assert!(Lei::parse("5493000IBP32UQZ0KL25").is_err());   // wrong check digits
```

## Validation rules

The string-based constructors run the following rules, in this order:

1. **Length** — after surrounding whitespace is trimmed, the input must contain exactly 20 characters. (`Lei::parse`
   rejects an empty string up front.)
2. **Character class** — positions 1–18 accept a digit or an uppercase letter; positions 19–20 accept only a digit.
3. **Check digits** — positions 19–20 must satisfy the ISO/IEC 7064 MOD 97-10 checksum computed from the first 18
   characters.

Each rule maps to exactly one `LeiError` variant; see [Error Handling](./error-handling.md) for the full list and how to
match on it.

## Validation policy: what this crate deliberately does *not* check

This crate validates the **ISO 17442 code definition** — structure plus the ISO/IEC 7064 MOD 97-10 checksum — and
nothing beyond it. One boundary is an intentional design decision rather than an oversight:

- **No `"00"` requirement at positions 5–6.** GLEIF's Global LEI System currently allocates codes with `"00"` in the two
  positions after the LOU prefix, but that is an *operational* convention, not a rule of ISO 17442. It can change, and
  encoding it here would reject codes that are otherwise standard-conformant. `Lei` therefore validates positions 5–18
  purely as alphanumeric, the same way `Isin` validates its country-code prefix structurally rather than against a live
  registry.

The check-digit values `00`, `01`, and `99` are a different matter — they are **not** accepted, because they can never
appear in a valid LEI. The check digits equal `98 - (n mod 97)`, which always lands in `02..=98`, and ISO 17442-1 limits
the pair to that range too. `Lei` enforces this by comparing the candidate's digits against the recomputed pair, so a
value carrying `00`, `01`, or `99` is rejected with `InvalidCheckDigits` like any other mismatch. (A weaker
*residue-only* `n mod 97 == 1` test would let some such values through; this crate does not use one.)

If you need to confirm that a specific LEI has actually been *issued and registered*, look it up in
the [Global LEI Index](https://search.gleif.org/); that existence check is out of scope for a checksum-oriented value
type.

## What it doesn't do

`Lei::parse` validates *shape and checksum*, not *existence*. It cannot tell you whether a particular LEI has actually
been allocated by an LOU, is currently active (LEIs must be renewed annually), or refers to the entity you think it
does. That requires a lookup against GLEIF or an LOU, which is out of scope for this crate.
