# Structure & Formats

An LEI always has **20 characters**, split into three segments (ISO 17442:2020):

| Positions | Length | Segment      | Meaning                                                        |
|-----------|--------|--------------|----------------------------------------------------------------|
| 1–4       | 4      | LOU prefix   | Identifies the Local Operating Unit (LEI issuer), alphanumeric |
| 5–18      | 14     | Entity part  | Entity-specific identifier assigned by the LOU, alphanumeric   |
| 19–20     | 2      | Check digits | ISO/IEC 7064 MOD 97-10 digits computed over the first 18 chars |

`Lei` exposes each segment as a borrowed accessor:

- `Lei::lou_prefix()` — the 4-character LOU (issuer) prefix.
- `Lei::entity_id()` — the 14-character entity-specific part.
- `Lei::check_digits()` — the two check digits, as a `u8` (`0..=99`).
- `Lei::computed_check_digits()` — recomputes the check digits from the first 18 characters (equal to
  `check_digits()` for any valid `Lei`).

LEI has **no conventional punctuated form**. Its canonical rendering is simply the 20-character string, so there is no
separate "compact vs. formatted" distinction. See [Formatting & Display](./formatting-and-display.md).

## The check digits

The final two characters are a checksum computed with **ISO/IEC 7064, MOD 97-10** — the same scheme used by IBAN (ISO
13616). Each of the 20 characters is expanded to a numeric value:

- Digits contribute their own value (`'0'` → 0, ..., `'9'` → 9).
- Letters contribute their two-digit ordinal value (`'A'` → 10, ..., `'Z'` → 35), i.e. two digits each.

The expanded characters are read as one large base-10 integer.

- **Validation:** the LEI is valid if and only if that integer is congruent to `1` modulo 97 (`n mod 97 == 1`).
- **Computation:** to derive the check digits for a fresh 18-character base, append `"00"`, take the integer modulo 97,
  and compute `98 - (n mod 97)`, rendered as two digits.

Because the 20-character expansion is far larger than any fixed-width integer, this crate folds the modulo character by
character, so the whole routine stays allocation-free and `no_std`-friendly.

```rust,ignore
use ftracker_identifiers::Lei;

let bbc = Lei::parse("5493000IBP32UQZ0KL24").unwrap();
assert_eq!(bbc.check_digits(), 24);
assert_eq!(bbc.computed_check_digits(), 24);
```

## What the LEI is (and isn't) validated against

The first 18 characters are validated *structurally*: they must be ASCII alphanumeric (`[A-Z0-9]`). The last two must be
ASCII digits. On top of that, the MOD 97-10 residue must equal `1`.

This crate **deliberately validates the ISO 17442 code definition only**, not GLEIF operational issuance policy. Two
specific things it does **not** enforce:

- **It does not require positions 5–6 to be `"00"`.** The Global LEI System currently allocates codes with `"00"` there,
  but that is an issuance convention of GLEIF, not a rule of ISO 17442, and it may change. Enforcing it here would
  reject otherwise standard-conformant identifiers. (This mirrors how `Isin` validates its country-code prefix purely
  structurally.)
- **It does not additionally reject the check-digit values `00`, `01`, or `99`.** GLEIF notes these never arise from a
  correctly *generated* LEI, but they are not excluded by the ISO/IEC 7064 arithmetic itself, so this crate keeps to the
  literal `n mod 97 == 1` test.

The consequence: a value that passes here is a structurally valid, MOD 97-10-correct LEI per ISO 17442. It is **not** a
guarantee that GLEIF has actually issued that specific code — see [Parsing & Validation](./parsing-and-validation.md).

## A note on ordering

`Lei` derives `Ord` directly over its underlying ASCII bytes, which matches `str` ordering on `Lei::as_str()`. This is *
*lexicographic string order**. Sorting a list of `Lei` values groups them by LOU prefix, but says nothing about issuance
date or any numeric interpretation of the code.
