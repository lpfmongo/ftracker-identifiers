# Fuzzing

Every identifier in this crate is a small value object built by parsing untrusted text, and each one
exposes its canonical string through an unchecked byte to string conversion (`from_utf8_unchecked`).
That makes the parsers a natural place for coverage guided fuzzing: it explores far more shapes of
malformed input than hand written tests, and it proves the unchecked conversion can never observe
invalid UTF-8.

The fuzz targets live in the `fuzz/` directory as a standalone crate. They reuse the `Arbitrary`
implementations (the `arbitrary` feature) to generate valid values, and the `serde` feature to check
serialization round trips.

## Prerequisites

Fuzzing uses [`cargo-fuzz`](https://github.com/rust-fuzz/cargo-fuzz), which builds with libFuzzer and
requires a nightly toolchain. The library itself stays on stable; only the fuzzer runtime needs
nightly.

```sh
cargo install cargo-fuzz
rustup toolchain install nightly
```

## Targets

There is one target per identifier:

* `country_code`
* `cfi`
* `isin`
* `cnpj`

### How each target drives the code

To reach the deep logic instead of bouncing off the length and character gates, every target feeds
the same input through four lenses:

1. As arbitrary UTF-8 text handed to `parse`.
2. As raw bytes handed to `from_bytes`.
3. As a structure aware candidate: a string built to the exact canonical length and character class
   (for example two letters plus nine alphanumerics plus a digit for an ISIN), so the checksum or
   taxonomy logic runs on near valid input on almost every execution.
4. As an `Arbitrary` generated value, which exercises the acceptance path directly.

A committed dictionary per target (`fuzz/dict/<target>.dict`) supplies tokens (category letters,
sample codes, boundary punctuation) that steer the mutator toward interesting inputs.

### Invariants every accepted value must satisfy

* Soundness: `as_str()` is valid UTF-8 equal to `as_bytes()` (this guards the `from_utf8_unchecked`).
* Shape: the canonical form has the fixed length, and every byte is in the allowed class for its
  position.
* Constructor agreement: `parse`, `from_bytes`, `FromStr`, and `TryFrom` all return the same value,
  and parsing the canonical form is idempotent.
* Accessor consistency: segment accessors match the canonical string, and for an ISIN the stored
  check digit equals the recomputed one.
* serde: the value round trips through its JSON string.
* Rendering: `Display` and `Debug` never panic. Formatting a rejected value's error never panics.

## Running

The `just` recipes wrap the nightly invocation and feed in the committed seed corpus:

```sh
just fuzz country_code      # run for 60 seconds (default)
just fuzz cfi 120           # run a target for 120 seconds
just fuzz-build             # just compile every target
```

Equivalently, without `just`:

```sh
cargo +nightly fuzz run country_code fuzz/corpus/country_code fuzz/seeds/country_code
```

A crash writes a reproducer under `fuzz/artifacts/<target>/`. Replay it with:

```sh
cargo +nightly fuzz run <target> fuzz/artifacts/<target>/<crash-file>
```

## Corpus and seeds

* `fuzz/seeds/<target>/` holds known inputs (valid codes and edge shapes such as wrong length, bad
  character, and reserved codes), committed to git, to give the fuzzer a head start. Treat this
  directory as read only.
* `fuzz/dict/<target>.dict` is the committed libFuzzer dictionary of useful tokens.
* `fuzz/corpus/<target>/` is the growing working corpus that libFuzzer writes to. It is ignored by
  git, along with `fuzz/artifacts/` and `fuzz/target/`.

When a target uncovers an interesting new input, consider minimizing it and adding it to the seeds
directory so the case is preserved.

## Measuring coverage

To see how much of the parsing and validation code the corpus actually exercises:

```sh
just fuzz-coverage cnpj
```

That writes `fuzz/coverage/<target>/coverage.profdata`. Render a per file report with the nightly
`llvm-cov`, for example:

```sh
LLVMCOV="$(rustc +nightly --print target-libdir)/../bin/llvm-cov"
BIN="$(find target -type f -path '*coverage*release/cnpj' ! -name '*.*' | head -1)"
"$LLVMCOV" report "$BIN" -instr-profile=fuzz/coverage/cnpj/coverage.profdata
```

The parser and validation modules should sit near full line coverage. Formatting and error modules
are also exercised, though some rendering branches are covered more thoroughly by the unit tests.
