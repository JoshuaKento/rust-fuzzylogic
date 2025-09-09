# Challenge 5 Guide: "The Universal Evaluator"

## 🌟 What You’re Building (Concept-Only)
You’ll design a common interface (a Rust trait) that multiple, different types can implement, then store those different types together and call the same method on each through trait objects. This guide follows the style of earlier guides and deliberately avoids domain-specific code or direct answers. All examples illustrate concepts using unrelated scenarios.

Core ideas:
- Traits as behavior contracts and object safety
- Trait objects (`dyn Trait`) for runtime polymorphism (dynamic dispatch)
- Heterogeneous collections using `Box<dyn Trait>` / `Arc<dyn Trait + Send + Sync>`
- Constructors for validation, minimal visibility, boundary-first tests


## 💡 New Rust Concepts You’ll Learn

### Traits as Interfaces
Traits define what behavior a type must have. Any type implementing the trait can be used anywhere that trait is expected.

Unrelated example — Logger
```rust
trait Logger { fn log(&self, msg: &str); }

struct Stdout;
struct Buffer { out: String }

impl Logger for Stdout { fn log(&self, msg: &str) { println!("{msg}"); } }
impl Logger for Buffer { fn log(&self, msg: &str) { /* push to self.out */ } }
```

### Trait Objects and Dynamic Dispatch
Use `Box<dyn Trait>` (or `&dyn Trait`, `Arc<dyn Trait>`) when you need a single container with values of different concrete types that all implement the same trait.

Unrelated example — Drawing API
```rust
trait Drawable { fn draw(&self); }
struct Button; struct Checkbox; struct Slider;
impl Drawable for Button   { fn draw(&self) { /* ... */ } }
impl Drawable for Checkbox { fn draw(&self) { /* ... */ } }
impl Drawable for Slider   { fn draw(&self) { /* ... */ } }

fn render_all(widgets: &[Box<dyn Drawable>]) {
    for w in widgets { w.draw(); }
}
```

### Object Safety (What Makes a Trait Usable as `dyn Trait`)
- No generic methods (in object-safe parts of the trait)
- No returning `Self` (unless in `where Self: Sized` methods)
- Methods take `&self`/`&mut self` or own `self`

Not object-safe (unrelated):
```rust
trait Bad { fn make<T>(&self, t: T) -> T; } // generic → not object-safe
```

Object-safe shape:
```rust
trait Action { fn run(&self); }
```


## 🔍 Step-by-Step Plan (Redacted for Domain)
1) Define a single-method trait that “scores” or “handles” an input (choose names that fit your domain). Keep it object-safe.
2) Create several distinct types with different internal rules that implement that trait.
3) Validate inputs via constructors; return `Option`/`Result` instead of panicking.
4) Store instances together in `Vec<Box<dyn YourTrait>>` (or `Arc<dyn YourTrait + Send + Sync>` if shared across threads).
5) Iterate and call the trait method uniformly on each item.
6) Add tests focused on edge cases and expected behavior for each implementor and for the mixed collection.

[All domain-specific names and code are intentionally omitted.]


## 🧰 Unrelated, Focused Examples for Concepts

Example A — Notifier
```rust
trait Notifier { fn notify(&self, message: &str); }
struct Email(String); struct Sms(String); struct Push(String);
impl Notifier for Email { fn notify(&self, m: &str) { println!("email {}: {m}", self.0); } }
impl Notifier for Sms   { fn notify(&self, m: &str) { println!("sms {}: {m}", self.0); } }
impl Notifier for Push  { fn notify(&self, m: &str) { println!("push {}: {m}", self.0); } }

fn blast(ns: &[Box<dyn Notifier>], msg: &str) { for n in ns { n.notify(msg) } }
```

Example B — Price Rules Pipeline
```rust
trait PriceRule { fn apply(&self, base: f64) -> f64; }
struct Tax(f64); struct Discount(f64); struct Fee(f64);
impl PriceRule for Tax      { fn apply(&self, b: f64) -> f64 { b * (1.0 + self.0) } }
impl PriceRule for Discount { fn apply(&self, b: f64) -> f64 { b + self.0 } }
impl PriceRule for Fee      { fn apply(&self, b: f64) -> f64 { b + self.0 } }

fn price_after(mut base: f64, rules: &[Box<dyn PriceRule>]) -> f64 { for r in rules { base = r.apply(base); } base }
```

Example C — Drawable Collection
```rust
trait Drawable { fn draw(&self); }
struct Button; struct Checkbox;
impl Drawable for Button   { fn draw(&self) { /* draw button */ } }
impl Drawable for Checkbox { fn draw(&self) { /* draw checkbox */ } }

fn render_all(items: &[Box<dyn Drawable>]) { for it in items { it.draw(); } }
```


## 🛡️ Validation Patterns (Unrelated)
```rust
#[derive(Clone, Copy, Debug)]
struct Percent(f64);
impl Percent { fn new(p: f64) -> Option<Self> { if (0.0..=1.0).contains(&p) && p.is_finite() { Some(Self(p)) } else { None } } }
```

```rust
#[derive(Debug)]
struct Dimensions { w: f64, h: f64 }
impl Dimensions {
    fn new(w: f64, h: f64) -> Result<Self, &'static str> {
        if w > 0.0 && h > 0.0 && w.is_finite() && h.is_finite() { Ok(Self { w, h }) } else { Err("non-positive or non-finite dimensions") }
    }
}
```


## 🧪 Testing Strategy (Style-Matched)
- Write inline tests beside code: `#[cfg(test)] mod tests { use super::*; }`.
- Cover boundaries and typical values for each implementor.
- Add a test that mixes multiple implementors in one `Vec<Box<dyn Trait>>` and asserts method calls succeed for each.
- Avoid `unwrap()`/`expect()` in non-test code; feel free to use them inside tests.

Unrelated test sketch:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pipeline_runs() {
        let rules: Vec<Box<dyn PriceRule>> = vec![Box::new(Tax(0.1)), Box::new(Discount(-2.0))];
        let out = price_after(10.0, &rules);
        assert!(out.is_finite());
    }
}
```


## 🕳️ Common Pitfalls and How to Avoid Them
- Non–object-safe traits: remove generic methods; don’t return `Self` unless using `where Self: Sized`.
- Forgetting to box trait objects: concrete types have different sizes; use `Box`, `Arc`, or references.
- Missing validation: add constructors to guard against invalid parameters.
- Overexposed APIs: keep helpers private; expose only what external callers need.


## ✅ Best Practices (Repo-Aligned)
- Minimal visibility; small, clear trait method surface.
- Constructors return `Option`/`Result` to enforce invariants.
- Tests emphasize edges and correctness first.
- Run: `cargo fmt --all`, `cargo clippy --all-targets -- -D warnings`, `cargo test`.


## 🚀 Your Next Step
Translate these patterns into your actual solution for this challenge. Keep the trait object-safe, validate inputs in constructors, mix multiple implementors in a single collection, and write boundary-first tests.

[All domain-specific code remains intentionally redacted.]
