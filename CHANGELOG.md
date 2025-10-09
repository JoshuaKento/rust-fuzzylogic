# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog, and this project adheres to
Semantic Versioning. Prior to 1.0.0, MINOR (0.x.0) may include breaking
changes, while PATCH (0.1.x) is reserved for backwards-compatible fixes
and small additions.

## [UNRELEASED]

### Added
- Added Basic Documention by way of "cargo doc".

### Changed
- Changed cargo.toml so that the crate upon publication only includes the necessary components.(e.g. excludes tests, examples... etc)

### Fixed
- Fixed the logic of defuzz.rs, rulespace.rs, aggregate.rs where the Rules Vector...etc would be erased after agrragate and defuzzify due to using mem::take, to borrowing. 

- fixed mamdani_test.rs according to the API change of above fix.

### Deprecated
-

### Removed
-

### Security
-

## [0.1.0] - 2025-10-XX

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
[UNRELEASED]: https://github.com/joushuakent/rust-fuzzylogic/compare/UNRELEASED
[0.1.0]: https://github.com/joushuakent/rust-fuzzylogic/releases/tag/v0.1.0

