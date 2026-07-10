# Structure & Formats

A country code always has **two characters**, both uppercase letters.

`CountryCode` exposes the value through two accessors:

* `CountryCode::as_str()`: the two character code, as a `&str`.
* `CountryCode::as_bytes()`: the two raw bytes, as a `&[u8; 2]`.

A country code has **no punctuated form**. Its canonical rendering is simply the two character string, so there is no
separate "compact vs. formatted" distinction. See [Formatting & Display](./formatting-and-display.md).

## What is modeled

This crate models exactly one thing: the officially assigned ISO 3166-1 alpha-2 code.

* Both letters together must name a code that the standard officially assigns. The set of assigned codes is embedded as
  a compile time bitmap.
* Reserved codes such as `EU` and `UK` are not assigned, so they are rejected.
* User assigned ranges (for example `AA`, `QM` to `QZ`, `XA` to `XZ`, and `ZZ`) are not assigned, so they are rejected.
* Codes that were once assigned and later withdrawn are rejected.

```rust,ignore
use ftracker_identifiers::CountryCode;

let usa = CountryCode::parse("US").unwrap();
assert_eq!(usa.as_str(), "US");

let brazil = CountryCode::parse("BR").unwrap();
assert_eq!(brazil.as_bytes(), b"BR");
```

## Codes, not names

This crate embeds only the set of assigned **codes**. It does not embed country names, the alpha-3 code, or the numeric
code.

As a result, `CountryCode` can confirm that `US` is an assigned code and reject `ZZ` as unassigned, but it will not
resolve `US` to "United States of America". If you need the name or the other code forms, consult the ISO 3166-1
standard itself.

## A note on ordering

`CountryCode` derives `Ord` directly over its underlying ASCII bytes, which matches `str` ordering on
`CountryCode::as_str()`. This is **lexicographic string order**. Sorting arranges codes alphabetically and carries no
geographic meaning beyond that.
