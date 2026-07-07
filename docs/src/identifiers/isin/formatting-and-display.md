# Formatting & Display

An ISIN has a single canonical rendering, the 12-character string. Rendering never allocates on the heap.

## Canonical form

`Isin::as_str()` returns the 12-character string, e.g. `"US0378331005"`. Internally this is a zero-cost borrow of the
identifier's own byte buffer. It never allocates and never panics, because the bytes are guaranteed to be valid ASCII
by construction.

```rust,ignore
use ftracker_identifiers::Isin;

let isin = Isin::parse("us0378331005").unwrap();
assert_eq!(isin.as_str(), "US0378331005"); // normalized to uppercase
```

If you need raw bytes instead of a `&str`, `Isin::as_bytes()` returns `&[u8; 12]` directly.

## `Display` and `Debug`

`Isin` implements `Display` by writing its canonical string, so `isin.to_string()` and `{}` formatting both produce the
12-character form:

```rust,ignore
use ftracker_identifiers::Isin;

let isin = Isin::parse("US0378331005").unwrap();
assert_eq!(isin.to_string(), "US0378331005");
```

`Debug` wraps the same string in a readable tuple-struct style, which is what you'll see in `assert_eq!` failure
messages, logs, and `{:?}` output:

```text
Isin("US0378331005")
```

This makes a mismatched or unexpected `Isin` easy to spot at a glance in test output or logs, without needing to
manually reformat raw bytes.
