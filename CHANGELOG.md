# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.2](https://github.com/lnivva/ftracker-identifiers/compare/v0.0.1...v0.0.2) - 2026-07-10

### Added

- add publish and release lifecycles ([#10](https://github.com/lnivva/ftracker-identifiers/pull/10))
- *(country)* add iso country code identifier and fuzzy testing ([#9](https://github.com/lnivva/ftracker-identifiers/pull/9))
- *(cfi)* add cfi module with parsing, validation, and comprehensive support ([#6](https://github.com/lnivva/ftracker-identifiers/pull/6))
- *(isin)* add ISIN module with parsing, validation, and feature flag support ([#4](https://github.com/lnivva/ftracker-identifiers/pull/4))
- *(cnpj)* make CNPJ module public and expose core types
- *(cnpj)* initial feature cnpj implementation

### Fixed

- exclude workspace crate from third-party license report ([#14](https://github.com/lnivva/ftracker-identifiers/pull/14))

### Other

- update docs badge
- make release fuzz gate always run so it can be a required check ([#13](https://github.com/lnivva/ftracker-identifiers/pull/13))
- add bounded fuzz gate with report for release PRs ([#12](https://github.com/lnivva/ftracker-identifiers/pull/12))
- update release-plz script
- update scripts
- *(proptest)* reorder imports and add `alloc::string` for consistency across modules ([#7](https://github.com/lnivva/ftracker-identifiers/pull/7))
- *(dependencies)* update Cargo.lock with new package versions and dependencies cleanup
- add workflows for dependency audit and license compliance checks
- add bug report issue template in GitHub
- *(dependabot)* add configuration for Cargo and GitHub Actions dependency updates
- add contributing guide and expand CNPJ documentation
- *(cnpj)* reorder imports for consistency across modules
- *(docs)* initialize mdBook with basic structure
- update gitignore with rust and mdbook templates
- Initial commit

## [0.0.1] - 2026-07-10

### Added

- Initial release.
- `Cnpj`: Brazil's Cadastro Nacional da Pessoa Jurídica, validated with the Módulo 11
  checksum. Supports the punctuated `AA.AAA.AAA/AAAA-DD` form, the compact 14-character
  form, the legacy numeric layout, and the 2026 alphanumeric layout.
- `Isin`: ISO 6166 International Securities Identification Number, validated with the
  ISO 6166 Luhn check digit.
- `Cfi`: ISO 10962 Classification of Financial Instruments code, validated against an
  embedded copy of the standard's code table.
- `CountryCode`: ISO 3166-1 alpha-2 country code, validated against the officially
  assigned set.
- `no_std`-first design; optional `serde`, `schemars`, `arbitrary`, and `proptest`
  integrations behind feature flags.

[0.0.1]: https://github.com/lnivva/ftracker-identifiers/releases/tag/v0.0.1
