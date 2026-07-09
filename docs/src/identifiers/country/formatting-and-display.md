# Formatting & Display

A country code has a single canonical rendering, the two character string. Rendering never allocates on the heap.

## Canonical form

`CountryCode::as_str()` returns the two character string, for example `"US"`. Internally this is a zero cost borrow of
the value's own byte buffer. It never allocates and never panics, because the bytes are guaranteed to be valid ASCII by
construction.

```rust,ignore
use ftracker_identifiers::CountryCode;

let code = CountryCode::parse("us").unwrap();
assert_eq!(code.as_str(), "US"); // normalized to uppercase
```

If you need raw bytes instead of a `&str`, `CountryCode::as_bytes()` returns `&[u8; 2]` directly.

## `Display` and `Debug`

`CountryCode` implements `Display` by writing its canonical string, so `code.to_string()` and `{}` formatting both
produce the two character form:

```rust,ignore
use ftracker_identifiers::CountryCode;

let code = CountryCode::parse("US").unwrap();
assert_eq!(code.to_string(), "US");
```

`Debug` wraps the same string in a readable tuple struct style, which is what you will see in `assert_eq!` failure
messages, logs, and `{:?}` output:

```text
CountryCode("US")
```

This makes a mismatched or unexpected value easy to spot at a glance in test output or logs, without needing to manually
reformat raw bytes.
