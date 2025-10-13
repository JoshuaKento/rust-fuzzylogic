//! Aggregation: combine rule outputs into per-variable membership samples.
//!
//! This module implements the aggregation stage for Mamdani-style fuzzy
//! inference systems. Given a set of rules and crisp inputs, each rule is
//! evaluated to produce its implied output membership functions on a
//! discretized grid. Aggregation then combines those contributions across all
//! rules by taking the pointwise supremum (max) for each output variable.
//!
//! Key characteristics:
//! - Sampling: output membership functions are discretized using a
//!   `UniformSampler` (evenly spaced samples across the variable domain).
//! - Implication: individual rules produce per-variable sample vectors via
//!   `Rule::implicate` using the chosen operator family.
//! - Aggregation: contributions for the same output variable are merged with
//!   a pointwise max (`elements_max`).
//!
//! Errors from `aggregation` originate from rule activation, implication,
//! variable lookup, or input lookup and propagate as `FuzzyError`.
//!
//! Example
//! ```rust
//! use std::collections::HashMap;
//! use rust_fuzzylogic::prelude::*;
//! use rust_fuzzylogic::membership::triangular::Triangular;
//! use rust_fuzzylogic::term::Term;
//! use rust_fuzzylogic::variable::Variable;
//! use rust_fuzzylogic::antecedent::Antecedent;
//! use rust_fuzzylogic::mamdani::{Rule, Consequent};
//! use rust_fuzzylogic::aggregate::aggregation;
//!
//! // Build a minimal system
//! let mut temp = Variable::new(-10.0, 10.0).unwrap();
//! temp.insert_term("hot", Term::new("hot", Triangular::new(0.0, 5.0, 10.0).unwrap())).unwrap();
//! let mut fan = Variable::new(0.0, 10.0).unwrap();
//! fan.insert_term("high", Term::new("high", Triangular::new(5.0, 7.5, 10.0).unwrap())).unwrap();
//!
//! let mut vars: HashMap<&str, Variable> = HashMap::new();
//! vars.insert("temp", temp);
//! vars.insert("fan", fan);
//!
//! // IF temp IS hot THEN fan IS high
//! let rule = Rule{
//!     antecedent: Antecedent::Atom{ var: "temp".into(), term: "hot".into() },
//!     consequent: vec![Consequent{ var: "fan".into(), term: "high".into() }],
//! };
//! let rules = vec![rule];
//!
//! let mut input: HashMap<&str, Float> = HashMap::new();
//! input.insert("temp", 7.5);
//! let sampler = UniformSampler::default();
//!
//! // Aggregate implied outputs for each consequent variable
//! let agg = aggregation(&rules, &input, &vars, &sampler).unwrap();
//! assert!(agg.contains_key("fan"));
//! assert_eq!(agg["fan"].len(), sampler.n);
//! ```

use crate::{mamdani::Rule, prelude::*, variable::Variable};
use std::{borrow::Borrow, collections::HashMap, hash::Hash};

/// Combine two membership sample vectors by taking the pointwise maximum.
///
/// Merges `src` into `data` in-place using `max` for every index. Both vectors
/// must have identical length; this function assumes consistent sampling
/// resolution produced by the same `UniformSampler`.
///
/// Example
/// ```rust
/// use rust_fuzzylogic::prelude::*;
/// use rust_fuzzylogic::aggregate::elements_max;
/// let mut a: Vec<Float> = vec![0.1, 0.3, 0.2];
/// let b: Vec<Float> = vec![0.0, 0.5, 0.4];
/// elements_max(&mut a, &b);
/// assert_eq!(a, vec![0.1, 0.5, 0.4]);
/// ```
pub fn elements_max(data: &mut Vec<Float>, src: &Vec<Float>) {
    for (d, s) in data.iter_mut().zip(src) {
        *d = d.max(*s)
    }
}

/// Aggregate the contributions of all rules into output membership functions.
///
/// For each `Rule`, this function computes its activation (`Rule::activation`)
/// from `input` and `vars`, applies implication (`Rule::implicate`) to obtain
/// output membership sample vectors, then merges those vectors into a single
/// map keyed by output variable name using pointwise maxima.
///
/// Type parameters and bounds:
/// - `KI`: key type for the input map (must borrow as `str`) — e.g., `&str`,
///   `String`, or `Arc<str>`.
/// - `KV`: key type for the variables map (must borrow as `str`).
///
/// Returns a map from output variable name to its aggregated membership samples
/// (length `sampler.n`).
///
/// Errors
/// - Propagates `FuzzyError::NotFound` if an input or variable is missing.
/// - Propagates `FuzzyError::TypeMismatch` if a term name is unknown.
/// - Propagates `FuzzyError::OutOfBounds` if inputs are outside variable domains.
///
/// Performance
/// - Time: O(R * C * N) where `R` = rules, `C` = average number of consequents
///   per rule, and `N` = sampler resolution per variable.
/// - Memory: O(V * N) for the aggregated output across `V` output variables.
pub fn aggregation<KI, KV>(
    rules: &[Rule],
    input: &HashMap<KI, Float>,
    vars: &HashMap<KV, Variable>,
    sampler: &UniformSampler,
) -> Result<HashMap<String, Vec<Float>>>
where
    KI: Eq + Hash + Borrow<str>,
    KV: Eq + Hash + Borrow<str>,
{
    let mut implicated_map: HashMap<String, Vec<Float>> = HashMap::new();
    for i in 0..rules.len() {
        let alpha = rules[i].activation(&input, &vars)?;
        let implicated = rules[i].implicate(alpha, vars, &sampler)?;

        for (k, v) in implicated {
            implicated_map
                .entry(k)
                .and_modify(|cur| elements_max(cur, &v))
                .or_insert(v);
        }
    }

    return Ok(implicated_map);
}
