# AGENTS.md

`ftracker-identifiers`: a `no_std`-first Rust library of validated identifier types —
`Cnpj`, `Isin`, `Cfi`, `CountryCode`. Single crate (not a real workspace, despite `--workspace` flags).

## Commands (`just` is the task runner; `just --list`)

- `just lint` — clippy, `--all-targets --all-features -D warnings` (warnings fail the build).
- `just test` — `cargo test --workspace --verbose`.
- `just format-check` / `just format`.
- `just cfi-check` — verifies the generated CFI table is in sync (see below).
- `just policy-check` — `licenses-check` + `deny-check` (needs `cargo-about`, `cargo-deny`).

Gotchas:

- **`just test`, `just build`, `just build-release` run `cargo clean` first** — every run is a full
  rebuild. For fast iteration use raw `cargo test` / `cargo clippy --all-features` directly.
- **`just test` does NOT pass `--all-features`**, so `serde`/`schemars`/`arbitrary`/`proptest`
  modules and their tests are skipped. Run `cargo test --all-features` to exercise them (clippy and
  CI already do).
- Run one test: `cargo test cnpj::` (module) or `cargo test <name>`.
- `just build` also enforces coverage: `cargo tarpaulin --fail-under 65`.
- CI (`.github/workflows/ci.yaml`, runs on PRs/pushes to `main`) checks: format, clippy
  (`--all-features`), `test --all-features`, `no_std` builds (`--no-default-features` and with all
  optional features), MSRV build (1.93.0, `--locked`), CFI-table drift, and `cargo doc`. It does NOT
  run coverage/tarpaulin or `policy-check`; those live in `audit.yaml` / `licenses.yaml`.

Toolchain: stable, edition 2024, `rust-version = 1.93.0`. Extra tools are not auto-installed:
`cargo-tarpaulin` (coverage), `cargo-about` + `cargo-deny` (policy), `mdbook` (`just docs-build`),
`cargo-fuzz` + nightly (fuzzing).

## Architecture

- **`no_std`**: `lib.rs` is `#![cfg_attr(not(feature = "std"), no_std)]` + `extern crate alloc`.
  Use `alloc::` (e.g. `alloc::string::String`), never `std::`, in library code.
- **One module per identifier**: `src/<id>.rs` (public type + module docs) plus `src/<id>/` with
  fixed roles: `error.rs`, `fmt.rs`, `parser.rs`, `validation.rs`, `serde.rs`, `schema.rs`,
  `arbitrary.rs`, `proptest.rs`, `tests.rs`. Copy `src/cnpj/` as the reference.
- **parser vs validation split**: `parser.rs` handles formatting only (strip punctuation, fold
  case); `validation.rs` handles rules only (character class, checksum). Put new logic on the right side.
- Feature-gated modules use **shared** feature names (`serde`, `schemars`→implies `serde`,
  `arbitrary`, `proptest`), not per-identifier features. `proptest` strategies are `pub` and gated
  `#[cfg(any(test, feature = "proptest"))]`.
- Re-export new public types from `lib.rs` (`pub use isin::{Isin, IsinError};`) — easy to forget.
- Full conventions + doc checklist: `docs/src/contributing/adding-a-new-identifier.md`.

## CFI generated table

`src/cfi/table.rs` is `@generated` — **do not edit by hand**. Edit the seed `data/cfi.json`, then
`just cfi-generate` (runs the `generate_cfi_table` bin under the `codegen` feature, then `cargo fmt`).
`just cfi-check` (and CI) guard against drift. `data/` and the generator bin are `exclude`d from the
published crate.

## Fuzzing

`fuzz/` is a separate cargo-fuzz crate (needs nightly + `cargo install cargo-fuzz`).

- `just fuzz <target> [secs]` runs a target; `just fuzz-build` builds all.
- `just fuzz-coverage <target>` reports corpus coverage.
- Targets: `country_code`, `cfi`, `isin`, `cnpj`. Note `country_code` maps to the `country` module.
- **Watch `git status`**: `fuzz/.gitignore` ignores `fuzz/target`, but the root `.gitignore` does
  NOT cover `fuzz/coverage` or `fuzz/artifacts`. Do not stage/commit fuzz build artifacts.

## Git / PRs

Fork model: `origin` = your fork, `upstream` = `lnivva/ftracker-identifiers`. PRs target `main`.
Branches: `feat/<name>`. Conventional Commits with scopes (`feat(cfi):`, `docs:`,
`refactor(proptest):`). CodeRabbit auto-reviews PRs labeled `review-ready`.
