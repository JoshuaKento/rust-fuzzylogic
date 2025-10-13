//! Defuzzification: reduce aggregated membership samples to crisp outputs.
//!
//! This module implements centroid-of-area (center of gravity) defuzzification
//! over discretized membership samples produced by aggregation. For each output
//! variable, the aggregated membership vector `μ[i]` is paired with the grid of
//! x-coordinates derived from the variable's domain and the vector length, and
//! the crisp output is computed as:
//!
//! `x* = (Σ x[i] · μ[i]) / (Σ μ[i])`, where `i = 0..N-1` and `N ≥ 2`.
//!
//! Notes
//! - The x-grid spacing is `step = (max - min) / (N - 1)` so both domain
//!   endpoints are always sampled.
//! - If the sum Σ μ[i] is numerically zero, the result will follow IEEE-754
//!   semantics (e.g., `NaN` or `inf`) since no special handling is applied.
//!   Ensure that your aggregated membership carries non-zero mass.
//!
//! Example
//! ```rust
//! use std::collections::HashMap;
//! use rust_fuzzylogic::prelude::*;
//! use rust_fuzzylogic::membership::triangular::Triangular;
//! use rust_fuzzylogic::term::Term;
//! use rust_fuzzylogic::variable::Variable;
//! use rust_fuzzylogic::defuzz::defuzzification;
//!
//! // Suppose aggregation produced a single output variable "fan" with N samples
//! let sampler_n = 5usize;
//! let mut agg: HashMap<String, Vec<Float>> = HashMap::new();
//! agg.insert("fan".into(), vec![0.0, 0.2, 0.6, 0.2, 0.0]);
//!
//! // The corresponding variable domain is needed to build x[i]
//! let mut fan = Variable::new(0.0, 10.0).unwrap();
//! fan.insert_term("high", Term::new("high", Triangular::new(5.0, 7.5, 10.0).unwrap())).unwrap();
//! let mut vars: HashMap<&str, Variable> = HashMap::new();
//! vars.insert("fan", fan);
//!
//! // Compute centroid for each aggregated output variable
//! let crisp = defuzzification(&agg, &vars).unwrap();
//! assert!(crisp["fan"] >= 0.0 && crisp["fan"] <= 10.0);
//! ```
// Defuzzification utilities for collapsing aggregated membership values.
use crate::{error::MissingSpace, prelude::*, variable::Variable};
use std::{borrow::Borrow, collections::HashMap, hash::Hash};

/// Defuzzify aggregated membership samples using the centroid-of-area method.
///
/// For each entry in `agg_memberships`, the function reconstructs a uniform
/// x-grid from the variable's domain and the vector length, then computes the
/// centroid `Σ x[i]·μ[i] / Σ μ[i]`.
///
/// Type parameters and bounds:
/// - `KV`: key type for the variables map; must borrow as `str`.
///
/// Returns a map from output variable name to its crisp centroid value.
///
/// Errors
/// - `FuzzyError::BadArity` if a membership vector has fewer than 2 samples.
/// - `FuzzyError::NotFound` if a required output variable is missing from `vars`.
pub fn defuzzification<KV>(
    agg_memberships: &HashMap<String, Vec<Float>>,
    vars: &HashMap<KV, Variable>,
) -> Result<HashMap<String, Float>>
where
    KV: Eq + Hash + Borrow<str>,
{
    let mut result_map: HashMap<String, Float> = HashMap::new();
    for (i, j) in agg_memberships {
        let num = j.len();
        if num < 2 {
            return Err(FuzzyError::BadArity);
        }

        let (var_min, var_max) = vars
            .get(&i)
            .ok_or(FuzzyError::NotFound {
                space: MissingSpace::Var,
                key: i.to_string(),
            })?
            .domain();
        let step = (var_max - var_min) / (num as Float - 1.0);

        let (mut sum_agg_memberships_x, mut sum_agg_memberships): (Float, Float) = (0.0, 0.0);
        let mut l: usize = 0;
        for k in j {
            let x = var_min + step * l as Float;
            sum_agg_memberships_x = sum_agg_memberships_x + (x * k);
            sum_agg_memberships = sum_agg_memberships + k;
            l += 1;
        }
        result_map.insert(i.to_string(), sum_agg_memberships_x / sum_agg_memberships);
    }

    return Ok(result_map);
}
