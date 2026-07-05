# Adding a New Identifier

This crate is organized so that every identifier type — CNPJ today, ISIN and CFI planned — follows
the same shape, both in `src/` and in this book. Sticking to the pattern keeps the crate
predictable to use and cheap to review.

## Source layout

Each identifier gets its own module directory, named after the identifier in lowercase (mirror `src/cnpj/`):

```text
src/
  <identifier>.rs             # public type, module docs, inherent methods
  <identifier>/
    error.rs                  # the <Identifier>Error enum + Display
    fmt.rs                    # Display/Debug + any zero-allocation formatted-string helper
    parser.rs                 # string -> normalized candidate (formatting/case only)
    validation.rs              # normalized candidate -> Result<(), Error> (structural + checksum rules)
    serde.rs                   # #[cfg(feature = "serde")]
    schema.rs                  # #[cfg(feature = "schemars")]
    arbitrary.rs                # #[cfg(feature = "arbitrary")]
    proptest.rs                 # #[cfg(any(test, feature = "proptest"))], pub
    tests.rs                    # #[cfg(test)]
```

The **parser vs. validation split** matters: `parser` only knows about *formatting* (stripping
punctuation, folding case) and has no opinion on which characters are valid where; `validation`
only knows about *rules* (character class, checksum, degenerate input) and has no opinion on how
the original string was formatted. This keeps each half testable independently and makes it
obvious where a new rule belongs.

## Feature flags

Follow the same `Cargo.toml` convention as CNPJ — every integration is optional and additive:

```toml
[features]
serde = ["dep:serde"]
schemars = ["dep:schemars", "serde"]
arbitrary = ["dep:arbitrary"]
proptest = ["dep:proptest"]
```

A new identifier should gate its `serde`/`schemars`/`arbitrary`/`proptest` modules behind the
*same* feature names as CNPJ (not per-identifier features like `cnpj-serde`), so enabling `serde`
once turns it on for every identifier type in the crate.

## lib.rs

Re-export the new public type (and its error type, and any formatted-string helper type) at the
crate root, the same way CNPJ does:

```rust,ignore
mod isin;
pub use isin::{Isin, IsinError, FormattedIsin};
```

Forgetting this step means the error type technically exists but can never be named by downstream
code — the same gap this documentation pass caught and fixed for `Cnpj`.

## Documentation checklist

For the module itself (`src/<identifier>.rs`):

- [ ] Module-level doc comment covering: what the identifier represents, its segment structure (a
  table works well), format history/variants if any, the full list of validation rules mapped
  to error variants, relevant design notes (allocation, `Copy`, ordering/hashing semantics),
  feature flags, and at least one runnable example.
- [ ] Every public method has a doc comment with a runnable `# Examples` section.
- [ ] Every public trait impl (`FromStr`, `TryFrom`, `AsRef`, ...) has a one-line doc comment on
  the implementing method.
- [ ] `# Errors` sections on every fallible public function.

For this book (`docs/src/identifiers/<identifier>/`):

- [ ] `README.md` — landing page: what the identifier is, a minimal example, a table of contents
  for the rest of the chapter.
- [ ] `format.md` — structure and any format history.
- [ ] `parsing-and-validation.md` — constructors and validation rules.
- [ ] `formatting-and-display.md` — rendering, `Display`/`Debug`.
- [ ] `error-handling.md` — error variant table and a matching example.
- [ ] `feature-flags.md` — one section per optional integration.
- [ ] `examples.md` — end-to-end usage beyond the quick start.

Code blocks in this book use ` ```rust,ignore ` rather than ` ```rust `, since `mdbook test` isn't
currently wired up to link against the compiled crate. The examples themselves should still mirror
real, compiler-verified doctests from the source — copy them from `src/<identifier>.rs` rather than
writing new, untested snippets from scratch.

## Wiring it into `SUMMARY.md`

Replace the draft placeholder for your identifier with a real link, and add the same sub-chapters
CNPJ has:

```markdown
- [ISIN](./identifiers/isin/README.md)
    - [Structure & Formats](./identifiers/isin/format.md)
    - [Parsing & Validation](./identifiers/isin/parsing-and-validation.md)
    - [Formatting & Display](./identifiers/isin/formatting-and-display.md)
    - [Error Handling](./identifiers/isin/error-handling.md)
    - [Feature Flags](./identifiers/isin/feature-flags.md)
    - [Examples](./identifiers/isin/examples.md)
```

A draft entry (`- [ISIN]()`, with no link target) renders as a disabled item in the sidebar and
requires no file to exist yet — that's how ISIN and CFI are currently listed. Only replace it once
the chapter files actually exist; a link to a missing file breaks the `mdbook build`.