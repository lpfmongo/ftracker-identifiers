# Structure & Formats

An ISIN always has **12 characters**, split into three segments:

| Positions | Length | Segment      | Meaning                                                            |
|-----------|--------|--------------|--------------------------------------------------------------------|
| 1–2       | 2      | Country code | ISO 3166-1 alpha-2 prefix of the issuing national numbering agency |
| 3–11      | 9      | NSIN         | National Securities Identifying Number, alphanumeric               |
| 12        | 1      | Check digit  | Luhn (modulus 10) digit computed over the first 11 characters      |

`Isin` exposes each segment as a borrowed accessor:

- `Isin::country_code()` — the 2-character country code.
- `Isin::nsin()` — the 9-character National Securities Identifying Number.
- `Isin::check_digit()` — the check digit, as a `u8` (`0..=9`).
- `Isin::computed_check_digit()` — recomputes the check digit from the first 11 characters (equal to
  `check_digit()` for any valid `Isin`).

ISIN has **no conventional punctuated form**. Its canonical rendering is simply the 12-character string, so there is no
separate "compact vs. formatted" distinction. See [Formatting & Display](./formatting-and-display.md).

## The check digit

The final character is a checksum computed with the modulus-10 "double-add-double" (Luhn) algorithm
described in ISO 6166 Annex C. Each of the first 11 characters is first expanded to decimal digits:

- Digits contribute their own value (`'0'` → 0, ..., `'9'` → 9).
- Letters contribute their two-digit ordinal value (`'A'` → 10, ..., `'Z'` → 35), i.e. two digits
  each.

The Luhn doubling rule is then applied to the expanded digit sequence so that the check digit, once
appended, lands in the units position. Because letters expand to two digits, the country code and
an alphanumeric NSIN both participate in the checksum.

```rust,ignore
use ftracker_identifiers::Isin;

// An all-numeric NSIN.
let amazon = Isin::parse("US0231351067").unwrap();

// An NSIN containing letters — same type, same validation.
let petrobras = Isin::parse("BRPETRACNOR9").unwrap();
assert_eq!(petrobras.nsin(), "PETRACNOR");
assert_eq!(petrobras.check_digit(), 9);
```

## What the country code is (and isn't)

The first two characters are validated *structurally*. They must be two uppercase ASCII letters.

This crate deliberately does **not** check them against the live ISO 3166-1 country registry (which changes over time
and includes special allocations such as `XS` for international securities). Validating "is this a real,
currently-assigned country code?" is out of scope for a checksum-oriented value type.

## A note on ordering

`Isin` derives `Ord` directly over its underlying ASCII bytes, which matches `str` ordering on `Isin::as_str()`. This is
**lexicographic string order**. Sorting a list of `Isin` values groups them by country-code prefix, but says nothing
about issuance date or any numeric interpretation of the NSIN.
