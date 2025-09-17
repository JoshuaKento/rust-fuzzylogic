// Variable: crisp scalar with domain and named fuzzy terms.
// This file defines the `Variable` type plus red tests for its API.
use crate::{Float, error::FuzzyError, membership::MembershipFn, term::Term};

use std::collections::HashMap;

/// A crisp variable with an inclusive numeric domain and a set of named terms.
pub struct Variable {
    /// Inclusive lower bound of the variable's domain.
    min: Float,

    /// Inclusive upper bound of the variable's domain.
    max: Float,

    /// Mapping from term name to its labeled membership function wrapper.
    terms: HashMap<String, Term>,
}
impl Variable {
    /// Constructs a new variable, validating that `min < max`.
    pub fn new(min: Float, max: Float) -> crate::error::Result<Self> {
        // Domain validation: bounds must be strictly ordered.
        if min >= max {
            Err(FuzzyError::OutOfBounds)
        } else {
            // Initialize with an empty term map.
            Ok(Self {
                min: min,
                max: max,
                terms: HashMap::new(),
            })
        }
    }

    /// Inserts a named term; rejects empty names and duplicates.
    ///
    /// - Empty name -> `FuzzyError::EmptyInput`
    /// - Duplicate name -> `FuzzyError::TypeMismatch`
    pub fn insert_term(&mut self, name: &str, t: Term) -> crate::error::Result<()> {
        // Reject empty label.
        if name == "" {
            Err(FuzzyError::EmptyInput)
        }
        // Reject duplicates to avoid silent overwrites.
        else if self.get(name).is_some() {
            Err(FuzzyError::TypeMismatch)
        } else {
            // Store the term by name.
            self.terms.insert(name.to_string(), t);
            Ok(())
        }
    }

    /// Returns a reference to the term for `name`, if present.
    pub fn get(&self, name: &str) -> Option<&Term> {
        self.terms.get(name)
    }

    /// Evaluates the membership degree for term `name` at input `x`.
    ///
    /// - Unknown term -> `FuzzyError::TypeMismatch`
    /// - `x` out of `[min, max]` -> `FuzzyError::OutOfBounds`
    pub fn eval(&self, name: &str, x: Float) -> crate::error::Result<Float> {
        // Resolve term by name.
        let v = &self.terms.get(name).ok_or(FuzzyError::TypeMismatch)?;
        // Domain check is inclusive: allow x == min or x == max.
        if self.max < x || self.min > x {
            Err(FuzzyError::OutOfBounds)
        }
        // Delegate to the term's membership function.
        else {
            Ok(v.eval(x))
        }
    }

    //Optional helpers:
    //pub fn domain(&self) -> (Float, Float)
    //pub fn names(&self) -> impl Iterator<Item=&str>
    //pub fn fuzzify(&self, x: Float) -> crate::error::Result<Vec<(String, Float)>> to get all memberships at x.
}

/// Unit tests describing the expected `Variable` API and behavior.
#[cfg(test)]
mod tests {
    use crate::error::FuzzyError;
    use crate::membership::MembershipFn;
    use crate::membership::triangular::Triangular;
    use crate::term::Term;

    /// `new` must reject invalid domain bounds (min >= max).
    #[test]
    fn test_new_rejects_invalid_domain() {
        assert!(matches!(
            crate::variable::Variable::new(1.0, 1.0),
            Err(FuzzyError::OutOfBounds)
        ));
        assert!(matches!(
            crate::variable::Variable::new(2.0, 1.0),
            Err(FuzzyError::OutOfBounds)
        ));
    }

    /// Insert two terms and evaluate memberships by name within the domain.
    #[test]
    fn test_insert_and_eval_by_name() {
        let mut v = crate::variable::Variable::new(-10.0, 10.0).unwrap();

        // Define two terms backed by triangular membership functions.
        let cold_tri = Triangular::new(-10.0, -5.0, 0.0).unwrap();
        let hot_tri = Triangular::new(0.0, 5.0, 10.0).unwrap();
        let cold = Term::new("cold", cold_tri);
        let hot = Term::new("hot", hot_tri);

        // Insert terms and verify lookup works.
        v.insert_term("cold", cold).unwrap();
        v.insert_term("hot", hot).unwrap();

        assert!(v.get("cold").is_some());
        assert!(v.get("warm").is_none());

        // Evaluate by name at a few in-domain points and compare to direct membership.
        let x1 = -5.0;
        let x2 = 7.5;

        let expected_cold_x1 = Triangular::new(-10.0, -5.0, 0.0).unwrap().eval(x1);
        let expected_hot_x2 = Triangular::new(0.0, 5.0, 10.0).unwrap().eval(x2);

        let eps = crate::Float::EPSILON;
        let y_cold_x1 = v.eval("cold", x1).unwrap();
        let y_hot_x2 = v.eval("hot", x2).unwrap();
        assert!((y_cold_x1 - expected_cold_x1).abs() < eps);
        assert!((y_hot_x2 - expected_hot_x2).abs() < eps);

        // Endpoints are considered in-domain: [min, max]
        assert!(v.eval("cold", -10.0).is_ok());
        assert!(v.eval("hot", 10.0).is_ok());
    }

    /// Reject duplicate term insertions for the same name.
    #[test]
    fn test_duplicate_term_rejected() {
        let mut v = crate::variable::Variable::new(0.0, 1.0).unwrap();
        let t1 = Term::new("x", Triangular::new(0.0, 0.5, 1.0).unwrap());
        let t2 = Term::new("x", Triangular::new(0.0, 0.25, 0.5).unwrap());

        v.insert_term("x", t1).unwrap();
        // Second insertion with the same name should error (reject duplicates).
        assert!(matches!(
            v.insert_term("x", t2),
            Err(FuzzyError::TypeMismatch)
        ));
    }

    /// Unknown term lookup during eval should return an error.
    #[test]
    fn test_eval_unknown_term_errors() {
        let v = crate::variable::Variable::new(0.0, 1.0).unwrap();
        // Unknown term name: return a consistent error variant.
        assert!(matches!(
            v.eval("missing", 0.3),
            Err(FuzzyError::TypeMismatch)
        ));
    }

    /// Evaluating outside the variable domain should return OutOfBounds.
    #[test]
    fn test_eval_out_of_domain_errors() {
        let mut v = crate::variable::Variable::new(0.0, 1.0).unwrap();
        v.insert_term("x", Term::new("x", Triangular::new(0.0, 0.5, 1.0).unwrap()))
            .unwrap();

        // Out-of-domain x should return OutOfBounds.
        assert!(matches!(v.eval("x", -0.1), Err(FuzzyError::OutOfBounds)));
        assert!(matches!(v.eval("x", 1.1), Err(FuzzyError::OutOfBounds)));
    }
}
