# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
