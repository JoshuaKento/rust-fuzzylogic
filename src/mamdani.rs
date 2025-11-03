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
#[derive(Clone)]
pub struct Consequent {
    /// Target output variable name this consequent refers to.
    pub var: String,
    /// Term label within the target variable to be used during implication.
    pub term: String,
    //pub weight: Float,
    //pub imp: Implication,
}

impl Consequent {
    /// Validate that `term` matches this consequent's term label.
    ///
    /// Returns `Ok(())` if the provided `term` equals `self.term`, otherwise
    /// returns `FuzzyError::NotFound { space: Term, key: term }`.
    pub fn get_term(&self, term: String) -> Result<()> {
        // Simple equality check; does not perform variable lookup.
        if term == self.term {
            return Ok(());
        } else {
            return Err(FuzzyError::NotFound {
                space: MissingSpace::Term,
                key: term,
            });
        }
    }

    /// Validate that `vars` matches this consequent's variable name.
    ///
    /// Returns `Ok(())` if the provided `vars` equals `self.var`, otherwise
    /// returns `FuzzyError::NotFound { space: Var, key: vars }`.
    pub fn get_vars(&self, vars: String) -> Result<()> {
        if vars == self.var {
            return Ok(());
        } else {
            return Err(FuzzyError::NotFound {
                space: MissingSpace::Var,
                key: vars,
            });
        }
    }
}

/// Full fuzzy rule pairing an antecedent with one or more consequents.
///
/// - `antecedent` encodes the IF-part as an expression tree of atomic
///   predicates combined with AND/OR/NOT using the default Min–Max family.
/// - `consequent` lists one or more THEN-part outputs; each is evaluated over
///   its variable domain during implication.
#[derive(Clone)]
pub struct Rule {
    /// IF-part expressed as an `Antecedent` AST over input variables/terms.
    pub antecedent: Antecedent,
    /// THEN-parts listing output variable/term pairs to implicate.
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
        vars: &HashMap<KV, Variable>,
        sampler: &UniformSampler,
    ) -> Result<HashMap<String, Vec<Float>>>
    where
        KV: Eq + Hash + Borrow<str>,
    {
        // Accumulate per-output-variable membership samples produced by this rule.
        let mut result_map: HashMap<String, Vec<Float>> = HashMap::new();

        for i in 0..self.consequent.len() {
            let cons = &self.consequent[i];
            let var_name = cons.var.as_str();
            let var_ref = vars.get(var_name).ok_or(FuzzyError::NotFound {
                space: MissingSpace::Var,
                key: cons.var.clone(),
            })?;

            let (dom_min, dom_max) = var_ref.domain();
            // Prepare a buffer of N samples for μ_B(x) clipped by alpha.
            let mut result_vec = vec![0.0; sampler.n];
            let step = (dom_max - dom_min) / (sampler.n - 1) as Float;

            for k in 0..sampler.n {
                // Uniform grid including both endpoints.
                let x = dom_min + (k as Float * step);
                // Evaluate μ_B(x) and apply Mamdani clipping: min(alpha, μ_B(x)).
                result_vec[k] = var_ref.eval(cons.term.as_str(), x)?.min(alpha);
            }

            // Store samples under the output variable name.
            result_map.insert(cons.var.clone(), result_vec);
        }
        return Ok(result_map);
        //TODO: Return type should be hashmap<string, Vec<Float>> where string signifies the variable(eg "fanspeed")
    }

    /// Placeholder: check if a given variable exists within a rule.
    pub fn get_vars(self) {
        unimplemented!()
    }

    /// Validate that this rule references only existing variables and terms.
    ///
    /// Ensures all antecedent atoms and consequents point to variables present
    /// in `vars` and that the referenced term labels exist within those
    /// variables' term maps. Returns `Ok(())` on success, or `NotFound` errors
    /// identifying the missing `Var` or `Term`.
    pub fn validate<KV>(&self, vars: &HashMap<KV, Variable>) -> Result<()>
    where
        KV: Eq + Hash + Borrow<str>,
    {
        // Validate antecedent atoms reference existing var/term in `vars`.
        fn validate_antecedent<KV>(ant: &Antecedent, vars: &HashMap<KV, Variable>) -> Result<()>
        where
            KV: Eq + Hash + Borrow<str>,
        {
            match ant {
                Antecedent::Atom { var, term } => {
                    // Variable must exist.
                    let v = vars.get(var.as_str()).ok_or(FuzzyError::NotFound {
                        space: MissingSpace::Var,
                        key: var.clone(),
                    })?;
                    // Term must exist on the variable.
                    if !v.terms.contains_key(term.as_str()) {
                        return Err(FuzzyError::NotFound {
                            space: MissingSpace::Term,
                            key: term.clone(),
                        });
                    }
                    Ok(())
                }
                Antecedent::And(a, b) | Antecedent::Or(a, b) => {
                    // Validate both sides.
                    validate_antecedent(a, vars)?;
                    validate_antecedent(b, vars)
                }
                Antecedent::Not(a) => validate_antecedent(a, vars),
            }
        }

        validate_antecedent(&self.antecedent, vars)?;

        // Validate consequents reference existing var/term.
        for cons in &self.consequent {
            let v = vars.get(cons.var.as_str()).ok_or(FuzzyError::NotFound {
                space: MissingSpace::Var,
                key: cons.var.clone(),
            })?;
            // Consequent term must exist on the target variable.
            if !v.terms.contains_key(cons.term.as_str()) {
                return Err(FuzzyError::NotFound {
                    space: MissingSpace::Term,
                    key: cons.term.clone(),
                });
            }
        }

        Ok(())
    }
}
