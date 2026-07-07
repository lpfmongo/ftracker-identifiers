# Formatting & Display

A CFI has a single canonical rendering, the 6-character string. Rendering never allocates on the
heap.

## Canonical form

`Cfi::as_str()` returns the 6-character string, e.g. `"ESVUFR"`. Internally this is a zero-cost
borrow of the identifier's own byte buffer. It never allocates and never panics, because the bytes
are guaranteed to be valid ASCII by construction.

```rust,ignore
use ftracker_identifiers::Cfi;

let cfi = Cfi::parse("esvufr").unwrap();
assert_eq!(cfi.as_str(), "ESVUFR"); // normalized to uppercase
```

If you need raw bytes instead of a `&str`, `Cfi::as_bytes()` returns `&[u8; 6]` directly.

## `Display` and `Debug`

`Cfi` implements `Display` by writing its canonical string, so `cfi.to_string()` and `{}` formatting
both produce the 6-character form:

```rust,ignore
use ftracker_identifiers::Cfi;

let cfi = Cfi::parse("ESVUFR").unwrap();
assert_eq!(cfi.to_string(), "ESVUFR");
```

`Debug` wraps the same string in a readable tuple-struct style, which is what you'll see in
`assert_eq!` failure messages, logs, and `{:?}` output:

```text
Cfi("ESVUFR")
```

This makes a mismatched or unexpected `Cfi` easy to spot at a glance in test output or logs, without
needing to manually reformat raw bytes.
