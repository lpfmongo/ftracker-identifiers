# Formatting & Display

`Cnpj` can render itself two ways, and neither one allocates on the heap.

## Compact form

`Cnpj::as_str()` returns the 14-character compact form with no punctuation, e.g.
`"00000000000191"`. Internally this is a zero-cost borrow of the identifier's own byte buffer — it
never allocates and never panics, because the bytes are guaranteed to be valid ASCII by
construction.

```rust,ignore
use ftracker_identifiers::Cnpj;

let cnpj = Cnpj::parse("00.000.000/0001-91").unwrap();
assert_eq!(cnpj.as_str(), "00000000000191");
```

If you need raw bytes instead of a `&str`, `Cnpj::as_bytes()` returns `&[u8; 14]` directly.

## Punctuated form

`Cnpj::formatted()` returns a `FormattedCnpj` — a small, stack-allocated, `Copy` value that renders
the conventional `AA.AAA.AAA/AAAA-DD` layout. It implements `Display`, `Deref<Target = str>`, and
`AsRef<str>`, so you can pass it almost anywhere a `&str` is expected without an explicit
conversion:

```rust,ignore
use ftracker_identifiers::Cnpj;

let cnpj = Cnpj::parse("00000000000191").unwrap();
let formatted = cnpj.formatted();

assert_eq!(formatted.as_str(), "00.000.000/0001-91");
assert_eq!(&*formatted, "00.000.000/0001-91"); // via Deref
println!("{formatted}");                        // via Display
```

## `Display` and `Debug` on `Cnpj` itself

`Cnpj` implements `Display` by delegating to `formatted()`, so `cnpj.to_string()` always produces
the punctuated form:

```rust,ignore
use ftracker_identifiers::Cnpj;

let cnpj = Cnpj::parse("00000000000191").unwrap();
assert_eq!(cnpj.to_string(), "00.000.000/0001-91");
```

`Debug` wraps the same punctuated form in a readable tuple-struct style, which is what you'll see
in `assert_eq!` failure messages, logs, and `{:?}` output:

```text
Cnpj("00.000.000/0001-91")
```

This makes a mismatched or unexpected `Cnpj` easy to spot at a glance in test output or logs,
without needing to manually reformat raw bytes.