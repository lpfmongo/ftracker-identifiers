# Structure & Formats

A CNPJ always has **14 meaningful characters**, split into three segments:

| Positions | Length | Segment                | Meaning                                                                  |
|-----------|--------|------------------------|--------------------------------------------------------------------------|
| 1–8       | 8      | Root (*raiz*)          | Identifies the entity itself; shared by the head office and every branch |
| 9–12      | 4      | Branch/order (*ordem*) | `"0001"` conventionally denotes the head office (*matriz*)               |
| 13–14     | 2      | Verification digits    | Computed from the first 12 characters via the Módulo 11 algorithm        |

`Cnpj` exposes each segment as a borrowed accessor:

- `Cnpj::root()` — the 8-character root.
- `Cnpj::branch_code()` — the 4-character branch/order segment.
- `Cnpj::is_root()` — `true` when the branch/order segment is `"0001"`.
- `Cnpj::branch_number()` — the branch/order segment as a `u16`, when it's purely numeric.
- `Cnpj::check_digits()` — the two verification digits, as `(u8, u8)`.

The conventional **punctuated** rendering is `AA.AAA.AAA/AAAA-DD`; the **compact** rendering drops
all punctuation. Both refer to the same 14 characters — see
[Formatting & Display](./formatting-and-display.md).

## Numeric vs. alphanumeric CNPJs

The public CNPJ format changed in 2026. Historically, all 14 characters were digits. As of the
2026 change, the first 12 positions (root + branch/order) may also contain **uppercase letters**;
the final two verification digits remain numeric either way.

This crate follows `Nota Técnica Conjunta COCAD/SUARA/RFB nº 49/2024`, which defines the checksum
so that the legacy numeric-only calculation is unchanged — it's simply the special case where every
character happens to be a digit. Each character contributes its ASCII code minus `'0'` to the
Módulo 11 sum:

- Digits contribute their own value (`'0'` → 0, ..., `'9'` → 9).
- Uppercase letters contribute 17 through 42 (`'A'` → 17, ..., `'Z'` → 42).

Because of this, **there is no separate "legacy" type** in this crate. `Cnpj` represents both
numeric-only and alphanumeric CNPJs uniformly, and there's a single code path (and a single test
suite) validating both.

```rust,ignore
use ftracker_identifiers::Cnpj;

// A numeric-only CNPJ (the historical format).
let numeric = Cnpj::parse("00.000.000/0001-91").unwrap();

// An alphanumeric CNPJ (the 2026 format) — same type, same validation.
let alphanumeric = Cnpj::parse("12ABC34501DE35").unwrap();
assert_eq!(alphanumeric.branch_code(), "01DE");
```

## A note on ordering

`Cnpj` derives `Ord` directly over its underlying ASCII bytes, which matches `str` ordering on
`Cnpj::as_str()`. Because ASCII digits (`'0'..='9'`) sort before uppercase letters (`'A'..='Z'`), a
numeric-format CNPJ always sorts before any alphanumeric CNPJ sharing the same leading digits. This
is **lexicographic string order**, not a numeric or chronological order — don't read a sorted list
of `Cnpj` values as meaning "issued earlier" or "smaller root number."