# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog, and this project adheres to
Semantic Versioning. Prior to 1.0.0, MINOR (0.x.0) may include breaking
changes, while PATCH (0.1.x) is reserved for backwards-compatible fixes
and small additions.

## [0.2.0] - 2025/11/03
### Added
- Added `Term` variants for MissingSpace struct to be used in `FuzzyError::NotFound`. (src/Error.rs)
- Added a System struct for handling Rules and Variables(terms and correspondin MFs) comprehensively. (src/System.rs)
- Added a Builder struct for above System struct for ease of initializing Rule Systems. (src/System.rs)
- Updated README.
- Runtime Validation for units `mamdani`,`antecedent`,`system` regarding rules.

### Changed
- Refactored and Added documentation to Modules [`Rulespace`, `Defuzz`, `Mamdanin`, `Aggregate`]
- Struct `Rule` and `Consequent` now derive `Clone` (src/mamdani.rs)

### Deprecated
- Module `Rulespace`'s functionality will be integrated into the `System` Module. (src/rulespace.rs)
  
### Removed
-  Removed some unnecessary folders (target/, .vscode/)

### Fixed
-  N/A

### Security 
- N/A

## [0.1.1] - 2025-10-10

### Added

- Crate-level and module documentation to improve API discoverability by way of `crate doc`.

### Changed

- Packaging: restrict published crate contents via Cargo.toml to exclude tests, examples, benches, and other development-only files.

### Fixed

- Prevent rule vectors from being cleared during aggregation/defuzzification by replacing uses of mem::take with borrowed access in defuzz.rs, rulespace.rs, and aggregate.rs.
- Update mamdani_test.rs to align with the above API changes.

## [0.1.0] - 2025-10-09

### Added
- Initial crate scaffolding and public module layout: `membership`, `variable`, `term`, `rulespace`,
  `antecedent`, `aggregate`, `defuzz`, `mamdani`, `ops`, `sampler`, `builder`, `error`, `prelude`.
- Feature flags declared in `Cargo.toml`:
  - `f32` / `f64` (default to `f64`).
  - `serde` (optional; re-exports `Serialize`, `Deserialize`).
  - `parallel` (optional; adds `rayon`).
  - `ops-minmax`, `ops-product`, `ops-lukasiewicz`.
  - `ops-dyn` (dynamic dispatch APIs).
  - `inference-mamdani` (engine placeholder).
- Example stubs under `examples/`: `temperature`, `batch`, `fuzzy_c_means`, `gradient_descent`
  (gated with `required-features = ["parallel"]` where relevant).
- Documentation: initial README and a memo guide for implementing `serde` and `rayon`
  (`memo/feature-implementation-guide.md`).

### Notes
- MSRV declared as `1.74` in `Cargo.toml`.
- Edition currently set to `2025` in `Cargo.toml`; this may change to maximize
  toolchain compatibility prior to 1.0.

### Known limitations
- `serde` derives and rayon-powered implementations are not yet wired into the
  domain types/algorithms; follow the memo guide to implement them.
- Public API is not stabilized and may change between 0.x releases.

<!-- Update links if the repository slug changes. -->
[0.2.0]: https://github.com/joushuakento/rust-fuzzylogic/releases/tag/v0.2.0
[0.1.1]: https://github.com/joushuakento/rust-fuzzylogic/releases/tag/v0.1.1
