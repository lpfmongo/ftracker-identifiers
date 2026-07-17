# Formatting & Display

An LEI has a single canonical rendering, the 20-character string. Rendering never allocates on the heap.

## Canonical form

`Lei::as_str()` returns the 20-character string, e.g. `"5493000IBP32UQZ0KL24"`. Internally this is a zero-cost borrow of
the identifier's own byte buffer. It never allocates and never panics, because the bytes are guaranteed to be valid
ASCII by construction.

```rust,ignore
use ftracker_identifiers::Lei;

let lei = Lei::parse("5493000ibp32uqz0kl24").unwrap();
assert_eq!(lei.as_str(), "5493000IBP32UQZ0KL24"); // normalized to uppercase
```

If you need raw bytes instead of a `&str`, `Lei::as_bytes()` returns `&[u8; 20]` directly.

## `Display` and `Debug`

`Lei` implements `Display` by writing its canonical string, so `lei.to_string()` and `{}` formatting both produce the
20-character form:

```rust,ignore
use ftracker_identifiers::Lei;

let lei = Lei::parse("5493000IBP32UQZ0KL24").unwrap();
assert_eq!(lei.to_string(), "5493000IBP32UQZ0KL24");
```

`Debug` wraps the same string in a readable tuple-struct style, which is what you'll see in
`assert_eq!` failure messages, logs, and `{:?}` output:

```text
Lei("5493000IBP32UQZ0KL24")
```

This makes a mismatched or unexpected `Lei` easy to spot at a glance in test output or logs, without needing to manually
reformat raw bytes.
