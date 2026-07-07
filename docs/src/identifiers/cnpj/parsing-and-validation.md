# Parsing & Validation

## Constructors

| Constructor                 | Accepts                                               |
|-----------------------------|-------------------------------------------------------|
| `Cnpj::parse` / `Cnpj::new` | Punctuated or compact strings, any ASCII case         |
| `Cnpj::from_bytes`          | Exactly 14 pre-normalized ASCII bytes, no punctuation |
| `FromStr` / `TryFrom<&str>` | Same as `parse`, for use in generic code              |

`Cnpj::new` is a plain alias for `Cnpj::parse`; `FromStr` and `TryFrom<&str>` both delegate to it
too, so `"...".parse::<Cnpj>()` and `Cnpj::try_from("...")` behave identically to calling
`Cnpj::parse` directly.

`Cnpj::from_bytes` is the lower-level constructor: it skips punctuation-stripping and case-folding
and expects an already-normalized `[u8; 14]`. It still runs every validation rule below — it just
assumes the caller has already dealt with formatting. Prefer `Cnpj::parse` unless you're
constructing bytes programmatically (for example, in a generator or migration script).

## What `Cnpj::parse` accepts

- The conventional punctuated form: `AA.AAA.AAA/AAAA-DD`.
- The compact 14-character form: `AAAAAAAAAAAADD`.
- Lowercase letters in the alphanumeric portion — they're folded to uppercase automatically.
- Extra ASCII spaces, anywhere in the string (leading, trailing, or embedded).

```rust,ignore
use ftracker_identifiers::Cnpj;

assert!(Cnpj::parse("00.000.000/0001-91").is_ok());
assert!(Cnpj::parse("00000000000191").is_ok());
assert!(Cnpj::parse(" 00.000.000/0001-91 ").is_ok());
assert!(Cnpj::parse("12abc34501de35").is_ok()); // lowercase is folded
```

## Validation rules

The string-based constructors run the following rules, in this order:

1. **Length** — after formatting is stripped, the input must contain exactly 14 meaningful
   characters.
2. **Character class** — positions 1–12 accept a digit or an uppercase letter; positions 13–14
   accept only a digit.
3. **Not degenerate** — the 14 characters cannot all be identical (e.g. `"00000000000000"`). Such
   inputs are structurally well-formed, and can even satisfy the checksum for certain repeated
   digits, but the Receita Federal never issues them — they're reliably placeholder or
   data-entry artifacts.
4. **Checksum** — both verification digits must match the Módulo 11 algorithm applied to the
   preceding characters.

Each rule maps to exactly one `CnpjError` variant — see [Error Handling](./error-handling.md) for
the full list and how to match on it.

## What it doesn't do

`Cnpj::parse` validates *shape and checksum*, not *existence*. It cannot tell you whether a
particular CNPJ has actually been issued, is active, or belongs to the company you think it does —
that requires a lookup against the Receita Federal's own systems (or a data provider), which is
out of scope for this crate.