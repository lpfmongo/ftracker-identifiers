# Structure & Formats

A CFI always has **6 characters**, all uppercase letters, split into three parts:

| Positions | Length | Segment    | Meaning                                                              |
|-----------|--------|------------|----------------------------------------------------------------------|
| 1         | 1      | Category   | The broadest class of instrument (e.g. `E` = equities)               |
| 2         | 1      | Group      | A subdivision within the category (meaning depends on the category)  |
| 3–6       | 4      | Attributes | Four attribute codes whose meaning depends on the category and group |

`Cfi` exposes each part as an accessor:

- `Cfi::category()` — the category code, as a `char`.
- `Cfi::group()` — the group code, as a `char`.
- `Cfi::attributes()` — the four attribute codes, as a `[char; 4]`.
- `Cfi::as_str()` — the whole 6-character code.

CFI has **no conventional punctuated form**. Its canonical rendering is simply the 6-character string, so there is no
separate "compact vs. formatted" distinction. See [Formatting & Display](./formatting-and-display.md).

## How the positions nest

The six positions are not independent: each narrows the meaning of the ones after it.

- **Position 1 (category)** must be one of the categories ISO 10962 defines (`E`, `C`, `D`, `R`, `O`, `F`, `S`, `H`,
  `I`, `J`, `K`, `L`, `T`, `M`).
- **Position 2 (group)** must be a group defined *for that category*. The same letter can be a valid group under one
  category and meaningless under another.
- **Positions 3–6 (attributes)** must each be a code the standard permits for that specific category **and** group at
  that attribute position. The letter `X` conventionally marks an attribute that is "not applicable" — but only where
  the standard actually lists it.

```rust,ignore
use ftracker_identifiers::Cfi;

// Equity (E) / common share (S), with four attribute codes.
let equity = Cfi::parse("ESVUFR").unwrap();
assert_eq!(equity.category(), 'E');
assert_eq!(equity.group(), 'S');

// Debt (D) / bond (B) — a different category with different valid attributes.
let bond = Cfi::parse("DBFTFB").unwrap();
assert_eq!(bond.category(), 'D');
assert_eq!(bond.attributes(), ['F', 'T', 'F', 'B']);
```

## Codes, not meanings

This crate embeds only the ISO 10962 **code structure** — which category, group, and attribute letters are valid, and in
which combinations. It does **not** embed ISO's descriptive text (the prose names and definitions of each code), which
is the copyrighted part of the standard.

As a result, `Cfi` can confirm that `ESVUFR` is a well-formed equity classification and pinpoint exactly which position
is wrong in a bad code, but it will not resolve `V` to "voting" or `S` to"common/ordinary shares". If you need the
human-readable meanings, consult the ISO 10962 standard itself.

## A note on ordering

`Cfi` derives `Ord` directly over its underlying ASCII bytes, which matches `str` ordering on `Cfi::as_str()`. This is
**lexicographic string order**: sorting groups CFIs by category letter, then group, and so on, but carries no taxonomic
meaning beyond that.
