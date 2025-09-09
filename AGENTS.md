# Repository Guidelines

## Project Structure & Module Organization
- `src/main.rs`: Binary entry point demonstrating fuzzy logic utilities.
- `src/challenges/`: Module folder with `mod.rs` and task files (`challenge1.rs` … `challenge4.rs`).
- Tests live beside code using `#[cfg(test)]` modules within each file.
- `docs/`: Learning guides and architecture notes for each challenge.
- `Cargo.toml`: Crate metadata (edition 2024) and dependencies.

## Build, Test, and Development Commands
- `cargo build`: Compile the project in debug mode.
- `cargo run`: Build and run the binary.
- `cargo test`: Run unit tests in all modules.
- `cargo fmt --all`: Format code with rustfmt.
- `cargo clippy --all-targets -- -D warnings`: Lint and fail on warnings.
- Optional: `cargo doc --no-deps --open`: Generate and view API docs.

## Coding Style & Naming Conventions
- Use rustfmt defaults (4-space indent, trailing commas, etc.).
- Functions, variables, modules: snake_case (`triangular_membership`, `temp_temp`).
- Types and traits: PascalCase (`TemperatureSet`).
- Keep visibility minimal; prefer private items unless needed (`pub` only when used across modules).
- Organize modules under `src/challenges/` and declare in `src/challenges/mod.rs`.
- Avoid `unwrap()`/`expect()` in non-test code; return results or validate inputs (e.g., triangle order).

## Testing Guidelines
- Framework: built-in `cargo test` with inline `mod tests { use super::*; }`.
- Name tests descriptively (`test_left_slope`, `test_adjustments`).
- Cover boundaries and edge cases (left/center/right points, NaN/INF handling if added).
- Run `cargo test` locally before opening a PR.

## Commit & Pull Request Guidelines
- Commits: short, imperative subject (<=72 chars). Example: `challenge3: add vector avg helper`.
- Prefer focused commits per change; include rationale in body if non-trivial.
- PRs: include summary, context/motivation, testing notes (`cargo test` output), and affected files.
- Link related issues or docs in `docs/` when applicable; add/update docs for new modules.

## Quality & Safety Tips
- Validate numeric parameters (e.g., `left < center < right`) and handle float precision carefully.
- Keep dependencies minimal; record any new crates in `Cargo.toml` with justification.
- Consider extracting reusable logic to a future `lib.rs` if APIs grow beyond examples.

