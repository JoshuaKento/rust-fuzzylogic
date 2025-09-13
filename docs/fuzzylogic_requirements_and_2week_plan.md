# Fuzzy Logic Library — Requirements and 2‑Week Implementation Plan

## Vision & Scope

- Goal: Provide a small, fast, and safe Rust library for building and evaluating Mamdani‑style fuzzy systems with clear APIs, strong validation, and excellent docs/tests.
- Non‑goals (initial 2 weeks): GUI tooling, full DSL/parser, advanced optimizers, plugin runtime loading, complete Sugeno/Takagi–Sugeno models. Keep the core tight and focused.

## Target Users & Primary Use Cases

- Embedded/control engineers modeling fuzzy controllers for single/multi‑input outputs.
- Data/ML engineers needing interpretable rule systems on numeric inputs.
- Educators/students learning fuzzy concepts with reliable examples.

Core use cases:
- Define variables (e.g., temperature) with terms (e.g., cold/warm/hot) via membership functions.
- Define rules with AND/OR/NOT between terms, infer fuzzy outputs, defuzzify to crisp values.
- Evaluate single inputs or batches efficiently; optionally parallelize.

## Design Principles

- Safety first: validate parameters; avoid `unwrap`/`expect` in non‑test code.
- Minimal dependencies; feature‑gate optional ones (e.g., `serde`, `rayon`).
- Predictable performance: avoid unnecessary allocations; prefer stack values, slices, and iterators.
- Clear ergonomics: obvious defaults; escape hatches for advanced tuning.
- Testable & documented: boundary tests, examples, and architecture notes in `docs/`.

## Core Concepts & Types (Rust API Sketch)

- `type Float = f64`
  - Feature `f32` switches to `f32`. All public APIs use `Float` for numeric values.

- Membership functions
  - Trait: `trait MembershipFn { fn eval(&self, x: Float) -> Float; }`
  - Shapes (initial): `Triangular { left, center, right }`, `Trapezoidal { left, left_top, right_top, right }`, `Gaussian { mean, sigma }`.
  - Validation: constructors return `Result<Self, Error>` ensuring monotonic ordering and positive scale (e.g., `sigma > 0`).
  - Optional shapes (backlog): `Sigmoid`, `PiecewiseLinear`.

- Variables and terms
  - `Variable { name, domain: (min, max), unit: Option<String> }` with domain validation (`min < max`).
  - `Term { name, func: Box<dyn MembershipFn + Send + Sync> }` for dynamic shape use; also provide typed helpers for zero‑cost static use.

- Fuzzy logic operators
  - T‑norms: `min` (default), `product` (feature `alt_tnorms`).
  - S‑norms: `max` (default), `probabilistic_sum` (feature `alt_snorms`).
  - Complement: `1.0 - µ` with clamping to `[0,1]` to mitigate FP drift.

- Rule representation
  - Antecedent expression AST with `And`, `Or`, `Not`, `Is(Variable, TermName)`.
  - Implication (Mamdani): `min` by default (feature: `product_implication`).
  - Aggregation across rules: `max` (default) over resulting fuzzy sets of an output term.

- Engine & evaluation
  - `System` holds input variables/terms, output variables/terms, and rules.
  - `Evaluator` takes crisp inputs (`&HashMap<String, Float>` or structured inputs) and returns crisp outputs.
  - Defuzzification strategies: `Centroid` (default), `Bisector`, `MeanOfMaxima`, `SmallestOfMaxima`.
  - Sampling for defuzzification: configurable resolution (default 101 points across domain); pluggable sampler API.

- Batch & parallel
  - Batch API evaluates slices/iterators; optional parallel via `rayon` behind feature `parallel`.

- Builders & ergonomics
  - Fluent builders to define variables, terms, and rules with compile‑time guidance and runtime validation.
  - Builders return `Result<System, Error>` with aggregated validation errors when possible.

- Serialization (optional)
  - Feature `serde` enables `Serialize`/`Deserialize` for model types and shapes. No I/O in core; users handle files/streams.

## Errors, Validation, and Edge Handling

- Custom error enum `Error` covers: invalid parameters (ordering, ranges), domain violations, missing variables/terms in rules, empty rule bases, defuzzification failures (e.g., zero area), and feature‑related operations when feature is disabled.
- FP handling: clamp outputs to `[0,1]` when numerically close; propagate NaN inputs as NaN results (documented). All constructors validate invariants; evaluation functions assume validated state.

## Performance & Concurrency

- No heap churn in hot paths; pre‑allocate buffers for sampling/aggregation; reuse workspaces in batch paths.
- All public types `Send + Sync` where sensible; `System` immutable after build; share via `Arc` for concurrent evals.
- Zero‑cost static dispatch for shapes is available via generic helpers; dynamic dispatch path exists for mixed collections.

## Documentation & Testing

- In‑file unit tests beside code (`#[cfg(test)]`) covering boundaries (left/center/right, plateaus, sigma→0) and rule evaluation edge cases.
- Examples: end‑to‑end thermostat example in crate docs and `src/main.rs`.
- `docs/` contains architecture notes and guides mapping concepts to code.

## Public API Surface (initial)

- `Triangular::new(left, center, right) -> Result<Self, Error>`; `.eval(x)`
- `Trapezoidal::new(l, lt, rt, r) -> Result<Self, Error>`; `.eval(x)`
- `Gaussian::new(mean, sigma) -> Result<Self, Error>`; `.eval(x)`
- `Variable::new(name, min, max)`
- `SystemBuilder` and `RuleBuilder` fluent APIs
- `System::evaluate(input: &impl Inputs) -> Result<Outputs, Error>`
- `Defuzzifier` enum with strategy selection
- Batch: `Evaluator::evaluate_all<'a, I>(&self, inputs: I) -> Result<Vec<Outputs>, Error>`

## Acceptance Criteria (end of week 2)

- API compiles on Rust 2024 edition; `cargo clippy --all-targets -- -D warnings` passes.
- Core shapes (triangular, trapezoidal, gaussian) with full validation and tests.
- Mamdani inference engine with AND/OR/NOT, aggregation, and at least `Centroid` + `MeanOfMaxima` defuzzifiers.
- Fluent builder builds a non‑trivial example system end‑to‑end in `src/main.rs`.
- Batch evaluation works; optional parallel feature behind `parallel`.
- Optional `serde` feature serializes/deserializes a system; round‑trip tests.
- Documentation covers concepts, API usage, and limitations; examples run in doctests.

---

# 2‑Week Implementation Plan (10 working days)

Day 1 — Architecture skeleton
- Set up modules for shapes, variables/terms, rules, engine, defuzz, builder.
- Define `Float` type alias and feature flags (`f32`, `serde`, `parallel`).
- Draft `Error` enum and result type; add scaffolding tests.

Day 2 — Membership shapes
- Implement `Triangular`, `Trapezoidal`, `Gaussian` with validation and unit tests (boundaries, monotonicity, sigma>0).
- Add `MembershipFn` trait and basic blanket helpers.

Day 3 — Variables and terms
- Implement `Variable`, `Term`, and lookups; enforce domain validation.
- Add tests for term evaluation across domain.

Day 4 — Operators and AST
- Implement T‑norm/S‑norm/complement (defaults + feature‑gated alternates).
- Define antecedent AST and evaluator over crisp inputs; tests for AND/OR/NOT behavior.

Day 5 — Mamdani core and aggregation
- Implement rule implication and output set aggregation per output term.
- Introduce sampling strategy abstraction and default sampler; tests for aggregations.

Day 6 — Defuzzification strategies
- Implement `Centroid`, `Bisector`, `MeanOfMaxima`, `SmallestOfMaxima`; tests including zero‑area behavior.

Day 7 — System/Evaluator APIs
- Implement `System`, `Evaluator`, and error paths for missing inputs/terms.
- Add an end‑to‑end thermostat example in `src/main.rs` with tests.

Day 8 — Builders
- Implement `SystemBuilder` and `RuleBuilder` with validation; ensure ergonomic method chaining.
- Add compile‑fail docs where appropriate; extend example to use builder API.

Day 9 — Batch + parallel feature
- Implement batch evaluation API; optional `rayon` parallel path behind `parallel`.
- Micro‑bench (criterion optional if permitted) and document performance notes.

Day 10 — Serialization + polish
- Add optional `serde` derives behind feature; round‑trip tests.
- Documentation pass: crate‑level docs, `docs/` architecture note linking to this file.
- Final lint/format/test sweep; prepare for initial tag.

## Stretch & Backlog (post week 2)

- Additional shapes (Sigmoid, GeneralizedBell), piecewise linear builder.
- Alternative inference methods (product implication, Sugeno models).
- DSL/parser for terms/rules; JSON/YAML loaders with versioning.
- Advanced sampling (adaptive, error‑bounded centroid).
- Benchmarks suite and criterion‑based CI gating.

## Repository Alignment

- Edition: 2024 per `Cargo.toml`.
- Tests adjacent to code with `#[cfg(test)]` modules.
- Keep visibility minimal; only `pub` for cross‑module API.
- Avoid `unwrap/expect` in non‑test code; return `Result` with `Error`.

---

This plan prioritizes a robust, validated core with crisp APIs and tests over breadth. It leaves space for performance refinements and optional features without over‑committing dependencies or complexity.

