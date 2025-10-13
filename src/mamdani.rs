//! Mamdani inference: rules, activation, and implication (clipping).
//!
//! This module defines the core building blocks of a Mamdani-style fuzzy
//! inference system:
//! - `Rule`: pairs an `Antecedent` (IF) with one or more `Consequent`s (THEN).
//! - `Rule::activation`: evaluates the antecedent using the default Min–Max
//!   operator family (AND=min, OR=max, NOT=1-x) to produce an activation
//!   degree `alpha ∈ [0, 1]` for the current crisp inputs.
//! - `Rule::implicate`: applies implication using clipping (`min(alpha, μ_B(x))`)
//!   to produce discretized output membership samples for each consequent.
//!
//! The resulting per-variable membership samples can be aggregated across rules
//! (see `crate::aggregate`) and then defuzzified to crisp outputs (see
//! `crate::defuzz`).
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
//!
//! // Variable and terms
//! let mut temp = Variable::new(-10.0, 10.0).unwrap();
//! temp.insert_term("hot", Term::new("hot", Triangular::new(0.0, 5.0, 10.0).unwrap())).unwrap();
//! let mut fan = Variable::new(0.0, 10.0).unwrap();
//! fan.insert_term("high", Term::new("high", Triangular::new(5.0, 7.5, 10.0).unwrap())).unwrap();
//!
//! // IF temp IS hot THEN fan IS high
//! let rule = Rule{
//!     antecedent: Antecedent::Atom{ var: "temp".into(), term: "hot".into() },
//!     consequent: vec![Consequent{ var: "fan".into(), term: "high".into() }],
//! };
//!
//! // Activation for a crisp input
//! let mut inputs: HashMap<&str, Float> = HashMap::new();
//! inputs.insert("temp", 7.5);
//! let mut vars: HashMap<&str, Variable> = HashMap::new();
//! vars.insert("temp", temp);
//! vars.insert("fan", fan);
//!
//! let alpha = rule.activation(&inputs, &vars).unwrap();
//! assert!(alpha > 0.0 && alpha <= 1.0);
//!
//! // Implication discretizes μ_B(x) clipped by alpha across the domain
//! let sampler = UniformSampler::default();
//! let implied = rule.implicate(alpha, &vars, &sampler).unwrap();
//! assert_eq!(implied["fan"].len(), sampler.n);
//! ```

use std::{borrow::Borrow, collections::HashMap, hash::Hash};

//#[cfg(feature = "inference-mamdani")]
use crate::{
    antecedent::{eval_antecedent, Antecedent},
    error::{FuzzyError, MissingSpace},
    prelude::*,
    sampler::UniformSampler,
    variable::Variable,
};

/// Output clause (THEN-part) referencing a linguistic variable and term.
///
/// A `Consequent` ties a linguistic variable (e.g., `"fan"`) to one of its
/// labeled membership functions/terms (e.g., `"high"`). During implication,
/// the membership function for `term` is sampled over the variable domain and
/// clipped by the rule activation `alpha`.
pub struct Consequent {
    pub var: String,
    pub term: String,
    //pub weight: Float,
    //pub imp: Implication,
}

/// Full fuzzy rule pairing an antecedent with one or more consequents.
///
/// - `antecedent` encodes the IF-part as an expression tree of atomic
///   predicates combined with AND/OR/NOT using the default Min–Max family.
/// - `consequent` lists one or more THEN-part outputs; each is evaluated over
///   its variable domain during implication.
pub struct Rule {
    pub antecedent: Antecedent,
    pub consequent: Vec<Consequent>,
}

//Mamdani Inference Engine
//#[cfg(feature = "inference-mamdani")]
impl Rule {
    /// Evaluate the antecedent against crisp inputs to obtain activation.
    ///
    /// Type parameters
    /// - `KI`: key type for `input`, must borrow as `str` (e.g., `&str`).
    /// - `KV`: key type for `vars`, must borrow as `str`.
    ///
    /// Returns the activation degree `alpha ∈ [0, 1]` for this rule.
    ///
    /// Errors
    /// - `FuzzyError::NotFound` if an input or variable is missing.
    /// - `FuzzyError::TypeMismatch` if the antecedent references an unknown term.
    /// - `FuzzyError::OutOfBounds` if an input value lies outside a variable's domain.
    pub fn activation<KI, KV>(
        &self,
        input: &HashMap<KI, Float>,
        vars: &HashMap<KV, Variable>,
    ) -> Result<Float>
    where
        KI: Eq + Hash + Borrow<str>,
        KV: Eq + Hash + Borrow<str>,
    {
        eval_antecedent(&self.antecedent, input, vars)
    }

    /// Apply implication to produce discretized membership outputs.
    ///
    /// For each `Consequent`, this function:
    /// 1) retrieves the target variable's domain, 2) builds an evenly spaced
    ///    grid of `sampler.n` points, 3) evaluates the consequent term's
    ///    membership `μ_B(x)` on that grid, and 4) applies clipping via
    ///    `min(alpha, μ_B(x))`.
    ///
    /// The result is a map from variable name to the vector of sampled values.
    ///
    /// Errors
    /// - `FuzzyError::NotFound` if a variable or term cannot be found.
    /// - `FuzzyError::OutOfBounds` if evaluation occurs outside the domain.
    ///
    /// Note
    /// - The x-grid spacing is `(max - min) / (sampler.n - 1)` so the vector
    ///   always includes both domain endpoints.
    pub fn implicate<KV>(
        &self,
        alpha: Float,
        vers: &HashMap<KV, Variable>,
        sampler: &UniformSampler,
    ) -> Result<HashMap<String, Vec<Float>>>
    where
        KV: Eq + Hash + Borrow<str>,
    {
        let mut result_map: HashMap<String, Vec<Float>> = HashMap::new();

        for i in 0..self.consequent.len() {
            let mut result_vec = vec![0.0; sampler.n];

            let (dom_min, dom_max) = vers
                .get(&self.consequent[i].var.as_str())
                .ok_or(FuzzyError::NotFound {
                    space: MissingSpace::Var,
                    key: self.consequent[i].var.clone(),
                })?
                .domain();

            let step = (dom_max - dom_min) / (sampler.n - 1) as Float;

            for k in 0..sampler.n {
                let x = dom_min + (k as Float * step);
                result_vec[k] = vers
                    .get(&self.consequent[i].var.as_str())
                    .ok_or(FuzzyError::NotFound {
                        space: MissingSpace::Var,
                        key: self.consequent[i].term.clone(),
                    })?
                    .eval(&self.consequent[i].term, x)?
                    .min(alpha);
            }

            result_map.insert(self.consequent[i].var.to_string(), result_vec);
        }
        return Ok(result_map);
        //TODO: Return type should be hashmap<string, Vec<Float>> where string signifies the variable(eg "fanspeed")
    }
}
