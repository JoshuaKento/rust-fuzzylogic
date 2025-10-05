# rust-fuzzylogic

A Rust crate that provides building blocks for authoring fuzzy inference systems.
The project aims to make it easy to describe linguistic variables, membership functions, and rule
bases while keeping room for experimentation with multiple aggregation operators and inference
strategies.

## Project status

The repository currently focuses on laying down the module structure of the crate. Several source
files exist as stubs and will be fleshed out in future iterations. Even though the public API is not
ready for consumption yet, the layout already highlights the intended responsibilities of each
module:

- `membership`: utilities for defining membership functions (triangular, trapezoidal, gaussian, …).
- `variable`: strongly-typed linguistic variables composed of terms and membership functions.
- `term`: basic linguistic terms that bind membership functions to human-readable labels.
- `rulespace`: abstractions for authoring rule bases and connecting antecedents to consequents.
- `antecedent`: helpers to compose fuzzy predicates out of linguistic terms.
- `aggregate`: algorithms to combine the contribution of multiple rules.
- `defuzz`: defuzzification routines that convert fuzzy outputs into crisp values.
- `mamdani`: reference implementation of a Mamdani-style inference engine.
- `ops`: configurable T-norm/T-conorm operators (minimum, product, Łukasiewicz, …).
- `sampler`: sampling utilities for visualisation or numerical integration tasks.
- `builder`: high-level ergonomics for constructing complete systems.
- `error`: error types returned by the crate.
- `prelude`: convenient re-exports for end users.

## Getting started

Because the implementation is still under construction the crate is not published on crates.io.
Once the API stabilises you will be able to add it as a Git dependency in your `Cargo.toml`:

```toml
[dependencies]
rust-fuzzylogic = { git = "https://github.com/your-org/rust-fuzzylogic" }
```

After the crate reaches its first release the dependency line will become:

```toml
[dependencies]
rust-fuzzylogic = "0.1"
```

## Development

If you would like to contribute, clone the repository and open it with your favourite editor:

```bash
git clone https://github.com/your-org/rust-fuzzylogic.git
cd rust-fuzzylogic
```

The crate targets Rust 1.76+ (edition 2024). Standard Cargo commands apply:

- `cargo check` — make sure the crate compiles.
- `cargo fmt` — format the codebase.
- `cargo test` — run the test suite (currently empty).

### Feature flags

The crate is organised around a set of feature flags to enable or disable components at compile
time:

- `f32` / `f64` — choose the floating-point precision used throughout the inference engine.
- `serde` — derive serialisation support for configuration data structures.
- `parallel` — enable rayon-powered parallel execution for suitable workloads.
- `ops-minmax`, `ops-product`, `ops-lukasiewicz` — opt into specific operator families.
- `ops-dyn` — use dynamic dispatch for selecting operators at runtime.
- `inference-mamdani` — compile the Mamdani inference engine implementation.

## Roadmap

- [ ] Make some example scripts under examles/, benches/.
- [ ] Implement comprehensive tests.
- [ ] Document the builder APIs and provide a quick start guide.
- [ ] Publish the crate and examples for real-world testing.

---

This README will evolve alongside the code. Contributions, bug reports, and ideas are warmly
welcome!
