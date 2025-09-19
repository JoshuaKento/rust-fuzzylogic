// Operators for fuzzy logic antecedents and inference.
// Provides a trait (`FuzzyOps`) and concrete families (`Ops`) implementing AND/OR/NOT.
use crate::Float;

/// Common interface for fuzzy logic operators (T-norm, S-norm, complement).
pub trait FuzzyOps {
    /// T-norm (logical AND) combining two degrees in [0, 1].
    fn t(&self, a: Float, b: Float) -> Float;

    /// S-norm (logical OR) combining two degrees in [0, 1].
    fn s(&self, a: Float, b: Float) -> Float;

    /// Complement (logical NOT) of a degree in [0, 1].
    fn c(&self, a: Float) -> Float;
}

#[cfg(feature = "ops-minmax")]
pub struct MinMax;
#[cfg(feature = "ops-minmax")]
impl FuzzyOps for MinMax {
    fn t(&self, a: Float, b: Float) -> Float {
        a.min(b)
    }

    fn s(&self, a: Float, b: Float) -> Float {
        a.max(b)
    }

    fn c(&self, a: Float) -> Float {
        1.0 - a
    }
}

#[cfg(feature = "ops-product")]
pub struct MinMax;
#[cfg(feature = "ops-product")]
impl FuzzyOps for MinMax {
    fn t(&self, a: Float, b: Float) -> Float {
        a * b
    }

    fn s(&self, a: Float, b: Float) -> Float {
        a + b - a * b
    }

    fn c(&self, a: Float) -> Float {
        1.0 - a
    }
}

#[cfg(feature = "ops-lukasiewicz")]
pub struct MinMax;
#[cfg(feature = "ops-lukasiewicz")]
impl FuzzyOps for MinMax {
    fn t(&self, a: Float, b: Float) -> Float {
        (a + b - 1.0).max(0.0)
    }

    fn s(&self, a: Float, b: Float) -> Float {
        (a + b).min(1.0)
    }

    fn c(&self, a: Float) -> Float {
        1.0 - a
    }
}

#[cfg(feature = "ops-dyn")]
#[derive(Clone, Copy, Debug)]
/// Built-in operator families providing AND/OR/NOT over degrees.
pub enum Ops {
    /// Min–Max family
    /// - T: `min(a, b)`
    /// - S: `max(a, b)`
    /// - C: `1 - a`
    MinMax,
    /// Product family
    /// - T: `a * b`
    /// - S: `a + b - a * b` (not algebraic sum; may exceed 1.0)
    /// - C: `1 - a`
    Product,
    /// Łukasiewicz family
    /// - T: `max(0, a + b - 1)`
    /// - S: `min(1, a + b)`
    /// - C: `1 - a`
    Lukasiewicz,
}
#[cfg(feature = "ops-dyn")]
/// Implements `FuzzyOps` for each `Ops` variant using the formulas above.
impl FuzzyOps for Ops {
    /// T-norm (AND) per family.
    fn t(&self, a: Float, b: Float) -> Float {
        match self {
            Ops::MinMax => a.min(b),
            Ops::Product => a * b,
            Ops::Lukasiewicz => (a + b - 1.0).max(0.0),
        }
    }

    /// S-norm (OR) per family.
    fn s(&self, a: Float, b: Float) -> Float {
        match self {
            Ops::MinMax => a.max(b),
            Ops::Product => a + b - a * b,
            Ops::Lukasiewicz => (a + b).min(1.0),
        }
    }

    /// Complement (NOT) shared by all families: `1 - a`.
    fn c(&self, a: Float) -> Float {
        1.0 - a
    }
}

#[cfg(feature = "ops-dyn")]
#[cfg(test)]
mod tests_dyn_ops {
    use crate::ops::*;

    // RED: expected default operator behavior (min/max/1-x).
    // This test references the intended API and should fail right now
    // because the operators are not implemented yet.
    #[test]
    fn red_minmax_defaults_and_or_not() {
        let v = crate::ops::Ops::MinMax;

        // Mixed values
        assert_eq!(v.t(0.2, 0.8), 0.2);
        assert_eq!(v.s(0.2, 0.8), 0.8);
        assert_eq!(v.c(0.2), 0.8);

        // Boundaries
        assert_eq!(v.t(0.0, 1.0), 0.0);
        assert_eq!(v.s(0.0, 1.0), 1.0);
        assert_eq!(v.c(0.0), 1.0);
        assert_eq!(v.c(1.0), 0.0);
    }

    #[test]
    fn product_ops_and_or_not_matches_code() {
        let v = Ops::Product;
        let eps = crate::Float::EPSILON;
        // t = a*b
        assert!((v.t(0.2, 0.8) - 0.16).abs() < eps);
        // s = a + b (per current code)
        assert!((v.s(0.1, 0.2) - 0.28).abs() < eps);
        // c = 1 - a
        assert!((v.c(0.2) - 0.8).abs() < eps);
    }

    #[test]
    fn lukasiewicz_ops_and_or_not_matches_code() {
        let v = Ops::Lukasiewicz;
        let eps = crate::Float::EPSILON;
        // t = max(0, a + b - 1)
        assert!((v.t(0.2, 0.8) - 0.0).abs() < eps);
        assert!((v.t(0.8, 0.3) - 0.1).abs() < eps);
        // s = min(1, a + b)
        assert!((v.s(0.2, 0.8) - 1.0).abs() < eps);
        assert!((v.s(0.4, 0.4) - 0.8).abs() < eps);
        // c = 1 - a
        assert!((v.c(0.2) - 0.8).abs() < eps);
    }
}
