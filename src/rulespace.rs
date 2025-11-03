#![deprecated(since = "0.2.0", note = "Use system::System and SystemBuilder instead")]
//! Deprecated: RuleSpace has been superseded by `system::System` and
//! `system::SystemBuilder`.
//!
//! As of 0.2.0, prefer building fuzzy inference systems via the `System`
//! API, which validates rules on construction and provides a cohesive
//! builder for variables, terms, and rules.
//!
//! This module remains for backward compatibility and will be removed in a
//! future release.
//!

//! RuleSpace: orchestrate aggregation and defuzzification for a rule base.
//!
//! `RuleSpace` bundles the fuzzy variables, rule set, and the intermediate
//! aggregated membership values produced during inference. It provides a
//! convenience API to:
//! - construct a rule space from variables and rules (`new`),
//! - append additional rules (`add_rules`),
//! - run aggregation only (`aggregate`), and
//! - run aggregation followed by defuzzification in one call (`defuzzify`).
//!
//! Typical flow
//! 1) Define input/output variables and their terms.
//! 2) Build rules from antecedents and consequents.
//! 3) Create a `RuleSpace` and call `defuzzify` with crisp inputs and a sampler.
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
//! use rust_fuzzylogic::rulespace::RuleSpace;
//!
//! // Variables
//! let mut temp = Variable::new(-10.0, 10.0).unwrap();
//! temp.insert_term("hot", Term::new("hot", Triangular::new(0.0, 5.0, 10.0).unwrap())).unwrap();
//! let mut fan = Variable::new(0.0, 10.0).unwrap();
//! fan.insert_term("high", Term::new("high", Triangular::new(5.0, 7.5, 10.0).unwrap())).unwrap();
//!
//! let mut vars: HashMap<String, Variable> = HashMap::new();
//! vars.insert("temp".into(), temp);
//! vars.insert("fan".into(), fan);
//!
//! // Rule: IF temp IS hot THEN fan IS high
//! let rules = vec![Rule{
//!     antecedent: Antecedent::Atom{ var: "temp".into(), term: "hot".into() },
//!     consequent: vec![Consequent{ var: "fan".into(), term: "high".into() }],
//! }];
//!
//! let sampler = UniformSampler::default();
//! let mut rs = RuleSpace::new(vars, rules).unwrap();
//! let mut input: HashMap<&str, Float> = HashMap::new();
//! input.insert("temp", 7.5);
//!
//! // One-shot aggregate + defuzzify
//! let crisp = rs.defuzzify(&input, &sampler).unwrap();
//! assert!(crisp["fan"] >= 0.0 && crisp["fan"] <= 10.0);
//! ```
use std::{borrow::Borrow, collections::HashMap, hash::Hash};

use crate::{
    aggregate::aggregation,
    defuzz::defuzzification,
    error::{self, FuzzyError},
    mamdani::Rule,
    sampler::UniformSampler,
    variable::Variable,
    Float,
};

/// Container for fuzzy variables, rules, and intermediate membership data.
///
/// Internally, `RuleSpace` stores:
/// - `vars`: the set of named input/output variables.
/// - `rules`: the rule base defining inference behavior.
/// - `agg_memberships`: aggregated membership samples per output variable from
///   the most recent call to `aggregate`/`defuzzify`.
#[deprecated(since = "0.2.0", note = "Use system::System and SystemBuilder instead")]
pub struct RuleSpace {
    vars: HashMap<String, Variable>,
    agg_memberships: HashMap<String, Vec<Float>>,
    rules: Vec<Rule>,
}

impl RuleSpace {
    /// Create a rule space with the supplied variables and rules.
    ///
    /// Errors
    /// - `FuzzyError::EmptyInput` if `vars` or `rules` is empty.
    #[deprecated(since = "0.2.0", note = "Use system::System and SystemBuilder instead")]
    pub fn new(vars: HashMap<String, Variable>, rules: Vec<Rule>) -> error::Result<Self> {
        if vars.is_empty() || rules.is_empty() {
            return Err(FuzzyError::EmptyInput);
        } else {
            return Ok(Self {
                vars: vars,
                agg_memberships: HashMap::new(),
                rules: rules,
            });
        }
    }

    /// Append additional rules to the existing rule set.
    ///
    /// Returns a mutable reference to `self` upon success.
    ///
    /// Errors
    /// - `FuzzyError::EmptyInput` if `rules` is empty.
    #[deprecated(since = "0.2.0", note = "Use system::System and SystemBuilder instead")]
    pub fn add_rules(&mut self, rules: &mut Vec<Rule>) -> error::Result<&mut Self> {
        if rules.is_empty() {
            Err(FuzzyError::EmptyInput)
        } else {
            let _ = &mut self.rules.append(rules);
            return Ok(self);
        }
    }

    /// Run the aggregation step for all rules with the provided crisp inputs.
    ///
    /// This populates `agg_memberships` with a sample vector for each output
    /// variable. Use `defuzzify` if you want crisp outputs in a single call.
    ///
    /// Type parameters
    /// - `KI`: key type for `input`, must borrow as `str`.
    ///
    /// Errors
    /// - Propagates errors from rule activation/implication and aggregation.
    #[deprecated(since = "0.2.0", note = "Use system::System and SystemBuilder instead")]
    pub fn aggregate<KI>(
        &mut self,
        input: &HashMap<KI, Float>,
        sampler: &UniformSampler,
    ) -> error::Result<()>
    where
        KI: Eq + Hash + Borrow<str>,
    {
        //let rules = std::mem::take(&mut self.rules);
        let agg_memberships = aggregation(&self.rules, input, &self.vars, sampler)?;
        self.agg_memberships = agg_memberships;

        Ok(())
    }

    /// Aggregate and then defuzzify each output variable using the supplied sampler.
    ///
    /// Combines `aggregate` and `defuzzification` to return crisp values for
    /// all output variables in a single call.
    ///
    /// Type parameters
    /// - `KI`: key type for `input`, must borrow as `str`.
    ///
    /// Errors
    /// - Propagates any error from aggregation or defuzzification.
    #[deprecated(since = "0.2.0", note = "Use system::System and SystemBuilder instead")]
    pub fn defuzzify<KI>(
        &mut self,
        input: &HashMap<KI, Float>,
        sampler: &UniformSampler,
    ) -> error::Result<HashMap<String, Float>>
    where
        KI: Eq + Hash + Borrow<str>,
    {
        let _ = self.aggregate(input, sampler)?;
        //let agg_memberships = std::mem::take(&mut self.agg_memberships);
        Ok(defuzzification(&self.agg_memberships, &self.vars)?)
    }
    //is there a nessecity?
    //pub fn consequent_keys() {}
}
