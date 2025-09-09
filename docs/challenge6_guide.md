# Challenge 6 Guide: "Error-Proof Input Parser"

## 🌟 What You’re Building (Concept-Only)
You’ll design a robust, user-friendly parser that reads definitions from strings (e.g., function-like forms) and converts them into well-validated structures. The key is precise error reporting: instead of panicking, return informative errors that point to what went wrong and where. This guide mirrors prior guides and intentionally avoids domain-specific code; examples focus on parsing techniques and error handling patterns.

Core ideas:
- Result-based flows with custom error types
- Parsing strategies (scan/consume, `FromStr`, small helpers)
- Input validation separated from parsing
- Clear, actionable error messages


## 💡 New Rust Concepts You’ll Learn

### `Result<T, E>` With Custom Errors
Create an enum for all error cases you want to communicate, implement `Display` for friendly messages, and optionally `std::error::Error`.

Unrelated example — error enum:
```rust
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    EmptyInput,
    UnknownFunc { name: String },
    ArityMismatch { expected: usize, found: usize },
    NumberParse { idx: usize },
    UnexpectedChar { pos: usize, found: char },
    TrailingInput { pos: usize },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EmptyInput => write!(f, "input is empty"),
            ParseError::UnknownFunc { name } => write!(f, "unknown function: {name}"),
            ParseError::ArityMismatch { expected, found } => write!(f, "expected {expected} args, found {found}"),
            ParseError::NumberParse { idx } => write!(f, "failed to parse number at arg {idx}"),
            ParseError::UnexpectedChar { pos, found } => write!(f, "unexpected character '{found}' at position {pos}"),
            ParseError::TrailingInput { pos } => write!(f, "trailing input starting at position {pos}"),
        }
    }
}

impl std::error::Error for ParseError {}
```

### Lightweight Parsers Without Extra Crates
- Use `str` utilities: `trim`, `strip_prefix`, `strip_suffix`, `split`, `find`, and iterating over `char`s.
- Prefer a simple “consume and advance” style: read head token, then arguments, then verify end.
- Implement `FromStr` for your target type to integrate with `"...".parse()`.

### Separate Parsing From Validation
- Parsing: turn text into structured parameters and arity.
- Validation: check numeric ranges, ordering, and finiteness. Return a different error variant for invalid semantics.


## 🔍 Step-by-Step Plan (Redacted for Domain)
1) Define the accepted syntax and arity per function name (case, whitespace, separators, etc.).
2) Implement a small parser that extracts `name(args...)` and splits arguments safely (respecting parentheses and whitespace).
3) Parse each argument into a numeric type; track index to report which argument failed.
4) Map `name + arity` to a constructor; validate ranges/ordering and return a precise error on failure.
5) Implement `FromStr` for your target type to provide `"...".parse()` ergonomics.
6) Write boundary-first tests for empty input, unknown names, arity mismatch, bad numbers, extra/trailing input, and invalid parameter relationships.

[All domain-specific names, arity, and validation rules are intentionally omitted.]


## 🧰 Unrelated, Focused Examples for Concepts

Example A — Minimal `name(args)` parser
```rust
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Call { name: String, args: Vec<f64> }

impl FromStr for Call {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() { return Err(ParseError::EmptyInput); }

        // name
        let open = s.find('(').ok_or(ParseError::UnexpectedChar { pos: s.len(), found: ')' })?;
        let name = s[..open].trim();
        if name.is_empty() { return Err(ParseError::UnknownFunc { name: "".into() }); }

        // args segment
        let close = s.rfind(')').ok_or(ParseError::UnexpectedChar { pos: s.len(), found: '(' })?;
        if close < open { return Err(ParseError::UnexpectedChar { pos: close, found: ')' }); }
        let after = &s[(close+1)..].trim();
        if !after.is_empty() { return Err(ParseError::TrailingInput { pos: close + 1 }); }

        let raw_args = &s[(open+1)..close];
        let mut args = Vec::new();
        if !raw_args.trim().is_empty() {
            for (i, part) in raw_args.split(',').enumerate() {
                let p = part.trim();
                let val: f64 = p.parse().map_err(|_| ParseError::NumberParse { idx: i })?;
                args.push(val);
            }
        }
        Ok(Call { name: name.to_string(), args })
    }
}
```

Example B — Dispatch and arity checks (unrelated)
```rust
#[derive(Debug, PartialEq)]
enum Built {
    KindA(f64, f64, f64),
    KindB(f64, f64),
}

fn build(call: Call) -> Result<Built, ParseError> {
    match (call.name.as_str(), call.args.len()) {
        ("kinda", 3) => Ok(Built::KindA(call.args[0], call.args[1], call.args[2])),
        ("kindb", 2) => Ok(Built::KindB(call.args[0], call.args[1])),
        (name, found) => Err(ParseError::UnknownFunc { name: format!("{name}/{found}") }),
    }
}
```

Example C — Validation separated from parsing
```rust
#[derive(Debug, PartialEq)]
struct Rule { a: f64, b: f64, c: f64 }

impl Rule {
    fn new(a: f64, b: f64, c: f64) -> Result<Self, ParseError> {
        if !(a.is_finite() && b.is_finite() && c.is_finite()) {
            return Err(ParseError::UnexpectedChar { pos: 0, found: '?' }); // placeholder error variant
        }
        if !(a < b && b < c) {
            // In real code, create a dedicated variant like InvalidOrder { a, b, c }
            return Err(ParseError::ArityMismatch { expected: 3, found: 0 }); // placeholder reuse
        }
        Ok(Self { a, b, c })
    }
}
```

Note: For your solution, define clear, dedicated error variants for invalid numeric relationships instead of reusing placeholders as shown above.


## 🛡️ Validation Patterns (Unrelated)
```rust
#[derive(Debug)]
struct Range01(f64);
impl Range01 {
    fn new(v: f64) -> Result<Self, &'static str> {
        if v.is_finite() && (0.0..=1.0).contains(&v) { Ok(Self(v)) } else { Err("value not in [0,1]") }
    }
}

#[derive(Debug)]
struct Ascending3(f64, f64, f64);
impl Ascending3 {
    fn new(a: f64, b: f64, c: f64) -> Result<Self, &'static str> {
        if !(a.is_finite() && b.is_finite() && c.is_finite()) { return Err("non-finite value"); }
        if !(a < b && b < c) { return Err("expected a < b < c"); }
        Ok(Self(a, b, c))
    }
}
```


## 🧪 Testing Strategy (Style-Matched)
- Write inline tests beside your parser and builders: `#[cfg(test)] mod tests { use super::*; }`.
- Cover boundaries: empty input, whitespace-only, missing `(` or `)`, trailing content, zero/one/many args.
- Validate unknown function names and arity mismatches with clear messages.
- Check number parsing failures with argument index annotations.
- Include semantic validation tests (e.g., ordering, ranges) returning specific error variants.

Unrelated test sketch:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_call() {
        let c: Call = "kinda(1, 2, 3)".parse().unwrap();
        assert_eq!(c.name, "kinda");
        assert_eq!(c.args, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn reports_bad_number() {
        let e: Result<Call, _> = "kinda(1, x, 3)".parse();
        assert!(matches!(e, Err(ParseError::NumberParse { idx: 1 })));
    }

    #[test]
    fn reports_trailing() {
        let e: Result<Call, _> = "kindb(1,2) extra".parse();
        assert!(matches!(e, Err(ParseError::TrailingInput { .. })));
    }
}
```


## 🕳️ Common Pitfalls and How to Avoid Them
- Naively `split(',')` without handling parentheses and whitespace carefully; always trim and verify brackets.
- Vague errors like “invalid input”; prefer precise variants (where, what, which arg).
- Panicking in parsing/validation; return `Result` from constructors and parsers.
- Confusing parsing with validation; keep them separate for clarity and reuse.
- Ignoring finiteness or range checks for floats; use `is_finite()` and clear bounds.


## ✅ Best Practices (Repo-Aligned)
- Minimal visibility; expose only the parsing entry points you need.
- Constructors return `Result` to enforce invariants; avoid `unwrap()`/`expect()` in non-test code.
- Implement `FromStr` for ergonomic `"...".parse()`; consider `TryFrom<&str>` if you prefer.
- Keep dependencies minimal; if you add an error helper crate, justify it in `Cargo.toml`.
- Run: `cargo fmt --all`, `cargo clippy --all-targets -- -D warnings`, `cargo test`.
- Write boundary-first tests and include edge inputs in your suite.


## 🚀 Your Next Step
Translate these parsing and error-handling patterns into your solution for this challenge. Define a clear grammar, produce descriptive error variants, separate parsing from validation, and test the tricky edges first.

[All domain-specific code remains intentionally redacted.]

